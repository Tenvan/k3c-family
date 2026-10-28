package balance

import (
	"fmt"
	"math"
	"strings"
)

// Klartext des Berichts: Jede Zahl bekommt Einheit und Bezug (Läufe, Korridor, Prozentpunkte), die Richtung steht als
// Wort („zu leicht“, „zu schwer“) und jede Stellschraube als Plan mit aktuellem Wert, Bereich und Startvorschlag.

// highMeans: was ein Wert über dem Korridor für das Spiel heißt; unbekannte Kennzahlen bleiben bei „zu hoch“.
var highMeans = map[string]string{
	"castleHeld": "zu leicht", "firstWallBeforeDusk1": "zu leicht", "endbossDefeated": "zu leicht", "caveBeforeDay6": "zu leicht",
	"endbossDay": "zu schwer", "destroyedPerWave": "zu schwer",
}

// measureClause: was bei einem Anteil „eingetreten“ ist (Satzteil nach „In 5 von 5 Läufen …“).
var measureClause = map[string]string{
	"castleHeld":           "hielt die Burg bis zum Ende",
	"firstWallBeforeDusk1": "stand die erste Mauer vor der Dämmerung von Tag 1",
	"endbossDefeated":      "wurde der Endboss besiegt",
	"caveBeforeDay6":       "wurde die Höhle vor Tag 6 erreicht",
}

// measureUnit: Einheit eines Medians.
var measureUnit = map[string]string{"endbossDay": "Tage", "destroyedPerWave": "Gebäude"}

// Anzahl der Schritte, die der Probierbereich eines Hebels umfasst (wie bei der Suche, insights_next.go tuneRange).
const tuneSteps = tuneRange

// LeverPlan ist die Stellschraube mit Zahlen zum Eintragen: aktueller Wert in data/, Probierbereich und Startvorschlag.
// Known: false, wenn der Bereich nicht seriös abschätzbar ist (Wert 0 oder nicht lesbar); Text sagt das dann.
type LeverPlan struct {
	Path    string `json:"path"`
	Label   string `json:"label"`
	Known   bool   `json:"known"`
	Current string `json:"current"`
	Min     string `json:"min"`
	Max     string `json:"max"`
	Start   string `json:"start"`
	Short   string `json:"short"` // ohne Wirkungssatz: aktueller Wert, Bereich, Startvorschlag
	Text    string `json:"text"`
}

func (t TargetInsight) isShare() bool { return t.Kind == KindShare }

// unitShort: „Pkt.“ bei Anteilen (Prozentpunkte), sonst die Einheit des Medians.
func (t TargetInsight) unitShort() string {
	if t.isShare() {
		return "Pkt."
	}
	return measureUnit[t.Measure]
}

// unitLong: „Prozentpunkte“ bei Anteilen, sonst die Einheit des Medians.
func (t TargetInsight) unitLong() string {
	if t.isShare() {
		return "Prozentpunkte"
	}
	return measureUnit[t.Measure]
}

// above: wahr, wenn der Wert über der Obergrenze liegt; below: unter der Untergrenze.
func (t TargetInsight) above() bool { return t.Upper != nil && t.Value > *t.Upper }
func (t TargetInsight) below() bool { return t.Lower != nil && t.Value < *t.Lower }

// gapValue: Abstand des Werts zur verletzten Grenze in der Einheit des Werts (Prozentpunkte bzw. Median-Einheit).
func (t TargetInsight) gapValue() float64 {
	switch {
	case t.above():
		return t.Value - *t.Upper
	case t.below():
		return *t.Lower - t.Value
	}
	return 0
}

// reserve: Abstand im Korridor zur nächsten wirksamen Grenze (nicht die natürlichen 0 und 100 % bei Anteilen) und deren Name.
func (t TargetInsight) reserve() (gap float64, side string, ok bool) {
	lo, hi := t.entry.bounds()
	best := math.Inf(1)
	if lo != nil && t.Value-*lo < best {
		best, side = t.Value-*lo, "unteren"
	}
	if hi != nil && *hi-t.Value < best {
		best, side = *hi-t.Value, "oberen"
	}
	return best, side, !math.IsInf(best, 1)
}

// word: „zu leicht“ / „zu schwer“ bzw. „zu hoch“ / „zu niedrig“ für die Richtung der Abweichung.
func (t TargetInsight) word() string {
	switch {
	case t.above():
		if m, ok := highMeans[t.Measure]; ok {
			return m
		}
		return "zu hoch"
	case t.below():
		switch highMeans[t.Measure] {
		case "zu leicht":
			return "zu schwer"
		case "zu schwer":
			return "zu leicht"
		}
		return "zu niedrig"
	}
	return ""
}

func (t TargetInsight) gapNum() string { return t.amount(t.gapValue()) }

// amount: Betrag mit Einheit („1 Prozentpunkt“, „10 Prozentpunkte“, „2 Gebäude“).
func (t TargetInsight) amount(v float64) string {
	unit := t.unitLong()
	if t.isShare() && round1(v) == 1 {
		unit = "Prozentpunkt"
	}
	return strings.TrimSpace(num(round1(v)) + " " + unit)
}

// gapText ist die Spalte „Abstand“ der Tabelle: „10 Pkt. zu hoch“ außerhalb, „5 Pkt. Reserve“ im Korridor.
func (t TargetInsight) gapText() string {
	if t.Distance == nil {
		return "–"
	}
	if t.Status == StatusViolated {
		return strings.TrimSpace(num(round1(t.gapValue()))+" "+t.unitShort()) + " " + dirWord(t)
	}
	if g, _, ok := t.reserve(); ok {
		return strings.TrimSpace(num(round1(g))+" "+t.unitShort()) + " Reserve"
	}
	return "keine Grenze nah"
}

// dirWord: Richtung als Wort für die Tabelle: „zu hoch“ / „zu niedrig“ (die Bewertung leicht/schwer steht im Text).
func dirWord(t TargetInsight) string {
	if t.above() {
		return "zu hoch"
	}
	return "zu niedrig"
}

// what: die Messgröße als Satz-Anfang; bei Anteilen „In c von n Läufen …“.
func (t TargetInsight) what() string {
	if !t.isShare() {
		return fmt.Sprintf("Median von „%s“: %s.", t.Name, t.amount(t.Value))
	}
	clause, ok := measureClause[t.Measure]
	if !ok {
		clause = "traf „" + t.Name + "“ zu"
	}
	if t.Runs <= 0 {
		return fmt.Sprintf("Bei %s %% der Läufe %s.", num(round1(t.Value)), clause)
	}
	c := int(math.Round(t.Value * float64(t.Runs) / 100))
	return fmt.Sprintf("In %d von %d Läufen %s (%s %%).", c, t.Runs, clause, num(round1(t.Value)))
}

// target: der Sollwert als Satz.
func (t TargetInsight) target() string {
	of := ""
	if t.isShare() {
		of = " der Läufe"
	}
	switch {
	case t.Lower != nil && t.Upper != nil:
		return fmt.Sprintf("Sollwert: %s%s.", t.boundsText(), of)
	case t.Lower != nil:
		return fmt.Sprintf("Sollwert: mindestens %s%s.", num(*t.Lower)+t.entry.unit(), of)
	}
	return fmt.Sprintf("Sollwert: höchstens %s%s.", num(*t.Upper)+t.entry.unit(), of)
}

// plain: Klartext zu einem Ziel in drei Sätzen: Messwert, Sollwert, Lage.
func (t TargetInsight) plain() string {
	if t.Status == StatusInvalid {
		return fmt.Sprintf("%s Kein belastbarer Messwert: %d Läufe abgebrochen oder ohne Ergebnis.", t.target(), len(t.InvalidSeeds))
	}
	s := t.what() + " " + t.target() + " "
	switch {
	case t.above(), t.below():
		over := "unter"
		if t.above() {
			over = "über"
		}
		return s + fmt.Sprintf("Damit %s %s dem Korridor, also %s.", t.gapNum(), over, t.word())
	}
	g, side, ok := t.reserve()
	switch {
	case !ok:
		return s + "Damit im Korridor."
	case t.Status == StatusNarrow:
		return s + fmt.Sprintf("Im Korridor, aber knapp: nur %s bis zur %s Grenze.", t.amount(g), side)
	}
	return s + fmt.Sprintf("Im Korridor, %s von der %s Grenze entfernt.", t.amount(g), side)
}

// trendPlain: Vergleich mit der Baseline in Worten; leer ohne Baseline.
func (t TargetInsight) trendPlain() string {
	if t.Trend == nil {
		return ""
	}
	unit := t.entry.unit()
	d := t.Trend.Delta
	switch {
	case math.Abs(d) < 1e-9:
		return fmt.Sprintf("Unverändert gegenüber der Baseline (%s%s).", num(t.Trend.Before), unit)
	case t.Trend.Better:
		return fmt.Sprintf("Gegenüber der Baseline (%s%s) näher am Korridor: %s%s.", num(t.Trend.Before), unit, signed(d), unit)
	}
	return fmt.Sprintf("Gegenüber der Baseline (%s%s) weiter weg vom Korridor: %s%s.", num(t.Trend.Before), unit, signed(d), unit)
}

// leverRange: Startvorschlag (ein Schritt) und Probierbereich (bis zu tuneSteps Schritte) ab dem Wert in data/. ok ist falsch
// bei Wert 0 oder wenn der Schritt (bei ganzen Zahlen gerundet) den Wert nicht ändert.
func leverRange(base float64, isInt bool, dir string, step float64) (start, lo, hi float64, ok bool) {
	sign := 1.0
	if dir == "-" {
		sign = -1
	}
	if base == 0 || step <= 0 {
		return 0, 0, 0, false
	}
	pick := func(n float64) float64 {
		v := base * (1 + sign*n*step)
		if sign < 0 {
			v = math.Max(0, v)
		}
		if !isInt {
			return v
		}
		if sign > 0 {
			return math.Max(math.Ceil(v), base+1)
		}
		return math.Max(0, math.Min(math.Floor(v), base-1))
	}
	start, far := pick(1), pick(tuneSteps)
	if start == base {
		return 0, 0, 0, false
	}
	return start, math.Min(base, far), math.Max(base, far), true
}

// effect: Satz zur Wirkung auf das Ziel; ehrlich, dass das Ausmaß nicht berechenbar ist.
func (t TargetInsight) effect() string {
	way := "steigen"
	switch {
	case t.above():
		way = "sinken"
	case !t.below():
		return fmt.Sprintf("Wirkung: „%s“ (%s) soll sich von der knappen Grenze entfernen, ohne den Korridor %s zu verlassen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.",
			t.Name, t.valueText(), t.boundsText())
	}
	return fmt.Sprintf("Wirkung: „%s“ soll von %s Richtung Korridor %s %s. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.",
		t.Name, t.valueText(), t.boundsText(), way)
}

// planFor baut den Plan zu einem Hebel; base und isInt kommen aus data/ (found == false: nicht lesbar).
func (t TargetInsight) planFor(l Lever, base float64, isInt, found bool) LeverPlan {
	p := LeverPlan{Path: l.Path, Label: l.Label}
	if !found {
		p.Short = "Aktueller Wert in data/ nicht lesbar: Bereich nicht abschätzbar, Wert von Hand wählen."
		p.Text = p.Short
		return p
	}
	tp := TuneParam{Base: base, Int: isInt}
	p.Current = tp.format(base)
	start, lo, hi, ok := leverRange(base, isInt, l.Direction, l.Step)
	if !ok {
		p.Short = fmt.Sprintf("Aktuell %s. Bereich nicht seriös abschätzbar (Wert 0 oder ein Schritt von %s %% ändert ihn nicht): Wert von Hand wählen.",
			p.Current, stepText(l.Step))
		p.Text = p.Short + " " + t.effect()
		return p
	}
	p.Known, p.Min, p.Max, p.Start = true, tp.format(lo), tp.format(hi), tp.format(start)
	way := "rauf"
	if l.Direction == "-" {
		way = "runter"
	}
	p.Short = fmt.Sprintf("Aktuell %s. Probierbereich %s … %s (1 bis %d Schritte à %s %s). Startvorschlag: %s.",
		p.Current, p.Min, p.Max, tuneSteps, stepText(l.Step), way, p.Start)
	p.Text = p.Short + " " + t.effect()
	return p
}

// plans: ein Plan je Hebel; der Wert kommt aus data/.
func (t TargetInsight) plans() []LeverPlan {
	out := make([]LeverPlan, 0, len(t.Levers))
	for _, l := range t.Levers {
		p, err := ParseTuneParam(l.Path + "=0..1")
		out = append(out, t.planFor(l, p.Base, p.Int, err == nil))
	}
	return out
}
