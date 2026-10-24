package main

import (
	"errors"
	"net/http"
	"sync/atomic"
	"testing"

	"k3c/tools/assetkit/ticket"
	"k3c/tools/k3c-workbench/internal/simrun"
)

// WZ28.7/AC-07 (B-595/AC-04, B-596/AC-05): Ticket-Button und Balancing laufen ohne Netz. Der Standard-Transport lehnt
// jede Verbindung ab und zählt sie; Ticket anlegen, Bericht lesen, bewerten, löschen und ein Offline-Lauf bleiben
// ohne einen einzigen Versuch.

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

	hold := make(chan struct{})
	b := balanceApp(t, hold)
	run, err := b.BalanceStart(simrun.Spec{Seeds: 1, Bot: "saver+passive", Players: 2})
	if err != nil {
		t.Fatal(err)
	}
	close(hold)
	waitRun(t, b, run.ID, func(r simrun.Info) bool { return r.State == simrun.Done })
	reports, err := b.BalanceReports()
	if err != nil || len(reports) == 0 {
		t.Fatalf("Berichte: %v %v", reports, err)
	}
	p := reports[0].Path
	if _, err := b.BalanceReport(p); err != nil {
		t.Fatal(err)
	}
	if err := b.BalanceReportRate(p, "gut"); err != nil {
		t.Fatal(err)
	}
	if err := b.BalanceReportDelete(p); err != nil {
		t.Fatal(err)
	}
	if n := denied.calls.Load(); n != 0 {
		t.Errorf("%d Netzwerkanfragen ohne Netz", n)
	}
}
