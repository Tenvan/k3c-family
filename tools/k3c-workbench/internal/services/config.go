// Package services führt die Entwicklungs-Dienste von k3c-workbench (B-067): Konfiguration aus services.json, ein
// Controller, der je Dienst serialisiert startet, prüft, überwacht und stoppt, und die Zustände für MCP und Oberfläche.
package services

import (
	"encoding/json"
	"fmt"
	"maps"
	"os"
	"path"
	"regexp"
	"strconv"
	"strings"
)

// Service ist ein Eintrag aus services.json.
type Service struct {
	Name        string            `json:"name"`
	Description string            `json:"description,omitempty"` // ein Satz: wofür der Dienst da ist (Oberfläche)
	Tags        []string          `json:"tags,omitempty"`        // Einordnung für die Oberfläche: Spiel/Tool, Backend/Client
	Command     []string          `json:"command"` // Programm und Argumente, ohne Shell gestartet
	Cwd         string            `json:"cwd"`     // relativ zur Repo-Wurzel
	Port        int               `json:"port"`   // 0 nur bei health process
	Health      string            `json:"health"` // http, tcp oder process (gesund, solange der Prozess läuft)
	Log         string            `json:"log,omitempty"`
	AutoRestart bool              `json:"autoRestart,omitempty"`
	Env         map[string]string `json:"env,omitempty"`
	// PortEnv nennt die Umgebungsvariable, in der jeder Dienst den Port dieses Dienstes bekommt (Shift).
	PortEnv string `json:"portEnv,omitempty"`
	// Watch: bei Änderungen dieser Dateien startet k3c-workbench den Dienst neu (Watch-Modus, B-097); nil = nie.
	Watch *Watch `json:"watch,omitempty"`
	// Manual: nur einzeln starten, nie über StartAll (das Spiel, B-1214).
	Manual bool `json:"manual,omitempty"`
	// Needs ist eine Datei relativ zur Repo-Wurzel, ohne die der Start mit NeedsHint abbricht (etwa die Paket-EXE).
	Needs     string `json:"needs,omitempty"`
	NeedsHint string `json:"needsHint,omitempty"`
	// StopBefore nennt Tasks, vor deren Start die Workbench den laufenden Dienst beendet (laufende EXE blockiert sonst
	// den Neubau, B-1214).
	StopBefore []string `json:"stopBefore,omitempty"`
}

// healthProcess: gesund, solange der Prozess läuft (ohne Port, etwa das Spiel).
const healthProcess = "process"

// name ist ein Dienst- bzw. Log-Name: er wird Konsolen-Quelle, MCP-Argument und Dateiname unter logs/.
var name = regexp.MustCompile(`^[A-Za-z0-9_.-]{1,40}$`)

var envName = regexp.MustCompile(`^[A-Z][A-Z0-9_]{0,63}$`)

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
		names[s.Name] = true
		if s.Port != 0 {
			ports[s.Port] = true
		}
	}
	return nil
}

// taskName ist ein Task-Name wie in `task --list` (Namensraum mit Doppelpunkt).
var taskName = regexp.MustCompile(`^[A-Za-z0-9_.:-]{1,80}$`)

// checkProcess prüft die Felder der Prozess-Dienste: kein Port, needs relativ zur Wurzel, gültige Task-Namen.
func checkProcess(s Service) error {
	if s.Health == healthProcess && s.Port != 0 {
		return fmt.Errorf("health process läuft ohne port (port %d)", s.Port)
	}
	if s.Needs != "" && !relPath(s.Needs) {
		return fmt.Errorf("needs %q muss ein Pfad relativ zur Repo-Wurzel sein (mit /, ohne ..)", s.Needs)
	}
	for _, t := range s.StopBefore {
		if !taskName.MatchString(t) {
			return fmt.Errorf("stopBefore %q ist kein Task-Name", t)
		}
	}
	return nil
}

// relPath: Pfad unterhalb der Repo-Wurzel, mit / und ohne ..
func relPath(p string) bool {
	clean := path.Clean(p)
	return p != "" && !path.IsAbs(p) && !strings.Contains(p, "\\") && !strings.Contains(p, ":") && clean != ".." &&
		!strings.HasPrefix(clean, "../")
}

func check(s Service, names map[string]bool, ports map[int]bool) error {
	switch {
	case !name.MatchString(s.Name):
		return fmt.Errorf("name fehlt oder enthält andere Zeichen als A–Z, a–z, 0–9, _ . - (höchstens 40)")
	case len(s.Command) == 0 || s.Command[0] == "":
		return fmt.Errorf("command fehlt")
	case s.Health != "http" && s.Health != "tcp" && s.Health != healthProcess:
		return fmt.Errorf("health %q: erlaubt sind http, tcp und process", s.Health)
	case checkProcess(s) != nil:
		return checkProcess(s)
	case s.Health != healthProcess && (s.Port < 1 || s.Port > 65535):
		return fmt.Errorf("port %d ungültig", s.Port)
	case s.Log != "" && !name.MatchString(s.Log):
		return fmt.Errorf("log %q ist kein gültiger Name", s.Log)
	case s.PortEnv != "" && !envName.MatchString(s.PortEnv):
		return fmt.Errorf("portEnv %q ist kein Variablenname wie K3C_HTTP_PORT", s.PortEnv)
	case checkWatch(s.Watch) != nil:
		return checkWatch(s.Watch)
	case names[s.Name]:
		return fmt.Errorf("name doppelt")
	case ports[s.Port]:
		return fmt.Errorf("port %d doppelt", s.Port)
	}
	return nil
}

// Shift verschiebt alle Ports um off (Dienste eines Worktrees neben denen der Repo-Wurzel) und gibt jedem Dienst
// die Ports aller Dienste mit PortEnv mit, z. B. einem Frontend den Port seines Backends für den Proxy. off 0 ändert
// nur die Umgebung.
func Shift(list []Service, off int) []Service {
	ports := map[string]string{}
	for _, s := range list {
		if s.PortEnv != "" {
			ports[s.PortEnv] = strconv.Itoa(s.Port + off)
		}
	}
	out := make([]Service, len(list))
	for i, s := range list {
		if s.Port != 0 { // Prozess-Dienste haben keinen Port
			s.Port += off
		}
		env := make(map[string]string, len(s.Env)+len(ports))
		maps.Copy(env, s.Env)
		maps.Copy(env, ports)
		s.Env = env
		out[i] = s
	}
	return out
}

// HealthURL ist die Adresse der Prüfung, wie sie Karten und svc_status zeigen; leer bei health process (die Karte
// zeigt dann „process“).
func (s Service) HealthURL() string {
	switch s.Health {
	case healthProcess:
		return ""
	case "tcp":
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
