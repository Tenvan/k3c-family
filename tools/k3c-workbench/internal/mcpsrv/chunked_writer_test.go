package mcpsrv

import (
	"bufio"
	"fmt"
	"io"
	"net"
	"net/http"
	"strings"
	"testing"
	"time"
)

// Regression: Das 202 auf notifications/initialized muss von Anfang an chunked und abgeschlossen sein, sonst hängt
// Claude Code (siehe forceChunked).
func TestBenachrichtigungAntwortIstChunkedAbgeschlossen(t *testing.T) {
	s := New(Config{Version: "test"})
	if err := s.Start(); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = s.Stop() })
	c, err := net.Dial("tcp", strings.TrimSuffix(strings.TrimPrefix(s.URL(), "http://"), "/mcp"))
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = c.Close() }()
	_ = c.SetDeadline(time.Now().Add(5 * time.Second))
	rd := bufio.NewReader(c)
	post := func(sid, body string) *http.Response {
		h := "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\n"
		if sid != "" {
			h += "Mcp-Session-Id: " + sid + "\r\nMcp-Protocol-Version: 2025-06-18\r\n"
		}
		_, _ = fmt.Fprintf(c, "%sContent-Length: %d\r\n\r\n%s", h, len(body), body)
		r, err := http.ReadResponse(rd, nil)
		if err != nil {
			t.Fatal(err)
		}
		if _, err := io.ReadAll(r.Body); err != nil {
			t.Fatalf("Body endet nicht sauber: %v", err)
		}
		return r
	}
	r := post("", `{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}`)
	r = post(r.Header.Get("Mcp-Session-Id"), `{"jsonrpc":"2.0","method":"notifications/initialized"}`)
	if r.StatusCode != http.StatusAccepted || len(r.TransferEncoding) != 1 || r.TransferEncoding[0] != "chunked" {
		t.Errorf("Status %d, Transfer-Encoding %v; erwartet 202 chunked", r.StatusCode, r.TransferEncoding)
	}
}
