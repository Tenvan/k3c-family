package github

import (
	"context"
	"errors"
	"reflect"
	"strings"
	"testing"
)

func TestParseOpen(t *testing.T) {
	b := []byte(`[{"number":452,"title":"K5","state":"OPEN","isDraft":false,"headRefName":"sprint/k5","mergeable":"MERGEABLE",
"url":"u","author":{"login":"tenvan"},"updatedAt":"2026-10-24T08:00:00Z",
"statusCheckRollup":[{"name":"ci","status":"COMPLETED","conclusion":"SUCCESS"}]},
{"number":447,"title":"W11","state":"OPEN","isDraft":true,"headRefName":"sprint/w11","mergeable":"CONFLICTING",
"url":"v","author":{"login":"x"},"updatedAt":"","statusCheckRollup":[{"context":"lint","state":"FAILURE"}]}]`)
	got, err := ParseOpen(b)
	if err != nil {
		t.Fatal(err)
	}
	want := []OpenPR{
		{Number: 452, Title: "K5", URL: "u", Head: "sprint/k5", Author: "tenvan", Updated: "2026-10-24T08:00:00Z", State: "offen", CI: "grün", Merge: "konfliktfrei"},
		{Number: 447, Title: "W11", URL: "v", Head: "sprint/w11", Author: "x", State: "Entwurf", CI: "rot", Merge: "Konflikt"},
	}
	if !reflect.DeepEqual(got, want) {
		t.Fatalf("ParseOpen = %+v", got)
	}
	if _, err := ParseOpen([]byte("kaputt")); err == nil {
		t.Fatal("kaputtes JSON muss scheitern")
	}
}

// fake merkt sich die Argumente und antwortet mit out bzw. err; `gh pr view` beantwortet view (Stand des PRs).
func fake(out string, err error) (*Client, *[][]string) {
	return fakeView(out, err, `{"state":"OPEN","baseRefName":"develop","headRefOid":"abc123"}`)
}

func fakeView(out string, err error, view string) (*Client, *[][]string) {
	var calls [][]string
	c := &Client{root: ".", run: func(_ context.Context, _ string, args []string) ([]byte, error) {
		calls = append(calls, args)
		if len(args) > 2 && args[2] == "view" && err == nil {
			return []byte(view), nil
		}
		return []byte(out), err
	}}
	return c, &calls
}

func TestCreateArgumenteUndPruefung(t *testing.T) {
	c, calls := fake("Warning: x\nhttps://github.com/tenvan/k3c-family/pull/460\n", nil)
	url, err := c.Create(context.Background(), "feat/git-seite", "-rm -rf: Titel", "Text")
	if err != nil || url != "https://github.com/tenvan/k3c-family/pull/460" {
		t.Fatalf("Create = %q, %v", url, err)
	}
	want := []string{"gh", "pr", "create", "--base=develop", "--head=feat/git-seite", "--title=-rm -rf: Titel", "--body=Text"}
	if !reflect.DeepEqual((*calls)[0], want) {
		t.Fatalf("Argumente = %q", (*calls)[0])
	}
	for _, bad := range [][2]string{{"feat/x", " "}, {"develop", "T"}, {"main", "T"}, {"feat/x", strings.Repeat("x", 201)}} {
		if _, err := c.Create(context.Background(), bad[0], bad[1], ""); err == nil {
			t.Errorf("Create(%q, %q) muss scheitern", bad[0], bad[1])
		}
	}
	if len(*calls) != 1 {
		t.Fatalf("abgelehnte Aufrufe dürfen gh nicht starten: %d Aufrufe", len(*calls))
	}
}

func TestMergeUndClose(t *testing.T) {
	c, calls := fake("", nil)
	ctx := context.Background()
	if err := c.Merge(ctx, 452, true); err != nil {
		t.Fatal(err)
	}
	if err := c.Merge(ctx, 452, false); err != nil {
		t.Fatal(err)
	}
	if err := c.Close(ctx, 447); err != nil {
		t.Fatal(err)
	}
	view := func(n string) []string {
		return []string{"gh", "pr", "view", n, "--json", "state,baseRefName,headRefOid"}
	}
	want := [][]string{
		view("452"), {"gh", "pr", "merge", "452", "--squash", "--match-head-commit=abc123"},
		view("452"), {"gh", "pr", "merge", "452", "--merge", "--match-head-commit=abc123"},
		view("447"), {"gh", "pr", "close", "447"},
	}
	if !reflect.DeepEqual(*calls, want) {
		t.Fatalf("Aufrufe = %q", *calls)
	}
	for _, v := range []string{`{"state":"OPEN","baseRefName":"main","headRefOid":"x"}`, `{"state":"MERGED","baseRefName":"develop","headRefOid":"x"}`, `kaputt`} {
		c, calls := fakeView("", nil, v)
		if err := c.Merge(ctx, 9, true); err == nil {
			t.Errorf("Merge bei %s muss scheitern", v)
		}
		if err := c.Close(ctx, 9); err == nil {
			t.Errorf("Close bei %s muss scheitern", v)
		}
		if len(*calls) != 2 {
			t.Errorf("bei %s nur view, kein merge/close: %q", v, *calls)
		}
	}
	if err := c.Merge(ctx, 0, true); err == nil {
		t.Error("PR 0 muss scheitern")
	}
	bad, _ := fake("", errors.New("boom"))
	if err := bad.Close(ctx, 1); err == nil || !strings.Contains(err.Error(), "boom") {
		t.Fatalf("Fehler von gh muss durchkommen: %v", err)
	}
	if list := bad.Open(ctx); list.Error == "" || len(list.PRs) != 0 {
		t.Fatalf("Open ohne gh: %+v", list)
	}
}
