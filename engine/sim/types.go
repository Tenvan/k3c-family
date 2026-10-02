package sim

import (
	"k3c/engine/level"
	"k3c/engine/rng"
)

// Zustand der Simulation (Port von src/world/sim/types.ts). Reine Daten, Positionen in Units, Zeiten in Sekunden.
// Die JSON-Form ist dieselbe wie in TS und wird gegen testdata/golden/sim-*.json geprüft: leere Listen sind `[]`,
// optionale Werte `null` (Zeiger).

// PlayerCommand ist die Eingabe eines Spielers für einen Tick.
type PlayerCommand struct {
	MoveX  float64 `json:"moveX"` // -1 (links) bis 1 (rechts)
	Sprint bool    `json:"sprint"`
	Pay    bool    `json:"pay"` // Bezahl-Taste gehalten: Münze geben bzw. fallen lassen
}

// Player ist ein Monarch.
type Player struct {
	ID          int     `json:"id"`
	Index       int     `json:"index"`
	X           float64 `json:"x"`
	VX          float64 `json:"vx"`
	Facing      int     `json:"facing"`
	Gold        int     `json:"gold"`
	HP          float64 `json:"hp"`
	MaxHP       float64 `json:"maxHp"`
	RespawnIn   float64 `json:"respawnIn"` // > 0: tot, Sekunden bis zum Respawn an der Burg
	PayCooldown float64 `json:"payCooldown"`
	Paying      bool    `json:"paying"`
	PayKey      *string `json:"payKey"` // Ziel, an dem gerade gezahlt wird ("site:3")
	PayAmount   int     `json:"payAmount"`
	// Free: Niemand steuert diesen Monarchen (vom Raum gesetzt, B-059). Er zählt nicht für den Stufenwechsel und
	// reist mit. Nur Go, im JSON nur bei true.
	Free bool `json:"free,omitempty"`
}

// Coin ist eine Münze am Boden.
type Coin struct {
	ID              int     `json:"id"`
	X               float64 `json:"x"`
	BlockedPlayerID *int    `json:"blockedPlayerId"`
	BlockedUntil    float64 `json:"blockedUntil"`
}

// Job ist der Auftrag eines Bauern; es sind nur die Felder seines Typs gesetzt (IDs sind nie 0).
type Job struct {
	Type     string `json:"type"` // build, gather, carry, fetchBow
	SiteID   int    `json:"siteId,omitempty"`
	NodeID   int    `json:"nodeId,omitempty"`
	Resource string `json:"resource,omitempty"`
	Amount   int    `json:"amount,omitempty"`
}

// Troop ist eine eigene Einheit: vagrant, peasant oder archer.
type Troop struct {
	ID       int     `json:"id"`
	Kind     string  `json:"kind"`
	X        float64 `json:"x"`
	HP       float64 `json:"hp"`
	MaxHP    float64 `json:"maxHp"`
	AnchorX  float64 `json:"anchorX"` // Camp (Landstreicher) bzw. Hub
	TargetX  float64 `json:"targetX"`
	Job      *Job    `json:"job"`
	Cooldown float64 `json:"cooldown"`
	TowerID  *int    `json:"towerId"`
	PaidGold int     `json:"paidGold"` // Landstreicher: bereits bezahltes Rekrutierungs-Gold
}

// ResourceNode ist ein Baum, Fels oder Kupfererz.
type ResourceNode struct {
	ID       int     `json:"id"`
	Kind     string  `json:"kind"`
	X        float64 `json:"x"`
	PaidGold int     `json:"paidGold"`
	Marked   bool    `json:"marked"`
	WorkerID *int    `json:"workerId"`
	Progress float64 `json:"progress"`
}

// Site ist ein Bauplatz im Hub. State: unpaid, waitingMaterial, waitingWorker, built.
type Site struct {
	ID            int     `json:"id"`
	Kind          string  `json:"kind"`
	X             float64 `json:"x"`
	State         string  `json:"state"`
	PaidGold      int     `json:"paidGold"`
	BuildProgress float64 `json:"buildProgress"`
	HP            float64 `json:"hp"`
	MaxHP         float64 `json:"maxHp"`
	WorkerID      *int    `json:"workerId"`
	Bows          int     `json:"bows"`        // Werkstatt: fertige Bögen im Regal
	BowPaidGold   int     `json:"bowPaidGold"` // Werkstatt: bezahltes Gold für den nächsten Bogen
}

// Castle ist die Burg in der Hub-Mitte.
type Castle struct {
	ID    int     `json:"id"`
	X     float64 `json:"x"`
	HP    float64 `json:"hp"`
	MaxHP float64 `json:"maxHp"`
}

// Enemy ist ein Gegner (Simulation ab SP06).
type Enemy struct {
	ID          int      `json:"id"`
	Kind        string   `json:"kind"`
	X           float64  `json:"x"`
	HP          float64  `json:"hp"`
	MaxHP       float64  `json:"maxHp"`
	Damage      float64  `json:"damage"`
	Speed       float64  `json:"speed"`
	Range       float64  `json:"range"`
	Traits      []string `json:"traits"`
	Cooldown    float64  `json:"cooldown"`
	Fleeing     bool     `json:"fleeing"`
	CarriedGold int      `json:"carriedGold"`
	HomeX       float64  `json:"homeX"` // Portal, aus dem der Gegner kam
}

// Projectile ist ein Pfeil oder Geschoss.
type Projectile struct {
	ID       int     `json:"id"`
	X        float64 `json:"x"`
	TargetID int     `json:"targetId"`
	Team     string  `json:"team"` // player, enemy
	Damage   float64 `json:"damage"`
	Speed    float64 `json:"speed"`
}

// Pickup ist eine Truhe oder ein Skill-Punkt.
type Pickup struct {
	ID   int     `json:"id"`
	Kind string  `json:"kind"`
	X    float64 `json:"x"`
}

// Camp ist ein Rekrutierungs-Camp.
type Camp struct {
	X         float64 `json:"x"`
	RespawnIn float64 `json:"respawnIn"`
}

// QueuedSpawn ist ein geplanter Gegner einer Welle.
type QueuedSpawn struct {
	Kind string  `json:"kind"`
	X    float64 `json:"x"`
	At   float64 `json:"at"`
}

// Stock ist der gemeinsame Hub-Vorrat an Baumaterial.
type Stock struct {
	Wood   int `json:"wood"`
	Stone  int `json:"stone"`
	Copper int `json:"copper"`
	// Eisen und Kristall: omitempty, damit Snapshots und Spielstände der Campaign gleich bleiben.
	Iron    int `json:"iron,omitempty"`
	Crystal int `json:"crystal,omitempty"`
}

// Travel: Alle Spieler stehen an einem Tiefen-Eingang oder einer Treppe, Fortschritt 0..1.
type Travel struct {
	X        float64 `json:"x"`
	ToDepth  int     `json:"toDepth"`
	Via      string  `json:"via"` // exit, stairsUp, stairsDown
	Progress float64 `json:"progress"`
}

// Event ist ein Ereignis des letzten Ticks, z. B. {"type": "night", "day": 1}. Eine Map, weil die Felder je
// Typ wechseln und Werte wie `player: 0` erhalten bleiben müssen.
type Event map[string]any

// World ist der vollständige Zustand einer Stufe.
type World struct {
	Seed       string       `json:"seed"`
	Biome      level.Biome  `json:"-"`
	Level      level.Layout `json:"-"`
	rng        *rng.Rng
	Time       float64 `json:"time"`       // simulierte Sekunden seit Start
	CycleSpeed float64 `json:"cycleSpeed"` // Faktor für den Tag/Nacht-Zyklus
	NextID     int     `json:"nextId"`

	WidthUnits float64   `json:"widthUnits"`
	HubX       float64   `json:"hubX"`
	Cycle      CycleInfo `json:"cycle"`
	Aggression *float64  `json:"aggression"` // nur unter Tage (0..100), sonst null
	Wave       int       `json:"wave"`

	Players     []*Player       `json:"players"`
	Coins       []*Coin         `json:"coins"`
	Troops      []*Troop        `json:"troops"`
	Nodes       []*ResourceNode `json:"nodes"`
	Sites       []*Site         `json:"sites"`
	Castle      Castle          `json:"castle"`
	Enemies     []*Enemy        `json:"enemies"`
	Projectiles []*Projectile   `json:"projectiles"`
	Pickups     []*Pickup       `json:"pickups"`
	Camps       []*Camp         `json:"camps"`
	Portals     []float64       `json:"portals"`
	SpawnQueue  []QueuedSpawn   `json:"spawnQueue"`

	Stock       *Stock  `json:"stock"` // Baumaterial gehört allen (in einer Insel: allen Stufen), Gold hat jeder Spieler selbst
	SkillPoints int     `json:"skillPoints"`
	Travel      *Travel `json:"travel"`
	Events      []Event `json:"events"` // wird bei jedem Step geleert

	// noTravel: Die Stufe gehört zu einer Insel, deren Spieler einzeln wechseln (island_travel.go); der gemeinsame
	// Stufenwechsel der Campaign (stepTravel) ist dort aus.
	noTravel bool
	// island: die Insel, zu der die Stufe gehört (nil = Campaign/einzelne Welt); nur dann gilt das Lager-Maximum.
	island *Island
}

func (w *World) newID() int {
	id := w.NextID
	w.NextID++
	return id
}
