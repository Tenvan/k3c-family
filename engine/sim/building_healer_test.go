package sim

import (
	"math"
	"testing"
)

// W3.2 (B-116/AC-01, Sprint W3 AC-01): Der Heilplatz heilt Bürger und lebende Spieler im Radius, auch im Kampf, bis
// MaxHP (Q32); Werte aus data/buildings.json › healer.heal.

// healWorld: Heilplatz, ein Landstreicher am Platz und einer außerhalb, zwei Spieler am Platz, ein Gegner in der Welt.
func healWorld(t *testing.T) (w *World, healer *Site, near, far *Troop, players []*Player) {
	t.Helper()
	w = dayWorld(t)
	healer, _ = hubSiteOf(t, w, "healer")
	r := buildings["healer"].Heal.RadiusUnits
	near = spawnVagrant(w, healer.X, healer.X+r/2)
	far = spawnVagrant(w, healer.X+r+4, healer.X+r+4)
	for _, tr := range []*Troop{near, far} {
		tr.HP = 1
	}
	players = []*Player{AddPlayer(w), AddPlayer(w)}
	for _, p := range players {
		p.X, p.HP = healer.X, 1
	}
	spawnEnemy(w, "skeleton", w.HubX+200)
	return w, healer, near, far, players
}

func TestHeilplatzHeiltImRadiusAuchImKampf(t *testing.T) {
	w, healer, near, far, players := healWorld(t)
	built(healer)
	run(w, 1)
	want := 1 + buildings["healer"].Heal.HPPerSecond
	for _, hp := range []float64{near.HP, players[0].HP, players[1].HP} {
		if math.Abs(hp-want) > 0.2 {
			t.Errorf("nach 1 s HP %v, laut Daten %v", hp, want)
		}
	}
	if far.HP != 1 {
		t.Errorf("außerhalb des Radius geheilt: HP %v", far.HP)
	}
	near.HP = near.MaxHP - 1
	players[0].HP = players[0].MaxHP - 1
	run(w, 2)
	if near.HP != near.MaxHP || players[0].HP != players[0].MaxHP {
		t.Errorf("über oder nicht bis MaxHP: Bürger %v/%v, Spieler %v/%v", near.HP, near.MaxHP, players[0].HP, players[0].MaxHP)
	}
}

func TestHeilplatzOhneGebaeudeKeineHeilung(t *testing.T) {
	w, healer, near, _, players := healWorld(t)
	players[1].RespawnIn = 10 // liegt am Boden: wird nicht geheilt
	run(w, 1)
	built(healer)
	destroySite(w, healer)
	run(w, 1)
	if near.HP != 1 || players[0].HP != 1 || players[1].HP != 1 {
		t.Errorf("ungebaut oder zerstört geheilt: Bürger %v, Spieler %v, %v", near.HP, players[0].HP, players[1].HP)
	}
}
