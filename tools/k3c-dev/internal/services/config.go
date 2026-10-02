// Package services führt die Entwicklungs-Dienste von k3c-dev (B-067): Konfiguration aus services.json, ein
// Controller, der je Dienst serialisiert startet, prüft, überwacht und stoppt, und die Zustände für MCP und Oberfläche.
package services

import (
	"encoding/json"
	"fmt"
	"os"
	"path"
	"regexp"
	"strings"
)

// Service ist ein Eintrag aus services.json.
type Service struct {
	Name        string            `json:"name"`
	Command     []string          `json:"command"` // Programm und Argumente, ohne Shell gestartet
	Cwd         string            `json:"cwd"`     // relativ zur Repo-Wurzel
	Port        int               `json:"port"`
	Health      string            `json:"health"` // http oder tcp
	Log         string            `json:"log,omitempty"`
	AutoRestart bool              `json:"autoRestart,omitempty"`
	Env         map[string]string `json:"env,omitempty"`
	// Watch: bei Änderungen dieser Dateien startet k3c-dev den Dienst neu (Watch-Modus, B-097); nil = nie.
	Watch *Watch `json:"watch,omitempty"`
}

// name ist ein Dienst- bzw. Log-Name: er wird Konsolen-Quelle, MCP-Argument und Dateiname unter logs/.
var name = regexp.MustCompile(`^[A-Za-z0-9_.-]{1,40}$`)

// Load liest und prüft die Konfiguration; unbekannte Felder sind ein Fehler.
func Load(path string) ([]Service, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer func() { _ = f.Close() }()
	dec := json.NewDecoder(f)
	dec.DisallowUnknownFields()
	var list []Service
	if err := dec.Decode(&list); err != nil {
		return nil, fmt.Errorf("%s: %w", path, err)
	}
	return list, validate(path, list)
}

func validate(path string, list []Service) error {
	names, ports := map[string]bool{}, map[int]bool{}
	for i, s := range list {
		where := fmt.Sprintf("%s: Dienst %d (%q)", path, i+1, s.Name)
		if err := check(s, names, ports); err != nil {
			return fmt.Errorf("%s: %s", where, err)
		}
		names[s.Name], ports[s.Port] = true, true
	}
	return nil
}

func check(s Service, names map[string]bool, ports map[int]bool) error {
	switch {
	case !name.MatchString(s.Name):
		return fmt.Errorf("name fehlt oder enthält andere Zeichen als A–Z, a–z, 0–9, _ . - (höchstens 40)")
	case len(s.Command) == 0 || s.Command[0] == "":
		return fmt.Errorf("command fehlt")
	case s.Port < 1 || s.Port > 65535:
		return fmt.Errorf("port %d ungültig", s.Port)
	case s.Health != "http" && s.Health != "tcp":
		return fmt.Errorf("health %q: erlaubt sind http und tcp", s.Health)
	case s.Log != "" && !name.MatchString(s.Log):
		return fmt.Errorf("log %q ist kein gültiger Name", s.Log)
	case checkWatch(s.Watch) != nil:
		return checkWatch(s.Watch)
	case names[s.Name]:
		return fmt.Errorf("name doppelt")
	case ports[s.Port]:
		return fmt.Errorf("port %d doppelt", s.Port)
	}
	return nil
}

// HealthURL ist die Adresse der Prüfung, wie sie Karten und svc_status zeigen.
func (s Service) HealthURL() string {
	if s.Health == "tcp" {
		return fmt.Sprintf("tcp://127.0.0.1:%d", s.Port)
	}
	return fmt.Sprintf("http://127.0.0.1:%d/", s.Port)
}

// checkWatch prüft den Eintrag `watch`: nur Pfade unterhalb der Repo-Wurzel, Endungen mit Punkt.
func checkWatch(w *Watch) error {
	if w == nil {
		return nil
	}
	if len(w.Paths) == 0 || len(w.Paths) > 16 {
		return fmt.Errorf("watch.paths: 1 bis 16 Pfade erwartet")
	}
	for _, p := range w.Paths {
		clean := path.Clean(p)
		if p == "" || path.IsAbs(p) || strings.Contains(p, "\\") || clean == ".." || strings.HasPrefix(clean, "../") {
			return fmt.Errorf("watch.paths: %q muss ein Pfad relativ zur Repo-Wurzel sein (mit /, ohne ..)", p)
		}
	}
	for _, e := range w.Ext {
		if len(e) < 2 || e[0] != '.' || strings.ContainsAny(e[1:], `./\*?`) {
			return fmt.Errorf("watch.ext: %q ist keine Endung wie \".go\"", e)
		}
	}
	return nil
}
