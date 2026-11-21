package assetsui

import (
	"os"
	"path/filepath"
	"testing"
)

func TestStufenSortiertNachHoeheUndBringtAssets(t *testing.T) {
	root := t.TempDir()
	if err := os.MkdirAll(filepath.Join(root, "data"), 0o755); err != nil {
		t.Fatal(err)
	}
	body := `{"bezugPx":121.5,"stufen":{"boss":{"m":1.3,"was":"Bosse"},"buerger":{"m":0.6,"was":"Bürger"},"busch":{"m":0.5,"was":"Busch"}},
		"assets":{"bosse/troll-koenig":{"stufe":"boss","sicht":0.765}}}`
	if err := os.WriteFile(filepath.Join(root, "data", "sizes.json"), []byte(body), 0o600); err != nil {
		t.Fatal(err)
	}
	s, err := Dienst{Root: root}.Stufen()
	if err != nil {
		t.Fatal(err)
	}
	if s.BezugPx != 121.5 || len(s.Stufen) != 3 || s.Stufen[0].ID != "busch" || s.Stufen[2].ID != "boss" {
		t.Errorf("Stufen: %+v", s)
	}
	if s.Assets["bosse/troll-koenig"].Stufe != "boss" {
		t.Errorf("Assets: %+v", s.Assets)
	}
}

func TestStufenMeldetFehlendeUndLeereDatei(t *testing.T) {
	root := t.TempDir()
	if _, err := (Dienst{Root: root}).Stufen(); err == nil {
		t.Fatal("fehlende Datei nicht gemeldet")
	}
	if err := os.MkdirAll(filepath.Join(root, "data"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "data", "sizes.json"), []byte(`{"stufen":{}}`), 0o600); err != nil {
		t.Fatal(err)
	}
	if _, err := (Dienst{Root: root}).Stufen(); err == nil {
		t.Fatal("leere Datei nicht gemeldet")
	}
}

// Die echte Datei (LA1.1) muss lesbar sein und den Monarchen-Bezug enthalten.
func TestStufenEchteDatei(t *testing.T) {
	s, err := Dienst{Root: filepath.Join("..", "..", "..", "..")}.Stufen()
	if err != nil {
		t.Skipf("keine data/sizes.json im Repo: %v", err)
	}
	if s.BezugPx <= 0 || len(s.Stufen) < 5 || len(s.Assets) == 0 {
		t.Errorf("Stufen: %+v", s)
	}
}
