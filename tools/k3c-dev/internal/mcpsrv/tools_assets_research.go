package mcpsrv

import (
	"context"
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-dev/internal/assets"
)

// newSearcher baut die Web-Recherche (Test-Naht: aufgezeichnete Antworten statt Netz).
var newSearcher = assets.NewSearcher

type assetsResearchIn struct {
	ID    string `json:"id" jsonschema:"Asset-Slot, z. B. farm oder sfx:coin"`
	Query string `json:"query,omitempty" jsonschema:"Suchbegriff; leer = Beschreibung des Slots (englische Begriffe finden mehr)"`
}

type assetsHitAddIn struct {
	ID       string `json:"id" jsonschema:"Asset-Slot, z. B. sfx:coin"`
	Titel    string `json:"titel" jsonschema:"Titel des Fundes"`
	Link     string `json:"link" jsonschema:"Seite des Fundes (https)"`
	Quelle   string `json:"quelle,omitempty" jsonschema:"z. B. itch.io, Kenney; Vorgabe Agent"`
	Urheber  string `json:"urheber,omitempty"`
	Lizenz   string `json:"lizenz,omitempty" jsonschema:"wie auf der Seite angegeben; leer = Lizenz prüfen"`
	Vorschau string `json:"vorschau,omitempty" jsonschema:"Bild- oder MP3-Adresse für die Vorschau"`
	Download string `json:"download,omitempty" jsonschema:"direkte Datei, nur wenn die Seite sie mit maschinenlesbarer Lizenz anbietet"`
}

type assetsImportIn struct {
	ID   string `json:"id" jsonschema:"Asset-Slot"`
	Hit  string `json:"hit" jsonschema:"ID des Treffers aus assets_hits"`
	Path string `json:"path,omitempty" jsonschema:"lokale Datei (png bzw. ogg, mp3, wav); leer = direkter Download des Treffers (nur mit erkannter Lizenz)"`
}

type assetsHitDropIn struct {
	ID  string `json:"id" jsonschema:"Asset-Slot"`
	Hit string `json:"hit" jsonschema:"ID des Treffers aus assets_hits"`
}

// registerAssetsResearch sind die Recherche-Tools (B-298/AC-04, AC-06): Treffer liegen in data/assets-recherche.json.
func registerAssetsResearch(s *Server) {
	closed, open, destructive := false, true, false
	add(s, &mcp.Tool{
		Name: "assets_research",
		Description: "Sucht für einen Asset-Slot im Netz (freesound mit Token, Wikimedia Commons, OpenGameArt) und speichert neue Treffer " +
			"mit Link, Quelle, Urheber und Lizenz-Status nach Q70. Nutze es bei: Alternativen für einen Slot finden. Statt: Fundseiten von Hand anlegen.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &open},
	}, func(ctx context.Context, in assetsResearchIn) (string, error) {
		root := s.ws(ctx).root
		res, err := assets.RunResearch(ctx, root, in.ID, in.Query, newSearcher(root), time.Now())
		if err != nil {
			return "", err
		}
		return fmt.Sprintf("%s: %d neu, %d gespeichert%s\n%s", in.ID, res.Added, len(res.Hits), notesText(res.Notes), hitsText(res.Hits)), nil
	})
	add(s, &mcp.Tool{
		Name: "assets_hits",
		Description: "Gespeicherte Recherche-Treffer eines Asset-Slots: ID, Lizenz-Status (erkannt, pruefen, ausgeschlossen), Quelle, Titel, Link. " +
			"Nutze es bei: welche Vorschläge liegen schon vor. Statt: data/assets-recherche.json lesen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in assetsIDIn) (string, error) {
		hits, err := assets.HitsOf(s.ws(ctx).root, in.ID)
		if err != nil {
			return "", err
		}
		if len(hits) == 0 {
			return in.ID + ": keine Treffer", nil
		}
		return fmt.Sprintf("%s: %d Treffer\n%s", in.ID, len(hits), hitsText(hits)), nil
	})
	add(s, &mcp.Tool{
		Name: "assets_hit_add",
		Description: "Legt einen selbst gefundenen Treffer zu einem Asset-Slot ab (Agent hat im Netz gesucht); Lizenz wird nach Q70 eingeordnet, " +
			"unklar bleibt „pruefen“. Nutze es bei: Fund von itch.io, Kenney oder anderen Seiten ohne API. Statt: Link nur im Chat nennen.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in assetsHitAddIn) (string, error) { return hitAdd(s.ws(ctx).root, in) })
	add(s, &mcp.Tool{
		Name: "assets_hit_drop",
		Description: "Verwirft einen Recherche-Treffer eines Asset-Slots. Nutze es bei: 🧑 hat einen Vorschlag abgelehnt. " +
			"Statt: data/assets-recherche.json von Hand ändern.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in assetsHitDropIn) (string, error) {
		if err := assets.DropHit(s.ws(ctx).root, in.ID, in.Hit); err != nil {
			return "", err
		}
		return fmt.Sprintf("%s: Treffer %s verworfen", in.ID, in.Hit), nil
	})
}

func notesText(notes []string) string {
	if len(notes) == 0 {
		return ""
	}
	return " · Hinweise: " + strings.Join(notes, "; ")
}

func hitsText(hits []assets.Hit) string {
	var b strings.Builder
	for _, h := range hits {
		lic := h.License
		if lic == "" {
			lic = "Lizenz ?"
		}
		if h.Flag != "" {
			lic += " [" + h.Flag + "]"
		}
		fmt.Fprintf(&b, "%s %s · %s · %s · %s · %s\n", h.ID, h.LicenseStatus, h.Source, lic, h.Title, h.Link)
	}
	return b.String()
}

// hitAdd legt einen Fund des Agenten ab; nur https-Links.
func hitAdd(root string, in assetsHitAddIn) (string, error) {
	if !strings.HasPrefix(in.Link, "https://") {
		return "", fmt.Errorf("link muss mit https:// beginnen: %q", in.Link)
	}
	quelle := in.Quelle
	if quelle == "" {
		quelle = "Agent"
	}
	hit := assets.Hit{Title: in.Titel, Link: in.Link, Source: quelle, Artist: in.Urheber, License: in.Lizenz, Preview: in.Vorschau, Download: in.Download}
	n, err := assets.AddHits(root, in.ID, []assets.Hit{hit}, time.Now())
	if err != nil {
		return "", err
	}
	if n == 0 {
		return in.ID + ": Link schon gespeichert", nil
	}
	return in.ID + ": Treffer gespeichert", nil
}

// registerAssetsImport übernimmt Treffer (B-298/AC-05); die Lizenz bestätigt nur 🧑 in der Oberfläche, nie ein Tool.
func registerAssetsImport(s *Server) {
	open, destructive := true, false
	add(s, &mcp.Tool{
		Name: "assets_import",
		Description: "Übernimmt einen Recherche-Treffer: lokale Datei (path) oder direkter Download; legt sie unter public/…/kandidaten/ ab, " +
			"trägt den Credit ein (Lizenz prüfen bis 🧑 bestätigt) und hängt sie als letzte Präferenz an. Nutze es bei: Fund übernehmen. " +
			"Statt: Datei von Hand kopieren und CREDITS bearbeiten.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &open},
	}, func(ctx context.Context, in assetsImportIn) (string, error) {
		root := s.ws(ctx).root
		var rel string
		var err error
		if in.Path == "" {
			rel, err = assets.DownloadHit(ctx, root, in.ID, in.Hit, http.DefaultClient, time.Now())
		} else {
			// Über MCP nur Dateien im eigenen Checkout (z. B. .work/): kein Agent kopiert fremde Dateien nach public/.
			var src string
			if src, err = insideRoot(root, in.Path); err == nil {
				rel, err = assets.ImportFile(root, in.ID, in.Hit, src, time.Now())
			}
		}
		if err != nil {
			return "", err
		}
		return fmt.Sprintf("%s: %s importiert, Credit eingetragen, letzte Präferenz", in.ID, rel), nil
	})
}
