package main

import (
	"errors"
	"net/http"
	"sync/atomic"
	"testing"

	"k3c/tools/assetkit/ticket"
)

// WZ28.7/AC-07 (B-595/AC-04): Der Ticket-Button läuft ohne Netz. Der Standard-Transport lehnt jede Verbindung ab und
// zählt sie; Ticket anlegen und lesen bleiben ohne einen einzigen Versuch. Den Balancing-Teil (B-596/AC-05) prüft seit
// WZ63.5 offline_test.go von k3c-studio.

type deniedTransport struct{ calls atomic.Int32 }

func (d *deniedTransport) RoundTrip(r *http.Request) (*http.Response, error) {
	d.calls.Add(1)
	return nil, errors.New("offline: " + r.URL.String())
}

func TestOfflineOhneNetz(t *testing.T) {
	denied := &deniedTransport{}
	old := http.DefaultTransport
	http.DefaultTransport = denied
	t.Cleanup(func() { http.DefaultTransport = old })

	a := ticketRepo(t)
	if _, err := a.TicketCreate(ticket.Input{Titel: "Offline-Probe", Beschreibung: "ohne Netz", Typ: "Idee", Prio: "niedrig", Domaene: "DEV"}); err != nil {
		t.Fatal(err)
	}
	if _, err := a.TicketRecent(0); err != nil {
		t.Fatal(err)
	}

	if n := denied.calls.Load(); n != 0 {
		t.Errorf("%d Netzwerkanfragen ohne Netz", n)
	}
}
