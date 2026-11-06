package paperassets

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// AnlegenErgebnis beschreibt ein neu angelegtes Asset: Vorlagen-Teile, Fehler von validate, Render-Ergebnis.
type AnlegenErgebnis struct {
	Asset  Asset
	Teile  []string
	Fehler []Fehler
	Render AssetErgebnis
}

// Anlegen legt public/grafik/<gruppe>/<id>.paper.json aus der Rig-Vorlage an, prüft und rendert sie. Eine Vorlage,
// die validate nicht besteht (Figur mit leeren Lagen: proportion), bleibt liegen; Fehler stehen im Ergebnis.
func Anlegen(ctx context.Context, cli CLI, id, rig, beschreibung, gruppe string) (AnlegenErgebnis, error) {
	if !name.MatchString(id) || !name.MatchString(gruppe) {
		return AnlegenErgebnis{}, fmt.Errorf("id „%s“ und gruppe „%s“: nur a-z, 0-9, - (Anfang a-z)", id, gruppe) //nolint:staticcheck // Satz für die Antwort
	}
	if id == "index" {
		return AnlegenErgebnis{}, fmt.Errorf("id „index“ ist für den Gruppen-Index reserviert")
	}
	if strings.TrimSpace(beschreibung) == "" {
		return AnlegenErgebnis{}, fmt.Errorf("beschreibung fehlt")
	}
	r, err := FindeRig(rig)
	if err != nil {
		return AnlegenErgebnis{}, err
	}
	if a, err := Finde(cli.Root, id); err == nil {
		return AnlegenErgebnis{}, fmt.Errorf("%s gibt es schon (%s)", id, a.Pfad)
	}
	text, err := Vorlage(rig, id, beschreibung)
	if err != nil {
		return AnlegenErgebnis{}, err
	}
	a := Asset{ID: id, Rig: rig, Gruppe: gruppe, Pfad: Grafik + "/" + gruppe + "/" + id + endung}
	frei := sperre(cli.Root, id)
	err = schreibeNeu(a.Abs(cli.Root), text)
	frei()
	if err != nil {
		return AnlegenErgebnis{}, err
	}
	out := AnlegenErgebnis{Asset: a, Teile: r.VorlageTeile()}
	if err := cli.Validate(ctx, a.Abs(cli.Root)); err != nil {
		ab, ok := AlsAbgelehnt(err)
		if !ok {
			return out, err
		}
		out.Fehler = ab.Fehler
		return out, nil
	}
	out.Render = RenderEins(ctx, cli, a)
	return out, nil
}

// schreibeNeu legt eine Datei über Temp-Datei und Umbenennen an; eine vorhandene Datei bleibt (Fehler).
func schreibeNeu(ziel string, text []byte) error {
	if _, err := os.Stat(ziel); err == nil {
		return fmt.Errorf("%s gibt es schon", filepath.Base(ziel))
	}
	tmp := ziel + ".tmp"
	if err := schreibeDatei(tmp, text); err != nil {
		return err
	}
	return os.Rename(tmp, ziel)
}

// AusgabePNGs zählt die PNGs im Ausgabe-Ordner eines Assets.
func AusgabePNGs(root, id string) (int, error) {
	a, err := Finde(root, id)
	if err != nil {
		return 0, err
	}
	treffer, _ := filepath.Glob(filepath.Join(a.Ausgabe(root), "*.png"))
	return len(treffer), nil
}

// LoeschErgebnis beschreibt ein gelöschtes Asset; Index = Gruppen-Index nachgezogen.
type LoeschErgebnis struct {
	Asset Asset
	PNGs  int
	Index bool
}

// Loesche entfernt Spec und Ausgabe-Ordner und den Eintrag im Gruppen-Index (falls vorhanden).
func Loesche(root, id string) (LoeschErgebnis, error) {
	n, err := AusgabePNGs(root, id)
	if err != nil {
		return LoeschErgebnis{}, err
	}
	a, _ := Finde(root, id)
	defer sperre(root, id)()
	if err := os.Remove(a.Abs(root)); err != nil {
		return LoeschErgebnis{}, err
	}
	if err := os.RemoveAll(a.Ausgabe(root)); err != nil {
		return LoeschErgebnis{}, err
	}
	index, err := ausIndexEntfernen(filepath.Join(filepath.Dir(a.Abs(root)), indexName), id)
	return LoeschErgebnis{Asset: a, PNGs: n, Index: index}, err
}

// ausIndexEntfernen schreibt den Gruppen-Index ohne den Eintrag id; false, wenn es keinen Index gibt.
func ausIndexEntfernen(pfad, id string) (bool, error) {
	b, err := os.ReadFile(pfad)
	if os.IsNotExist(err) {
		return false, nil
	}
	if err != nil {
		return false, err
	}
	var eintraege []json.RawMessage
	if err := json.Unmarshal(b, &eintraege); err != nil {
		return false, fmt.Errorf("%s: %w", indexName, err)
	}
	var rest [][]byte
	for _, e := range eintraege {
		var kopf struct {
			ID string `json:"id"`
		}
		if json.Unmarshal(e, &kopf) == nil && kopf.ID == id {
			continue
		}
		rest = append(rest, e)
	}
	text, err := Format(append(append([]byte("["), bytes.Join(rest, []byte(","))...), ']'))
	if err != nil {
		return false, err
	}
	return true, ersetzeText(pfad, text)
}

func ersetzeText(pfad string, text []byte) error {
	tmp := pfad + ".tmp"
	if err := os.WriteFile(tmp, text, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, pfad)
}
