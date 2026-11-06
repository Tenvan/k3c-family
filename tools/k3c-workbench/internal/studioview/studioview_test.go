package studioview

import (
	"encoding/base64"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// WB1.4 (005 › 5, Sprint-README WB1 › D-03): Sound und Quellen nur lesend aus data/assets.json, data/assets-recherche.json
// und dem Katalog unter public/.

func write(t *testing.T, root, rel, content string) {
	t.Helper()
	full := filepath.Join(root, filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(full), 0o750); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(full, []byte(content), 0o600); err != nil {
		t.Fatal(err)
	}
}

func assetRoot(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	write(t, root, "data/assets.json", `{"_comment": "", "slots": [
		{"id": "music:tag", "art": "sound", "kategorie": "musik", "beschreibung": "Tag", "bus": "music", "wahl": [{"datei": "audio/tag.ogg"}]},
		{"id": "sfx:fehlt", "art": "sound", "kategorie": "sfx", "beschreibung": "Fehlt", "wahl": [{"datei": "audio/weg.ogg"}]},
		{"id": "sfx:luecke", "art": "sound", "kategorie": "sfx", "beschreibung": "Lücke", "wahl": []},
		{"id": "monarch", "art": "grafik", "kategorie": "figuren", "beschreibung": "Monarch", "wahl": []}
	]}`)
	write(t, root, "public/audio/tag.ogg", "OggS-probe")
	write(t, root, "data/assets-recherche.json", `{"_comment": "", "treffer": {"music:tag": [
		{"id": "h1", "titel": "Morgen", "link": "https://example.org/m", "quelle": "OpenGameArt", "lizenz": "CC0", "lizenzStatus": "erkannt", "erstellt": "2026-11-01"}
	]}}`)
	return root
}

func TestSoundsListetNurSoundSlotsMitStatus(t *testing.T) {
	list, err := Sounds(assetRoot(t))
	if err != nil {
		t.Fatal(err)
	}
	got := map[string]SoundEintrag{}
	for _, s := range list {
		got[s.ID] = s
	}
	if len(list) != 3 || got["monarch"].ID != "" {
		t.Fatalf("Sound-Slots: %+v", list)
	}
	if s := got["music:tag"]; s.Status != "zugeordnet" || s.Kategorie != "musik" || s.Bus != "music" || len(s.Dateien) != 1 || s.Dateien[0] != "audio/tag.ogg" {
		t.Errorf("music:tag: %+v", s)
	}
	if got["sfx:fehlt"].Status != "fehlt" || got["sfx:luecke"].Status != "luecke" {
		t.Errorf("Status: %+v", got)
	}
}

func TestSoundDataNurAudioUnterPublic(t *testing.T) {
	root := assetRoot(t)
	write(t, root, "geheim.ogg", "x")
	write(t, root, "public/grafik/bild.png", "x")
	url, err := SoundData(root, "audio/tag.ogg")
	if err != nil || !strings.HasPrefix(url, "data:audio/ogg;base64,") {
		t.Fatalf("Data-URL %q, Fehler %v", url, err)
	}
	if raw, _ := base64.StdEncoding.DecodeString(strings.TrimPrefix(url, "data:audio/ogg;base64,")); string(raw) != "OggS-probe" {
		t.Errorf("Inhalt %q", raw)
	}
	for _, bad := range []string{"../geheim.ogg", "/etc/x.ogg", `audio\tag.ogg`, "C:/x.ogg", "audio/../../geheim.ogg", "grafik/bild.png", "audio/weg.ogg", ""} {
		if _, err := SoundData(root, bad); err == nil {
			t.Errorf("%q angenommen", bad)
		}
	}
}

func TestSourcesKatalogUndTreffer(t *testing.T) {
	q, err := Sources(assetRoot(t))
	if err != nil {
		t.Fatal(err)
	}
	if hits := q.Treffer["music:tag"]; len(hits) != 1 || hits[0].Title != "Morgen" || hits[0].License != "CC0" {
		t.Errorf("Treffer: %+v", q.Treffer)
	}
	if q.Katalog == nil {
		t.Error("Katalog nil statt leer")
	}
}

func TestSourcesOhneRecherche(t *testing.T) {
	root := t.TempDir()
	q, err := Sources(root)
	if err != nil || len(q.Treffer) != 0 {
		t.Fatalf("ohne Dateien: %+v, %v", q, err)
	}
}
