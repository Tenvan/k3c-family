package game

// Versuchsschalter --pacing frame (RU0.2, B-1019): Statt Ebitengine die 30 Takte je Sekunde verteilen zu lassen (je
// Bild kein, ein oder zwei Updates) läuft ein Update je Bild (ebiten.SyncWithFPS, main.go). Die Simulation taktet aus
// einer eigenen Zeitsammlung: Die echte Bilddauer wird gesammelt, je voller Takt-Länge läuft ein Takt, der Rest geteilt
// durch die Takt-Länge ist alpha für die Zwischenbilder (blend.go). Ohne den Schalter bleibt alles wie bisher.

// PacingFrame ist der Wert des Schalters --pacing für die eigene Zeitsammlung.
const PacingFrame = "frame"

// PacingSync ist die eigene Zeitsammlung plus Ebitengine im Single-Thread-Modus (RunGameOptions.SingleThread, main.go):
// Im Vergleich ohne Spielcode (RU0.2) lief das Quadrat damit optisch flüssig, mit PacingFrame allein nicht.
const PacingSync = "sync"

// Paced meldet, ob mode die eigene Zeitsammlung nutzt (ein Update je Bild).
func Paced(mode string) bool { return mode == PacingFrame || mode == PacingSync }

// pacerMaxTicks begrenzt die Takte je Bild: Nach einem Hänger holt die Simulation nicht beliebig viel auf einmal nach.
const pacerMaxTicks = 4

// pacer ist die Zeitsammlung.
type pacer struct {
	last float64 // Zeitpunkt des letzten Bilds in Sekunden; 0 = noch keins
	acc  float64 // gesammelte Zeit in Sekunden, nach step immer unter einer Takt-Länge
}

// newPacer liefert die Zeitsammlung für mode, nil ohne den Schalter.
func newPacer(mode string) *pacer {
	if !Paced(mode) {
		return nil
	}
	return &pacer{}
}

// step sammelt die Zeit bis now (Sekunden) und liefert die Takte, die in diesem Bild zu rechnen sind.
func (p *pacer) step(now float64) int {
	const tick = 1.0 / TPS
	if p.last > 0 {
		p.acc += now - p.last
	}
	p.last = now
	n := int(p.acc / tick)
	if n > pacerMaxTicks { // Hänger: Rest verwerfen statt lange nachzuholen
		p.acc, n = float64(pacerMaxTicks)*tick, pacerMaxTicks
	}
	p.acc -= float64(n) * tick
	return n
}

// alpha ist der Anteil der Takt-Länge, der seit dem letzten gerechneten Takt gesammelt ist, 0 bis unter 1.
func (p *pacer) alpha() float64 { return p.acc * TPS }
