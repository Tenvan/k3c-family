package hud

// Zustand je Spieler, einmal je Frame fortgeschrieben (Listen): Texte (nur bei geändertem Schlüssel neu), Meldungen,
// Hervorhebung des Goldtopfs, „gesichert“, Hinweise, Radar und Bürger-Labels. Draw liest nur.

import (
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// savedSeconds: so lange zeigt die Uhr nach dem Speichern „gesichert“ (B-186).
const savedSeconds = 2.0

// playerColors sind die Farben der Spieler (src/core/constants.ts › PLAYER_COLORS).
var playerColors = [...]uint32{0x3a86ff, 0xff006e, 0xfb5607, 0x8338ec}

// label ist ein Text in der Welt (Beruf über einem Bauern, Händler) mit Ort in Units.
type label struct {
	x, y float64 // y: Höhe über dem Boden in Units
	text string
	dot  uint32 // Farbe des Berufssymbols, 0 = keins
}

// seatState ist alles, was das HUD für einen Spieler (Seat.Player) zeichnet.
type seatState struct {
	active          bool
	monarch, stage  int
	device          view.DeviceID
	x, hp           float64
	down, flash     bool
	color           uint32
	widthUnits      float64
	goldS, clockS   string
	fightS, sharedS string
	placeS, citS    string
	travelS, evS    string
	barS, hintS     string
	guideS          string
	boss            bossView
	bossS           string
	guide           guideHint
	banner          bannerQueue
	savedUntil      float64
	gold            goldFlash
	radar           []radarMarker
	labels          []label

	mGold     memo[goldKey]
	mClock    memo[clockKey]
	mFight    memo[waveKey]
	mShared   memo[stockKey]
	mPlace    memo[placeKey]
	mCit      memo[hubKey]
	mTravel   memo[travelKey]
	mEvents   memo[eventsKey]
	mBar      memo[barKey]
	mHint     memo[hintKey]
	mBoss     memo[bossView]
	mMerchant memo[placeKey]

	menu    menuState
	entries []menuEntry
	mMenu   memo[menuKey]
	menuV   menuView
	slots   [4]slotView
}

// listen schreibt den Zustand aller Spieler des Frames fort (view.Listener).
func (h *hud) Listen(f *view.Frame) {
	h.paint.outline.Sweep()
	h.lang = view.Settings{Language: f.Settings.Language}.Normalized().Language
	h.cells = min(len(f.Seats), view.MaxPlayers) // Pause zeigt das Pause-Menü (menu), kein eigenes Band
	h.joinS = h.mJoin.get(newJoinKey(h.lang, f.Unseated), joinFromKey)
	h.cues(f)
	var seen [view.MaxPlayers]bool
	for _, s := range f.Seats {
		if s.Player < 0 || s.Player >= view.MaxPlayers {
			continue
		}
		seen[s.Player] = true
		h.seats[s.Player].update(h, f, s)
	}
	for p := range h.seats {
		if !seen[p] {
			h.seats[p].active, h.seats[p].menu = false, menuState{}
		}
	}
	for p := range h.seats {
		h.seats[p].banner.step(f.Clock)
	}
}

// cues: gespeichert setzt „gesichert“, Fehler und Ablehnungen kommen als Meldung (Spieler -1: an alle).
func (h *hud) cues(f *view.Frame) {
	for _, c := range f.Cues {
		for p := range h.seats {
			st := &h.seats[p]
			mine := c.Player < 0 || c.Player == p
			switch {
			case c.Kind == view.CueSaved:
				st.savedUntil = f.Clock + savedSeconds
			case (c.Kind == view.CueDenied || c.Kind == view.CueSaveFailed || c.Kind == view.CueError) && mine && c.Text != "":
				st.banner.add(c.Text)
			}
		}
	}
}

// update liest den Platz s aus dem Frame.
func (st *seatState) update(h *hud, f *view.Frame, s view.Seat) {
	stage, ok := f.Stage(s.Stage)
	m := f.Monarch(s.Player)
	st.active = ok && stage.World != nil && m != nil
	if !st.active {
		st.menu = menuState{}
		return
	}
	w, lang := stage.World, h.lang
	st.monarch, st.stage, st.device, st.x = m.Index, s.Stage, s.Device, m.X
	st.color = playerColors[m.Index%len(playerColors)]
	e := sim.EconomyOf(w)
	st.card(lang, e, m, f.Clock)
	st.world(lang, w, e, f.Clock)
	st.events(lang, f, stage)
	st.hints(lang, w, m, &h.seen)
	st.radar = radarMarkers(st.radar, w, m.Index)
	st.widthUnits = w.WidthUnits
	st.citizenLabels(lang, w)
	st.skills(lang, w, m)
}

// card: Werte der Spielerkarte (Leben, Gold, Topf) und der Skill-Leiste.
func (st *seatState) card(lang string, e sim.Economy, m *sim.Player, now float64) {
	st.hp, st.down = 0, m.RespawnIn > 0
	if m.MaxHP > 0 {
		st.hp = max(0, min(1, m.HP/m.MaxHP))
	}
	purse := e.PurseMax
	if purse <= 0 {
		purse = purseMaxGold
	}
	out := 0
	if st.down {
		out = int(ceilPos(m.RespawnIn))
	}
	st.goldS = st.mGold.get(goldKey{lang, m.Gold, purse, out}, goldText)
	st.flash = st.gold.step(m.Gold, now)
	st.slots = slotViews(m.Slots, m.Cooldowns)
	st.barS = st.mBar.get(newBarKey(lang, st.slots), slotBar)
}

// world: Uhr, Kampf, Vorrat, Hub und Bürger, Ort, Reise, Boss und Events der Stufe.
func (st *seatState) world(lang string, w *sim.World, e sim.Economy, now float64) {
	st.clockS = st.mClock.get(newClockKey(lang, w.Cycle, w.CycleSpeed, w.Aggression, now < st.savedUntil), clockText)
	st.fightS = st.mFight.get(waveKey{lang, w.Wave, len(w.Enemies) + len(w.SpawnQueue)}, waveText)
	st.sharedS = st.mShared.get(stockKey{lang, w.Biome.PrimaryResource, stockOf(w.Stock), e.StockMax, w.SkillPoints}, sharedText)
	st.citS = st.mCit.get(newHubKey(lang, e, w.Merchant), citizensText)
	st.placeS = st.mPlace.get(placeKey{lang, w.Biome.ID, w.Biome.Name, st.monarch}, placeText)
	st.travelS = st.mTravel.get(newTravelKey(lang, w.Travel), travelText)
	st.evS = st.mEvents.get(newEventsKey(lang, w.NightEvents, w.CycleSpeed, w.Merchant), eventBanner)
	st.boss = bossBar(lang, w.Enemies)
	st.bossS = st.mBoss.get(st.boss, bossTitle)
}

// events: Meldungen aus den Ereignissen der Stufe des Spielers.
func (st *seatState) events(lang string, f *view.Frame, stage *view.Stage) {
	for e := range f.EventsOf(stage.Index) {
		if t := eventText(lang, e, stage.World.Biome.ID); t != "" {
			st.banner.add(t)
		}
	}
}

// hints: Aktions-Hinweis über dem Monarchen und Hinweis der geführten ersten Nacht.
func (st *seatState) hints(lang string, w *sim.World, m *sim.Player, seen *guideSeen) {
	st.hintS = st.mHint.get(actionHint(lang, w, m), hintText)
	st.guide = stepGuide(st.guide, w, m.X, m.RespawnIn > 0, seen)
	st.guideS = ""
	if st.guide.id != guideNone {
		st.guideS = tr(lang, guideKeys[st.guide.id])
	}
}

// Farben der Berufssymbole (citizenView.ts › ICON_COLOR) und Höhen über dem Boden in Units.
var professionColors = map[string]uint32{"miner": 0x8d99ae, "builder": 0xe9c46a, "craftsman": 0xb87333}

const (
	badgeUnits    = 150.0 / 32
	merchantUnits = 116.0 / 32
	merchantShift = 8.0 // Hub-Mitte + 8 Units (docs/rules/buerger.md)
)

// citizenLabels: Händler im Hub zuerst (Vorrang beim Budget maxLabels), dann der Beruf über jedem Bauern mit Beruf
// (citizenView.ts, Text ohne Figur).
func (st *seatState) citizenLabels(lang string, w *sim.World) {
	st.labels = st.labels[:0]
	if m := w.Merchant; m != nil {
		text := st.mMerchant.get(placeKey{lang: lang, id: m.Resource}, merchantLabel)
		st.labels = append(st.labels, label{x: w.Castle.X + merchantShift, y: merchantUnits, text: text})
	}
	for _, t := range w.Troops {
		if t == nil || t.Profession == "" {
			continue
		}
		dot, ok := professionColors[t.Profession]
		if !ok {
			dot = 0xcccccc
		}
		st.labels = append(st.labels, label{x: t.X, y: badgeUnits, text: professionText(lang, t.Profession, true), dot: dot})
	}
}

// merchantLabel ist der kurze Händler-Text über dem Hub (Schlüssel: Sprache und Rohstoff in id).
func merchantLabel(k placeKey) string { return merchantText(k.lang, k.id, 0, 0, true) }

// skills: Einträge des offenen Skill-Menüs und sein Text.
func (st *seatState) skills(lang string, w *sim.World, m *sim.Player) {
	if !st.menu.open {
		return
	}
	points := st.refreshEntries(w, m)
	k := newMenuKey(lang, st.device, st.menu.cursor, points, st.entries, st.slots)
	if !st.mMenu.set || st.mMenu.key != k {
		st.mMenu.key, st.mMenu.set = k, true
		st.menuV = buildMenu(k, st.entries)
	}
}

// refreshEntries liest die Einträge des Menüs aus der Simulation und liefert die freien Punkte.
func (st *seatState) refreshEntries(w *sim.World, m *sim.Player) int {
	points := sim.AvailablePoints(w, m)
	can := func(id string) bool { return sim.CanLearn(w, m, id) == nil }
	respec := m.RespawnIn <= 0 && sim.CanRespec(w, m) == nil
	st.entries = menuEntries(st.entries, m.Skills, points, can, respec)
	return points
}

// ceilPos rundet auf und begrenzt nach unten auf 0.
func ceilPos(x float64) float64 {
	if x <= 0 {
		return 0
	}
	n := float64(int(x))
	if n < x {
		n++
	}
	return n
}
