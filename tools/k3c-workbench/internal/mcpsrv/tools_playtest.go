package mcpsrv

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/planning"
	"k3c/tools/k3c-workbench/internal/playtest"
)

// Tools des Spieltests (B-1224): Claude liest Session, Transkript, Marker und Bilder und schlägt Befunde vor; die
// Workbench ruft selbst keine KI auf. Angehakte Befunde werden über die Planungslogik zu Tickets.

const playtestPageLimit = 60 // Zeilen der Zeitleiste je Aufruf von playtest_get

type playtestListIn struct{}

type playtestGetIn struct {
	ID     string `json:"id" jsonschema:"Session-Kennung, z. B. 2026-11-22-1 (aus playtest_list)"`
	Limit  int    `json:"limit,omitempty" jsonschema:"Zeilen der Zeitleiste je Aufruf (Standard 60, höchstens 500)"`
	Cursor int    `json:"cursor,omitempty" jsonschema:"ab welcher Zeile der Zeitleiste (aus der letzten Antwort)"`
}

type playtestFrameIn struct {
	ID string `json:"id" jsonschema:"Session-Kennung"`
	T  int64  `json:"t" jsonschema:"Zeitpunkt in Unix-ms (ts eines Markers oder einer Zeile von playtest_get)"`
}

type playtestFindingsIn struct {
	ID       string               `json:"id" jsonschema:"Session-Kennung"`
	Findings []playtest.FindingIn `json:"findings" jsonschema:"Befunde; vorhandene (gleiche id) werden aktualisiert, Haken und Ticket bleiben"`
}

type playtestTicketsIn struct {
	ID      string `json:"id" jsonschema:"Session-Kennung"`
	Domain  string `json:"domain,omitempty" jsonschema:"Domäne für Befunde ohne eigene (REG, SIM, SRV, CLI, PLAT, INF, DEV, PAP)"`
	Type    string `json:"type,omitempty" jsonschema:"Typ für Befunde ohne eigenen (Idee, Problem, Schuld, Frage)"`
	Prio    string `json:"prio,omitempty" jsonschema:"Prio für Befunde ohne eigene, sonst mittel"`
	Project string `json:"project,omitempty" jsonschema:"Projekt-Kürzel für Befunde ohne eigenes"`
	Sprint  string `json:"sprint,omitempty" jsonschema:"Sprint für Befunde ohne eigenen (fehlt er, zuerst plan_create kind sprint)"`
}

// playtestStore ist die Ablage des Checkouts des Aufrufs.
func (s *Server) playtestStore(ctx context.Context) *playtest.Store {
	return &playtest.Store{Root: filepath.Join(s.ws(ctx).Root, "playtests")}
}

// registerPlaytest sind die Tools der Seite Spieltest.
func registerPlaytest(s *Server) {
	closed, destructive := false, false
	write := &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed}
	add(s, &mcp.Tool{
		Name: "playtest_list",
		Description: "Aufgenommene Spieltests (playtests/) mit Stand, Dauer, Markern und Befunden, neueste zuerst. " +
			"Nutze es bei: Auswertung eines Spieltests beginnen. Statt: playtests/ durchsuchen.",
		Annotations: readOnly(),
	}, s.playtestList)
	add(s, &mcp.Tool{
		Name: "playtest_get",
		Description: "Eine Session: Kopf, Fragebogen, Befunde und die Zeitleiste aus Transkript-Sätzen und Markern (Spielkontext, Bildpfad, " +
			"Zeitpunkt ts in Unix-ms), seitenweise mit limit und cursor. Nutze es bei: Transkript und Marker lesen, um Befunde zu bilden. " +
			"Statt: transcript.json und markers.jsonl von Hand öffnen.",
		Annotations: readOnly(),
	}, s.playtestGet)
	add(s, &mcp.Tool{
		Name: "playtest_frame",
		Description: "Zieht ein Bild an Zeitpunkt t (Unix-ms) aus video.mkv nach frames/t-<t>.jpg und nennt den Pfad (dann mit Read ansehen). " +
			"Nutze es bei: ein Bild zu einer Transkript-Stelle ohne Marker. Statt: ffmpeg von Hand aufrufen.",
		Annotations: write,
	}, s.playtestFrame)
	add(s, &mcp.Tool{
		Name: "playtest_findings_set",
		Description: "Schreibt oder aktualisiert Befunde (findings.json) einer Session; je Befund id, quote, ts, Vorschlag für Typ, Prio, Projekt, " +
			"Sprint und duplicate_of (offenes Ticket, vorher mit plan_list geprüft). Haken (selected) und Ticket setzt nur Ralf bzw. playtest_tickets_create. " +
			"Nutze es bei: Befunde aus Transkript und Markern vorschlagen. Statt: findings.json von Hand schreiben.",
		Annotations: write,
	}, s.playtestFindingsSet)
	add(s, &mcp.Tool{
		Name: "playtest_tickets_create",
		Description: "Legt für alle angehakten Befunde ohne Ticket per Planungslogik ein Ticket an (Zitat, Bild, Spielkontext; Domäne, Typ, Prio, Projekt, Sprint " +
			"vom Befund oder aus den Argumenten) und trägt die ID ein; Befunde mit duplicate_of bekommen stattdessen einen Hinweis im vorhandenen Ticket. " +
			"Nutze es bei: Ralf hat Befunde angehakt und Tickets angefordert. Statt: je Befund plan_create und plan_sections von Hand.",
		Annotations: write,
	}, s.playtestTicketsCreate)
}

func (s *Server) playtestList(ctx context.Context, _ playtestListIn) (string, error) {
	list, err := s.playtestStore(ctx).List()
	if err != nil {
		return "", err
	}
	if len(list) == 0 {
		return "keine Spieltests in playtests/", nil
	}
	var b strings.Builder
	fmt.Fprintf(&b, "%d Spieltests (neueste zuerst):\n", len(list))
	for _, x := range list {
		n, _ := s.playtestStore(ctx).Findings(x.ID)
		fmt.Fprintf(&b, "- %s · %s · %s · %d Marker · %d Befunde · Sprache %s, Video %s\n", x.ID, x.Stage, minutes(x.DurationMs), x.Markers, len(n),
			yesNo(x.HasVoice), yesNo(x.HasVideo))
	}
	return b.String(), nil
}

func yesNo(b bool) string {
	if b {
		return "ja"
	}
	return "nein"
}

func minutes(ms int64) string {
	if ms <= 0 {
		return "Dauer ?"
	}
	return fmt.Sprintf("%d min", max(1, int(time.Duration(ms)*time.Millisecond/time.Minute)))
}

func clock(ms int64) string {
	sec := max(ms, 0) / 1000
	return fmt.Sprintf("%02d:%02d", sec/60, sec%60)
}

func (s *Server) playtestGet(ctx context.Context, in playtestGetIn) (string, error) {
	st := s.playtestStore(ctx)
	ses, err := st.Get(in.ID)
	if err != nil {
		return "", err
	}
	tl, err := st.Timeline(in.ID)
	if err != nil {
		return "", err
	}
	limit := in.Limit
	if limit <= 0 {
		limit = playtestPageLimit
	}
	limit = min(limit, 500)
	cur := max(in.Cursor, 0)
	var b strings.Builder
	if cur == 0 {
		s.playtestHead(&b, st, ses, tl)
	}
	end := min(cur+limit, len(tl.Entries))
	fmt.Fprintf(&b, "Zeitleiste %d–%d von %d (ts = Unix-ms, mm:ss = Videozeit):\n", min(cur+1, end), end, len(tl.Entries))
	dir := filepath.Join(st.Root, ses.ID)
	for i := cur; i < end; i++ {
		b.WriteString(timelineLine(dir, tl.Entries[i]) + "\n")
	}
	if end < len(tl.Entries) {
		fmt.Fprintf(&b, "weiter mit cursor=%d\n", end)
	}
	return b.String(), nil
}

// playtestHead ist der Kopf der ersten Seite: Stand, Hinweise, Fragebogen, Befunde.
func (s *Server) playtestHead(b *strings.Builder, st *playtest.Store, ses playtest.Session, tl playtest.Timeline) {
	fmt.Fprintf(b, "Session %s · %s · %s · %d Marker\n", ses.ID, ses.Stage, minutes(ses.DurationMs), ses.Markers)
	if !tl.HasTranscript {
		b.WriteString("Hinweis: noch kein Transkript (Ralf muss auf der Seite Spieltest Auswerten drücken); nur Marker.\n")
	}
	if !ses.HasVideo {
		b.WriteString("Hinweis: video.mkv fehlt, keine neuen Bilder möglich.\n")
	}
	if raw, err := os.ReadFile(filepath.Join(st.Root, ses.ID, "survey.json")); err == nil {
		txt := strings.Join(strings.Fields(string(raw)), " ")
		if r := []rune(txt); len(r) > 1500 {
			txt = string(r[:1500]) + " …"
		}
		fmt.Fprintf(b, "Fragebogen: %s\n", txt)
	} else {
		b.WriteString("Fragebogen: –\n")
	}
	list, _ := st.Findings(ses.ID)
	fmt.Fprintf(b, "Befunde: %d\n", len(list))
	for _, f := range list {
		fmt.Fprintf(b, "- %s%s%s · ts %d · „%s“\n", f.ID, checkMark(f), ticketNote(f), f.Ts, oneLine(f.Quote, 100))
	}
}

func checkMark(f playtest.Finding) string {
	if f.Selected {
		return " [angehakt]"
	}
	return ""
}

func ticketNote(f playtest.Finding) string {
	switch {
	case f.Ticket != "":
		return " → " + f.Ticket
	case f.DuplicateOf != "":
		return " (Duplikat von " + f.DuplicateOf + ")"
	case f.Error != "":
		return " (Fehler: " + f.Error + ")"
	}
	return ""
}

func oneLine(s string, n int) string {
	s = strings.Join(strings.Fields(s), " ")
	if r := []rune(s); len(r) > n {
		return string(r[:n]) + " …"
	}
	return s
}

func timelineLine(dir string, e playtest.Entry) string {
	head := fmt.Sprintf("[%s ts=%d]", clock(e.VideoMs), e.Ms)
	if e.Kind == "sprache" {
		return head + " Sprache: " + e.Text
	}
	m := e.Marker
	line := fmt.Sprintf("%s Marker #%d %s", head, e.Nr, e.Text)
	if m != nil {
		line += fmt.Sprintf(" · Welt %q Phase %s Tag %d · schlechtester Frame %.1f ms", m.World, m.Phase, m.Day, m.WorstFrameMs)
		if m.FrameMs != nil {
			line += fmt.Sprintf(" · Ruckler %.1f ms", *m.FrameMs)
		}
	}
	if e.Skip {
		line += " · Bild nicht ausgewählt (nicht ansehen)"
	} else if e.Frame != "" {
		if e.Desc != "" {
			line += " · Bildbeschreibung: " + strings.Join(strings.Fields(e.Desc), " ")
		}
		line += " · Bild " + filepath.Join(dir, filepath.FromSlash(e.Frame))
	}
	return line
}

func (s *Server) transcriber(ctx context.Context) *playtest.Transcriber {
	root := s.ws(ctx).Root
	return playtest.NewTranscriber(root, s.playtestStore(ctx), nil)
}

func (s *Server) playtestFrame(ctx context.Context, in playtestFrameIn) (string, error) {
	p, err := s.transcriber(ctx).FrameAt(in.ID, in.T)
	if err != nil {
		return "", err
	}
	return "Bild: " + p + "\nCheckout: " + s.ws(ctx).Root, nil
}

func (s *Server) playtestFindingsSet(ctx context.Context, in playtestFindingsIn) (string, error) {
	added, updated, err := s.playtestStore(ctx).SetFindings(in.ID, in.Findings)
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("Befunde geschrieben: %d neu, %d aktualisiert (Session %s). Ralf hakt sie auf der Seite Spieltest an.\nCheckout: %s",
		added, updated, in.ID, s.ws(ctx).Root), nil
}

func (s *Server) playtestTicketsCreate(ctx context.Context, in playtestTicketsIn) (string, error) {
	root := s.ws(ctx).Root
	planner := playtest.Planner{
		Create: func(slug, title string, fields map[string]string) (string, error) {
			return planning.Create(root, planning.NewDoc{Kind: "ticket", Slug: slug, Title: title, Fields: fields})
		},
		Get:      func(id string) (string, error) { return planning.Get(root, id) },
		Sections: func(id string, b map[string]string) error { _, err := planning.Sections(root, id, b); return err },
	}
	res, err := s.playtestStore(ctx).CreateTickets(in.ID, playtest.TicketDefaults{Domain: in.Domain, Type: in.Type, Prio: in.Prio,
		Project: in.Project, Sprint: in.Sprint}, planner)
	if err != nil {
		return "", err
	}
	return ticketReport(res) + "\nCheckout: " + root, nil
}

func ticketReport(r playtest.TicketResult) string {
	var b strings.Builder
	fmt.Fprintf(&b, "Tickets: %d angelegt, %d Hinweise in vorhandenen Tickets, %d fehlgeschlagen, %d übersprungen (nicht angehakt oder schon erledigt)\n",
		len(r.Created), len(r.Noted), len(r.Failed), r.Skipped)
	for _, group := range [][]string{r.Created, r.Noted, r.Failed, r.Similar} {
		for _, l := range group {
			b.WriteString("- " + l + "\n")
		}
	}
	return strings.TrimRight(b.String(), "\n")
}
