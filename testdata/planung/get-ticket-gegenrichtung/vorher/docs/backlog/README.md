# Backlog

Offene und eingeplante Tickets, eine Zeile pro Ticket; erledigte und verworfene stehen in [`archiv/README.md`](archiv/README.md).
Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md) und eine Spec (SDD, siehe
[`../arbeitsweise.md`](../arbeitsweise.md)). Anlegen und ändern nur über k3c-workbench (`plan_create`, `plan_set`);
`task test` prüft Dateien und beide Index-Dateien.

## Offen

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-001](B-001-altes-ticket.md) | SRV | Idee | hoch | eingeplant | X1 | Altes Ticket |
| [B-002](B-002-zweites-ticket.md) | DEV | Problem | niedrig | offen | – | Zweites Ticket |
| [B-003](B-003-drittes-ticket.md) | SIM | Schuld | mittel | eingeplant | Y1 | Drittes Ticket |
| [B-004](B-004-viertes-ticket.md) | PAP | Frage | hoch | offen | – | Viertes Ticket |
| [B-007](B-007-ersatz-ticket.md) | SRV | Idee | mittel | offen | – | Ersatz-Ticket |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/), ihre Zeilen in
[`archiv/README.md`](archiv/README.md) (nur auf Nachfrage lesen). Ändert ein Ticket seinen Status auf `erledigt` oder
`verworfen`, verschiebt k3c-workbench Datei und Zeile dorthin, beim Zurücksetzen zurück.
