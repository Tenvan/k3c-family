package main

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"strconv"
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

// status liefert den Zustand des Servers.
func (c *client) status(ctx context.Context) (status, error) {
	var st status
	return st, c.do(ctx, http.MethodGet, "/api/status", &st)
}

// do ruft einen Diagnose-Weg und übersetzt die Statuscodes in Meldungen. Antwortet der Server mit {"error":"…"}, ist das die Meldung
// (z. B. „Raum nicht gefunden“, „Log aus“, „Kennung nicht eindeutig“); ein leeres 404 heißt: Diagnose am Server aus.
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
	if res.StatusCode == http.StatusOK {
		if err := json.NewDecoder(res.Body).Decode(out); err != nil {
			return failf("Antwort von %s nicht lesbar", c.base)
		}
		return nil
	}
	var body struct{ Error string }
	_ = json.NewDecoder(res.Body).Decode(&body)
	switch {
	case res.StatusCode == http.StatusUnauthorized:
		return failf("401, Token prüfen (%s)", envToken)
	case body.Error != "":
		return failf("%s", body.Error)
	case res.StatusCode == http.StatusNotFound:
		return failf("Diagnose am Server aus (%s dort nicht gesetzt)", envToken)
	}
	return failf("Server antwortet mit %d", res.StatusCode)
}

// Raum, Geräte und Log (Sprint D1: GET /api/status?room=, GET /api/status/log, POST /api/status/disconnect und /save).
type (
	device struct {
		ID        string `json:"id"`
		Connected bool   `json:"connected"`
		Slots     []int  `json:"slots"`
	}
	roomDetail struct {
		Code    string         `json:"code"`
		Depth   int            `json:"depth"`
		Tick    int            `json:"tick"`
		Phase   string         `json:"phase"`
		Day     int            `json:"day"`
		Gold    []int          `json:"gold"`
		Troops  map[string]int `json:"troops"`
		Enemies int            `json:"enemies"`
		Castle  float64        `json:"castleHp"`
		Wave    int            `json:"wave"`
		Devices []device       `json:"devices"`
	}
	logPage struct {
		Cursor    int64    `json:"cursor"`
		Truncated bool     `json:"truncated"`
		Lines     []string `json:"lines"`
	}
)

func (c *client) room(ctx context.Context, code string) (roomDetail, error) {
	var d roomDetail
	return d, c.do(ctx, http.MethodGet, "/api/status?room="+url.QueryEscape(code), &d)
}

func (c *client) disconnect(ctx context.Context, room, device string) error {
	path := "/api/status/disconnect?room=" + url.QueryEscape(room) + "&device=" + url.QueryEscape(device)
	return c.do(ctx, http.MethodPost, path, &struct{}{})
}

func (c *client) save(ctx context.Context, room string) error {
	return c.do(ctx, http.MethodPost, "/api/status/save?room="+url.QueryEscape(room), &struct{}{})
}

func (c *client) log(ctx context.Context, since int64, limit int) (logPage, error) {
	var p logPage
	return p, c.do(ctx, http.MethodGet, "/api/status/log?since="+strconv.FormatInt(since, 10)+"&limit="+strconv.Itoa(limit), &p)
}
