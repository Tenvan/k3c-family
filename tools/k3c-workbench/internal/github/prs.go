package github

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"strings"
	"time"
	"unicode/utf8"
)

// Offene PRs und PR-Aktionen für die Seite Git (B-580). Ziel ist immer develop; Branches löscht die Seite selbst
// (gitops.Delete), deshalb nie --delete-branch: gh würde dabei auch den Checkout wechseln.

// OpenPR ist ein offener PR gegen develop.
type OpenPR struct {
	Number  int    `json:"number"`
	Title   string `json:"title"`
	URL     string `json:"url"`
	Head    string `json:"head"`
	Author  string `json:"author"`
	Updated string `json:"updated"` // RFC 3339
	State   string `json:"state"`   // offen | Entwurf
	CI      string `json:"ci"`      // grün | rot | läuft | –
	Merge   string `json:"merge"`   // konfliktfrei | Konflikt | unbekannt
}

// PRList sind die offenen PRs; Error ist ein Hinweis (gh fehlt, nicht angemeldet).
type PRList struct {
	PRs     []OpenPR `json:"prs"`
	Error   string   `json:"error,omitempty"`
	Fetched string   `json:"fetched,omitempty"`
}

var openArgs = []string{"gh", "pr", "list", "--state", "open", "--base", "develop", "--limit", "50", "--json",
	"number,title,state,isDraft,headRefName,mergeable,statusCheckRollup,url,author,updatedAt"}

const (
	maxTitle = 200
	maxBody  = 32 << 10
)

// Open holt die offenen PRs ohne Zwischenspeicher (die Seite fragt nur auf Knopfdruck).
func (c *Client) Open(ctx context.Context) PRList {
	ctx, cancel := context.WithTimeout(ctx, timeout)
	defer cancel()
	out, err := c.run(ctx, c.root, openArgs)
	if err != nil {
		return PRList{PRs: []OpenPR{}, Error: hint(err)}
	}
	prs, err := ParseOpen(out)
	if err != nil {
		return PRList{PRs: []OpenPR{}, Error: err.Error()}
	}
	return PRList{PRs: prs, Fetched: time.Now().Format(time.RFC3339)}
}

// ParseOpen wertet `gh pr list --json` mit Autor und Zeit aus, neueste Nummer zuerst.
func ParseOpen(b []byte) ([]OpenPR, error) {
	var prs []struct {
		pr
		Author struct {
			Login string `json:"login"`
		} `json:"author"`
		UpdatedAt string `json:"updatedAt"`
	}
	if err := json.Unmarshal(b, &prs); err != nil {
		return nil, fmt.Errorf("gh pr list: %w", err)
	}
	list := make([]OpenPR, 0, len(prs))
	for _, p := range prs {
		list = append(list, OpenPR{Number: p.Number, Title: p.Title, URL: p.URL, Head: p.Head, Author: p.Author.Login,
			Updated: p.UpdatedAt, State: prState(p.pr), CI: ciState(p.Checks), Merge: mergeState(p.pr)})
	}
	return list, nil
}

// MergedHead ist ein gemergter PR mit seinem letzten Commit; die Seite Git erkennt daran Squash-Merges.
type MergedHead struct {
	Number int    `json:"number"`
	Head   string `json:"headRefName"`
	SHA    string `json:"headRefOid"`
}

// Nur PRs nach develop: ein PR in einen Sprint- oder Feature-Branch bringt nichts nach develop.
var mergedArgs = []string{"gh", "pr", "list", "--state", "merged", "--base", "develop", "--limit", "500", "--json",
	"number,headRefName,headRefOid"}

// Merged liest die letzten 500 nach develop gemergten PRs (eine Abfrage für alle Branches).
func (c *Client) Merged(ctx context.Context) ([]MergedHead, error) {
	ctx, cancel := context.WithTimeout(ctx, 2*timeout)
	defer cancel()
	out, err := c.run(ctx, c.root, mergedArgs)
	if err != nil {
		return nil, errors.New(hint(err))
	}
	var list []MergedHead
	if err := json.Unmarshal(out, &list); err != nil {
		return nil, fmt.Errorf("gh pr list: %w", err)
	}
	return list, nil
}

// Create legt einen PR von head nach develop an und liefert seine Adresse.
func (c *Client) Create(ctx context.Context, head, title, body string) (string, error) {
	title = strings.TrimSpace(title)
	switch {
	case title == "":
		return "", errors.New("kein Titel angegeben")
	case utf8.RuneCountInString(title) > maxTitle || len(body) > maxBody:
		return "", errors.New("zu langer Titel oder zu lange Beschreibung")
	case head == "develop" || head == "main":
		return "", fmt.Errorf("von %s gibt es keinen PR", head)
	}
	// Werte mit "=": ein Titel, der mit "-" beginnt, bleibt so ein Wert und wird keine Option.
	out, err := c.do(ctx, "gh", "pr", "create", "--base=develop", "--head="+head, "--title="+title, "--body="+body)
	return strings.TrimSpace(lastOf(out)), err
}

// Merge mergt PR number, mit squash als Squash-Merge. Nur offene PRs gegen develop; --match-head-commit lässt GitHub
// ablehnen, wenn seit dem Lesen jemand nachgepusht hat.
func (c *Client) Merge(ctx context.Context, number int, squash bool) error {
	head, err := c.checkOpen(ctx, number)
	if err != nil {
		return err
	}
	how := "--merge"
	if squash {
		how = "--squash"
	}
	_, err = c.do(ctx, "gh", "pr", "merge", strconv.Itoa(number), how, "--match-head-commit="+head)
	return err
}

// Close schließt PR number (nur offen und gegen develop) ohne Merge.
func (c *Client) Close(ctx context.Context, number int) error {
	if _, err := c.checkOpen(ctx, number); err != nil {
		return err
	}
	_, err := c.do(ctx, "gh", "pr", "close", strconv.Itoa(number))
	return err
}

// checkOpen liest den PR frisch und liefert seinen Head-Commit; die Liste der Oberfläche kann veraltet sein.
func (c *Client) checkOpen(ctx context.Context, number int) (string, error) {
	if number <= 0 {
		return "", errors.New("ungültige PR-Nummer")
	}
	ctx, cancel := context.WithTimeout(ctx, timeout)
	defer cancel()
	out, err := c.run(ctx, c.root, []string{"gh", "pr", "view", strconv.Itoa(number), "--json", "state,baseRefName,headRefOid"})
	if err != nil {
		return "", errors.New(hint(err))
	}
	var p struct {
		State string `json:"state"`
		Base  string `json:"baseRefName"`
		Head  string `json:"headRefOid"`
	}
	switch {
	case json.Unmarshal(out, &p) != nil || p.Head == "":
		return "", fmt.Errorf("PR #%d: Antwort von gh unlesbar", number)
	case p.State != "OPEN":
		return "", fmt.Errorf("PR #%d ist nicht mehr offen", number)
	case p.Base != "develop":
		return "", fmt.Errorf("PR #%d zielt auf %s, die Seite bearbeitet nur PRs gegen develop", number, p.Base)
	}
	return p.Head, nil
}

// do ruft gh für eine Aktion auf und verwirft den Zwischenspeicher der Planung, damit sie den neuen Stand zeigt.
func (c *Client) do(ctx context.Context, args ...string) (string, error) {
	ctx, cancel := context.WithTimeout(ctx, 2*timeout)
	defer cancel()
	out, err := c.run(ctx, c.root, args)
	c.mu.Lock()
	c.at = time.Time{}
	c.mu.Unlock()
	if err != nil {
		return "", errors.New(hint(err))
	}
	return string(out), nil
}

func lastOf(s string) string {
	lines := strings.Split(strings.TrimSpace(s), "\n")
	return lines[len(lines)-1]
}
