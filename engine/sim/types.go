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
	Pay    bool    `json:"pay"`              // Bezahl-Taste gehalten: Münze geben bzw. fallen lassen
	Attack bool    `json:"attack,omitempty"` // Schlag (Taste X, monarch.md § 1)
	Skill  int     `json:"skill,omitempty"`  // aktiven Skill in Slot 1 bis 4 auslösen, 0 = keiner (skills.go)
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
	// Skills: gelernte Skills (IDs, Lernreihenfolge); Slots: aktive Skills in den Slots 1 bis 4 ("" = frei).
	Skills         []string `json:"skills,omitempty"`
	Slots          []string `json:"slots,omitempty"`
	AttackCooldown float64  `json:"attackCooldown,omitempty"` // Sekunden bis zum nächsten Schlag
	// Cooldowns: Abklingzeit je Slot (Index 0 bis 3, Sekunden), nil bis zum ersten Skill-Einsatz.
	Cooldowns []float64 `json:"cooldowns,omitempty"`
	// Shield: Schild-HP, die applyDamage zuerst abzieht, für ShieldFor Sekunden. LastStandFor: so lange lässt ein
	// tödlicher Treffer 1 HP stehen.
	Shield       float64 `json:"shield,omitempty"`
	ShieldFor    float64 `json:"shieldFor,omitempty"`
	LastStandFor float64 `json:"lastStandFor,omitempty"`
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
	Type     string `json:"type"` // build, repair, gather, carry, fetchBow
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
	// carried: Material, das ein Träger in einer Insel bei vollem Maximum behält, während er einen Bauauftrag übernimmt.
	carried *Job
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
	// Ausbau (hub_level.go): Level = Stufe des gebauten Platzes (0 = 1), Upgrade = Zustand des Ausbaus auf die nächste
	// Stufe ("", waitingMaterial, waitingWorker), UpgradePaid = dafür gezahltes Gold.
	Level       int    `json:"level,omitempty"`
	Upgrade     string `json:"upgrade,omitempty"`
	UpgradePaid int    `json:"upgradePaid,omitempty"`
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
	// Stun: Sekunden betäubt (kein Schritt, kein Angriff). TauntFor: so lange zielt der Gegner auf den Spieler TauntID.
	Stun     float64 `json:"stun,omitempty"`
	TauntID  int     `json:"tauntId,omitempty"`
	TauntFor float64 `json:"tauntFor,omitempty"`
	// Slow: Faktor der Geschwindigkeit (Ice Wall, skills_caster.go) für SlowFor Sekunden; Angriffe bleiben gleich.
	Slow    float64 `json:"slow,omitempty"`
	SlowFor float64 `json:"slowFor,omitempty"`
}

// Storm ist ein laufender Lightning Storm (skills_caster.go): PerSecond Schaden je Sekunde an Gegnern im Radius um X,
// noch Left Sekunden; Owner ist die ID des wirkenden Spielers.
type Storm struct {
	X         float64 `json:"x"`
	Radius    float64 `json:"radius"`
	PerSecond float64 `json:"perSecond"`
	Left      float64 `json:"left"`
	Owner     int     `json:"owner"`
}

// Projectile ist ein Pfeil oder Geschoss.
type Projectile struct {
	ID       int     `json:"id"`
	X        float64 `json:"x"`
	TargetID int     `json:"targetId"`
	Team     string  `json:"team"` // player, enemy
	Damage   float64 `json:"damage"`
	Speed    float64 `json:"speed"`
	Cause    string  `json:"-"` // Gegnerart des Schützen für playerDown (B-182), ohne Ausgabe
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
	// Grad-Faktoren der Insel zum Wellenstart (startWave setzt sie immer, ohne Insel 1); ein Gradwechsel ändert wartende Gegner nicht.
	hpFactor, damageFactor float64
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

	WidthUnits float64 `json:"widthUnits"`
	HubX       float64 `json:"hubX"`
	// HubLevel ist die Hub-Stufe (Start 1, Ausbau an der Burg: hub_level.go); ohne JSON-Ausgabe (B-208).
	HubLevel int `json:"-"`
	// hubSite ist der Zahlplatz für den Hub-Ausbau an der Burg (nicht in Sites, hub_level.go).
	hubSite    *Site
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
	Storms      []*Storm        `json:"storms,omitempty"` // laufende Lightning Storms in Wirk-Reihenfolge

	Stock       *Stock  `json:"stock"` // Baumaterial gehört allen (in einer Insel: allen Stufen), Gold hat jeder Spieler selbst
	SkillPoints int     `json:"skillPoints"`
	Travel      *Travel `json:"travel"`
	Events      []Event `json:"events"` // wird bei jedem Step geleert
	// EventsDropped zählt die Ereignisse, die die Obergrenze je Tick in diesem Step verworfen hat (capEvents).
	EventsDropped int `json:"eventsDropped,omitempty"` // 0 fehlt im JSON (Protokoll-Beispiele unverändert, Übertragung: B-190)

	// noTravel: Die Stufe gehört zu einer Insel, deren Spieler einzeln wechseln (island_travel.go); der gemeinsame
	// Stufenwechsel der Campaign (stepTravel) ist dort aus.
	noTravel bool
	// island: die Insel, zu der die Stufe gehört (nil bei Campaign und einzelnen Welten: keine Optionen, Faktor 1,
	// kein Lager-Maximum).
	island *Island
}

func (w *World) newID() int {
	id := w.NextID
	w.NextID++
	return id
}
