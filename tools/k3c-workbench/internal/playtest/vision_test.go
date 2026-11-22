package playtest

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
)

func TestListVisionModelsPaartMmproj(t *testing.T) {
	root := t.TempDir()
	dir := filepath.Join(root, "models", "vision")
	_ = os.MkdirAll(dir, 0o755)
	for _, n := range []string{"Qwen2.5-VL-3B-Instruct-Q4_K_M.gguf", "mmproj-Qwen2.5-VL-3B-Instruct-Q8_0.gguf", "Ohne-Projektor-Q4_K_M.gguf"} {
		if err := os.WriteFile(filepath.Join(dir, n), []byte("x"), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	got := ListVisionModels(root)
	if len(got) != 1 || got[0].ID != "models/vision/Qwen2.5-VL-3B-Instruct-Q4_K_M.gguf" || !strings.HasSuffix(got[0].MMProj, "mmproj-Qwen2.5-VL-3B-Instruct-Q8_0.gguf") {
		t.Fatalf("Modelle: %+v", got)
	}
}

// visionRig wertet die Session aus (Marker 1 und 2 mit Bild) und baut einen Describer mit Attrappe.
func visionRig(t *testing.T) (*Describer, *[][]string, string) {
	t.Helper()
	tr, _, dir := trRig(t, true)
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); err != nil {
		t.Fatal(err)
	}
	tr.Wait()
	var mu sync.Mutex
	calls := &[][]string{}
	d := &Describer{Store: tr.Store,
		Models: func() []VisionModel {
			return []VisionModel{{ID: "models/vision/m.gguf", Name: "m.gguf", Path: "m.gguf", MMProj: "mmproj-m.gguf"}}
		},
		RunExe: func() string { return "llama-mtmd-cli.exe" },
		Run: func(_ context.Context, args []string) (string, error) {
			mu.Lock()
			*calls = append(*calls, args)
			mu.Unlock()
			return "  Ein Menü.\r\n", nil
		}}
	return d, calls, dir
}

func TestDescribeNurAusgewaehlte(t *testing.T) {
	d, calls, dir := visionRig(t)
	if err := d.Store.SetFrameSkip("2026-11-22-1", 1, true); err != nil { // Marker 1 abgewählt
		t.Fatal(err)
	}
	if _, err := d.Start("2026-11-22-1", "", nil); err != nil {
		t.Fatal(err)
	}
	d.Wait()
	if st := d.Status(); st.Running || st.Error != "" || st.Done != 1 || st.Total != 1 {
		t.Fatalf("Stand: %+v", st)
	}
	if len(*calls) != 1 || !strings.Contains(strings.Join((*calls)[0], " "), "marker-2.jpg") {
		t.Fatalf("nur Marker 2 wird beschrieben: %v", *calls)
	}
	descs := ReadDescriptions(dir)
	if len(descs) != 1 || descs[2].Text != "Ein Menü." || descs[2].Model != "m.gguf" {
		t.Fatalf("Beschreibungen: %+v", descs)
	}
	tl, _ := d.Store.Timeline("2026-11-22-1")
	if tl.Entries[3].Desc != "Ein Menü." || tl.Entries[1].Desc != "" {
		t.Errorf("Zeitleiste: %+v", tl.Entries)
	}
}

func TestDescribeGezielteNummernUndAblehnungen(t *testing.T) {
	d, calls, dir := visionRig(t)
	if _, err := d.Start("2026-11-22-1", "unbekannt", nil); err == nil {
		t.Error("unbekanntes Modell muss ablehnen")
	}
	if _, err := d.Start("2026-11-22-1", "models/vision/m.gguf", []int{2}); err != nil {
		t.Fatal(err)
	}
	d.Wait()
	if len(*calls) != 1 || len(ReadDescriptions(dir)) != 1 {
		t.Fatalf("nur Nummer 2: %v %v", *calls, ReadDescriptions(dir))
	}
	if err := d.Store.SetFrameSkip("2026-11-22-1", 1, true); err != nil {
		t.Fatal(err)
	}
	if _, err := d.Start("2026-11-22-1", "", []int{1}); err == nil {
		t.Error("abgewähltes Bild darf nicht beschrieben werden")
	}
	d.RunExe = func() string { return "" }
	if _, err := d.Start("2026-11-22-1", "", nil); err == nil {
		t.Error("ohne Laufzeit muss ablehnen")
	}
	if in := d.Info(); in.Ready || in.Message == "" {
		t.Errorf("Info ohne Laufzeit: %+v", in)
	}
}

func TestCleanVisionOhneDenkblock(t *testing.T) {
	in := "<think>\n\n</think>\n\nEin Menü.\r\nZweite Zeile.\n"
	if got := cleanVision(in); got != "Ein Menü.\nZweite Zeile." {
		t.Fatalf("cleanVision = %q", got)
	}
}
