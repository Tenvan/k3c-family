// Package gamedata macht Xbox-Berichte (reports/*.json) und Spielstände (saves/*.json) für Agenten lesbar (B-063):
// nur lesen, verdichtet zu Text, Pfade nie außerhalb der beiden Ordner.
package gamedata

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"time"
)

// buttonNames sind die Tasten der Standard-Belegung je Index, wie auf der Gamepad-Testseite (src/tools/gamepadTest.ts).
var buttonNames = strings.Fields("A B X Y LB RB LT RT View Menu LS RS ↑ ↓ ← → Xbox")

// fileName ist ein Name ohne Ordner und ohne führenden Punkt, mit Endung .json.
var fileName = regexp.MustCompile(`^[A-Za-z0-9_-][A-Za-z0-9_.-]*\.json$`)

type report struct {
	CreatedAt           string         `json:"createdAt"`
	ReceivedAt          string         `json:"receivedAt"`
	UserAgent           string         `json:"userAgent"`
	Pads                map[string]pad `json:"pads"`
	Perf                []perf         `json:"perf"`
	FullscreenAttempts  []attempt      `json:"fullscreenAttempts"`
	BackNavigations     int            `json:"backNavigations"`
	KeyEvents           []string       `json:"keyEvents"`
	MaxSimultaneousPads int            `json:"maxSimultaneousPads"`
}

type pad struct {
	ID          string `json:"id"`
	Mapping     string `json:"mapping"`
	ButtonsSeen []int  `json:"buttonsSeen"`
}

type perf struct {
	Sprites int     `json:"sprites"`
	AvgFps  float64 `json:"avgFps"`
	MinFps  float64 `json:"minFps"`
}

type attempt struct {
	Via   string `json:"via"`
	OK    bool   `json:"ok"`
	Error string `json:"error"`
}

type named struct {
	name string
	r    *report // nil = nicht lesbar
}

// ReportsList ist reports_list: neueste zuerst, eine Zeile je Bericht.
func ReportsList(root string) (string, error) {
	files, err := filepath.Glob(filepath.Join(root, "reports", "*.json"))
	if err != nil || len(files) == 0 {
		return "keine Berichte", err
	}
	all := make([]named, 0, len(files))
	for _, f := range files {
		r, err := readReport(f)
		if err != nil {
			r = nil
		}
		all = append(all, named{filepath.Base(f), r})
	}
	sort.Slice(all, func(i, j int) bool {
		if (all[i].r == nil) != (all[j].r == nil) {
			return all[i].r != nil // unlesbare ans Ende
		}
		return sortKey(all[i]) > sortKey(all[j])
	})
	lines := make([]string, len(all))
	for i, n := range all {
		lines[i] = listLine(n)
	}
	return strings.Join(lines, "\n"), nil
}

func sortKey(n named) string {
	if n.r != nil && n.r.ReceivedAt != "" {
		return n.r.ReceivedAt
	}
	return n.name
}

func listLine(n named) string {
	if n.r == nil {
		return n.name + " · nicht lesbar"
	}
	r := n.r
	line := fmt.Sprintf("%s · %s · %s · %d Controller", n.name, r.when(), r.device(), len(r.Pads))
	if best, ok := r.biggestStage(); ok {
		line += fmt.Sprintf(" · %.0f FPS bei %d Sprites", best.AvgFps, best.Sprites)
	}
	return line
}

// ReportRead ist report_read: ein Bericht in wenigen Zeilen.
func ReportRead(root, name string) (string, error) {
	path, err := inside(filepath.Join(root, "reports"), name)
	if err != nil {
		return "", err
	}
	r, err := readReport(path)
	if err != nil {
		return "", fmt.Errorf("%s nicht lesbar: %w", name, err)
	}
	lines := []string{fmt.Sprintf("%s · %s · %s", name, r.when(), r.device())}
	for _, key := range sortedKeys(r.Pads) {
		p := r.Pads[key]
		lines = append(lines, fmt.Sprintf("Controller %s: %s (%s) · Tasten %s", key, p.ID, p.Mapping, buttons(p.ButtonsSeen)))
	}
	lines = append(lines, "FPS min/Ø: "+r.fps(), "Vollbild: "+r.fullscreen(),
		fmt.Sprintf("Gleichzeitig max. %d Controller · Zurück-Navigationen %d · Tasten-Ereignisse %d",
			r.MaxSimultaneousPads, r.BackNavigations, len(r.KeyEvents)))
	return strings.Join(lines, "\n"), nil
}

// inside prüft einen Dateinamen und liefert den Pfad nur, wenn er in dir liegt.
func inside(dir, name string) (string, error) {
	if !fileName.MatchString(name) || strings.Contains(name, "..") {
		return "", fmt.Errorf("name %q abgelehnt: nur ein Dateiname aus reports/ mit Endung .json", name)
	}
	path := filepath.Join(dir, name)
	rel, err := filepath.Rel(dir, path)
	if err != nil || rel != name {
		return "", fmt.Errorf("name %q liegt nicht in reports/", name)
	}
	return path, nil
}

func readReport(path string) (*report, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var r report
	if err := json.Unmarshal(data, &r); err != nil {
		return nil, err
	}
	return &r, nil
}

func (r *report) when() string {
	for _, s := range []string{r.ReceivedAt, r.CreatedAt} {
		if t, err := time.Parse(time.RFC3339, s); err == nil {
			return t.Local().Format("2006-01-02 15:04")
		}
	}
	return "ohne Datum"
}

// device ist der Inhalt der ersten Klammer im User-Agent, z. B. "Xbox; Xbox One".
func (r *report) device() string {
	_, rest, ok := strings.Cut(r.UserAgent, "(")
	if dev, _, ok2 := strings.Cut(rest, ")"); ok && ok2 {
		return dev
	}
	return "unbekanntes Gerät"
}

func (r *report) biggestStage() (perf, bool) {
	var best perf
	for _, p := range r.Perf {
		if p.Sprites > best.Sprites {
			best = p
		}
	}
	return best, len(r.Perf) > 0
}

func (r *report) fps() string {
	if len(r.Perf) == 0 {
		return "keine Messung"
	}
	parts := make([]string, len(r.Perf))
	for i, p := range r.Perf {
		parts[i] = fmt.Sprintf("%d Sprites %.0f/%.0f", p.Sprites, p.MinFps, p.AvgFps)
	}
	return strings.Join(parts, " · ")
}

func (r *report) fullscreen() string {
	if len(r.FullscreenAttempts) == 0 {
		return "kein Versuch"
	}
	parts := make([]string, len(r.FullscreenAttempts))
	for i, a := range r.FullscreenAttempts {
		parts[i] = a.Via + " → ok"
		if !a.OK {
			parts[i] = a.Via + " → Fehler: " + a.Error
		}
	}
	return strings.Join(parts, " · ")
}

func buttons(seen []int) string {
	if len(seen) == 0 {
		return "keine"
	}
	names := make([]string, len(seen))
	for i, b := range seen {
		names[i] = "#" + strconv.Itoa(b)
		if b >= 0 && b < len(buttonNames) {
			names[i] = buttonNames[b]
		}
	}
	return strings.Join(names, " ")
}

func sortedKeys(m map[string]pad) []string {
	keys := make([]string, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	return keys
}
