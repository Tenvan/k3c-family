package main

import (
	"fmt"
	"os"
	"strings"

	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/mcpsrv"
	"k3c/tools/k3c-dev/internal/services"
)

// Arten und Zustände einer Quelle der Logs-Seite (B-064 › Quellenleiste). Dienste tragen ihren eigenen Zustand
// (services.State).
const (
	kindService = "service"
	kindRun     = "run"
	kindLog     = "log"
	kindConsole = "console" // Konsolen-Quelle ohne Dienst, Lauf oder Log-Datei

	runRunning = "running"
	runOK      = "ok"
	runFailed  = "failed"
	runTimeout = "timeout"
	logEntries = "entries"
	logEmpty   = "empty"
)

// Source ist ein Eintrag der Quellenleiste. Der Name ist zugleich die Konsolen-Quelle (Lauf: check:<ziel>).
type Source struct {
	Name   string `json:"name"`
	Kind   string `json:"kind"`
	State  string `json:"state"`
	Detail string `json:"detail"`
}

// Sources liefert alle Quellen: Dienste in der Reihenfolge der Konfiguration, dann Log-Dateien, dann Läufe, dann
// übrige Konsolen-Quellen (Binding).
func (a *App) Sources() []Source {
	a.wait()
	out := []Source{}
	seen := map[string]bool{}
	add := func(s Source) {
		if !seen[s.Name] {
			seen[s.Name] = true
			out = append(out, s)
		}
	}
	if a.ctl != nil {
		for _, st := range a.ctl.Statuses() {
			add(serviceSource(st))
		}
	}
	for _, name := range a.srv.LogSources() {
		add(a.logSource(name))
	}
	for _, st := range a.srv.Checks() {
		add(checkSource(st))
	}
	for _, name := range a.store.Sources() {
		add(Source{Name: name, Kind: kindConsole})
	}
	return out
}

// ConsoleTail liefert den ganzen Puffer einer Quelle, ANSI-Farben eingeschlossen (Binding). Eine bekannte Quelle
// ohne Ausgabe liefert eine leere Liste, eine unbekannte einen Fehler.
func (a *App) ConsoleTail(source string) ([]console.Line, error) {
	a.wait()
	lines, ok := a.store.Tail(source, 0)
	if ok {
		return append([]console.Line{}, lines...), nil
	}
	names := []string{}
	for _, s := range a.Sources() {
		if s.Name == source {
			return []console.Line{}, nil
		}
		names = append(names, s.Name)
	}
	return nil, fmt.Errorf("unbekannte Quelle %q; gültig: %s", source, strings.Join(names, ", "))
}

func serviceSource(st services.Status) Source {
	detail := fmt.Sprintf("Port %d", st.Port)
	if st.PID > 0 {
		detail += fmt.Sprintf(" · PID %d", st.PID)
	}
	if st.LastError != "" {
		detail += " · " + st.LastError
	}
	return Source{Name: st.Name, Kind: kindService, State: string(st.State), Detail: detail}
}

func checkSource(st mcpsrv.CheckState) Source {
	src := Source{Name: "check:" + st.Name, Kind: kindRun}
	at := st.At.Local().Format("15:04:05")
	switch {
	case st.Running:
		src.State, src.Detail = runRunning, "läuft seit "+at
	case st.Error != "":
		src.State, src.Detail = runFailed, "nicht gestartet: "+st.Error
	case st.TimedOut:
		src.State, src.Detail = runTimeout, "Zeitlimit nach "+seconds(st.Ms)+" · "+at
	case st.Exit != 0:
		src.State, src.Detail = runFailed, fmt.Sprintf("Exit %d · %s · %s", st.Exit, seconds(st.Ms), at)
	default:
		src.State, src.Detail = runOK, "Exit 0 · "+seconds(st.Ms)+" · "+at
	}
	return src
}

// seconds schreibt Millisekunden deutsch als Sekunden: 12400 → „12,4 s“.
func seconds(ms float64) string {
	return strings.Replace(fmt.Sprintf("%.1f s", ms/1000), ".", ",", 1)
}

func (a *App) logSource(name string) Source {
	src := Source{Name: name, Kind: kindLog, State: logEmpty, Detail: "noch keine Einträge"}
	path, err := a.srv.LogPath(name)
	if err != nil {
		return src
	}
	if info, err := os.Stat(path); err == nil && info.Size() > 0 {
		src.State, src.Detail = logEntries, fmt.Sprintf("logs/%s.jsonl · %d KB", name, (info.Size()+1023)/1024)
	}
	return src
}
