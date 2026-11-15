package view

import "math"

// Stresstests im Spielmenü (Testing, B-896): menu stellt einen StressTest ein und startet ihn über die optionale
// Rolle StressStarter der Session (game erfüllt sie); game startet das Spiel, führt die Werte über Dev-Aktionen aus
// und schreibt am Ende einen Leistungsbericht nach ReportsDir. Die Vorgaben stehen in data/stresstests.json.

// Anstieg der Gegnerzahl je Welle (StressTest.Growth).
const (
	GrowthLinear = "linear"      // Gegner + Faktor je weitere Welle
	GrowthExp    = "exponentiell" // Gegner × Faktor je weitere Welle
)

// StressMaxEnemies ist die Obergrenze zusätzlicher Gegner je Welle (Obergrenze der Dev-Aktion spawn,
// engine/sim › DevMaxSpawn).
const StressMaxEnemies = 200

// StressTest sind die Werte eines Stresstests; vergleichbar (steht in Settings.Stress). Null-Werte: Standard-Welt,
// Oberfläche, kein Ausbau, keine Truppen, kein Zeitraffer, kein Abbruch.
type StressTest struct {
	Preset    string  `json:"id"`        // Vorgabe, aus der die Werte stammen ("" = eigene)
	Players   int     `json:"players"`   // Spieler 1 … MaxPlayers; ohne Gerät wartet der Platz auf eins
	Depth     int     `json:"depth"`     // Stufe (Tiefe), in der die Spieler beginnen
	Seed      int     `json:"seed"`      // 0 … 999; 0 = Name des Spielstands
	Enemies   int     `json:"enemies"`   // zusätzliche Gegner in der ersten Welle
	Growth    string  `json:"growth"`    // GrowthLinear oder GrowthExp
	Factor    float64 `json:"factor"`    // linear: + Faktor je Welle; exponentiell: × Faktor je Welle
	Build     int     `json:"build"`     // Ausbaustufe vorab (Dev-Aktion buildAll), 0 = keiner
	Troops    int     `json:"troops"`    // Truppen vorab (Dev-Aktion spawn troop)
	Timescale int     `json:"timescale"` // Zeitraffer (Timescales), 0 oder 1 = normal
	Waves     int     `json:"waves"`     // Abbruch nach so vielen Wellen, 0 = nie
	Minutes   int     `json:"minutes"`   // Abbruch nach so vielen Minuten Wanduhr, 0 = nie
	Enemy     string  `json:"enemy"`     // Gegner-Sorte der zusätzlichen Gegner (data/enemies.json)
	Troop     string  `json:"troop"`     // Truppen-Sorte vorab (data/troops.json)
	Pure      bool    `json:"pure"`      // reiner Stressmodus (B-921): Gegner entstehen an der Burg, sonst nichts von selbst
}

// EnemiesFor liefert die zusätzlichen Gegner der Welle wave (1 = erste) nach Anstieg und Faktor, 0 … StressMaxEnemies.
func (t StressTest) EnemiesFor(wave int) int {
	if wave < 1 || t.Enemies <= 0 {
		return 0
	}
	n := float64(t.Enemies)
	steps := float64(wave - 1)
	if t.Growth == GrowthExp {
		n *= math.Pow(math.Max(t.Factor, 0), steps)
	} else {
		n += t.Factor * steps
	}
	if math.IsNaN(n) || n < 0 {
		return 0
	}
	return int(math.Min(math.Round(n), StressMaxEnemies))
}

// StressStarter ist eine optionale Rolle der Session, die menu bekommt (game erfüllt sie, B-896): StartStress startet
// ein neues Spiel mit den Werten t; by ist das Gerät von Spieler 1. Danach liefert der Screen StepPlay.
type StressStarter interface {
	StartStress(t StressTest, by DeviceID) error
}
