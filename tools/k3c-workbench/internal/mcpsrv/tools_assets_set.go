package mcpsrv

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"sort"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/assetsui"
	pa "k3c/tools/k3c-workbench/internal/paperassets"
)

type assetsSetIn struct {
	ID    string         `json:"id" jsonschema:"Asset-ID, z. B. monarch"`
	Patch map[string]any `json:"patch" jsonschema:"JSON-Merge-Patch (RFC 7396) auf die Spec: null löscht, Objekte werden gemischt, Listen ersetzt; teile auch als Objekt nach Teil-ID ({teile: {kopf: {...}}}): ändert nur die genannten Teile"`
}

type assetsStyleSetIn struct {
	Patch map[string]any `json:"patch" jsonschema:"JSON-Merge-Patch (RFC 7396) auf public/grafik/stil.paper.json"`
}

type assetsDeleteIn struct {
	ID      string `json:"id" jsonschema:"Asset-ID"`
	Confirm bool   `json:"confirm,omitempty" jsonschema:"true löscht wirklich; ohne nur Hinweis"`
}

type leerIn struct{}

func registerAssetsSet(s *Server) {
	notDestructive, destructive, closed := false, true, false
	add(s, &mcp.Tool{
		Name: "assets_set",
		Description: "Ändert eine Paper-Spec per JSON-Merge-Patch atomar (validate, render, dann ersetzen); ungültig = nichts geändert, " +
			"Antwort je Fehler feld · regel · text. Nutze es bei: jede Änderung an einem Asset. Statt: .paper.json von Hand ändern.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &notDestructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in assetsSetIn) (string, error) {
		return assetsSetText(ctx, s.ws(ctx).root, in)
	})
	add(s, &mcp.Tool{
		Name: "assets_style_get",
		Description: "Stil-Datei verdichtet: Pfad, stilHash, Materialien, Einzeltöne, Papier-Werte. " +
			"Nutze es bei: welche Materialien gibt es. Statt: public/grafik/stil.paper.json lesen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, _ leerIn) (string, error) { return assetsStyleGetText(s.ws(ctx).root) })
	add(s, &mcp.Tool{
		Name: "assets_style_set",
		Description: "Merge-Patch auf die Stil-Datei (nur Sessions mit Lane: stil); prüft Schema und ob Specs entfernte Materialien nutzen, " +
			"startet den Render der betroffenen Assets als Job. Nutze es bei: Farbton oder Papier-Wert ändern. Statt: Stil-Datei von Hand ändern.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &notDestructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in assetsStyleSetIn) (string, error) {
		return assetsStyleSetText(ctx, s.ws(ctx).root, in)
	})
	add(s, &mcp.Tool{
		Name: "assets_delete",
		Description: "Löscht Spec, Ausgabe-Ordner und den Eintrag im Gruppen-Index; ohne confirm=true nur Hinweis. " +
			"Nutze es bei: Asset entfernen. Statt: Dateien von Hand löschen.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in assetsDeleteIn) (string, error) {
		return assetsDeleteText(s.ws(ctx).root, in)
	})
}

// ablehnung macht aus einem *pa.Abgelehnt den Vertragstext; andere Fehler bleiben.
func ablehnung(err error) error {
	ab, ok := pa.AlsAbgelehnt(err)
	if !ok {
		return err
	}
	text := fmt.Sprintf("abgelehnt, nichts geändert: %d Fehler\n%s", len(ab.Fehler), fehlerZeilen(ab.Fehler))
	for _, f := range ab.Fehler {
		if f.Regel == "material-unbekannt" {
			return errors.New(text + "\nErlaubt: Papier-Materialien und Einzeltöne aus assets_style_get")
		}
	}
	return errors.New(text + "\nRegeln: docs/standards/paper-spec.md › Regeln")
}

func feldText(f assetsui.FeldAenderung) string {
	return f.Pfad + " " + pa.Wert(f.Alt) + " → " + pa.Wert(f.Neu)
}

func felderText(felder []assetsui.FeldAenderung) string {
	z := make([]string, len(felder))
	for i, f := range felder {
		z[i] = feldText(f)
	}
	return strings.Join(z, "; ")
}

// prozent schreibt einen Anteil 0..1 als Prozent mit Komma und zwei Nachkommastellen.
func prozent(f float64) string {
	return strings.Replace(fmt.Sprintf("%.2f", f*100), ".", ",", 1) + " %"
}

func assetsSetText(ctx context.Context, root string, in assetsSetIn) (string, error) {
	patch, err := json.Marshal(in.Patch)
	if err != nil {
		return "", err
	}
	r, err := pa.Set(ctx, pa.NewCLI(root), in.ID, patch)
	if err != nil {
		return "", ablehnung(err)
	}
	var b strings.Builder
	if len(r.Felder) == 0 {
		fmt.Fprintf(&b, "%s geändert: keine Feldänderung\n", r.Asset.ID)
	} else {
		fmt.Fprintf(&b, "%s geändert: %d %s · %s\n", r.Asset.ID, len(r.Felder), mehrzahl(len(r.Felder), "Feld", "Felder"), felderText(r.Felder))
	}
	fmt.Fprintf(&b, "Render: %d %s", r.PNGs, mehrzahl(r.PNGs, "PNG", "PNGs"))
	if len(r.Animationen) > 0 {
		fmt.Fprintf(&b, " (%s)", strings.Join(r.Animationen, ", "))
	}
	b.WriteString(pixelText(r.Pixel))
	var vorschau []string
	if len(r.Dateien) > 0 {
		vorschau = append(vorschau, "Vorschau: "+r.Dateien[0])
	}
	if o := groessteAbweichung(r.Pixel); o != nil && o.Overlay != "" {
		vorschau = append(vorschau, "Overlay: "+o.Overlay)
	}
	if len(vorschau) > 0 {
		b.WriteString("\n" + strings.Join(vorschau, " · "))
	}
	if len(r.Reiter) > 0 {
		fmt.Fprintf(&b, "\nReiter veraltet: %s · Weiter: assets_render {id} je Reiter", strings.Join(r.Reiter, ", "))
	}
	return b.String(), nil
}

func pixelText(px []pa.Pixel) string {
	if len(px) == 0 {
		return ""
	}
	teile := make([]string, len(px))
	for i, p := range px {
		switch {
		case p.Fehler == "neu":
			teile[i] = p.Animation + " neu"
		case p.Fehler != "":
			teile[i] = p.Animation + " –"
		default:
			teile[i] = p.Animation + " " + prozent(p.Anteil)
		}
	}
	t := " · Pixel-Anteil " + strings.Join(teile, ", ")
	if g := groessteAbweichung(px); g != nil && len(g.Boxen) > 0 {
		bx := g.Boxen[0]
		t += fmt.Sprintf(" (Box %s %d,%d %d×%d)", g.Animation, bx[0], bx[1], bx[2], bx[3])
	}
	return t
}

// groessteAbweichung ist die Animation mit dem größten Pixel-Anteil (> 0), sonst nil.
func groessteAbweichung(px []pa.Pixel) *pa.Pixel {
	var best *pa.Pixel
	for i := range px {
		if px[i].Fehler == "" && px[i].Anteil > 0 && (best == nil || px[i].Anteil > best.Anteil) {
			best = &px[i]
		}
	}
	return best
}

func assetsStyleGetText(root string) (string, error) {
	roh, k, err := leseStil(root)
	if err != nil {
		return "", err
	}
	hash, err := pa.StilHash(roh)
	if err != nil {
		return "", err
	}
	mat, ton := pa.Schluessel(roh, "materialien"), pa.Schluessel(roh, "einzeltoene")
	p := k.Papier
	return fmt.Sprintf("%s · stilHash %s\nMaterialien (%d): %s\nEinzeltöne (%d): %s\n"+
		"papier: lage %s/%s/%s/%s %s · auflage %s/%s/%s/%s · struktur %s/%s/%s · falz %s · licht %s\n"+
		"Vollständig: %s (ändern nur mit assets_style_set)", pa.StilPfad, hash, len(mat), strings.Join(mat, ", "), len(ton), strings.Join(ton, ", "),
		zahl(p.Lage.Dx), zahl(p.Lage.Dy), zahl(p.Lage.Unschaerfe), zahl(p.Lage.Deckkraft), p.Lage.Farbe,
		zahl(p.Auflage.Dx), zahl(p.Auflage.Dy), zahl(p.Auflage.Unschaerfe), zahl(p.Auflage.Deckkraft),
		zahl(p.Struktur.Frequenz), zahl(p.Struktur.Oktaven), zahl(p.Struktur.Staerke), zahl(p.Falz), k.Licht, pa.StilPfad), nil
}

func assetsStyleSetText(ctx context.Context, root string, in assetsStyleSetIn) (string, error) {
	patch, err := json.Marshal(in.Patch)
	if err != nil {
		return "", err
	}
	cli := pa.NewCLI(root)
	r, err := pa.StilSet(ctx, cli, patch)
	if err != nil {
		return "", ablehnung(err)
	}
	var b strings.Builder
	if len(r.Felder) == 0 {
		b.WriteString("Stil unverändert: der Patch ändert kein Feld")
		return b.String(), nil
	}
	z := make([]string, len(r.Felder))
	for i, f := range r.Felder {
		z[i] = feldText(f)
	}
	fmt.Fprintf(&b, "Stil geändert: %s\n", strings.Join(z, ", "))
	if len(r.Betroffen) == 0 {
		b.WriteString("Betroffen: keine Assets")
		return b.String(), nil
	}
	ids := make([]string, len(r.Betroffen))
	for i, a := range r.Betroffen {
		ids[i] = a.ID
	}
	sort.Strings(ids)
	job := assetJobs.Start(cli, r.Betroffen)
	fmt.Fprintf(&b, "Betroffen: %d %s (%s) · Render als Job %s gestartet\nWeiter: assets_render {job: %q, wait: true}",
		len(ids), mehrzahl(len(ids), "Asset", "Assets"), strings.Join(ids, ", "), job.ID, job.ID)
	return b.String(), nil
}

func assetsDeleteText(root string, in assetsDeleteIn) (string, error) {
	if !in.Confirm {
		n, err := pa.AusgabePNGs(root, in.ID)
		if err != nil {
			return "", err
		}
		return fmt.Sprintf("nicht gelöscht: assets_delete braucht confirm=true (%s, %d %s)", in.ID, n, mehrzahl(n, "PNG", "PNGs")), nil
	}
	r, err := pa.Loesche(root, in.ID)
	if err != nil {
		return "", err
	}
	index := "kein Gruppen-Index"
	if r.Index {
		index = "Gruppen-Index " + r.Asset.Gruppe + " nachgezogen"
	}
	return fmt.Sprintf("%s gelöscht: %s und %s/%s/ (%d %s); %s", r.Asset.ID, r.Asset.Pfad, pa.Grafik+"/"+r.Asset.Gruppe, r.Asset.ID,
		r.PNGs, mehrzahl(r.PNGs, "PNG", "PNGs"), index), nil
}
