// Package github liest den GitHub-Stand der Sprints über die GitHub CLI `gh` (B-212): je Sprint-Branch
// `sprint/<präfix>` den neuesten PR gegen develop mit CI- und Merge-Stand, dazu den letzten CI-Lauf auf develop.
// Die Anmeldung liegt bei `gh`; k3c-dev speichert kein Token.
package github

import (
	"encoding/json"
	"fmt"
	"sort"
	"strings"
	"time"
)

// SprintPR ist der Stand eines Sprints auf GitHub.
type SprintPR struct {
	Number int    `json:"number"`
	Title  string `json:"title"`
	URL    string `json:"url"`
	State  string `json:"state"` // offen | Entwurf | gemergt | geschlossen
	CI     string `json:"ci"`    // grün | rot | läuft | –
	Merge  string `json:"merge"` // konfliktfrei | Konflikt | unbekannt | – (nicht offen)
}

// Run ist der letzte CI-Lauf auf develop.
type Run struct {
	CI      string `json:"ci"`
	Title   string `json:"title"`
	URL     string `json:"url"`
	Created string `json:"created"`
}

// Data ist der Stand für Oberfläche und Tool; Error ist ein Hinweis (gh fehlt, nicht angemeldet, veraltet).
type Data struct {
	Sprints map[string]SprintPR `json:"sprints"` // Schlüssel: Sprint-ID in Großbuchstaben (M8)
	Develop *Run                `json:"develop,omitempty"`
	Error   string              `json:"error,omitempty"`
	Fetched string              `json:"fetched,omitempty"` // RFC 3339, leer = nie geholt
}

type check struct {
	Name       string `json:"name"`
	Context    string `json:"context"`
	Status     string `json:"status"`     // CheckRun: QUEUED, IN_PROGRESS, COMPLETED
	Conclusion string `json:"conclusion"` // CheckRun: SUCCESS, FAILURE …
	State      string `json:"state"`      // StatusContext: SUCCESS, FAILURE, PENDING, ERROR
	StartedAt  string `json:"startedAt"`
}

type pr struct {
	Number    int     `json:"number"`
	Title     string  `json:"title"`
	State     string  `json:"state"`
	IsDraft   bool    `json:"isDraft"`
	Head      string  `json:"headRefName"`
	Mergeable string  `json:"mergeable"`
	URL       string  `json:"url"`
	Checks    []check `json:"statusCheckRollup"`
}

type run struct {
	Status     string `json:"status"`
	Conclusion string `json:"conclusion"`
	Title      string `json:"displayTitle"`
	URL        string `json:"url"`
	CreatedAt  string `json:"createdAt"`
}

// Parse wertet die JSON-Ausgaben von `gh pr list` und `gh run list` aus; runsJSON darf leer sein.
func Parse(prsJSON, runsJSON []byte, now time.Time) (Data, error) {
	var prs []pr
	if err := json.Unmarshal(prsJSON, &prs); err != nil {
		return Data{}, fmt.Errorf("gh pr list: %w", err)
	}
	d := Data{Sprints: map[string]SprintPR{}, Fetched: now.Format(time.RFC3339)}
	for _, p := range prs {
		id, ok := strings.CutPrefix(p.Head, "sprint/")
		if !ok || id == "" {
			continue
		}
		id = strings.ToUpper(id)
		if old, seen := d.Sprints[id]; seen && old.Number > p.Number {
			continue // je Sprint der neueste PR
		}
		d.Sprints[id] = SprintPR{Number: p.Number, Title: p.Title, URL: p.URL, State: prState(p), CI: ciState(p.Checks),
			Merge: mergeState(p)}
	}
	if len(runsJSON) > 0 {
		var runs []run
		if err := json.Unmarshal(runsJSON, &runs); err != nil {
			return Data{}, fmt.Errorf("gh run list: %w", err)
		}
		if len(runs) > 0 {
			r := runs[0]
			d.Develop = &Run{CI: runCI(r.Status, r.Conclusion), Title: r.Title, URL: r.URL, Created: r.CreatedAt}
		}
	}
	return d, nil
}

func prState(p pr) string {
	switch {
	case p.State == "MERGED":
		return "gemergt"
	case p.State == "CLOSED":
		return "geschlossen"
	case p.IsDraft:
		return "Entwurf"
	}
	return "offen"
}

func mergeState(p pr) string {
	switch {
	case p.State != "OPEN":
		return "–"
	case p.Mergeable == "MERGEABLE":
		return "konfliktfrei"
	case p.Mergeable == "CONFLICTING":
		return "Konflikt"
	}
	return "unbekannt" // GitHub rechnet noch
}

// ciState fasst die Checks zusammen; ein neu gestarteter Check gleichen Namens ersetzt den älteren.
func ciState(checks []check) string {
	latest := map[string]check{}
	for _, c := range checks {
		key := c.Name + c.Context
		if old, ok := latest[key]; !ok || c.StartedAt >= old.StartedAt {
			latest[key] = c
		}
	}
	if len(latest) == 0 {
		return "–"
	}
	state := "grün"
	for _, c := range latest {
		switch checkResult(c) {
		case "rot":
			return "rot"
		case "läuft":
			state = "läuft"
		}
	}
	return state
}

func checkResult(c check) string {
	if c.State == "" { // CheckRun
		return runCI(c.Status, c.Conclusion)
	}
	switch c.State { // StatusContext
	case "SUCCESS":
		return "grün"
	case "PENDING", "EXPECTED":
		return "läuft"
	}
	return "rot"
}

func runCI(status, conclusion string) string {
	if !strings.EqualFold(status, "COMPLETED") {
		return "läuft"
	}
	switch strings.ToUpper(conclusion) {
	case "SUCCESS", "NEUTRAL", "SKIPPED":
		return "grün"
	}
	return "rot"
}

// Text ist die knappe Fassung für das MCP-Tool gh_status: eine Zeile je Sprint, dann develop.
func Text(d Data) string {
	var out []string
	if d.Error != "" {
		out = append(out, "Hinweis: "+d.Error)
	}
	ids := make([]string, 0, len(d.Sprints))
	for id := range d.Sprints {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	for _, id := range ids {
		p := d.Sprints[id]
		line := fmt.Sprintf("%s #%d %s · CI %s", id, p.Number, p.State, p.CI)
		if p.Merge != "–" {
			line += " · " + p.Merge
		}
		out = append(out, line+" · "+p.URL)
	}
	if d.Develop != nil {
		out = append(out, fmt.Sprintf("develop: CI %s · %s (%s)", d.Develop.CI, d.Develop.Title, d.Develop.Created))
	}
	if len(out) == 0 {
		return "keine Sprint-PRs"
	}
	return strings.Join(out, "\n")
}
