package net

import (
	"context"
	"testing"
	"time"
)

// K4.2, B-154/AC-04: Ein Client der vorigen Protokollversion erhält `version` und wird getrennt (Zustand mit Kampffeldern
// und voller Zustand nach dem Inselwechsel gibt es erst ab dieser Version).
func TestKampfVersionAlterClientAbgelehnt(t *testing.T) {
	srv, _ := wsServer(t)
	c := dial(t, srv)
	c.send(map[string]any{"t": "hello", "v": ProtocolVersion - 1, "device": "alt"})
	if m := c.next(); m["code"] != "version" {
		t.Fatalf("v%d: %v", ProtocolVersion-1, m)
	}
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	if _, _, err := c.ws.Read(ctx); err == nil {
		t.Fatal("Verbindung nicht geschlossen")
	}
}
