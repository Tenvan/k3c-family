package net

import (
	"encoding/json"
	"math"
	"sync"

	"k3c/engine/sim"
)

// Zustand ohne JSON-Roundtrip (B-731 Stufe 1): die Map direkt aus sim.World und sim.Economy, mit Schlüsseln und Go-Typen
// wie nach json.Marshal + json.Unmarshal (Zahlen float64, Listen []any, nil als `null`, `omitempty` wie die Tags der
// Sim). Orakel (stateOfJSON) und Vergleichs-/Reflection-Tests: state_oracle_test.go, state_reflect_test.go.

// stateOf ist der Zustand für `snap`: die Welt ohne seed, biome, level, rng, widthUnits, mit events und depth, oben
// mit dem Wirtschaftsstand aus sim.EconomyOf (stockMax, hubLevel, hubUpgrade, danger; W5.1), je
// Spieler mit points und actions (actions.go), je Bauplatz payable und je Knoten minable (payable.go), Bauplätze mit hubLevel
// und nur bis zur nächsten Hub-Stufe (filterSites; nach addPayable, beide ordnen über die Position in w.Sites zu). Das Feld
// `free` der Spieler (B-059) fällt weg, der Zustand der Monarchen steht in `seats`. timescale > 0 (nur Dev-Mode) steht
// als `devTimescale` darin, daneben die Dev-Pause als `devPaused` (B-231).
func stateOf(w *sim.World, timescale int, paused bool) map[string]any {
	s := worldState(w)
	economyInto(sim.EconomyOf(w), s)
	addActions(w, asList(s["players"]))
	addPayable(w, s)
	filterSites(w, s)
	s["depth"] = w.Biome.Depth
	if timescale > 0 {
		s["devTimescale"] = timescale
		s["devPaused"] = paused // immer mitsenden: das Delta kennt kein Entfernen von Feldern
	}
	return s
}

// worldState ist json.Marshal(w) als Map, ohne seed, widthUnits und players[].free.
func worldState(w *sim.World) map[string]any {
	s := make(map[string]any, 48)
	s["time"], s["cycleSpeed"], s["nextId"], s["hubX"] = num(w.Time), num(w.CycleSpeed), inum(w.NextID), num(w.HubX)
	putInt(s, "armorLevel", w.ArmorLevel)
	s["cycle"] = map[string]any{"phase": word(w.Cycle.Phase), "day": inum(w.Cycle.Day), "progress": num(w.Cycle.Progress),
		"secondsLeft": num(w.Cycle.SecondsLeft)}
	s["aggression"] = ptrVal(w.Aggression, num)
	s["wave"] = inum(w.Wave)
	s["players"] = ptrList(w.Players, playerState)
	s["coins"] = ptrList(w.Coins, coinState)
	s["troops"] = ptrList(w.Troops, troopState)
	s["nodes"] = ptrList(w.Nodes, nodeState)
	s["sites"] = ptrList(w.Sites, siteState)
	s["castle"] = map[string]any{"id": inum(w.Castle.ID), "x": num(w.Castle.X), "hp": num(w.Castle.HP), "maxHp": num(w.Castle.MaxHP)}
	s["enemies"] = ptrList(w.Enemies, enemyState)
	s["projectiles"] = ptrList(w.Projectiles, projectileState)
	s["pickups"] = ptrList(w.Pickups, func(p *sim.Pickup) map[string]any {
		return map[string]any{"id": inum(p.ID), "kind": word(p.Kind), "x": num(p.X)}
	})
	s["camps"] = ptrList(w.Camps, func(c *sim.Camp) map[string]any { return map[string]any{"x": num(c.X), "respawnIn": num(c.RespawnIn)} })
	s["portals"] = valList(w.Portals, num)
	s["spawnQueue"] = valList(w.SpawnQueue, func(q sim.QueuedSpawn) any {
		return map[string]any{"kind": word(q.Kind), "x": num(q.X), "at": num(q.At)}
	})
	if len(w.Storms) > 0 {
		s["storms"] = ptrList(w.Storms, stormState)
	}
	if len(w.Hazards) > 0 {
		s["hazards"] = ptrList(w.Hazards, hazardState)
	}
	s["stock"] = ptrObj(w.Stock, stockState)
	s["skillPoints"] = inum(w.SkillPoints)
	s["travel"] = ptrObj(w.Travel, travelState)
	if w.Merchant != nil {
		s["merchant"] = merchantState(w.Merchant)
	}
	if len(w.Drops) > 0 {
		s["drops"] = ptrList(w.Drops, dropState)
	}
	s["events"] = valList(w.Events, eventState)
	if len(w.NightEvents) > 0 {
		s["nightEvents"] = valList(w.NightEvents, func(e sim.ActiveEvent) any {
			return map[string]any{"id": word(e.ID), "secondsLeft": num(e.SecondsLeft)}
		})
	}
	if is := w.IslandSwitch; is != nil {
		s["islandSwitch"] = map[string]any{"open": is.Open, "progress": num(is.Progress), "ready": is.Ready}
	}
	putInt(s, "eventsDropped", w.EventsDropped)
	return s
}

// economyInto ergänzt s um json.Marshal(e) (die Felder der Wirtschaft, Werte unverändert aus der Sim).
func economyInto(e sim.Economy, s map[string]any) {
	putInt(s, "stockMax", e.StockMax)
	putInt(s, "purseMax", e.PurseMax)
	s["hubLevel"] = inum(e.HubLevel)
	putInt(s, "hubStep", e.HubStep)
	if u := e.HubUpgrade; u != nil {
		m := map[string]any{"gold": inum(u.Gold), "material": stockState(&u.Material), "paid": inum(u.Paid),
			"steps": inum(u.Steps)}
		putStr(m, "state", u.State)
		s["hubUpgrade"] = m
	}
	putBool(s, "danger", e.Danger)
	s["fighters"], s["troopLimit"] = inum(e.Fighters), inum(e.TroopLimit)
}

func playerState(p *sim.Player) map[string]any {
	m := make(map[string]any, 24) // Platz für points, actions und auras (addActions)
	m["id"], m["index"], m["x"], m["vx"] = inum(p.ID), inum(p.Index), num(p.X), num(p.VX)
	m["facing"], m["gold"], m["hp"], m["maxHp"] = inum(p.Facing), inum(p.Gold), num(p.HP), num(p.MaxHP)
	m["respawnIn"], m["payCooldown"], m["paying"] = num(p.RespawnIn), num(p.PayCooldown), p.Paying
	m["payKey"], m["payAmount"] = ptrVal(p.PayKey, func(s string) any { return s }), inum(p.PayAmount)
	if len(p.Skills) > 0 {
		m["skills"] = valList(p.Skills, word)
	}
	if len(p.Slots) > 0 {
		m["slots"] = valList(p.Slots, word)
	}
	putInt(m, "skillBase", p.SkillBase)
	putFloat(m, "attackCooldown", p.AttackCooldown)
	if len(p.Cooldowns) > 0 {
		m["cooldowns"] = valList(p.Cooldowns, num)
	}
	putFloat(m, "shield", p.Shield)
	putFloat(m, "shieldFor", p.ShieldFor)
	putFloat(m, "lastStandFor", p.LastStandFor)
	putFloat(m, "stealthFor", p.StealthFor)
	putFloat(m, "reviveProgress", p.ReviveProgress)
	return m
}

func coinState(c *sim.Coin) map[string]any {
	return map[string]any{"id": inum(c.ID), "x": num(c.X), "blockedPlayerId": ptrVal(c.BlockedPlayerID, inum),
		"blockedUntil": num(c.BlockedUntil)}
}

func troopState(t *sim.Troop) map[string]any {
	m := make(map[string]any, 13)
	m["id"], m["kind"], m["x"], m["hp"], m["maxHp"] = inum(t.ID), word(t.Kind), num(t.X), num(t.HP), num(t.MaxHP)
	m["anchorX"], m["targetX"], m["job"], m["cooldown"] = num(t.AnchorX), num(t.TargetX), ptrObj(t.Job, jobState), num(t.Cooldown)
	m["towerId"], m["paidGold"] = ptrVal(t.TowerID, inum), inum(t.PaidGold)
	putStr(m, "profession", t.Profession)
	putInt(m, "workSite", t.WorkSite)
	return m
}

func jobState(j *sim.Job) map[string]any {
	m := map[string]any{"type": word(j.Type)}
	putInt(m, "siteId", j.SiteID)
	putInt(m, "dropId", j.DropID)
	putInt(m, "nodeId", j.NodeID)
	putStr(m, "resource", j.Resource)
	putInt(m, "amount", j.Amount)
	return m
}

func nodeState(n *sim.ResourceNode) map[string]any {
	m := map[string]any{"id": inum(n.ID), "kind": word(n.Kind), "x": num(n.X), "paidGold": inum(n.PaidGold),
		"marked": n.Marked, "workerId": ptrVal(n.WorkerID, inum), "progress": num(n.Progress)}
	if n.MarkedBy != nil {
		m["markedBy"] = inum(*n.MarkedBy)
	}
	return m
}

func siteState(s *sim.Site) map[string]any {
	m := make(map[string]any, 16) // Platz für payable, clearing und hubLevel (addPayable, filterSites)
	m["id"], m["kind"], m["x"], m["state"] = inum(s.ID), word(s.Kind), num(s.X), word(s.State)
	m["paidGold"], m["buildProgress"], m["hp"], m["maxHp"] = inum(s.PaidGold), num(s.BuildProgress), num(s.HP), num(s.MaxHP)
	m["workerId"], m["bows"], m["bowPaidGold"] = ptrVal(s.WorkerID, inum), inum(s.Bows), inum(s.BowPaidGold)
	putFloat(m, "craftDone", s.CraftDone)
	if len(s.OfferPaid) > 0 {
		m["offerPaid"] = valList(s.OfferPaid, inum)
	}
	putFloat(m, "offerDone", s.OfferDone)
	putInt(m, "swords", s.Swords)
	putInt(m, "swordPaidGold", s.SwordPaidGold)
	putFloat(m, "swordDone", s.SwordDone)
	putInt(m, "level", s.Level)
	putStr(m, "upgrade", s.Upgrade)
	putInt(m, "upgradePaid", s.UpgradePaid)
	putBool(m, "ruin", s.Ruin)
	return m
}

func enemyState(e *sim.Enemy) map[string]any {
	m := make(map[string]any, 16)
	m["id"], m["kind"], m["x"], m["hp"], m["maxHp"] = inum(e.ID), word(e.Kind), num(e.X), num(e.HP), num(e.MaxHP)
	m["damage"], m["speed"], m["range"], m["traits"] = num(e.Damage), num(e.Speed), num(e.Range), valList(e.Traits, word)
	m["cooldown"], m["fleeing"], m["carriedGold"], m["homeX"] = num(e.Cooldown), e.Fleeing, inum(e.CarriedGold), num(e.HomeX)
	if e.Carried != nil {
		m["carried"] = dropState(e.Carried)
	}
	putFloat(m, "stun", e.Stun)
	putInt(m, "tauntId", e.TauntID)
	putFloat(m, "tauntFor", e.TauntFor)
	putFloat(m, "slow", e.Slow)
	putFloat(m, "slowFor", e.SlowFor)
	putFloat(m, "aoeIn", e.AoeIn)
	putFloat(m, "spawned", e.Spawned)
	putInt(m, "phase", e.Phase)
	if e.Warn != nil {
		m["warn"] = map[string]any{"x": num(e.Warn.X), "r": num(e.Warn.R), "in": num(e.Warn.In)}
	}
	putBool(m, "boss", e.Boss)
	putBool(m, "guard", e.Guard)
	return m
}

func dropState(d *sim.Drop) map[string]any {
	m := map[string]any{"id": inum(d.ID), "kind": word(d.Kind), "x": num(d.X)}
	putInt(m, "workSite", d.WorkSite)
	return m
}

func projectileState(p *sim.Projectile) map[string]any {
	m := map[string]any{"id": inum(p.ID), "x": num(p.X), "targetId": inum(p.TargetID), "team": word(p.Team),
		"damage": num(p.Damage), "speed": num(p.Speed)}
	putFloat(m, "splash", p.Splash)
	return m
}

func stormState(s *sim.Storm) map[string]any {
	return map[string]any{"x": num(s.X), "radius": num(s.Radius), "perSecond": num(s.PerSecond), "left": num(s.Left),
		"owner": inum(s.Owner)}
}

func hazardState(h *sim.Hazard) map[string]any {
	return map[string]any{"x": num(h.X), "radius": num(h.Radius), "damagePerSecond": num(h.DamagePerSecond),
		"secondsLeft": num(h.SecondsLeft)}
}

func stockState(s *sim.Stock) map[string]any {
	m := map[string]any{"wood": inum(s.Wood), "stone": inum(s.Stone), "copper": inum(s.Copper)}
	putInt(m, "iron", s.Iron)
	putInt(m, "crystal", s.Crystal)
	return m
}

func travelState(t *sim.Travel) map[string]any {
	return map[string]any{"x": num(t.X), "toDepth": inum(t.ToDepth), "via": word(t.Via), "progress": num(t.Progress)}
}

func merchantState(mer *sim.Merchant) map[string]any {
	m := map[string]any{"resource": word(mer.Resource), "leaves": inum(mer.Leaves), "hp": num(mer.HP), "maxHp": num(mer.MaxHP)}
	putInt(m, "buyPaid", mer.BuyPaid)
	putBool(m, "raid", mer.Raid)
	return m
}

// eventState ist ein Ereignis wie nach dem JSON-Roundtrip (Werte über jsonValue, nil-Ereignis als `null`).
func eventState(e sim.Event) any {
	if e == nil {
		return nil
	}
	return jsonMap(e)
}

func jsonMap(m map[string]any) map[string]any {
	out := make(map[string]any, len(m))
	for k, v := range m {
		out[k] = jsonValue(v)
	}
	return out
}

// jsonValue ist v nach json.Marshal + json.Unmarshal; bekannte Typen direkt, alles andere über den JSON-Weg.
func jsonValue(v any) any {
	switch x := v.(type) {
	case nil, bool, string:
		return x
	case float64:
		return num(x)
	case int:
		return inum(x)
	case []string:
		return valList(x, word)
	case []any:
		return valList(x, jsonValue)
	case map[string]any:
		return eventState(x)
	case sim.Event:
		return eventState(x)
	}
	raw, err := json.Marshal(v)
	if err != nil {
		return nil
	}
	var out any
	_ = json.Unmarshal(raw, &out)
	return out
}

// ptrList ist eine Liste von Zeigern als []any (nil-Liste und nil-Einträge wie JSON `null`).
func ptrList[T any](l []*T, f func(*T) map[string]any) any {
	if l == nil {
		return nil
	}
	out := make([]any, len(l))
	for i, e := range l {
		if e != nil {
			out[i] = f(e)
		}
	}
	return out
}

// ptrObj ist ein Zeiger auf ein Objekt: nil wie JSON `null`.
func ptrObj[T any](p *T, f func(*T) map[string]any) any {
	if p == nil {
		return nil
	}
	return f(p)
}

// valList ist eine Liste von Werten als []any (nil-Liste wie JSON `null`).
func valList[T any](l []T, f func(T) any) any {
	if l == nil {
		return nil
	}
	out := make([]any, len(l))
	for i, e := range l {
		out[i] = f(e)
	}
	return out
}

// ptrVal ist ein Zeiger auf einen Wert: nil wie JSON `null`.
func ptrVal[T any](p *T, f func(T) any) any {
	if p == nil {
		return nil
	}
	return f(*p)
}

// boxed hält die ganzen Zahlen 1 bis 4095 fertig als any (float64), num nimmt sie: Ids, Gold, Lebenspunkte und Zähler
// kosten so keine Allokation (0 verpackt die Laufzeit selbst ohne, -0 bleibt mit Vorzeichen).
var boxed = func() []any {
	b := make([]any, 4096)
	for i := 1; i < len(b); i++ {
		b[i] = float64(i)
	}
	return b
}()

func num(v float64) any {
	if v >= 1 && v < float64(len(boxed)) && v == math.Trunc(v) {
		return boxed[int(v)]
	}
	return v
}

func inum(i int) any { return num(float64(i)) }

// words hält Texte aus festen Mengen (kind, state, team, Phase, Skill-Ids aus data/) fertig als any, word nimmt sie;
// höchstens 1024, darüber wird jeder Text neu verpackt.
var words = struct {
	sync.RWMutex
	m map[string]any
}{m: map[string]any{}}

func word(s string) any {
	words.RLock()
	v, ok := words.m[s]
	words.RUnlock()
	if ok {
		return v
	}
	v = s
	words.Lock()
	if len(words.m) < 1024 {
		words.m[s] = v
	}
	words.Unlock()
	return v
}

// put* setzen ein Feld mit `omitempty`: der Nullwert fehlt.
func putInt(m map[string]any, k string, v int) {
	if v != 0 {
		m[k] = inum(v)
	}
}

func putFloat(m map[string]any, k string, v float64) {
	if v != 0 {
		m[k] = num(v)
	}
}

func putStr(m map[string]any, k, v string) {
	if v != "" {
		m[k] = word(v)
	}
}

func putBool(m map[string]any, k string, v bool) {
	if v {
		m[k] = true
	}
}
