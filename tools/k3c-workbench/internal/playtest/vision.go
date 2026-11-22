package playtest

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"sync"
	"time"

	"k3c/tools/workkit/proc"
)

// Bildbeschreibung (B-1241): ein lokales Vision-Modell (llama.cpp, llama-mtmd-cli) beschreibt die ausgewählten Marker-Bilder
// vorab, damit der Chat sie nicht ansehen muss. Modell und Laufzeit liegen in models/vision/ (ignoriert, nie automatisch
// geladen); das Ergebnis steht in descriptions.json der Session.

const (
	fileDescriptions = "descriptions.json"
	visionDir        = "models/vision"
	visionEnv        = "K3C_LLAMA_DIR" // Ordner mit llama-mtmd-cli; sonst models/vision/llama
	visionExe        = "llama-mtmd-cli.exe"
	// visionPrompt: kurz, deutsch, ohne Erfindungen.
	visionPrompt = "Beschreibe diesen Screenshot aus dem Side-Scroller-Spiel Family Three Crowns in zwei bis vier Sätzen auf Deutsch: " +
		"sichtbare Texte, Menüs mit Werten, Gebäude, Figuren, Ressourcen und Auffälligkeiten. Erfinde nichts, was nicht zu sehen ist."
)

// Fehler der Bildbeschreibung.
var (
	ErrBusyVision = errors.New("Bildbeschreibung läuft schon")
	ErrNoVision   = errors.New("keine Bildbeschreibung aktiv")
	ErrNoRuntime  = errors.New("llama-mtmd-cli.exe fehlt: llama.cpp (Vulkan-Build) nach models/vision/llama/ entpacken oder K3C_LLAMA_DIR setzen")
)

// VisionModel ist ein Bildmodell (Hauptdatei samt mmproj) für die Auswahl der Seite; ID ist der Pfad relativ zur Repo-Wurzel.
type VisionModel struct {
	ID     string `json:"id"`
	Name   string `json:"name"`
	SizeMB int    `json:"sizeMb"`
	Path   string `json:"-"`
	MMProj string `json:"-"`
}

var quantRe = regexp.MustCompile(`(?i)-(q\d.*|f16|f32|bf16)\.gguf$`)

// ListVisionModels sind die Dateien models/vision/*.gguf (ohne mmproj-*), zu denen es eine passende mmproj-Datei gibt.
func ListVisionModels(root string) []VisionModel {
	dir := filepath.Join(root, filepath.FromSlash(visionDir))
	files, _ := filepath.Glob(filepath.Join(dir, "*.gguf"))
	sort.Strings(files)
	out := []VisionModel{}
	for _, f := range files {
		name := filepath.Base(f)
		if strings.HasPrefix(name, "mmproj") {
			continue
		}
		base := quantRe.ReplaceAllString(name, "")
		mm, _ := filepath.Glob(filepath.Join(dir, "mmproj-"+base+"*.gguf"))
		if len(mm) == 0 {
			continue
		}
		st, err := os.Stat(f)
		if err != nil {
			continue
		}
		out = append(out, VisionModel{ID: visionDir + "/" + name, Name: name, SizeMB: int(st.Size() >> 20), Path: f, MMProj: mm[0]})
	}
	return out
}

// VisionRuntime ist llama-mtmd-cli.exe (K3C_LLAMA_DIR oder models/vision/llama); leer, wenn sie fehlt.
func VisionRuntime(root string) string {
	dir := strings.TrimSpace(os.Getenv(visionEnv))
	if dir == "" {
		dir = filepath.Join(root, filepath.FromSlash(visionDir), "llama")
	}
	p := filepath.Join(dir, visionExe)
	if st, err := os.Stat(p); err == nil && !st.IsDir() {
		return p
	}
	return ""
}

// VisionArgs ist der Aufruf für ein Bild (absolute Pfade); -ngl 99 nimmt die Grafikkarte (Vulkan), die Laufzeit fällt ohne sie auf CPU zurück.
func VisionArgs(exe string, m VisionModel, image string) []string {
	return []string{exe, "-m", m.Path, "--mmproj", m.MMProj, "--image", image, "-p", visionPrompt, "-n", "300", "--temp", "0.1", "-ngl", "99"}
}

// FrameDesc ist die Beschreibung eines Marker-Bilds.
type FrameDesc struct {
	Text  string `json:"text"`
	Model string `json:"model"`
	Ms    int64  `json:"ms"` // Dauer der Beschreibung
}

// ReadDescriptions liest descriptions.json (Marker-Nummer → Beschreibung); fehlt sie, ist die Karte leer.
func ReadDescriptions(dir string) map[int]FrameDesc {
	out := map[int]FrameDesc{}
	raw, err := os.ReadFile(filepath.Join(dir, fileDescriptions))
	if err != nil {
		return out
	}
	var m map[string]FrameDesc
	if json.Unmarshal(raw, &m) != nil {
		return out
	}
	for k, v := range m {
		var n int
		if _, err := fmt.Sscanf(k, "%d", &n); err == nil && n > 0 {
			out[n] = v
		}
	}
	return out
}

func writeDescriptions(dir string, d map[int]FrameDesc) error {
	m := map[string]FrameDesc{}
	for n, v := range d {
		m[fmt.Sprint(n)] = v
	}
	raw, _ := json.MarshalIndent(m, "", "  ")
	tmp := filepath.Join(dir, fileDescriptions+".tmp")
	if err := os.WriteFile(tmp, raw, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, filepath.Join(dir, fileDescriptions))
}

// cleanVision schneidet die Antwort der Laufzeit auf den Text (stdout enthält nur ihn; Leerraum außen weg).
func cleanVision(out string) string {
	return strings.TrimSpace(thinkRe.ReplaceAllString(strings.ReplaceAll(out, "\r\n", "\n"), ""))
}

// thinkRe findet Denkblöcke (<think>…</think>), die Qwen3.5 leer voranstellt.
var thinkRe = regexp.MustCompile(`(?s)<think>.*?</think>`)

// VisionStatus ist der Stand der Bildbeschreibung für die Seite.
type VisionStatus struct {
	Running bool   `json:"running"`
	ID      string `json:"id"`
	Done    int    `json:"done"`
	Total   int    `json:"total"`
	Error   string `json:"error"`
}

// Describer steuert höchstens eine Bildbeschreibung.
type Describer struct {
	Store  *Store
	Root   string
	Run    func(ctx context.Context, args []string) (string, error) // Aufruf mit stdout (Tests setzen eine Attrappe)
	Models func() []VisionModel
	RunExe func() string // Pfad der Laufzeit (leer: fehlt)
	Log    func(level, msg string, kv ...any)

	mu     sync.Mutex
	st     VisionStatus
	cancel context.CancelFunc
	done   chan struct{}
}

func (d *Describer) log(level, msg string, kv ...any) {
	if d.Log != nil {
		d.Log(level, msg, kv...)
	}
}

// NewDescriber baut die Bildbeschreibung für die Ablage store unter root mit echten Prozessen.
func NewDescriber(root string, store *Store, log func(level, msg string, kv ...any)) *Describer {
	return &Describer{Store: store, Root: root, Log: log,
		Models: func() []VisionModel { return ListVisionModels(root) },
		RunExe: func() string { return VisionRuntime(root) },
		Run: func(ctx context.Context, args []string) (string, error) {
			var buf bytes.Buffer
			c := proc.Command(ctx, args)
			c.Stdout = &buf // stderr (Ladeprotokoll) bleibt leer
			err := c.Run()
			return buf.String(), err
		}}
}

// VisionInfo sagt der Seite, welche Bildmodelle es gibt und ob die Laufzeit bereitsteht.
type VisionInfo struct {
	Ready   bool          `json:"ready"`
	Models  []VisionModel `json:"models"`
	Message string        `json:"message"`
}

// Info prüft Laufzeit und Modelle.
func (d *Describer) Info() VisionInfo {
	in := VisionInfo{Models: d.Models()}
	switch {
	case d.RunExe() == "":
		in.Message = ErrNoRuntime.Error()
	case len(in.Models) == 0:
		in.Message = "kein Bildmodell in models/vision/ (Hauptdatei und mmproj-Datei, z. B. Qwen2.5-VL-3B)"
	default:
		in.Ready = true
	}
	return in
}

// Status liefert den Stand.
func (d *Describer) Status() VisionStatus {
	d.mu.Lock()
	defer d.mu.Unlock()
	return d.st
}

// Cancel bricht die Beschreibung ab; schon fertige Beschreibungen bleiben.
func (d *Describer) Cancel() error {
	d.mu.Lock()
	defer d.mu.Unlock()
	if !d.st.Running || d.cancel == nil {
		return ErrNoVision
	}
	d.cancel()
	return nil
}

// Wait wartet in Tests aufs Ende.
func (d *Describer) Wait() {
	d.mu.Lock()
	c := d.done
	d.mu.Unlock()
	if c != nil {
		<-c
	}
}

// Start beschreibt die Marker-Bilder nrs der Session (leer: alle ausgewählten, also die nicht abgewählten mit Bild) im
// Hintergrund mit dem Modell model (ID aus ListVisionModels, leer: das erste). Abgewählte Bilder werden nie beschrieben.
func (d *Describer) Start(id, model string, nrs []int) (VisionStatus, error) {
	ses, err := d.Store.Get(id)
	if err != nil {
		return VisionStatus{}, err
	}
	if ses.Running {
		return VisionStatus{}, ErrRunning
	}
	exe := d.RunExe()
	if exe == "" {
		return VisionStatus{}, ErrNoRuntime
	}
	m, err := d.pick(model)
	if err != nil {
		return VisionStatus{}, err
	}
	tl, err := d.Store.Timeline(id)
	if err != nil {
		return VisionStatus{}, err
	}
	want := map[int]bool{}
	for _, n := range nrs {
		want[n] = true
	}
	var todo []int
	for _, e := range tl.Entries {
		if e.Kind != "marker" || e.Frame == "" || e.Skip || (len(want) > 0 && !want[e.Nr]) {
			continue
		}
		todo = append(todo, e.Nr)
	}
	if len(todo) == 0 {
		return VisionStatus{}, errors.New("kein ausgewähltes Marker-Bild zum Beschreiben (erst auswerten, Haken prüfen)")
	}
	d.mu.Lock()
	defer d.mu.Unlock()
	if d.st.Running {
		return d.st, ErrBusyVision
	}
	ctx, cancel := context.WithCancel(context.Background())
	d.cancel, d.done = cancel, make(chan struct{})
	d.st = VisionStatus{Running: true, ID: id, Total: len(todo)}
	d.log("info", "🚀 Bildbeschreibung gestartet", "id", id, "bilder", len(todo), "modell", m.Name)
	go d.run(ctx, exe, id, m, todo, d.done)
	return d.st, nil
}

func (d *Describer) pick(id string) (VisionModel, error) {
	list := d.Models()
	if len(list) == 0 {
		return VisionModel{}, errors.New("kein Bildmodell in models/vision/ (Hauptdatei und mmproj-Datei, z. B. Qwen2.5-VL-3B)")
	}
	if id == "" {
		return list[0], nil
	}
	for _, m := range list {
		if m.ID == id {
			return m, nil
		}
	}
	return VisionModel{}, fmt.Errorf("unbekanntes Bildmodell %q: nur Dateien aus models/vision/", id)
}

func (d *Describer) run(ctx context.Context, exe, id string, m VisionModel, todo []int, done chan struct{}) {
	defer close(done)
	defer d.cancel()
	dir := filepath.Join(d.Store.Root, id)
	var err error
	for i, nr := range todo {
		t0 := time.Now()
		var out string
		out, err = d.Run(ctx, VisionArgs(exe, m, filepath.Join(dir, filepath.FromSlash(FrameName(nr)))))
		if ctx.Err() != nil {
			err = errCanceled
			break
		}
		if err != nil {
			break
		}
		descs := ReadDescriptions(dir) // frisch lesen: andere Nummern bleiben erhalten
		descs[nr] = FrameDesc{Text: cleanVision(out), Model: m.Name, Ms: time.Since(t0).Milliseconds()}
		if err = writeDescriptions(dir, descs); err != nil {
			break
		}
		d.mu.Lock()
		d.st.Done = i + 1
		d.mu.Unlock()
	}
	d.mu.Lock()
	d.st.Running = false
	if err != nil && !errors.Is(err, errCanceled) {
		d.st.Error = err.Error()
	}
	d.mu.Unlock()
	switch {
	case errors.Is(err, errCanceled):
		d.log("info", "🛑 Bildbeschreibung abgebrochen", "id", id)
	case err != nil:
		d.log("warn", "❌ Bildbeschreibung fehlgeschlagen: "+err.Error(), "id", id)
	default:
		d.log("info", "✅ Bildbeschreibung fertig", "id", id)
	}
}
