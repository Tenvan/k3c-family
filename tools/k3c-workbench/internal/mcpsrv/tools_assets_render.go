package mcpsrv

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	pa "k3c/tools/k3c-workbench/internal/paperassets"
)

// assetJobs sind die Render-Jobs aller Checkouts (eigener Mechanismus, nicht task_*; IDs j-<n> je Prozess).
var assetJobs = pa.NewJobs()

// renderWarten begrenzt {job, wait: true} ohne progressToken (wie check_run); mit Fortschritt wird bis zum Ende gewartet.
var renderWarten = 4 * time.Minute

type assetsRenderIn struct {
	ID   string `json:"id,omitempty" jsonschema:"ein Asset synchron rendern"`
	Alle bool   `json:"alle,omitempty" jsonschema:"true startet einen Job für alle Assets"`
	Job  string `json:"job,omitempty" jsonschema:"Job-ID j-<n>: Stand abfragen"`
	Wait bool   `json:"wait,omitempty" jsonschema:"mit job: bis zum Ende warten"`
}

func registerAssetsRender(s *Server) {
	notDestructive, closed := false, false
	add(s, &mcp.Tool{
		Name: "assets_render",
		Description: "Rendert Assets über die Render-CLI: {id} synchron, {alle: true} als Job (sofort zurück), {job} Stand, " +
			"{job, wait: true} bis zum Ende. Nutze es bei: Ausgabe neu erzeugen, nach assets_style_set warten. Statt: CLI in der Shell.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &notDestructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in assetsRenderIn) (string, error) {
		return assetsRenderText(ctx, s.ws(ctx).root, in)
	})
}

func assetsRenderText(ctx context.Context, root string, in assetsRenderIn) (string, error) {
	formen := 0
	for _, gesetzt := range []bool{in.ID != "", in.Alle, in.Job != ""} {
		if gesetzt {
			formen++
		}
	}
	if formen != 1 || (in.Wait && in.Job == "") {
		return "", errors.New("assets_render braucht genau eins: {id}, {alle: true} oder {job[, wait: true]}")
	}
	cli := pa.NewCLI(root)
	switch {
	case in.ID != "":
		a, err := pa.Finde(root, in.ID)
		if err != nil {
			return "", err
		}
		return renderEinsText(root, pa.RenderEins(ctx, cli, a))
	case in.Alle:
		assets, err := pa.Assets(root)
		if err != nil {
			return "", err
		}
		st := assetJobs.Start(cli, assets)
		return fmt.Sprintf("Job %s gestartet: %d %s (alle)\nWeiter: assets_render {job: %q} oder {job: %q, wait: true}",
			st.ID, st.N, mehrzahl(st.N, "Asset", "Assets"), st.ID, st.ID), nil
	}
	if !in.Wait {
		st, err := assetJobs.Status(in.Job)
		if err != nil {
			return "", err
		}
		return jobText(st), nil
	}
	return renderWait(ctx, in.Job)
}

// renderWait wartet auf den Job: mit progressToken bis zum Ende und Lebenszeichen, sonst höchstens renderWarten.
func renderWait(ctx context.Context, id string) (string, error) {
	progress := progressOf(ctx)
	limit := renderWarten
	if progress != nil {
		limit = 24 * time.Hour
	}
	wctx, cancel := context.WithTimeout(ctx, limit)
	defer cancel()
	if progress != nil {
		go func() {
			start := time.Now()
			tick := time.NewTicker(5 * time.Second)
			defer tick.Stop()
			for {
				select {
				case <-wctx.Done():
					return
				case <-tick.C:
					if st, err := assetJobs.Status(id); err == nil {
						progress(fmt.Sprintf("Job %s %d/%d (%s)", id, st.Erledigt, st.N, st.Aktuell), time.Since(start))
					}
				}
			}
		}()
	}
	st, err := assetJobs.Wait(wctx, id)
	if err != nil && !errors.Is(err, context.DeadlineExceeded) {
		return "", err
	}
	return jobText(st), nil
}

// jobText: laufend eine Zeile, fertig das Ergebnis je Asset.
func jobText(st pa.Stand) string {
	if !st.Fertig {
		return fmt.Sprintf("Job %s läuft %d/%d (%s)", st.ID, st.Erledigt, st.N, st.Aktuell)
	}
	pngs, fehler := 0, 0
	var zeilen []string
	for _, e := range st.Ergebnisse {
		pngs += e.PNGs
		switch {
		case e.Fehler != "":
			fehler++
			zeilen = append(zeilen, e.ID+" · Fehler: "+e.Fehler)
		case e.Uebersprungen != "":
			zeilen = append(zeilen, e.ID+" · übersprungen (ungültig: "+e.Uebersprungen+")")
		default:
			z := fmt.Sprintf("%s · %d %s", e.ID, e.PNGs, mehrzahl(e.PNGs, "PNG", "PNGs"))
			if e.Manifest != "" {
				z += " · " + e.Manifest
			}
			zeilen = append(zeilen, z)
		}
	}
	dauer := strings.Replace(fmt.Sprintf("%.1f", st.Dauer.Seconds()), ".", ",", 1)
	kopf := fmt.Sprintf("Job %s fertig: %d/%d Assets, %d %s, %d Fehler, %s s", st.ID, st.Erledigt, st.N, pngs, mehrzahl(pngs, "PNG", "PNGs"), fehler, dauer)
	return kopf + "\n" + strings.Join(zeilen, "\n")
}

func renderEinsText(root string, e pa.AssetErgebnis) (string, error) {
	if e.Fehler != "" {
		return "", errors.New(e.Fehler)
	}
	if e.Uebersprungen != "" {
		return "", fmt.Errorf("%s nicht gerendert: ungültig (%s); assets_list zeigt die Fehler", e.ID, e.Uebersprungen)
	}
	return fmt.Sprintf("%s gerendert: %d %s, manifest.json · renderKey %s\n%s", e.ID, e.PNGs, mehrzahl(e.PNGs, "PNG", "PNGs"),
		renderKey(root, e.Manifest), e.Manifest), nil
}

// renderKey liest den renderKey aus dem Manifest (Pfad relativ zum Checkout); unbekannt = –.
func renderKey(root, manifest string) string {
	b, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(manifest)))
	if err != nil {
		return "–"
	}
	var m struct {
		RenderKey string `json:"renderKey"`
	}
	if jsonErr := json.Unmarshal(b, &m); jsonErr != nil || m.RenderKey == "" {
		return "–"
	}
	return m.RenderKey
}
