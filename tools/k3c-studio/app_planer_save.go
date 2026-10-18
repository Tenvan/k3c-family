package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"os"
	"path/filepath"
	"sync"

	"k3c/engine/sim"
	assets "k3c/tools/assetkit"
	"k3c/tools/regeltabelle/tabelle"
)

// Speichern im Gebäude-Planer (B-462/AC-02, AC-03, AC-05, AC-06; WZ9.3): nur ein geprüfter Kandidat ohne Befund; erst
// data/hub.json (format-stabil, hub_patch.go), dann der erzeugte Block der Regeltabelle, beide atomar. Schreibregel:
// Hat sich eine der Dateien seit dem Laden geändert, wird nichts geschrieben.

// Meldungen des Speicherns.
const (
	rulesStale = "Regeltabelle veraltet, " + rulesTask
	regNotice  = "REG-Änderung: Commit mit Begründung (data/hub.json und Regeltabelle), task golden:update nur bei gewollter Änderung"
	staleHint  = "seit dem Laden geändert: nicht gespeichert, bitte neu laden (Eingaben bleiben bis dahin erhalten)"
)

// HubStand ist der Inhalt von data/hub.json und der Regeltabelle beim Laden (SHA-256, hex); leer = Datei fehlte.
type HubStand struct {
	Hub   string `json:"hub"`
	Rules string `json:"rules"`
}

// HubSaveResult ist das Ergebnis des Speicherns. Saved: data/hub.json geschrieben. Stale: eine Datei wurde extern
// geändert (nichts geschrieben). RulesError: hub.json gespeichert, der Block der Regeltabelle nicht. Notice: Hinweis
// auf die REG-Änderung. Unchanged: Kandidat bedeutet dasselbe wie die Datei, nichts geschrieben. Stand: neuer Stand
// beider Dateien.
type HubSaveResult struct {
	Saved      bool             `json:"saved"`
	Unchanged  bool             `json:"unchanged"`
	Stale      string           `json:"stale"`
	Findings   []HubFindingView `json:"findings"`
	Error      string           `json:"error"`
	RulesError string           `json:"rulesError"`
	Notice     string           `json:"notice"`
	Stand      HubStand         `json:"stand"`
}

// hubSaveMu reiht Speichervorgänge dieses Prozesses (Stand prüfen und schreiben am Stück).
var hubSaveMu sync.Mutex

// HubSave speichert den Kandidaten, wenn beide Dateien noch den Stand stand haben (Binding).
func (a *App) HubSave(candidate string, stand HubStand) HubSaveResult {
	hubSaveMu.Lock()
	defer hubSaveMu.Unlock()
	return hubSaveIn(a.root, []byte(candidate), stand)
}

// stamp ist der Stand eines Inhalts: SHA-256 (hex).
func stamp(b []byte) string {
	sum := sha256.Sum256(b)
	return hex.EncodeToString(sum[:])
}

// fileStamp ist der Stand der Datei rel unter root; leer, wenn sie nicht lesbar ist.
func fileStamp(root, rel string) string {
	b, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(rel)))
	if err != nil {
		return ""
	}
	return stamp(b)
}

// hubSaveIn prüft cand, vergleicht den Stand und schreibt data/hub.json und danach die Regeltabelle unter root.
func hubSaveIn(root string, cand []byte, stand HubStand) HubSaveResult {
	res := HubSaveResult{Findings: []HubFindingView{}, Stand: stand}
	h, err := sim.ParseHub(cand)
	if err != nil {
		res.Error = "Kandidat nicht lesbar: " + err.Error()
		return res
	}
	if res.Findings = findingViews(sim.ValidateHubData(h)); len(res.Findings) > 0 {
		res.Error = fmt.Sprintf("nicht gespeichert: %d Befunde der Prüfung", len(res.Findings))
		return res
	}
	hubPath := filepath.Join(root, filepath.FromSlash(hubFile))
	old, err := os.ReadFile(hubPath)
	if err != nil {
		res.Error = fmt.Sprintf("%s nicht lesbar: %v", hubFile, err)
		return res
	}
	if stale := staleFile(root, old, stand); stale != "" {
		res.Stale = stale + " " + staleHint
		return res
	}
	patched, err := patchJSON(old, cand)
	if err == nil && bytes.Equal(patched, old) {
		res.Unchanged = true // gleicher Inhalt: nichts geschrieben, keine REG-Änderung
		return res
	}
	if err == nil {
		err = assets.WriteAtomic(hubPath, patched)
	}
	if err != nil {
		res.Error = fmt.Sprintf("%s nicht geschrieben: %v", hubFile, err)
		return res
	}
	res.Saved, res.Notice, res.Stand.Hub = true, regNotice, stamp(patched)
	if err := writeRules(root, patched); err != nil {
		res.RulesError = fmt.Sprintf("%s (%v)", rulesStale, err)
	}
	res.Stand.Rules = fileStamp(root, rulesFile)
	return res
}

// staleFile nennt die Datei, die nicht mehr den Stand beim Laden hat; leer = beide unverändert.
func staleFile(root string, hub []byte, stand HubStand) string {
	if stamp(hub) != stand.Hub {
		return hubFile
	}
	if fileStamp(root, rulesFile) != stand.Rules {
		return rulesFile
	}
	return ""
}

// writeRules schreibt den aus hubJSON erzeugten Block zwischen die Marken der Regeltabelle; der Rest bleibt
// byte-gleich (tabelle.Ersetzen), ein unveränderter Block wird nicht geschrieben.
func writeRules(root string, hubJSON []byte) error {
	path := filepath.Join(root, filepath.FromSlash(rulesFile))
	doc, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	block, err := tabelle.Block(hubJSON)
	if err != nil {
		return err
	}
	next, err := tabelle.Ersetzen(string(doc), block)
	if err != nil || next == string(doc) {
		return err
	}
	return assets.WriteAtomic(path, []byte(next))
}
