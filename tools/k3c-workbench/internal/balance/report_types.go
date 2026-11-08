package balance

import (
	"encoding/json"
	"fmt"

	"k3c/data"
)

// Lese-Typen der Berichte von bin/k3c-sim balance (RS2.2, B-726): Felder und Reihenfolge wie engine/balance, damit
// die Workbench Berichte liest und neu bewertet, ohne engine/sim zu importieren. Stock und Cost sind Kopien der
// Typen aus engine/sim (nur die JSON-Form zählt).

// Stock wie Stock: Vorrat an Baumaterial.
type Stock struct {
	Wood    int `json:"wood"`
	Stone   int `json:"stone"`
	Copper  int `json:"copper"`
	Iron    int `json:"iron,omitempty"`
	Crystal int `json:"crystal,omitempty"`
}

// Cost wie Cost: Kosten in Gold und Baumaterial (ohne Tags wie dort).
type Cost struct {
	Gold, Wood, Stone, Copper, Iron, Crystal int
}

// Kennzahlen je Lauf (B-099 › Anforderungen), alle aus Zustand und Ereignissen der Simulation. Ticks zählen ab 1
// (Tick n = nach dem n-ten Schritt). Zeiger sind `null`, solange es den Zeitpunkt im Lauf nicht gab.

// Metrics sind die Kennzahlen eines Laufs; feste Feldreihenfolge, damit das JSON byte-gleich bleibt.
type Metrics struct {
	// GoldThresholdTick: erster Tick, an dem das Gold aller Monarchen zusammen mindestens die Schwelle erreicht.
	GoldThresholdTick *int `json:"goldThresholdTick"`
	FirstWallTick     *int `json:"firstWallTick"` // erstes Ereignis built mit kind wall
	// FirstBowTick: erstes Ereignis armed (ein Bauer holt einen Bogen aus der Werkstatt); Start-Bogenschützen zählen nicht.
	FirstBowTick   *int `json:"firstBowTick"`
	CastleFallTick *int `json:"castleFallTick"` // erstes castleFallen
	// FirstWorkshopTick: erstes Ereignis built mit kind workshop (WZ28.4).
	FirstWorkshopTick *int `json:"firstWorkshopTick"`
	// FirstTowerTick: erstes Ereignis built mit kind tower; HubLevel2Tick/HubLevel3Tick: erstes Ereignis upgraded mit
	// kind castle auf Stufe 2 bzw. 3 (BAL6, B-347).
	FirstTowerTick *int          `json:"firstTowerTick"`
	HubLevel2Tick  *int          `json:"hubLevel2Tick"`
	HubLevel3Tick  *int          `json:"hubLevel3Tick"`
	ReachedDepth   int           `json:"reachedDepth"` // tiefste Stufe, auf der am Ende ein Monarch steht
	Days           []DayMetrics  `json:"days"`
	Waves          []WaveMetrics `json:"waves"`
	Dawns          []DawnMetrics `json:"dawns"`
	IslandMetrics
}

// DawnMetrics: Stand zu Tagesbeginn (BAL6, B-347): Tag 1 ist der Start (Tick 0), jeder weitere das Ereignis dawn der
// Stufe des Szenarios, auch der Morgen nach dem letzten Tag. Gold je Monarch in der Reihenfolge der Spieler, Vorrat der
// Insel, Hub-Stufe und Kämpfer (Truppen außer Landstreichern und Bauern) der Stufe.
type DawnMetrics struct {
	Day      int   `json:"day"`
	Tick     int   `json:"tick"`
	Gold     []int `json:"gold"`
	Stock    Stock `json:"stock"`
	HubLevel int   `json:"hubLevel"`
	Fighters int   `json:"fighters"`
}

// DayMetrics: Stand zu Dämmerungsbeginn (Ereignis dusk) und Gold-Fluss eines Tags. Der Fluss ist die Änderung des
// Golds aller Monarchen je Tick: Zunahme = Einnahmen, Abnahme = Ausgaben (auch gestohlenes und fallen gelassenes Gold).
// Ein Tag reicht vom Morgen bis zum nächsten Morgen, Nacht n gehört also zu Tag n (WZ28.4):
//   - NightIncome/NightExpense: der Teil von Income/Expense in der Phase night (Tag-Anteil = Differenz)
//   - StockIn/StockOut: Zu- und Abgang des Insel-Vorrats je Rohstoff (Änderung je Tick)
//   - CastleFallen: In diesem Tag (samt Nacht) fiel die Burg
//   - PeasantTicks/PeasantIdleTicks: Bauern-Ticks und davon ohne Auftrag und ohne Arbeitsplatz (Wartezeit)
type DayMetrics struct {
	Day              int    `json:"day"`
	DuskTick         *int   `json:"duskTick"`
	GoldAtDusk       *int   `json:"goldAtDusk"`
	StockAtDusk      *Stock `json:"stockAtDusk"`
	Income           int    `json:"income"`
	Expense          int    `json:"expense"`
	NightIncome      int    `json:"nightIncome"`
	NightExpense     int    `json:"nightExpense"`
	StockIn          Stock  `json:"stockIn"`
	StockOut         Stock  `json:"stockOut"`
	CastleFallen     bool   `json:"castleFallen"`
	PeasantTicks     int    `json:"peasantTicks"`
	PeasantIdleTicks int    `json:"peasantIdleTicks"`
}

// WaveMetrics: eine Welle vom Ereignis wave bis zum nächsten dawn (oder zur nächsten Welle). Verluste sind Truppen
// (ohne Landstreicher), die zu Wellenbeginn lebten und am Ende fehlen; überlebt = kein castleFallen in der Welle.
type WaveMetrics struct {
	Wave               int  `json:"wave"`
	StartTick          int  `json:"startTick"`
	EndTick            *int `json:"endTick"` // null: Lauf endete in der Welle
	Enemies            int  `json:"enemies"`
	TroopsAtStart      int  `json:"troopsAtStart"`
	TroopsLost         int  `json:"troopsLost"`
	BuildingsDestroyed int  `json:"buildingsDestroyed"`
	PlayersDown        int  `json:"playersDown"`
	Survived           bool `json:"survived"`
}

// Kennzahlen eines Insel-Laufs mit World (BS11, B-642): je Tiefe der Insel Ankunft und Burgfall, besiegte Bosse mit
// Tag, Endboss und Sieg. Ohne World (eine Stufe) bleiben alle Felder leer und stehen nicht im JSON.

// StageMetrics: eine Stufe der Insel (Reihenfolge = Stufenindex). Arrival* ist der erste Tick, an dem ein Monarch auf
// der Stufe stand (Stufe 0: Tick 0); null: nie erreicht.
type StageMetrics struct {
	Depth          int  `json:"depth"`
	ArrivalDay     *int `json:"arrivalDay"`
	ArrivalTick    *int `json:"arrivalTick"`
	CastleFallTick *int `json:"castleFallTick"`
	Downs          int  `json:"downs,omitempty"`    // Ereignisse playerDown auf dieser Stufe (ein Monarch fiel)
	HubLevel       int  `json:"hubLevel,omitempty"` // Hub-Stufe am Ende des Laufs
	Fighters       int  `json:"fighters,omitempty"` // Kämpfer (Truppen außer Landstreichern und Bauern) am Ende
}

// BossMetrics: ein besiegter Boss (Ereignis bossDefeated) mit Tag und Tick.
type BossMetrics struct {
	Boss string `json:"boss"`
	Day  int    `json:"day"`
	Tick int    `json:"tick"`
}

// IslandMetrics fasst die Felder zusammen, die nur ein Insel-Lauf füllt.
type IslandMetrics struct {
	Stages []StageMetrics `json:"stages,omitempty"`
	Bosses []BossMetrics  `json:"bosses,omitempty"`
	// BossSpawns: Ereignis bossSpawned (Miniboss mit der Welle, Endboss am Bau); zeigt, ob ein Boss überhaupt kam.
	BossSpawns  []BossMetrics `json:"bossSpawns,omitempty"`
	EndbossDay  *int          `json:"endbossDay,omitempty"`  // Tag des Siegs über den Endboss der Insel
	EndbossTick *int          `json:"endbossTick,omitempty"` // Tick dazu
	VictoryTick *int          `json:"victoryTick,omitempty"` // Ereignis victory (nur auf der letzten Insel der Welt)
	// Ship: je Schiff-Abschnitt der Tag, ab dem er bezahlbar gewesen wäre (ship_need.go, BS3.3); nur Insel-Lauf.
	Ship []ShipNeed `json:"ship,omitempty"`
}

// Matrix beschreibt alle Läufe: jedes Paar aus Seed × Spieleranzahl × Bot × Tiefe läuft `Days` Tage.
type Matrix struct {
	Seeds   []string `json:"seeds"`
	Players []int    `json:"players"`
	Bots    []string `json:"bots"`
	Depths  []int    `json:"depths"` // Tiefe = Biom (0 Wald, 1 Höhle, …)
	Days    int      `json:"days"`
	// GoldThreshold: Schwelle für Metrics.GoldThresholdTick (Gold aller Monarchen zusammen); 0 = nicht messen.
	GoldThreshold int    `json:"goldThreshold"`
	Grade         string `json:"grade,omitempty"` // Schwierigkeitsgrad aller Läufe, leer = normal (Grad-Kurven, BAL3.3)
	// World: ID der Welt (z. B. standard); gesetzt baut der Lauf alle Tiefen der ersten Insel (BS11, B-642), Depths gilt dann nicht.
	World string `json:"world,omitempty"`
	// Helps: Dev-Hilfen wie im Stufen-Test (hilfen, voll, boss; helps.go), nur mit World; leer = keine.
	Helps string `json:"helps,omitempty"`
}

// Scenario ist ein einzelner Lauf.
type Scenario struct {
	Seed    string `json:"seed"`
	Players int    `json:"players"`
	Bot     string `json:"bot"`
	Depth   int    `json:"depth"`
	Days    int    `json:"days"`
	Grade   string `json:"grade,omitempty"` // leer = normal; steht so auch im Replay
	World   string `json:"world,omitempty"` // leer = eine Stufe der Tiefe Depth; sonst alle Tiefen der ersten Insel der Welt (BS11)
	Helps   string `json:"helps,omitempty"` // Dev-Hilfen (helps.go), leer = keine
}

// Result ist ein Lauf mit Kennzahlen; ein ungültiger Lauf hat Error statt Metrics.
type Result struct {
	Scenario
	Valid   bool     `json:"valid"`
	Error   string   `json:"error,omitempty"`
	Ticks   int      `json:"ticks"`
	Metrics *Metrics `json:"metrics"`
}

// Report ist die Ausgabe: die Matrix und alle Läufe in der Reihenfolge der Matrix.
type Report struct {
	Matrix  Matrix   `json:"matrix"`
	Results []Result `json:"results"`
}

// ShipStage ist ein Abschnitt aus data/ship.json.
type ShipStage struct {
	ID   string `json:"id"`
	Name string `json:"name"`
	Cost Cost   `json:"cost"`
}

// ShipNeed ist der früheste Tag, an dem ein Abschnitt (samt den Abschnitten davor) bezahlbar gewesen wäre; null: nie im Lauf.
type ShipNeed struct {
	ID            string `json:"id"`
	AffordableDay *int   `json:"affordableDay"`
}

// LoadShip liest die Abschnitte aus data/ship.json (BS3.2).
func LoadShip() ([]ShipStage, error) {
	raw, err := data.Files.ReadFile("ship.json")
	if err != nil {
		return nil, fmt.Errorf("ship.json: %w", err)
	}
	var f struct {
		Stages []ShipStage `json:"stages"`
	}
	if err := json.Unmarshal(raw, &f); err != nil {
		return nil, fmt.Errorf("ship.json: %w", err)
	}
	return f.Stages, nil
}
