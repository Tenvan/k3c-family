package planning

import (
	"fmt"
	"os"
	"slices"
	"sort"
	"strings"

	ticketkit "k3c/tools/assetkit/ticket"
)

// Index-Verdichtung (B-696 Teil B): docs/backlog/README.md führt nur offene und eingeplante Tickets, die Zeilen der
// erledigten und verworfenen stehen in docs/backlog/archiv/README.md. RebuildIndex erzeugt beide aus den Ticket-Dateien
// neu; Ticket-Dateien ändert es nie.

const (
	mainIndex = "backlog/README.md"
	tableHead = "| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |\n|---|---|---|---|---|---|---|\n"
	mainHead  = "# Backlog\n\n" +
		"Offene und eingeplante Tickets, eine Zeile pro Ticket; erledigte und verworfene stehen in [`archiv/README.md`](archiv/README.md).\n" +
		"Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md) und eine Spec (SDD, siehe\n" +
		"[`../arbeitsweise.md`](../arbeitsweise.md)). Anlegen und ändern nur über k3c-workbench (`plan_create`, `plan_set`);\n" +
		"`task test` prüft Dateien und beide Index-Dateien.\n"
	mainArchive = "## Archiv\n\nErledigte und verworfene Tickets liegen in [`archiv/`](archiv/), ihre Zeilen in\n" +
		"[`archiv/README.md`](archiv/README.md) (nur auf Nachfrage lesen). Ändert ein Ticket seinen Status auf `erledigt` oder\n" +
		"`verworfen`, verschiebt k3c-workbench Datei und Zeile dorthin, beim Zurücksetzen zurück.\n"
	archiveHead = "# Backlog · Archiv\n\n" +
		"Erledigte und verworfene Tickets, eine Zeile pro Ticket (nur auf Nachfrage lesen). Die Dateien liegen in diesem Ordner\n" +
		"und bleiben beim Verschieben unverändert; offene und eingeplante Tickets stehen in [`../README.md`](../README.md).\n"
)

// indexTicket ist ein Ticket mit seinem Dateinamen und Ordner für den Neuaufbau.
type indexTicket struct {
	t        Ticket
	file     string
	archived bool
}

// RebuildIndex schreibt beide Index-Dateien aus den Ticket-Dateien neu. Die Reihenfolge bisheriger Zeilen bleibt,
// neue kommen nach Nummer ans Ende. dry zeigt nur die Zahlen.
func RebuildIndex(root string, dry bool) (string, error) {
	writeMu.Lock()
	defer writeMu.Unlock()
	c := newChangeSet(root)
	old, err := c.read(mainIndex)
	if err != nil {
		return "", err
	}
	if arch, err := c.read(archiveIndex); err == nil {
		old += "\n" + arch
	} else if exists(root, archiveIndex) {
		return "", fmt.Errorf("%s: %w", archiveIndex, err)
	}
	tickets, err := indexTickets(root)
	if err != nil {
		return "", err
	}
	order := rowOrder(old)
	sort.SliceStable(tickets, func(i, j int) bool { return less(order, tickets[i].t.Nr, tickets[j].t.Nr) })
	var open, done []string
	for _, x := range tickets {
		row := ticketkit.IndexRow(ticketkit.Row{Nr: x.t.Nr, Link: x.file, Domain: x.t.Domain, Typ: x.t.Typ, Prio: x.t.Prio,
			Status: x.t.Status, Sprint: x.t.Sprint, Title: x.t.Title})
		if x.archived {
			done = append(done, row)
		} else {
			open = append(open, row)
		}
	}
	c.write(mainIndex, mainHead+"\n## Offen\n\n"+tableHead+joinRows(open)+"\n"+mainArchive)
	c.write(archiveIndex, archiveHead+"\n## Archiv\n\n"+tableHead+joinRows(done))
	c.crlf[archiveIndex] = c.crlf[mainIndex]
	note := fmt.Sprintf("Index neu: %d offen/eingeplant in docs/%s, %d erledigt/verworfen in docs/%s",
		len(open), mainIndex, len(done), archiveIndex)
	if dry {
		return note + " (dry, nichts geschrieben)", nil
	}
	return note, c.apply()
}

func joinRows(rows []string) string {
	if len(rows) == 0 {
		return ""
	}
	return strings.Join(rows, "\n") + "\n"
}

// indexTickets liest die Köpfe aller Tickets; der Ordner muss zum Status passen, sonst bricht der Neuaufbau ab.
func indexTickets(root string) ([]indexTicket, error) {
	var out []indexTicket
	for _, folder := range []string{"backlog", "backlog/archiv"} {
		entries, err := os.ReadDir(docsPath(root, folder))
		if err != nil && folder == "backlog" {
			return nil, err
		}
		for _, e := range entries {
			if e.IsDir() || !ticket.MatchString(e.Name()) {
				continue
			}
			text, _, err := readText(root, folder+"/"+e.Name())
			if err != nil {
				return nil, err
			}
			t := ParseTicket(text)
			archived := folder != "backlog"
			if archived != (t.Status == "erledigt" || t.Status == "verworfen") {
				return nil, fmt.Errorf("%s: Status %s passt nicht zum Ordner docs/%s, nichts geschrieben", e.Name(), t.Status, folder)
			}
			out = append(out, indexTicket{t: t, file: e.Name(), archived: archived})
		}
	}
	return out, nil
}

// rowOrder ist die Position jeder Ticket-Nummer in den bisherigen Index-Zeilen.
func rowOrder(text string) map[string]int {
	order := map[string]int{}
	for _, l := range splitLines(text) {
		if m := reIndexRow.FindStringSubmatch(l); m != nil {
			if _, ok := order[m[1]]; !ok {
				order[m[1]] = len(order)
			}
		}
	}
	return order
}

// less: bekannte Zeilen in alter Reihenfolge, unbekannte danach nach Nummer.
func less(order map[string]int, a, b string) bool {
	ia, oka := order[a]
	ib, okb := order[b]
	switch {
	case oka && okb:
		return ia < ib
	case oka != okb:
		return oka
	}
	return slices.Compare([]byte(a), []byte(b)) < 0
}
