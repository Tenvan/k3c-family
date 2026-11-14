package actors

// Figuren mit Zustand: je Figur der Simulation (Stufe, Art, ID) ein Eintrag mit Rig-Actor bzw. Sheet, Pose, Richtung
// und dem Vor-Zustand. Listen gleicht die Einträge einmal je Frame mit der Welt ab; Draw liest sie nur (bis auf den
// Pose-Puffer des Actors, siehe draw.go).

import (
	"errors"
	"fmt"
	"log/slog"
	"math"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// class unterscheidet die Listen der Welt; IDs sind nur je Liste und Stufe eindeutig.
type class uint8

const (
	classPlayer class = iota
	classTroop
	classEnemy
)

// key ist die stabile Kennung einer Figur über die Frames.
type key struct {
	stage int
	class class
	id    int
}

// Höhe des Turm-Platzes (Browser-Client: 270 Spiel-Pixel) und Abstand, ab dem eine Truppe auf ihrem Turm steht.
const (
	towerLift  = 270.0 / unitPx
	onTowerMax = 0.3
	workNear   = 0.5  // Bauer arbeitet, wenn er so nah am Ziel steht (Units)
	fleeAlpha  = 0.7  // fliehende Gegner (Browser: worldRenderer.ts)
	stealthMax = 0.35 // unsichtbarer Monarch, Smoke Bomb (Browser: worldRenderer.ts, B-577)
)

// defaultOnce ist die Länge einer Einmal-Animation, wenn weder Rig noch Sheet sie nennen (ohne Grafik, Tests).
const defaultOnce = 0.5

// figure ist eine Figur mit Zustand.
type figure struct {
	look   Look
	actor  *art.Actor // nil: Sheet (look.Sheet) oder kaputt
	broken bool       // Rig fehlt: zeichnet nichts (gemeldet beim Anlegen)
	pose   Pose
	left   bool
	shift  float64
	lift   float64 // Fußpunkt über dem Boden in Units
	alpha  float64
	ui     float64 // Deckkraft der Anzeigen (Balken, Punkt, Gold): Container-Alpha des Browsers, ohne Sprite-Alpha
	gen    uint64

	known       bool // Vor-Zustand gesetzt
	x, cd, dead float64

	posed   bool    // der Actor hat eine Pose (Umhang hängt)
	dirty   bool    // Pose seit der letzten Rechnung gewechselt
	posedAt float64 // Darstellungszeit der Pose
}

type warnKey struct{ what, name string }

// Listen gleicht die Figuren mit den Welten des Frames ab: neue anlegen, Zustand → Pose, verschwundene entfernen,
// dann die Pose aller Rig-Figuren im Takt fortschreiben (pose.go).
func (d *drawer) Listen(f *view.Frame) {
	if d.cat == nil {
		return
	}
	if d.started && !f.Paused {
		d.now += math.Max(f.Clock-d.last, 0)
	}
	d.last, d.started = f.Clock, true
	d.gen++
	for i := range f.Stages {
		st := &f.Stages[i]
		if st.World == nil {
			continue
		}
		d.listenPlayers(st)
		for _, t := range st.World.Troops {
			d.listenTroop(st.World, st.Index, t)
		}
		for _, e := range st.World.Enemies {
			d.listenEnemy(st.Index, e)
		}
	}
	for k, fg := range d.figs { // Reihenfolge egal: entfernt nur
		if fg.gen != d.gen {
			delete(d.figs, k)
		}
	}
	d.advanceAll()
}

func (d *drawer) listenPlayers(st *view.Stage) {
	for _, p := range st.World.Players {
		if p == nil {
			continue
		}
		fg := d.figure(key{st.Index, classPlayer, p.ID}, d.cat.Player(p.Index), true)
		prev := MonarchFrame{Known: fg.known, RespawnIn: fg.dead}
		cur := MonarchFrame{Known: true, RespawnIn: p.RespawnIn, VX: p.VX}
		d.apply(fg, MonarchWant(prev, cur, d.cat.MountRef, fg.hasGait))
		if p.Facing != 0 {
			fg.left = p.Facing < 0
		}
		fg.known, fg.x, fg.dead, fg.lift, fg.alpha = true, p.X, p.RespawnIn, 0, MonarchAlpha(p.StealthFor)
		fg.ui = fg.alpha
	}
}

// MonarchAlpha liefert die Deckkraft des Monarchen: fast durchsichtig, solange die Smoke Bomb wirkt.
func MonarchAlpha(stealthFor float64) float64 {
	if stealthFor > 0 {
		return stealthMax
	}
	return 1
}

func (d *drawer) listenTroop(w *sim.World, stage int, t *sim.Troop) {
	look, ok := d.cat.Troops[t.Kind]
	if !ok {
		d.warn(warnKey{"truppe", t.Kind}, errors.New("unbekannte Truppen-Art"))
		return
	}
	fg := d.figure(key{stage, classTroop, t.ID}, look, false)
	m := Motion{Known: fg.known, PrevX: fg.x, X: t.X, PrevCD: fg.cd, CD: t.Cooldown}
	want, dir := TroopWant(t.Kind, m, working(t), nearestEnemyDir(w, t.X))
	d.apply(fg, want)
	if dir != 0 {
		fg.left = dir < 0
	}
	fg.known, fg.x, fg.cd, fg.alpha = true, t.X, t.Cooldown, look.Alpha
	fg.lift = look.Lift
	if onTower(w, t) {
		fg.lift += towerLift
	}
}

func (d *drawer) listenEnemy(stage int, e *sim.Enemy) {
	look, ok := d.cat.Enemy(e.Kind, e.Boss)
	if !ok {
		d.warn(warnKey{"gegner", e.Kind}, errors.New("unbekannte Gegner-Art"))
		return
	}
	fg := d.figure(key{stage, classEnemy, e.ID}, look, false)
	want, dir := FigureWant(Motion{Known: fg.known, PrevX: fg.x, X: e.X, PrevCD: fg.cd, CD: e.Cooldown, AttackCooldown: true})
	d.apply(fg, want)
	if dir != 0 {
		fg.left = dir < 0
	}
	fg.known, fg.x, fg.cd, fg.lift, fg.alpha = true, e.X, e.Cooldown, look.Lift, look.Alpha
	fg.ui = 1
	if e.Fleeing {
		fg.alpha *= fleeAlpha
		fg.ui = fleeAlpha
	}
}

// TroopWant wählt die Animation einer Truppe: Kämpfer schlagen nach Cooldown, Landstreicher und Bauern nutzen ihn
// fürs Warten; ein arbeitender Bauer schlägt in Schleife, ein stehender Bogenschütze schaut zum nächsten Gegner.
func TroopWant(kind string, m Motion, working bool, enemyDir int) (Want, int) {
	m.AttackCooldown = kind != "vagrant" && kind != "peasant"
	want, dir := FigureWant(m)
	if dir == 0 && (kind == "archer" || kind == "eliteArcher") {
		dir = enemyDir
	}
	if kind == "peasant" && working && want.Anim == animIdle {
		want = Want{Anim: animAttack}
	}
	return want, dir
}

// working: Bauer bei der Arbeit (Holz hacken, bauen) am Ziel.
func working(t *sim.Troop) bool {
	return t.Job != nil && (t.Job.Type == "gather" || t.Job.Type == "build") && math.Abs(t.TargetX-t.X) < workNear
}

// nearestEnemyDir: Richtung zum nächsten Gegner (0 = keiner da).
func nearestEnemyDir(w *sim.World, x float64) int {
	best, dist := 0, math.Inf(1)
	for _, e := range w.Enemies {
		if d := math.Abs(e.X - x); d < dist {
			best, dist = int(math.Copysign(1, e.X-x)), d
		}
	}
	return best
}

// onTower: Truppe steht auf ihrem Turm (viewRules.ts › isOnTower).
func onTower(w *sim.World, t *sim.Troop) bool {
	if t.TowerID == nil {
		return false
	}
	for _, s := range w.Sites {
		if s.ID == *t.TowerID {
			return math.Abs(s.X-t.X) < onTowerMax
		}
	}
	return false
}

// figure liefert die Figur zu k und legt sie an, wenn sie neu ist oder sich ihr Look geändert hat (Landstreicher
// wird Bauer). mounted: Monarch auf dem Reittier des Katalogs.
func (d *drawer) figure(k key, look Look, mounted bool) *figure {
	fg := d.figs[k]
	if fg == nil || fg.look != look {
		fg = &figure{look: look, shift: Shift(k.id), alpha: look.Alpha, ui: 1}
		if !look.Sheet {
			fg.actor, fg.broken = d.newActor(look, mounted)
		}
		d.figs[k] = fg
	}
	fg.gen = d.gen
	return fg
}

// newActor legt den Rig-Actor an: Monarch beritten, Vierbeiner als Reittier ohne Reiter, sonst figur zu Fuß. Fehlt
// das Rig, steht es einmal je Asset im Log und die Figur bleibt unsichtbar (keine stille Ersatz-Grafik).
// Ohne Bibliothek (lib nil) gibt es keinen Actor und keine Meldung.
func (d *drawer) newActor(look Look, mounted bool) (*art.Actor, bool) {
	if d.lib == nil {
		return nil, false // ohne Grafik (Tests): nur Posen
	}
	var a *art.Actor
	var err error
	switch {
	case mounted:
		a, err = d.lib.NewActor(look.Asset, d.cat.Mount, look.Variant)
	default:
		a, err = d.lib.NewActor(look.Asset, "", look.Variant)
		if err != nil || a.Rider.Kind == "vierbeiner" {
			if q, qerr := d.lib.NewActor("", look.Asset, look.Variant); qerr == nil {
				a, err = q, nil
			}
		}
	}
	if err != nil {
		d.warn(warnKey{"rig", look.Asset}, err)
		return nil, true
	}
	return a, false
}

// apply setzt den Wunsch w als Pose; kennt die Figur die Animation nicht, zeigt sie die Ruhe-Pose (Meldung einmal).
func (d *drawer) apply(fg *figure, w Want) {
	p, changed := Next(fg.pose, w, d.now)
	if !changed {
		return
	}
	if err := d.setState(fg, p); err != nil {
		d.warn(warnKey{fg.look.Asset, p.Anim}, err)
		p = Pose{Anim: animIdle, Start: d.now}
		if fg.actor != nil && fg.actor.Mount != nil && fg.actor.Rider != nil {
			p = Pose{Anim: gaitStand, Gait: gaitStand, Start: d.now}
		}
		_ = d.setState(fg, p) // idle bzw. stand hat jedes Rig und Sheet
	} else if w.Once {
		p.Dur = d.duration(fg, p)
	}
	fg.pose, fg.dirty = p, true
}

// setState stellt die Pose am Actor ein bzw. prüft das Sheet.
func (d *drawer) setState(fg *figure, p Pose) error {
	a := fg.actor
	switch {
	case a == nil && fg.look.Sheet:
		if d.lib != nil && d.lib.Sheet(fg.look.Asset, p.Anim) == nil {
			return fmt.Errorf("sheet %s hat keine Animation „%s“", fg.look.Asset, p.Anim)
		}
	case a == nil:
	case p.Gait != "":
		if !fg.hasGait(p.Gait) {
			return fmt.Errorf("reittier %s hat keine Gangart „%s“", a.Mount.ID, p.Gait)
		}
		a.State = art.State{Name: p.Anim, Gait: p.Gait}
	case a.Rider != nil:
		if _, err := a.Rider.FigureFreq(p.Anim); err != nil {
			return err
		}
		a.State = art.State{Name: p.Anim}
	default:
		return a.Play(p.Anim)
	}
	return nil
}

// duration: Länge der eingestellten Einmal-Animation (ein Zyklus des Rigs bzw. alle Bilder des Sheets).
func (d *drawer) duration(fg *figure, p Pose) float64 {
	if a := fg.actor; a != nil {
		if f, err := a.Freq(); err == nil && f > 0 {
			return 1 / f
		}
	}
	if fg.look.Sheet && d.lib != nil {
		if s := d.lib.Sheet(fg.look.Asset, p.Anim); s != nil && s.FPS > 0 {
			return float64(len(s.Frames)) / s.FPS
		}
	}
	return defaultOnce
}

// hasGait meldet, ob das Reittier der Figur die Gangart hat (ohne Rig: alle).
func (fg *figure) hasGait(g string) bool {
	if fg.actor == nil || fg.actor.Mount == nil {
		return true
	}
	a, ok := fg.actor.Mount.Animations[g]
	return ok && a.Gait != ""
}

// warn meldet einen Fehler einmal je Schlüssel.
func (d *drawer) warn(k warnKey, err error) {
	if d.warned[k] {
		return
	}
	d.warned[k] = true
	slog.Error("❌ Figur ohne Grafik, Ruhe-Pose bzw. unsichtbar", "art", k.what, "name", k.name, "err", err)
}
