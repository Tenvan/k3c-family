package menu

import (
	"strconv"

	"k3c/native/k3c-game/view"
)

// key ist eine Menü-Taste nach dem Zusammenfassen der Aktionen: B und Menu gehen beide zurück (Esc liefert beide im
// selben Update, das zählt einmal), View löscht in der Spielstand-Liste, Home (View + Menu, Pos1) führt zur Wurzel.
type key int

const (
	keyNone key = iota
	keyUp
	keyDown
	keyLeft
	keyRight
	keyOK
	keyBack
	keyHome
	keyDelete
)

// keyOf ordnet eine Aktion ihrer Menü-Taste zu; alles andere (Seiten, Skill-Menü, Vollbild) ist keyNone.
func keyOf(a view.Action) key {
	switch a {
	case view.ActionUp:
		return keyUp
	case view.ActionDown:
		return keyDown
	case view.ActionLeft:
		return keyLeft
	case view.ActionRight:
		return keyRight
	case view.ActionConfirm:
		return keyOK
	case view.ActionBack, view.ActionMenu:
		return keyBack
	case view.ActionHome:
		return keyHome
	case view.ActionView:
		return keyDelete
	}
	return keyNone
}

// handler bedient eine Taste eines Geräts und sagt, was danach geschieht (Stay, Push … mit dem nächsten Screen).
type handler interface {
	handle(d view.DeviceID, k key, s view.Session) (view.StepKind, view.Screen)
}

// base ist der gemeinsame Zustand der Screens: Cues des Updates, das zuletzt bedienende Gerät, Meldung und Version
// (jede Änderung erhöht ver; Draw baut seine Texte nur bei neuer Version oder Sprache neu).
type base struct {
	cues []view.Cue
	by   view.DeviceID
	byOK bool
	note note
	ver  int
}

// note ist die Meldung eines Screens: Schlüssel (Text in der Sprache) oder fertiger Text der Session (deutsch).
type note struct {
	key textKey
	arg string
	raw string // gesetzt: Text der Session, ersetzt key
	bad bool   // Fehler (rot) statt Hinweis
	set bool
}

// run gibt die Tasten aller Geräte in Druck-Reihenfolge an h, bis ein Schritt den Screen verlässt.
func (b *base) run(in view.Input, s view.Session, h handler) view.Step {
	b.cues = b.cues[:0]
	for _, di := range in.Devices {
		back := false
		for _, a := range di.Actions {
			k := keyOf(a)
			if k == keyNone || (k == keyBack && back) {
				continue
			}
			back = back || k == keyBack
			if !b.byOK || b.by != di.Device {
				b.by, b.byOK = di.Device, true
				b.ver++
			}
			if kind, next := h.handle(di.Device, k, s); kind != view.StepStay {
				return view.Step{Kind: kind, Next: next, Cues: b.cues}
			}
		}
	}
	return view.Step{Cues: b.cues}
}

// cue merkt einen Cue für dieses Update vor.
func (b *base) cue(kind view.CueKind, player int, text string) {
	b.cues = append(b.cues, view.Cue{Kind: kind, Player: player, Text: text})
}

// fail meldet einen Fehler der Session: rote Meldung und CueDenied mit dem Text.
func (b *base) fail(err error) {
	b.note = note{raw: err.Error(), bad: true, set: true}
	b.cue(view.CueDenied, -1, err.Error())
	b.ver++
}

// deny meldet eine abgelehnte Eingabe mit Text k.
func (b *base) deny(k textKey, arg string) {
	b.note = note{key: k, arg: arg, bad: true, set: true}
	b.cue(view.CueDenied, -1, tr1(view.DefaultLanguage, k, arg))
	b.ver++
}

// inform zeigt einen Hinweis (kein Fehler).
func (b *base) inform(k textKey, arg string) {
	b.note = note{key: k, arg: arg, set: true}
	b.ver++
}

// text liefert die Meldung in der Sprache lang ("" ohne Meldung).
func (n note) text(lang string) string {
	switch {
	case !n.set:
		return ""
	case n.raw != "":
		return n.raw
	}
	return tr1(lang, n.key, n.arg)
}

// deviceName nennt das Gerät für die Anzeige (Tastatur links/rechts, Pad n).
func deviceName(lang string, d view.DeviceID) string {
	if d.Kind == view.DeviceGamepad {
		return tr1(lang, txtPad, strconv.Itoa(d.Index+1))
	}
	if d.Index == 1 {
		return tr(lang, txtKeysRight)
	}
	return tr(lang, txtKeysLeft)
}

// move liefert die nächste wählbare Zeile ab sel in Richtung dir (-1 hoch, 1 runter) unter n Zeilen. Am Rand bleibt
// die Auswahl (wrap false) oder springt ans andere Ende; ausgegraute Zeilen (off) werden übersprungen. Ist keine andere
// Zeile wählbar, bleibt sel.
func move(sel, dir, n int, wrap bool, off func(int) bool) int {
	i := sel
	for range n {
		i += dir
		if wrap {
			i = (i%n + n) % n
		} else if i < 0 || i >= n {
			return sel
		}
		if off == nil || !off(i) {
			return i
		}
	}
	return sel
}

// first liefert die erste wählbare Zeile (0, wenn keine wählbar ist).
func first(n int, off func(int) bool) int {
	for i := range n {
		if off == nil || !off(i) {
			return i
		}
	}
	return 0
}

// window liefert die erste sichtbare Zeile einer Liste mit n Einträgen, von denen vis sichtbar sind, sodass sel
// möglichst mittig sichtbar ist.
func window(sel, n, vis int) int {
	if n <= vis || vis <= 0 {
		return 0
	}
	return max(0, min(sel-vis/2, n-vis))
}

// cycle verschiebt den Index i in einer Liste mit n Einträgen um dir und springt am Ende zum Anfang (A in Wahl-Zeilen).
func cycle(i, dir, n int) int {
	if n <= 0 {
		return 0
	}
	return ((i+dir)%n + n) % n
}
