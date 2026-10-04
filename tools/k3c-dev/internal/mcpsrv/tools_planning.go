package mcpsrv

import (
	"context"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-dev/internal/github"
	"k3c/tools/k3c-dev/internal/planning"
)

type planIDIn struct {
	ID string `json:"id" jsonschema:"Ticket B-123, Sprint M8 oder Session M8.1"`
}

type planSetIn struct {
	ID     string            `json:"id" jsonschema:"Ticket B-123, Sprint M8 oder Session M8.1"`
	Fields map[string]string `json:"fields" jsonschema:"Kopf-Felder wie in der Vorlage, z. B. {\"Status\": \"fertig\"}"`
}

type planSectionIn struct {
	ID      string `json:"id" jsonschema:"Ticket B-123, Sprint M8 oder Session M8.1"`
	Section string `json:"section" jsonschema:"Abschnitt der Vorlage ohne ##, z. B. Ausgangslage, Ergebnis"`
	Text    string `json:"text" jsonschema:"neuer Inhalt (Markdown, Unterüberschriften ab ###)"`
}

type ghStatusIn struct {
	Force bool `json:"force,omitempty" jsonschema:"Zwischenspeicher (60 s) umgehen"`
}

type planCreateIn struct {
	Kind   string            `json:"kind" jsonschema:"ticket, sprint oder session"`
	ID     string            `json:"id,omitempty" jsonschema:"Sprint M9 oder Session M9.1; beim Ticket leer (nächste freie Nummer)"`
	Slug   string            `json:"slug" jsonschema:"Kurzname für Datei oder Ordner: a-z, 0-9, -"`
	Title  string            `json:"title" jsonschema:"Titel, beim Ticket als Aussage"`
	Fields map[string]string `json:"fields,omitempty" jsonschema:"Kopf-Felder; Ticket braucht Domäne, Typ, Prio; Sprint braucht Domäne"`
}

// registerPlanning sind die Planungs-Tools (B-210): lesen und schreiben docs/sprints und docs/backlog.
func registerPlanning(s *Server) {
	closed, destructive, yes := false, false, true
	write := &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed}
	add(s, &mcp.Tool{
		Name:        "plan_list",
		Description: "Sprints mit Sessions und Tickets als je eine Zeile, filterbar nach kind, status, domain, sprint; archive mit erledigten Tickets.",
		Annotations: readOnly(),
	}, func(_ context.Context, in planning.Filter) (string, error) { return planning.List(s.cfg.Root, in) })
	add(s, &mcp.Tool{
		Name:        "plan_get",
		Description: "Ein Ticket, eine Sprint-README oder eine Session als Markdown, mit Pfad in der ersten Zeile.",
		Annotations: readOnly(),
	}, func(_ context.Context, in planIDIn) (string, error) { return planning.Get(s.cfg.Root, in.ID) })
	add(s, &mcp.Tool{
		Name: "plan_create",
		Description: "Legt Ticket (nächste Nummer, Index), Sprint (in geplant/, Fahrplan) oder Session (Session-Tabelle) als " +
			"Kopie der Vorlage an. Inhalte danach mit plan_section füllen.",
		Annotations: write,
	}, func(_ context.Context, in planCreateIn) (string, error) {
		return planning.Create(s.cfg.Root, planning.NewDoc{Kind: in.Kind, ID: in.ID, Slug: in.Slug, Title: in.Title, Fields: in.Fields})
	})
	add(s, &mcp.Tool{
		Name: "plan_set",
		Description: "Setzt Kopf-Felder (Status, Prio, Sprint, Reife, Spec, Revision, Freigabe …) und zieht nach: Index, " +
			"Session-Tabelle, Fahrplan, Archiv bei erledigt/verworfen, Sprint-Ordner beim Status.",
		Annotations: write,
	}, func(_ context.Context, in planSetIn) (string, error) {
		return planning.Set(s.cfg.Root, in.ID, in.Fields)
	})
	add(s, &mcp.Tool{
		Name:        "plan_section",
		Description: "Ersetzt den Inhalt eines Abschnitts (## Name) eines Tickets, Sprints oder einer Session.",
		Annotations: write,
	}, func(_ context.Context, in planSectionIn) (string, error) {
		return planning.Section(s.cfg.Root, in.ID, in.Section, in.Text)
	})
	add(s, &mcp.Tool{
		Name:        "plan_delete",
		Description: "Löscht einen Sprint-Entwurf in geplant/ oder eine Session darin. Tickets nie: plan_set Status verworfen.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &yes, OpenWorldHint: &closed},
	}, func(_ context.Context, in planIDIn) (string, error) { return planning.Delete(s.cfg.Root, in.ID) })
	gh := github.New(s.cfg.Root)
	add(s, &mcp.Tool{
		Name: "gh_status",
		Description: "GitHub-Stand je Sprint über die gh-CLI: PR (offen, Entwurf, gemergt), CI (grün, rot, läuft), " +
			"Merge-Konflikt, dazu der letzte CI-Lauf auf develop. 60 s zwischengespeichert, force holt neu.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in ghStatusIn) (string, error) {
		return github.Text(gh.Status(ctx, in.Force)), nil
	})
}
