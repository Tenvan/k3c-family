// Package studioview hält, was vom Asset-Studio lesend blieb (Entscheidung 005 › 5, Sprint WB1): Sound-Slots und Quellen
// für den Bereich Assets der Workbench und „Im Spiel ansehen“ mit der eingebundenen Engine (game.go). Geändert wird
// nichts an data/ oder public/: Zuordnen, Recherche und Import bleiben die MCP-Tools assets_*.
package studioview

import (
	"encoding/base64"
	"errors"
	"fmt"
	"io"
	"os"
	"path"
	"path/filepath"
	"strings"

	assets "k3c/tools/assetkit"
)

// hint ist ein Hinweis für die Oberfläche in ganzen Sätzen (großer Anfangsbuchstabe, kein Fehlerwert im Sinn von ST1005).
type hint string

func (h hint) Error() string { return string(h) }

// SoundEintrag ist ein Sound-Slot aus data/sounds.json mit Status (zugeordnet, luecke, fehlt) und seinen Dateien.
type SoundEintrag struct {
	ID           string   `json:"id"`
	Kategorie    string   `json:"kategorie"`
	Beschreibung string   `json:"beschreibung"`
	Bus          string   `json:"bus,omitempty"`
	Status       string   `json:"status"`
	Dateien      []string `json:"dateien"`
}

// Sounds listet die Sound-Slots in der Reihenfolge des Registers.
func Sounds(root string) ([]SoundEintrag, error) {
	reg, err := assets.Load(root)
	if err != nil {
		return nil, err
	}
	out := []SoundEintrag{}
	for _, s := range reg.Slots {
		if s.Kind != assets.KindSound {
			continue
		}
		files := make([]string, 0, len(s.Choices))
		for _, c := range s.Choices {
			files = append(files, c.File)
		}
		out = append(out, SoundEintrag{ID: s.ID, Kategorie: s.Category, Beschreibung: s.Description, Bus: s.Bus,
			Status: assets.Status(root, s), Dateien: files})
	}
	return out, nil
}

// audioTypes sind die Endungen, die SoundData ausliefert.
var audioTypes = map[string]string{".ogg": "audio/ogg", ".mp3": "audio/mpeg", ".wav": "audio/wav", ".m4a": "audio/mp4"}

// maxSoundBytes begrenzt eine Hörprobe (Data-URL über RPC, Base64 macht sie um ein Drittel größer).
const maxSoundBytes = 10 << 20

// SoundData liefert eine Audiodatei unter public/ als Data-URL (Hörprobe). Nur relative Pfade mit `/`, ohne `..`, nur
// Audio-Endungen; gelesen wird über os.Root, ein Pfad außerhalb von public/ ist so ausgeschlossen.
func SoundData(root, rel string) (string, error) {
	mime, ok := audioTypes[strings.ToLower(path.Ext(rel))]
	switch {
	case rel == "" || strings.Contains(rel, `\`) || path.IsAbs(rel) || filepath.IsAbs(rel) || strings.Contains(rel, ":"):
		return "", hint(fmt.Sprintf("Pfad %q ungültig: relativ zu public/ mit /", rel))
	case path.Clean(rel) != rel || strings.HasPrefix(rel, "..") || strings.Contains(rel, "/../"):
		return "", hint(fmt.Sprintf("Pfad %q ungültig: ohne ..", rel))
	case !ok:
		return "", hint(fmt.Sprintf("Nur Audio (.ogg, .mp3, .wav, .m4a), nicht %q", rel))
	}
	pub, err := os.OpenRoot(filepath.Join(root, "public"))
	if err != nil {
		return "", err
	}
	defer func() { _ = pub.Close() }()
	f, err := pub.Open(filepath.FromSlash(rel))
	if errors.Is(err, os.ErrNotExist) {
		return "", hint(fmt.Sprintf("%s fehlt unter public/", rel))
	}
	if err != nil {
		return "", err
	}
	defer func() { _ = f.Close() }()
	raw, err := io.ReadAll(io.LimitReader(f, maxSoundBytes+1))
	if err != nil {
		return "", err
	}
	if len(raw) > maxSoundBytes {
		return "", hint(fmt.Sprintf("%s ist größer als 10 MB", rel))
	}
	return "data:" + mime + ";base64," + base64.StdEncoding.EncodeToString(raw), nil
}

// Quellen sind der Katalog unter public/ (mit Lizenz-Kennzeichen) und die Recherche-Treffer je Slot.
type Quellen struct {
	Katalog []assets.Asset          `json:"katalog"`
	Treffer map[string][]assets.Hit `json:"treffer"`
}

// Sources liest Katalog und Recherche-Treffer; fehlende Dateien ergeben leere Listen.
func Sources(root string) (Quellen, error) {
	cat, err := assets.Catalog(root)
	if err != nil && !errors.Is(err, os.ErrNotExist) {
		return Quellen{}, err
	}
	if cat == nil {
		cat = []assets.Asset{}
	}
	res, err := assets.LoadResearch(root)
	if err != nil {
		return Quellen{}, err
	}
	return Quellen{Katalog: cat, Treffer: res.Hits}, nil
}
