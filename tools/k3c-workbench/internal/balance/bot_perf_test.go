package balance

import (
	"testing"

	"k3c/engine/sim"
	"k3c/tools/k3c-workbench/internal/enginetools"
)

// B-654 (BAL9.2): wantSet.wanted und nodeFor liefern Tick für Tick dasselbe wie die Fassung vor B-654 (wantedRef,
// nodeForRef). Der Lauf ist der Meilenstein-Lauf (coop2, Welt standard) über die ersten Tage, geprüft wird jeden
// fünften Tick in jeder Stufe mit Monarch, bevor die Bots ihre Befehle geben. Byte-Gleichheit des ganzen Laufs: Ergebnis BAL9.2.
func TestBotSchnellGleich(t *testing.T) {
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
	sc := Scenario{Seed: "k3c-dev", Players: 2, Bot: "coop2", Days: 3, World: "standard"}
	isl, err := newIsland(sc)
	if err != nil {
		t.Fatal(err)
	}
	inner, _ := BotByName(sc.Bot)
	bot := withBossFight(withDescent(inner, isl, deepestOf(isl)), isl, deepestOf(isl))
	needs := []sim.Cost{{Wood: 150}, {Stone: 40, Iron: 10}, {Wood: 1 << 20, Stone: 1 << 20, Copper: 1 << 20, Iron: 1 << 20, Crystal: 1 << 20}}
	cmds := make([]sim.PlayerCommand, sc.Players)
	checks := 0
	for tick := 1; tick <= sc.Days*maxTicksPerDay && isl.Stages[0].Cycle.Day <= sc.Days; tick++ {
		for _, w := range isl.Stages {
			if len(w.Players) > 0 && tick%5 == 0 {
				checks += sameAsRef(t, w, needs, tick)
			}
			for _, p := range w.Players {
				cmds[p.Index] = bot(w, p)
			}
		}
		if t.Failed() {
			return
		}
		sim.StepIsland(isl, cmds, 1.0/enginetools.TickHz)
		afterStep(isl, sc, tick, enginetools.TickHz)
	}
	if checks == 0 {
		t.Fatal("nichts verglichen")
	}
}

// sameAsRef vergleicht alle unbezahlten Plätze und jeden Bedarf für jeden Monarchen der Stufe; liefert die Zahl der Vergleiche.
func sameAsRef(t *testing.T, w *sim.World, needs []sim.Cost, tick int) int {
	t.Helper()
	n := 0
	want := wantSet{w: w}
	for _, s := range w.Sites {
		if s.State == "unpaid" {
			if got, ref := want.wanted(s), wantedRef(w, s); got != ref {
				t.Errorf("Tick %d: wanted(%s @%.1f) %v, vorher %v", tick, s.Kind, s.X, got, ref)
			}
			n++
		}
	}
	for _, p := range w.Players {
		for _, need := range needs {
			if got, ref := nodeFor(w, p, need), nodeForRef(w, p, need); got != ref {
				t.Errorf("Tick %d: nodeFor(%+v) %v, vorher %v", tick, need, got, ref)
			}
			n++
		}
	}
	return n
}

// wantedRef und nodeForRef sind die Fassungen vor B-654 (Vergleichsbasis).
func wantedRef(w *sim.World, s *sim.Site) bool {
	z, ok := sim.SiteZone(w, s)
	if !ok {
		return true
	}
	for _, o := range w.Sites {
		if o != s && o.Kind == s.Kind && o.State != "unpaid" && (o.X < w.HubX) == (s.X < w.HubX) {
			if oz, ok := sim.SiteZone(w, o); ok && oz == z {
				return false
			}
		}
	}
	return true
}

func nodeForRef(w *sim.World, p *sim.Player, need sim.Cost) *sim.ResourceNode {
	pending := map[string]int{}
	for _, n := range w.Nodes {
		if np, ok := prices.nodes[n.Kind]; ok && n.Marked {
			pending[np.Resource] += np.Amount
		}
	}
	blocked := payableSitesX(w)
	var best *sim.ResourceNode
	for _, n := range w.Nodes {
		np, ok := prices.nodes[n.Kind]
		if !ok || n.Marked || p.Gold < np.MarkCost-n.PaidGold || nearAny(n.X, blocked) || nearLair(w, n.X) ||
			amountOf(*w.Stock, np.Resource)+pending[np.Resource] >= costOf(need, np.Resource) {
			continue
		}
		if best == nil || closer(p.X, n.X, best.X) {
			best = n
		}
	}
	return best
}
