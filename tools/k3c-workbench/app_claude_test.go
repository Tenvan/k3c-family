package main

import (
	"net/url"
	"strings"
	"testing"
)

func TestClaudeLink(t *testing.T) {
	prompt := "Fix #12 & prüfe a+b=c\nzweite Zeile"
	link, err := claudeLink(prompt, `C:\WORKSPACE\k3c`)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.HasPrefix(link, "claude://code/new?") || strings.Contains(link, "+") || strings.Contains(link, " ") {
		t.Fatalf("Link falsch kodiert: %s", link)
	}
	// Rundreise: Claude Desktop dekodiert genau das zurück.
	q, err := url.ParseQuery(strings.TrimPrefix(link, "claude://code/new?"))
	if err != nil || q.Get("q") != prompt || q.Get("folder") != `C:\WORKSPACE\k3c` {
		t.Fatalf("Rundreise: %v %+v", err, q)
	}
	if _, err := claudeLink("  ", ""); err == nil {
		t.Fatal("leerer Prompt angenommen")
	}
	if _, err := claudeLink(strings.Repeat("x", claudeMaxPrompt+1), ""); err == nil {
		t.Fatal("überlanger Prompt angenommen")
	}
	if _, err := claudeLink(strings.Repeat("ü", 6000), ""); err == nil {
		t.Fatal("kodiert überlanger Link angenommen")
	}
}
