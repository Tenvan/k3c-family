package main

import (
	"context"
	"encoding/json"
	"net/http"
	"strings"
	"time"
)

// api fragt GET /api/status (Bearer-Token). Das Token steht nie in einem Fehlertext.
type api struct {
	base, token string
	http        *http.Client
}

func newAPI(base, token string) *api {
	return &api{base: base, token: token, http: &http.Client{Timeout: 5 * time.Second}}
}

// statusRoom ist der Teil einer Raumzeile aus /api/status, den das Werkzeug braucht.
type statusRoom struct {
	Code    string `json:"code"`
	Name    string `json:"name"`
	Devices int    `json:"devices"`
}

func (a *api) rooms(ctx context.Context) ([]statusRoom, error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, a.base+"/api/status", nil)
	if err != nil {
		return nil, failf("Adresse ungültig: %s", a.base)
	}
	req.Header.Set("Authorization", "Bearer "+a.token)
	res, err := a.http.Do(req)
	if err != nil {
		return nil, failf("Server nicht erreichbar unter %s", a.base)
	}
	defer func() { _ = res.Body.Close() }()
	switch res.StatusCode {
	case http.StatusOK:
	case http.StatusUnauthorized:
		return nil, failf("Token falsch (401 von %s)", a.base)
	case http.StatusNotFound:
		return nil, failf("Diagnose am Server aus (%s dort nicht gesetzt)", envToken)
	default:
		return nil, failf("Server antwortet mit %d", res.StatusCode)
	}
	var body struct {
		Rooms []statusRoom `json:"rooms"`
	}
	if err := json.NewDecoder(res.Body).Decode(&body); err != nil {
		return nil, failf("Status von %s nicht lesbar", a.base)
	}
	return body.Rooms, nil
}

// waitDisconnected wartet bis zu 5 s, bis kein Raum mit prefix mehr ein verbundenes Gerät hat.
func (a *api) waitDisconnected(ctx context.Context, prefix string) error {
	var open []string
	for range 50 {
		rooms, err := a.rooms(ctx)
		if err != nil {
			return err
		}
		open = open[:0]
		for _, r := range rooms {
			if strings.HasPrefix(r.Name, prefix) && r.Devices > 0 {
				open = append(open, r.Name)
			}
		}
		if len(open) == 0 {
			return nil
		}
		time.Sleep(100 * time.Millisecond)
	}
	return failf("Test-Räume noch verbunden: %s", strings.Join(open, ", "))
}
