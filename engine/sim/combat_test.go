package sim

import (
	"io/fs"
	"strings"
	"testing"

	"k3c/data"
)

// Aktiver Angriff (B-932, B-933, docs/rules/kampf.md § 3): Gruppen mit Schild und Auslöser, General mit Banner; immer mit
// 2 Spielern über CreateIsland.

// combatIsland baut eine Portal-Insel mit der Variante mode und je einer gebauten Mauer als Tor links und rechts.
func combatIsland(t *testing.T, mode string) (*World, float64, float64) {
	t.Helper()
	_, w := portalIsland(t)
	w.vals = defaultValues()
	w.vals.Combat = CombatSettings{Mode: mode, GroupSize: 3}
	w.HubLevel = w.values().Economy.Combat.ShieldMinHubLevel
	for _, side := range []float64{-1, 1} {
		s := emptySite(w, "wall", w.HubX+side*10)
		s.State = "built"
		w.Sites = append(w.Sites, s)
	}
	for _, p := range w.Players {
		p.Gold, p.PayCooldown = 40, 0
	}
	w.Portals = []float64{w.HubX - 40, w.HubX + 40} // nah genug für kurze Läufe
	return w, w.HubX - 10, w.HubX + 10
}

// soldiers stellt n Krieger auf die rechte Seite am Tor.
func spawnSoldiers(w *World, n int) []*Troop {
	var out []*Troop
	for range n {
		tr := spawnVagrant(w, w.HubX, w.HubX+8)
		makeFighter(w, tr, "warrior")
		tr.AnchorX = w.HubX + 1
		out = append(out, tr)
	}
	return out
}

// combatPay lässt Spieler i an x stehen und so lange zahlen, bis ticks Takte vorbei sind.
func combatPay(w *World, i int, x float64, ticks int) {
	for range ticks {
		w.Players[i].X, w.Players[i].VX = x, 0
		cmds := make([]PlayerCommand, len(w.Players))
		cmds[i].Pay = true
		Step(w, cmds, dt)
	}
}

func firstLeader(w *World) *Troop {
	for _, t := range w.Troops {
		if t.Leader {
			return t
		}
	}
	return nil
}

func spotFor(w *World, kind string, side int) CombatSpot {
	for _, s := range CombatSpots(w) {
		if s.Kind == kind && s.Side == side {
			return s
		}
	}
	return CombatSpot{}
}

func TestCombatWeltFeldStandardUndPruefung(t *testing.T) {
	std := StandardWorld()
	if std.Combat.Mode != CombatGroups || std.Combat.GroupSize != 5 {
		t.Fatalf("Standard-Welt: %+v", std.Combat)
	}
	for body, want := range map[string]string{
		`"combat": {"mode": "general"}`:                "",
		`"combat": {"mode": "gruppe"}`:                 "combat.mode",
		`"combat": {"groupSize": 0}`:                   "combat.groupSize",
		`"combat": {"modus": "groups"}`:                "modus",
		`"combat": {"mode": "groups", "groupSize": 7}`: "",
	} {
		w, err := ParseWorld([]byte(minimalWorld(body)), "t")
		if want == "" && err != nil {
			t.Errorf("%s: %v", body, err)
		}
		if want != "" && (err == nil || !strings.Contains(err.Error(), want)) {
			t.Errorf("%s: Fehler %v, erwartet %q", body, err, want)
		}
		if body == `"combat": {"mode": "groups", "groupSize": 7}` && w.Combat.GroupSize != 7 {
			t.Errorf("groupSize %d", w.Combat.GroupSize)
		}
	}
	w, _, _ := combatIsland(t, "")
	if combatMode(w) != CombatGroups || groupSize(&World{}) != DefaultGroupSize {
		t.Error("Standard ohne Feld ist groups/5")
	}
}

// minimalWorld: eine gültige Welt-Datei mit dem zusätzlichen Feld extra.
func minimalWorld(extra string) string {
	raw, _ := fs.ReadFile(data.Files, "worlds/standard.json")
	std := strings.Replace(string(raw), `"id": "standard"`, `"id": "t"`, 1)
	return strings.Replace(std, `"combat": { "mode": "groups", "groupSize": 5 },`, extra+",", 1)
}

func TestSchildMachtZumGruppenfuehrerUndGruppeFuellt(t *testing.T) {
	w, _, right := combatIsland(t, CombatGroups)
	spawnSoldiers(w, 5)
	shield := spotFor(w, "shield", 0)
	combatPay(w, 0, shield.X, 76)
	if w.Players[0].Gold != 40-10 {
		t.Fatalf("Schild kostet 10 Gold, Gold %d", w.Players[0].Gold)
	}
	run(w, 10)
	l := firstLeader(w)
	if l == nil {
		t.Fatal("kein Führer nach dem Schild")
	}
	if n := groupCount(w, l); n != 3 {
		t.Errorf("Gruppe hat %d Soldaten, erwartet groupSize 3", n)
	}
	if trigger := spotFor(w, "trigger", 1); trigger.X <= right {
		t.Errorf("Auslöser liegt nicht außerhalb vor dem Tor: %v", trigger)
	}
}

func TestSchildNurMitFreiemSoldaten(t *testing.T) {
	w, _, _ := combatIsland(t, CombatGroups)
	combatPay(w, 0, spotFor(w, "shield", 0).X, 76)
	if w.Players[0].Gold != 40 || firstLeader(w) != nil {
		t.Errorf("ohne Soldat darf das Schild nichts kosten: Gold %d", w.Players[0].Gold)
	}
}

// groupReady baut einen Führer mit voller Gruppe am rechten Tor.
func groupReady(t *testing.T) (*World, *Troop) {
	t.Helper()
	w, _, _ := combatIsland(t, CombatGroups)
	spawnSoldiers(w, 5)
	combatPay(w, 0, spotFor(w, "shield", 0).X, 76)
	run(w, 8)
	l := firstLeader(w)
	if l == nil || groupCount(w, l) != 3 {
		t.Fatal("Gruppe nicht bereit")
	}
	return w, l
}

func TestAusloeserSchicktGruppenZumPortalUndZurueck(t *testing.T) {
	w, l := groupReady(t)
	right := 1
	w.PortalHP[right] = 40 // schnell zerstört
	before := w.Players[1].Gold
	combatPay(w, 1, spotFor(w, "trigger", 1).X, 20)
	if w.Players[1].Gold != before-2 || l.sortie != 1 {
		t.Fatalf("Auslöser: Gold %d -> %d, sortie %v", before, w.Players[1].Gold, l.sortie)
	}
	x0 := w.Portals[right]
	for range 30 * 90 {
		Step(w, nil, dt)
		if len(w.Portals) < 2 {
			break
		}
	}
	if len(w.Portals) != 1 || len(w.PortalsDown) != 1 || w.PortalsDown[0] != x0 {
		t.Fatalf("Portal nicht zerstört: %v %v", w.Portals, w.PortalHP)
	}
	run(w, 30)
	if l.sortie != 0 || l.X > w.HubX+10 {
		t.Errorf("Gruppe nicht zurück am Tor: sortie %v, x %v", l.sortie, l.X)
	}
}

func TestAusloeserNurMitGruppeDerSeite(t *testing.T) {
	w, _, _ := combatIsland(t, CombatGroups)
	combatPay(w, 1, spotFor(w, "trigger", 1).X, 40)
	if w.Players[1].Gold != 40 {
		t.Errorf("Auslöser ohne Gruppe nahm Gold: %d", w.Players[1].Gold)
	}
}

func TestFuehrerFaelltNurAlsLetzterDerGruppe(t *testing.T) {
	w, l := groupReady(t)
	for _, tr := range freeWarriors(w) { // kein Nachrücken in die Gruppe
		tr.Kind, tr.AnchorX = "peasant", w.HubX
	}
	l.HP = 0
	run(w, 0.2)
	if !l.Leader {
		t.Fatal("Führer fiel mit stehender Gruppe")
	}
	for _, tr := range w.Troops {
		if tr.Group == l.ID && tr != l {
			tr.HP = 0
		}
	}
	run(w, 0.2)
	l.HP = 0
	run(w, 0.2)
	if l.Leader || l.Kind != "peasant" || l.Group != 0 {
		t.Errorf("letzter Führer blieb Führer: %+v", l)
	}
}

// takeBannerAt: Spieler i nimmt das Banner am rechten Tor.
func takeBannerAt(w *World, i int) {
	combatPay(w, i, spotFor(w, "banner", 1).X, 20)
}

func TestBannerTruppFolgtUndKehrtZurueck(t *testing.T) {
	w, wallL, wallR := combatIsland(t, CombatGeneral)
	_ = wallL
	for _, tr := range spawnSoldiers(w, 7) {
		_ = tr
	}
	for range 7 {
		a := spawnVagrant(w, w.HubX, w.HubX+8)
		makeFighter(w, a, "archer")
		a.AnchorX = w.HubX + 1
	}
	takeBannerAt(w, 0)
	p := w.Players[0]
	if p.Banner != 1 || w.Players[1].Banner != 0 {
		t.Fatalf("Banner: %d / %d", p.Banner, w.Players[1].Banner)
	}
	run(w, 12)
	s, a := squadSize(w, p)
	if s != 5 || a != 5 {
		t.Fatalf("Trupp %d Soldaten, %d Schützen, erwartet 5 + 5", s, a)
	}
	p.X = wallR + 3 // hinter das Tor hinaus: Trupp folgt
	run(w, 1)
	if p.Banner == 0 {
		t.Fatal("Banner zu früh zurückgestellt")
	}
	p.X = wallR - 5 // zurück hinter das äußerste Tor
	run(w, 1)
	left := 0
	for _, tr := range w.Troops {
		if tr.Group != 0 {
			left++
		}
	}
	if p.Banner != 0 || left != 0 {
		t.Errorf("Banner nicht zurückgestellt: %d, %d Mitglieder", p.Banner, left)
	}
}

func TestBannerJeMonarchUndNachbesetzung(t *testing.T) {
	w, _, _ := combatIsland(t, CombatGeneral)
	for range 12 {
		makeFighter(w, spawnVagrant(w, w.HubX, w.HubX+8), "warrior")
	}
	for _, tr := range w.Troops {
		tr.AnchorX = w.HubX + 1
	}
	takeBannerAt(w, 0)
	takeBannerAt(w, 1)
	run(w, 15)
	if w.Players[0].Banner == 0 || w.Players[1].Banner == 0 {
		t.Fatal("jeder Monarch nimmt sein eigenes Banner")
	}
	var fallen *Troop
	for _, tr := range w.Troops {
		if tr.Group == w.Players[0].ID {
			fallen = tr
			break
		}
	}
	fallen.HP = 0
	run(w, 1)
	n := 0
	for _, tr := range w.Troops {
		if tr.Group == w.Players[0].ID {
			n++
		}
	}
	if fallen.Group != 0 || n != 5 {
		t.Errorf("Ausfall nicht ersetzt: %d Soldaten im Trupp", n)
	}
}

func TestTruppImAngriffBeschaedigtPortal(t *testing.T) {
	w, _, _ := combatIsland(t, CombatGeneral)
	spawnSoldiers(w, 5)
	takeBannerAt(w, 0)
	p := w.Players[0]
	right := 1
	run(w, 3)
	for range 30 * 60 {
		p.X, p.VX = w.Portals[right]-4, 0
		Step(w, nil, dt)
		if w.PortalHP[right] < w.PortalMaxHP {
			return
		}
	}
	t.Fatal("der Trupp beschädigte das Portal nicht")
}

func TestKroneFaelltBannerWirdZurueckgestellt(t *testing.T) {
	w, _, _ := combatIsland(t, CombatGeneral)
	takeBannerAt(w, 0)
	w.Players[0].Crown = &Crown{}
	run(w, 0.2)
	if w.Players[0].Banner != 0 {
		t.Error("Banner blieb bei fehlender Krone")
	}
}

// squadSize zählt Soldaten und Schützen im Trupp des Monarchen p.
func squadSize(w *World, p *Player) (soldiers, archers int) {
	for _, tr := range w.Troops {
		switch {
		case tr.Group != p.ID:
		case isSoldier(tr):
			soldiers++
		default:
			archers++
		}
	}
	return soldiers, archers
}
