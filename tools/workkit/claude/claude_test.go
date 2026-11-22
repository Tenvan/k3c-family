package claude

import (
	"net/url"
	"strings"
	"testing"
)

func TestClaudeLink(t *testing.T) {
	prompt := "Fix #12 & prüfe a+b=c\nzweite Zeile"
	link, err := Link(prompt, `C:\WORKSPACE\k3c`)
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
	if _, err := Link("  ", ""); err == nil {
		t.Fatal("leerer Prompt angenommen")
	}
	if _, err := Link(strings.Repeat("x", MaxPrompt+1), ""); err == nil {
		t.Fatal("überlanger Prompt angenommen")
	}
	if _, err := Link(strings.Repeat("ü", 6000), ""); err == nil {
		t.Fatal("kodiert überlanger Link angenommen")
	}
}
