package sim

import (
	"fmt"
	"math"
	"slices"
	"testing"

	"k3c/engine/core"
)

// Spike RS1.6 (B-725): `collectCoins` als Rust-Kern gegen das optimierte Go, bitgleich (Nehmer, Gold) und gemessen.

// coinInput ist die Eingabe eines Aufrufs, flach (so liefert sie Go an den Rust-Kern).
type coinInput struct {
	cx, cu, px    []float64
	cb, pid, gold []int32
	alive         []uint8
	out           []int32
	time, reach   float64
	purse         int32
}

func packCoins(w *World) *coinInput {
	in := &coinInput{time: w.Time, reach: w.values().Economy.PickupRangeUnits, purse: int32(purseMax(w))}
	for _, c := range w.Coins {
		in.cx = append(in.cx, c.X)
		in.cu = append(in.cu, c.BlockedUntil)
		id := int32(-1)
		if c.BlockedPlayerID != nil {
			id = int32(*c.BlockedPlayerID)
		}
		in.cb = append(in.cb, id)
	}
	for _, p := range w.Players {
		in.px = append(in.px, p.X)
		in.pid = append(in.pid, int32(p.ID))
		in.gold = append(in.gold, int32(p.Gold))
		a := uint8(0)
		if isAlive(p) {
			a = 1
		}
		in.alive = append(in.alive, a)
	}
	in.out = make([]int32, len(w.Coins))
	return in
}

func (in *coinInput) call() *core.CoinsCall {
	c := &core.CoinsCall{NCoins: uintptr(len(in.cx)), NPlayers: uintptr(len(in.px)), Time: in.time, Reach: in.reach, Purse: in.purse}
	if len(in.cx) > 0 {
		c.CoinX, c.CoinBlockedID, c.CoinBlockedUntil, c.OutTaker = &in.cx[0], &in.cb[0], &in.cu[0], &in.out[0]
	}
	if len(in.px) > 0 {
		c.PlayerX, c.PlayerID, c.PlayerGold, c.PlayerAlive = &in.px[0], &in.pid[0], &in.gold[0], &in.alive[0]
	}
	return c
}

// coinWorld liefert eine Stufe nach Spielzeit mit vielen Münzen (n > 0: zusätzlich n synthetische) und die Spielerzahl.
func coinWorld(tb testing.TB, extra int) *World {
	tb.Helper()
	isl, cmds := benchIsland(tb)
	for range 6000 {
		StepIsland(isl, cmds, dt)
	}
	w := isl.Stages[0]
	for i := range extra {
		x := float64(i%400) * 0.5
		var blocked *int
		if i%7 == 0 {
			b := w.Players[i%len(w.Players)].ID
			blocked = &b
		}
		w.Coins = append(w.Coins, &Coin{ID: 9000 + i, X: x, BlockedPlayerID: blocked, BlockedUntil: w.Time + float64(i%5)})
	}
	for i, p := range w.Players {
		p.X = float64(i) * 37
		p.Gold = 0
	}
	return w
}

// goCollect rechnet collectCoins auf dem Zustand, liefert die Nehmer in Münz-Reihenfolge (aus den coinPickup-Ereignissen)
// und das Gold je Spieler und stellt den Zustand wieder her.
func goCollect(w *World) (takers []int, gold []int32) {
	coins, golds, collected := slices.Clone(w.Coins), make([]int, len(w.Players)), 0
	if w.island != nil {
		collected = w.island.GoldCollected
	}
	for i, p := range w.Players {
		golds[i] = p.Gold
	}
	events := len(w.Events)
	collectCoins(w)
	for _, e := range w.Events[events:] {
		if e["type"] == "coinPickup" {
			takers = append(takers, e["player"].(int))
		}
	}
	gold = make([]int32, len(w.Players))
	for i, p := range w.Players {
		gold[i] = int32(p.Gold)
		p.Gold = golds[i]
	}
	w.Coins, w.Events = coins, w.Events[:events]
	if w.island != nil {
		w.island.GoldCollected = collected
	}
	return takers, gold
}

func TestRustCollectCoinsBitgleich(t *testing.T) {
	lib, err := core.Find()
	if err != nil {
		t.Skip("Rust-Kern nicht gebaut (task rust:build): ", err)
	}
	call, err := lib.CollectCoins()
	if err != nil {
		t.Skip(err)
	}
	for _, extra := range []int{0, 300, 3000} {
		w := coinWorld(t, extra)
		in := packCoins(w)
		wantTaker, wantGold := goCollect(w)
		call(in.call())
		for i := range wantGold {
			if in.gold[i] != wantGold[i] {
				t.Fatalf("extra %d: Gold Spieler %d: Rust %d, Go %d", extra, i, in.gold[i], wantGold[i])
			}
		}
		var gotTaker []int
		for _, v := range in.out {
			if v >= 0 {
				gotTaker = append(gotTaker, w.Players[v].Index)
			}
		}
		if !slices.Equal(gotTaker, wantTaker) {
			t.Fatalf("extra %d: Nehmer Rust %v, Go %v", extra, gotTaker, wantTaker)
		}
		t.Logf("extra %d: %d Münzen, %d Spieler, genommen: %d", extra, len(w.Coins), len(w.Players), countTaken(in.out))
	}
}

func countTaken(out []int32) int {
	n := 0
	for _, v := range out {
		if v >= 0 {
			n++
		}
	}
	return n
}

func BenchmarkCollectCoins(b *testing.B) {
	lib, err := core.Find()
	if err != nil {
		b.Skip(err)
	}
	call, err := lib.CollectCoins()
	if err != nil {
		b.Skip(err)
	}
	for _, extra := range []int{0, 300, 3000} {
		w := coinWorld(b, extra)
		name := fmt.Sprintf("%dMuenzen", len(w.Coins))
		// Go: dieselbe Arbeit auf Kopien; Münzen und Gold werden je Durchlauf zurückgesetzt (Kosten in beiden Fällen gleich klein).
		b.Run(name+"/Go", func(b *testing.B) {
			orig := slices.Clone(w.Coins)
			events := len(w.Events)
			for b.Loop() {
				w.Coins = append(w.Coins[:0], orig...)
				for _, p := range w.Players {
					p.Gold = 0
				}
				w.Events = w.Events[:events]
				collectCoins(w)
			}
		})
		b.Run(name+"/RustFlach", func(b *testing.B) { // Eingabe einmal gepackt: reiner Aufruf + Rechnung
			in := packCoins(w)
			gold0 := slices.Clone(in.gold)
			c := in.call()
			for b.Loop() {
				copy(in.gold, gold0)
				call(c)
			}
		})
		b.Run(name+"/RustMitPacken", func(b *testing.B) { // je Aufruf aus den Go-Strukturen packen, Ergebnis zurückschreiben
			for b.Loop() {
				in := packCoins(w)
				call(in.call())
				_ = math.Abs
			}
		})
	}
}
