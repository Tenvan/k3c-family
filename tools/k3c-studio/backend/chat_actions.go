package backend

import (
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"strings"
	"sync"
)

// Aktionskarten des Chats (B-679): Der Chat hat keine Werkzeuge. Er nennt Bearbeitungen des Studios in einem Block
// ```aktionen; das Backend prüft jede mit derselben Prüfung wie die Oberfläche (Trockenlauf), merkt den Stand der Datei
// und zeigt sie als Karte. Ausgeführt wird erst, wenn 🧑 „Ausführen“ drückt, über dieselbe Funktion wie der Knopf der
// Seite, höchstens einmal. Karten leben nur im Speicher (wie die Vorschlagskarten).

const (
	actionsTag   = "aktionen"
	actionsFence = "```" + actionsTag
	actionsMax   = 12 // Aktionen je Antwort
	actionsKeep  = 40 // Karten je Chat
)

// Zustand einer Karte.
const (
	actionOpen    = "offen"
	actionRunning = "laeuft"
	actionDone    = "ausgefuehrt"
	actionFailed  = "fehler"
	actionDropped = "verworfen"
)

// ChatAction ist eine Aktionskarte (TS: api/actionTypes.ts).
type ChatAction struct {
	ID      string   `json:"id"`
	Art     string   `json:"art"`
	Title   string   `json:"titel"`
	Details []string `json:"details"`
	Block   string   `json:"gesperrt,omitempty"` // Grund, warum sie nicht ausführbar ist (Prüfung vorab)
	Status  string   `json:"status"`
	Result  string   `json:"ergebnis,omitempty"`
}

// actionPlan ist eine geprüfte Aktion: Anzeige und der Aufruf mit dem Stand zur Zeit der Prüfung.
type actionPlan struct {
	title   string
	details []string
	run     func() (string, error)
}

// actionSpec ist eine Art des Katalogs: Felder und Zweck für die Instruktion, prepare prüft die Felder vorab.
type actionSpec struct {
	art     string
	fields  string
	help    string
	prepare func(a *App, raw json.RawMessage) (actionPlan, error)
}

// actionCatalog sind alle Arten; die Teile stehen bei ihren Seiten (chat_actions_assets.go, chat_actions_world.go).
func actionCatalog() []actionSpec {
	return append(append(assetActions(), worldActions()...), hubActions()...)
}

func findSpec(art string) (actionSpec, bool) {
	for _, s := range actionCatalog() {
		if s.art == art {
			return s, true
		}
	}
	return actionSpec{}, false
}

// actionStore hält die Karten je Chat.
type actionStore struct {
	mu    sync.Mutex
	seq   int
	chats map[string][]*actionEntry
}

type actionEntry struct {
	card ChatAction
	run  func() (string, error)
}

var actionStores sync.Map

func (a *App) actions() *actionStore {
	v, _ := actionStores.LoadOrStore(a, &actionStore{chats: map[string][]*actionEntry{}})
	return v.(*actionStore)
}

// fenceBody liefert den Inhalt des letzten Blocks fence einer Antwort.
func fenceBody(text, fence string) (string, bool) {
	i := strings.LastIndex(text, fence)
	if i < 0 {
		return "", false
	}
	body := text[i+len(fence):]
	if j := strings.Index(body, "```"); j >= 0 {
		body = body[:j]
	}
	return body, true
}

// withoutFence entfernt alle Blöcke fence (z. B. damit „Als Entwurf laden“ nicht den Aktionsblock liest).
func withoutFence(text, fence string) string {
	for {
		i := strings.Index(text, fence)
		if i < 0 {
			return text
		}
		end := len(text)
		if j := strings.Index(text[i+len(fence):], "```"); j >= 0 {
			end = i + len(fence) + j + 3
		}
		text = text[:i] + text[end:]
	}
}

// parseActions liest die Aktionen aus dem letzten Block ```aktionen; ok = false, wenn ein Block da, aber nicht lesbar ist.
func parseActions(text string) (list []json.RawMessage, found, ok bool) {
	body, found := fenceBody(text, actionsFence)
	if !found {
		return nil, false, true
	}
	var v struct {
		List []json.RawMessage `json:"aktionen"`
	}
	if err := json.Unmarshal([]byte(body), &v); err != nil {
		return nil, true, false
	}
	return v.List[:min(len(v.List), actionsMax)], true, true
}

// prepareAction prüft eine Aktion; eine unbekannte Art oder ein Befund sperrt die Karte mit Grund.
func (a *App) prepareAction(raw json.RawMessage) (ChatAction, func() (string, error)) {
	var head struct {
		Art string `json:"art"`
	}
	_ = json.Unmarshal(raw, &head)
	card := ChatAction{Art: head.Art, Title: head.Art, Details: []string{}, Status: actionOpen}
	spec, ok := findSpec(head.Art)
	if !ok {
		card.Block = fmt.Sprintf("unbekannte Art %q", head.Art)
		return card, nil
	}
	plan, err := spec.prepare(a, raw)
	card.Title = plan.title
	if card.Title == "" {
		card.Title = spec.help
	}
	if plan.details != nil {
		card.Details = plan.details
	}
	if err != nil {
		card.Block = err.Error()
		return card, nil
	}
	return card, plan.run
}

// applyActions macht aus der Antwort Karten des Chats chat und liefert den Hinweis, der an die Antwort kommt ("" ohne Block).
func (a *App) applyActions(chat, answer string) string {
	list, found, ok := parseActions(answer)
	switch {
	case !found:
		return ""
	case !ok:
		return "\n\n(Studio: Block ```aktionen nicht lesbar, keine Karten.)"
	}
	entries := make([]*actionEntry, 0, len(list))
	blocked := 0
	for _, raw := range list { // prüfen ohne Sperre: manche Prüfungen lesen Dateien oder fragen das Netz
		card, run := a.prepareAction(raw)
		if card.Block != "" {
			blocked++
		}
		entries = append(entries, &actionEntry{card: card, run: run})
	}
	st := a.actions()
	st.mu.Lock()
	defer st.mu.Unlock()
	for _, e := range entries {
		st.seq++
		e.card.ID = fmt.Sprintf("a%d", st.seq)
		st.chats[chat] = append(st.chats[chat], e)
	}
	if n := len(st.chats[chat]); n > actionsKeep {
		st.chats[chat] = st.chats[chat][n-actionsKeep:]
	}
	slog.Info("📨 Chat-Aktionen vorbereitet", "ns", "main", "chat", chat, "anzahl", len(list), "gesperrt", blocked)
	return fmt.Sprintf("\n\n(Studio: %d Aktionen als Karten vorbereitet, %d gesperrt; ausgeführt wird nur per Knopf.)", len(list), blocked)
}

// ChatActions liefert die Karten des Chats chat, die älteste zuerst.
func (a *App) ChatActions(chat string) []ChatAction {
	st := a.actions()
	st.mu.Lock()
	defer st.mu.Unlock()
	out := []ChatAction{}
	for _, e := range st.chats[chat] {
		out = append(out, e.card)
	}
	return out
}

// take sperrt die offene Karte id für einen Lauf (oder Verwerfen).
func (st *actionStore) take(chat, id, next string) (*actionEntry, error) {
	st.mu.Lock()
	defer st.mu.Unlock()
	for _, e := range st.chats[chat] {
		if e.card.ID != id {
			continue
		}
		switch {
		case e.card.Status != actionOpen:
			return nil, fmt.Errorf("aktion %s ist schon %s", id, e.card.Status)
		case e.card.Block != "" && next == actionRunning:
			return nil, errors.New("aktion gesperrt: " + e.card.Block)
		}
		e.card.Status = next
		return e, nil
	}
	return nil, fmt.Errorf("keine Aktion %s in diesem Chat", id)
}

// ChatActionRun führt die Karte id des Chats chat aus (nach „Ausführen“), höchstens einmal.
func (a *App) ChatActionRun(chat, id string) (ChatAction, error) {
	st := a.actions()
	e, err := st.take(chat, id, actionRunning)
	if err != nil {
		return ChatAction{}, err
	}
	res, err := safeRun(e.run)
	st.mu.Lock()
	defer st.mu.Unlock()
	if err != nil {
		e.card.Status, e.card.Result = actionFailed, err.Error()
		slog.Warn("❌ Chat-Aktion fehlgeschlagen", "ns", "main", "art", e.card.Art, "fehler", err)
		return e.card, nil
	}
	e.card.Status, e.card.Result = actionDone, res
	slog.Info("✅ Chat-Aktion ausgeführt", "ns", "main", "art", e.card.Art, "titel", e.card.Title)
	return e.card, nil
}

// safeRun fängt eine Panik des Aufrufs ab: die Karte endet dann mit Fehler statt auf „läuft“ zu bleiben.
func safeRun(run func() (string, error)) (res string, err error) {
	defer func() {
		if r := recover(); r != nil {
			slog.Error("💥 Chat-Aktion abgestürzt", "ns", "main", "panik", r)
			err = fmt.Errorf("abgestürzt: %v", r)
		}
	}()
	return run()
}

// forget räumt die Karten eines gelöschten Chats weg.
func (st *actionStore) forget(chat string) {
	st.mu.Lock()
	delete(st.chats, chat)
	st.mu.Unlock()
}

// ChatActionDrop verwirft die Karte id des Chats chat.
func (a *App) ChatActionDrop(chat, id string) (ChatAction, error) {
	e, err := a.actions().take(chat, id, actionDropped)
	if err != nil {
		return ChatAction{}, err
	}
	return e.card, nil
}

// fields liest die Felder einer Aktion streng (unbekannte Felder außer art sind ein Fehler).
func fields(raw json.RawMessage, v any) error {
	var m map[string]json.RawMessage
	if err := json.Unmarshal(raw, &m); err != nil {
		return errors.New("aktion nicht lesbar: " + err.Error())
	}
	delete(m, "art")
	clean, _ := json.Marshal(m)
	dec := json.NewDecoder(strings.NewReader(string(clean)))
	dec.DisallowUnknownFields()
	if err := dec.Decode(v); err != nil {
		return errors.New("felder: " + err.Error())
	}
	return nil
}

// need meldet ein fehlendes Pflichtfeld.
func need(name, value string) error {
	if strings.TrimSpace(value) == "" {
		return fmt.Errorf("feld %q fehlt", name)
	}
	return nil
}
