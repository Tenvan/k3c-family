package balance

import (
	"strings"
	"testing"

	"k3c/engine/sim"
)

// BS3.3/AC-04 (B-638/AC-02): Der Bedarf gegen Bestand folgt dem Verlauf; das Schiff ist ab dem Tag bezahlbar, an dem
// Gold und Material der Zuflüsse die kumulierten Kosten decken.
func shipRun(endDay *int) *Metrics {
	days := make([]DayMetrics, 10)
	for i := range days {
		days[i] = DayMetrics{Day: i + 1, Income: 8, StockIn: sim.Stock{Wood: 30}}
	}
	days[3].StockIn.Stone, days[3].StockIn.Copper = 100, 40  // Tag 4: Stein und Kupfer für den Rumpf
	days[6].StockIn.Iron, days[6].StockIn.Crystal = 40, 20   // Tag 7: Eisen und Kristall für das Segel
	return &Metrics{Dawns: []DawnMetrics{{Gold: []int{14, 14}}}, Days: days,
		IslandMetrics: IslandMetrics{EndbossDay: endDay}}
}

func TestSchiffBedarfAusDemVerlauf(t *testing.T) {
	stages, err := LoadShip()
	if err != nil || len(stages) != 3 {
		t.Fatalf("%v %+v", err, stages)
	}
	m := shipRun(nil)
	got := shipNeeds(stages, m)
	// Kiel: 10 Gold + 80 Holz → Holz reicht ab Tag 3 (90), Gold schon am Start (28).
	// Rumpf (kumuliert 30 Gold, 80 Holz, 60 Stein, 40 Kupfer): Stein erst Tag 4.
	// Segel (kumuliert 60 Gold, 80 Holz, 60 Stein, 40 Kupfer, 40 Eisen, 20 Kristall): Eisen und Kristall erst Tag 7, Gold 28+8·d ≥ 60 ab Tag 4.
	want := []int{3, 4, 7}
	for i, w := range want {
		if got[i].AffordableDay == nil || *got[i].AffordableDay != w {
			t.Errorf("%s bezahlbar ab %v, erwartet Tag %d", got[i].ID, got[i].AffordableDay, w)
		}
	}
	m.Days = m.Days[:5] // Lauf endet vor Tag 7: das Segel war nie bezahlbar
	if g := shipNeeds(stages, m); g[2].AffordableDay != nil {
		t.Errorf("Segel ohne Eisen: %v", g[2].AffordableDay)
	}
}

// Ampel gegen den Korridor „Endboss-Sieg −2 bis +1 Tag“ (zielkorridore.md § 6).
func TestSchiffAmpel(t *testing.T) {
	stages, _ := LoadShip()
	cases := []struct {
		end   *int
		light string
		want  string
	}{
		{ptr(8), LightGreen, "Ziel T6–T9"},        // Segel T7 liegt im Korridor
		{ptr(11), LightYellow, "zu früh"},         // Segel T7, Korridor T9–T12: 2 Tage davor
		{ptr(20), LightRed, "zu früh"},            // weit davor
		{nil, LightGrey, "Endboss nicht besiegt"}, // kein Ziel
	}
	for _, c := range cases {
		m := shipRun(c.end)
		m.Ship = shipNeeds(stages, m)
		f := shipFiguresOf(Report{Results: []Result{{Valid: true, Metrics: m}}})
		if f == nil || f.Light != c.light || !strings.Contains(f.Text, c.want) {
			t.Errorf("Endboss %v: %+v, erwartet %s mit %q", c.end, f, c.light, c.want)
			continue
		}
		if !strings.Contains(f.Text, "Schiff (Bedarf): Kiel bezahlbar T3, Rumpf bezahlbar T4, Segel bezahlbar T7") {
			t.Errorf("Zeile: %s", f.Text)
		}
	}
	m := shipRun(ptr(8))
	m.Ship = shipNeeds(stages, m)
	f := shipFiguresOf(Report{Results: []Result{{Valid: true, Metrics: m}}})
	var b strings.Builder
	f.markdown(&b)
	if !strings.Contains(b.String(), "🟢 Schiff (Bedarf):") {
		t.Errorf("Markdown: %q", b.String())
	}
}

// Ein Lauf ohne World hat keinen Schiff-Bedarf (altes JSON bleibt gleich).
func TestSchiffNurImInselLauf(t *testing.T) {
	r := mustRun(t, Matrix{Seeds: []string{"1"}, Players: []int{1}, Bots: []string{"passive"}, Depths: []int{0}, Days: 1})
	if m := r.Results[0].Metrics; len(m.Ship) != 0 {
		t.Errorf("Ein-Stufen-Lauf mit Schiff: %+v", m.Ship)
	}
	if f := FiguresOf(r); f == nil || f.Ship != nil {
		t.Errorf("Kennzahlen ohne World dürfen kein Schiff nennen: %+v", f)
	}
}
