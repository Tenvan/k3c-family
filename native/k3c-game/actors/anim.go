package actors

// Zustand → Animation als reine Funktionen (wie src/scenes/render/monarchAnim.ts, mountGait.ts und animate in
// worldRenderer.ts): welche Animation bzw. Gangart eine Figur zeigt, wann sie beginnt und wie lange eine
// Einmal-Animation läuft. Ohne Grafik, ohne Zustand außerhalb der Argumente.

import "math"

// Animationen der Rigs und Sheets (Namen wie in den Papier-Specs) und Gangarten der Reittiere.
const (
	animIdle    = "idle"
	animRun     = "lauf"
	animAttack  = "angriff"
	animDeath   = "death"
	animRespawn = "respawn"

	gaitStand  = "stand"
	gaitWalk   = "schritt"
	gaitTrot   = "trab"
	gaitGallop = "galopp"
)

// Schwellen der Gangart als Anteil des Referenz-Tempos (mountGait.ts) und Ruhe-Grenze in Units/s.
const (
	trotFrom   = 0.45
	gallopFrom = 0.8
	moveEps    = 0.05
)

// movingEps: Weg je Takt in Units, ab dem eine Figur läuft (MOVING_PX = 0,5 Spiel-Pixel im Browser-Client).
const movingEps = 0.5 / unitPx

// Pose ist die gezeigte Animation einer Figur: Name (zu Fuß) bzw. Gangart (beritten, Vierbeiner), Beginn und Länge.
type Pose struct {
	Anim  string  // Animation der Figur bzw. bei Gait der Name der Gangart
	Gait  string  // Gangart des Reittiers; leer = zu Fuß bzw. Vierbeiner nach Anim
	Start float64 // Zeit (Sekunden der Darstellung) des Beginns
	Dur   float64 // Länge einer Einmal-Animation in Sekunden; 0 = Schleife
	Hold  bool    // Einmal-Animation hält am Ende (Tod)
}

// Want ist der Wunsch des Zustands: Animation, Einmal-Animation und ob sie neu beginnt (neuer Schlag).
type Want struct {
	Anim, Gait string
	Once, Hold bool
	Restart    bool
}

// Running meldet, ob eine Einmal-Animation (ohne Halten) zur Zeit now noch läuft.
func (p Pose) Running(now float64) bool { return p.Dur > 0 && !p.Hold && now-p.Start < p.Dur }

// Next liefert die Pose nach dem Wunsch w zur Zeit now und ob sie neu beginnt. Dieselbe Animation läuft ohne
// Restart weiter; eine laufende Einmal-Animation (Schlag, Respawn) geht einer Schleife vor; Dur setzt der Aufrufer.
func Next(cur Pose, w Want, now float64) (Pose, bool) {
	if cur.Anim == w.Anim && cur.Gait == w.Gait && !w.Restart {
		return cur, false
	}
	if !w.Once && cur.Running(now) {
		return cur, false
	}
	return Pose{Anim: w.Anim, Gait: w.Gait, Start: now, Hold: w.Hold}, true
}

// AnimTime liefert die Zeit in der Animation: Schleifen mit Versatz shift (Figuren derselben Art laufen nicht im
// Gleichschritt), Einmal-Animationen ab 0, haltende bleiben kurz vor dem Ende stehen.
func AnimTime(p Pose, now, shift float64) float64 {
	t := math.Max(now-p.Start, 0)
	switch {
	case p.Dur == 0:
		return t + shift
	case p.Hold:
		return math.Min(t, p.Dur*0.999)
	}
	return t
}

// SheetFrame liefert das Bild eines Sheets mit frames Bildern bei fps zur Animationszeit t; eine Einmal-Animation
// (once) bleibt auf dem letzten Bild stehen.
func SheetFrame(t, fps float64, frames int, once bool) int {
	if frames <= 0 || fps <= 0 {
		return 0
	}
	i := int(t * fps)
	if once {
		return min(i, frames-1)
	}
	return i % frames
}

// MountGait liefert die Gangart beim Tempo vx und Referenz-Tempo ref (mountGait.ts) und ob das Reittier läuft;
// im Stand steht es (Gangart stand).
func MountGait(vx, ref float64) (string, bool) {
	v := math.Abs(vx)
	if v <= moveEps {
		return gaitStand, false
	}
	r := 1.0
	if ref > 0 {
		r = v / ref
	}
	switch {
	case r >= gallopFrom:
		return gaitGallop, true
	case r >= trotFrom:
		return gaitTrot, true
	}
	return gaitWalk, true
}

// FallbackGait liefert die Gangart, die das Reittier hat: fehlt sie (Elefant ohne Galopp), die nächst langsamere.
func FallbackGait(g string, has func(string) bool) string {
	order := [...]string{gaitGallop, gaitTrot, gaitWalk, gaitStand}
	for i, o := range order {
		if o != g {
			continue
		}
		for _, f := range order[i:] {
			if has(f) {
				return f
			}
		}
	}
	return g
}

// MonarchFrame ist der Teil des Monarchen-Zustands, den die Animation braucht (wie monarchAnim.ts).
type MonarchFrame struct {
	Known     bool // es gibt einen Vor-Zustand
	RespawnIn float64
	VX        float64
}

// MonarchWant wählt die Animation des Monarchen: tot zu Fuß mit death (hält), nach dem Tod respawn zu Fuß, sonst
// beritten in der Gangart nach Tempo. Treffer und Münze-Geben zeigt er beritten nicht (wie mountView.ts).
func MonarchWant(prev, cur MonarchFrame, ref float64, has func(gait string) bool) Want {
	if cur.RespawnIn > 0 {
		return Want{Anim: animDeath, Once: true, Hold: true}
	}
	if prev.Known && prev.RespawnIn > 0 {
		return Want{Anim: animRespawn, Once: true, Restart: true}
	}
	g, _ := MountGait(cur.VX, ref)
	g = FallbackGait(g, has)
	return Want{Anim: g, Gait: g}
}

// Motion ist die Bewegung einer Truppe oder eines Gegners zwischen zwei Takten.
type Motion struct {
	Known          bool // es gibt einen Vor-Zustand
	PrevX, X       float64
	PrevCD, CD     float64
	AttackCooldown bool // Cooldown zählt Schläge (Kämpfer, Gegner), nicht die Wartezeit beim Umherwandern
}

// FigureWant wählt die Animation einer Truppe oder eines Gegners (animate in worldRenderer.ts) und die Richtung
// (-1 links, 1 rechts, 0 unverändert): Ein steigender Cooldown ist ein neuer Schlag.
func FigureWant(m Motion) (Want, int) {
	if !m.Known {
		return Want{Anim: animIdle}, 0
	}
	dx := m.X - m.PrevX
	dir := 0
	if math.Abs(dx) > movingEps {
		dir = int(math.Copysign(1, dx))
	}
	switch {
	case m.AttackCooldown && m.CD > m.PrevCD+1e-6:
		return Want{Anim: animAttack, Once: true, Restart: true}, dir
	case dir != 0:
		return Want{Anim: animRun}, dir
	}
	return Want{Anim: animIdle}, 0
}

// Shift liefert den Zeitversatz der Schleifen einer Figur aus ihrer ID (fest, ohne Zufall).
func Shift(id int) float64 { return float64(((id%17)+17)%17) * 0.113 }
