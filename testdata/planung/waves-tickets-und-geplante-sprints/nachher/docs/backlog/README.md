# Backlog

Offene und eingeplante Tickets, eine Zeile pro Ticket; erledigte und verworfene stehen in [`archiv/README.md`](archiv/README.md).
Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md) und eine Spec (SDD, siehe
[`../arbeitsweise.md`](../arbeitsweise.md)). Anlegen und ändern nur über k3c-workbench (`plan_create`, `plan_set`);
`task test` prüft Dateien und beide Index-Dateien.

## Offen

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-001](B-001-frei.md) | SRV | Idee | hoch | offen | – | Freigegeben |
| [B-002](B-002-entwurf.md) | SRV | Idee | mittel | offen | – | Entwurf |
| [B-003](B-003-frei-b.md) | SRV | Idee | mittel | offen | – | Freigegeben ohne Bereich |
| [B-004](B-004-mensch.md) | SRV | Idee | mittel | offen | – | Braucht Ralf |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/), ihre Zeilen in
[`archiv/README.md`](archiv/README.md) (nur auf Nachfrage lesen). Ändert ein Ticket seinen Status auf `erledigt` oder
`verworfen`, verschiebt k3c-workbench Datei und Zeile dorthin, beim Zurücksetzen zurück.
