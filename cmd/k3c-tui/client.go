package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"time"
)

// Umgebung wie beim Server und bei k3c-dev (M6): Token K3C_STATUS_TOKEN, Adresse K3C_SERVER_URL, sonst 127.0.0.1:K3C_HTTP_PORT.
const (
	envURL   = "K3C_SERVER_URL"
	envPort  = "K3C_HTTP_PORT"
	envToken = "K3C_STATUS_TOKEN"
)

// client fragt die Diagnose-Schnittstelle des Servers (/api/status…, B-027, B-088). Das Token steht nie in einem Fehlertext.
type client struct {
	base, token string
	http        *http.Client
}

func clientFromEnv(get func(string) string) *client {
	base := get(envURL)
	if base == "" {
		port := get(envPort)
		if port == "" {
			port = "8080"
		}
		base = "http://127.0.0.1:" + port
	}
	return &client{base: base, token: get(envToken), http: &http.Client{Timeout: 5 * time.Second}}
}

// JSON-Formen von engine/room (Status, Failure) und engine/net/status.go; die TUI importiert nichts aus engine/.
type (
	tickMs struct {
		Last float64 `json:"last"`
		P99  float64 `json:"p99"`
	}
	room struct {
		Code     string   `json:"code"`
		Name     string   `json:"name"`
		Depth    int      `json:"depth"`
		Devices  int      `json:"devices"`
		Monarchs []string `json:"monarchs"`
		Tick     int      `json:"tick"`
		TickMs   tickMs   `json:"tickMs"`
	}
	failure struct {
		Code  string `json:"code"`
		Name  string `json:"name"`
		Error string `json:"error"`
		At    string `json:"at"`
	}
	memory struct {
		HeapMB float64 `json:"heapMB"`
		SysMB  float64 `json:"sysMB"`
	}
	status struct {
		Version  string    `json:"version"`
		UptimeS  int64     `json:"uptimeS"`
		Saves    int       `json:"saves"`
		Reports  int       `json:"reports"`
		Memory   memory    `json:"memory"`
		Rooms    []room    `json:"rooms"`
		Failures []failure `json:"failures"`
	}
)

// meldung ist ein Fehlertext für Menschen (großgeschrieben, deutsch).
type meldung string

func (m meldung) Error() string { return string(m) }

func failf(format string, args ...any) error { return meldung(fmt.Sprintf(format, args...)) }

var errNotFound = errors.New("nicht gefunden")

// status liefert den Zustand des Servers.
func (c *client) status(ctx context.Context) (status, error) {
	var st status
	return st, c.do(ctx, http.MethodGet, "/api/status", &st)
}

// do ruft einen Diagnose-Weg und übersetzt die Statuscodes in Meldungen.
func (c *client) do(ctx context.Context, method, path string, out any) error {
	req, err := http.NewRequestWithContext(ctx, method, c.base+path, nil)
	if err != nil {
		return failf("Adresse ungültig: %s", c.base)
	}
	req.Header.Set("Authorization", "Bearer "+c.token)
	res, err := c.http.Do(req)
	if err != nil {
		return failf("Server nicht erreichbar unter %s", c.base)
	}
	defer func() { _ = res.Body.Close() }()
	switch res.StatusCode {
	case http.StatusOK:
		if err := json.NewDecoder(res.Body).Decode(out); err != nil {
			return failf("Antwort von %s nicht lesbar", c.base)
		}
		return nil
	case http.StatusUnauthorized:
		return failf("401, Token prüfen (%s)", envToken)
	case http.StatusNotFound:
		var body struct{ Error string }
		if json.NewDecoder(res.Body).Decode(&body) == nil && body.Error != "" {
			return fmt.Errorf("%w: %s", errNotFound, body.Error)
		}
		return failf("Diagnose am Server aus (%s dort nicht gesetzt)", envToken)
	}
	return failf("Server antwortet mit %d", res.StatusCode)
}
