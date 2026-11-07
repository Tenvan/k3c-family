# PS1.5 · Befunde als Tickets, Sprint schließen

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ps1
- **Abhängig von:** PS1.4
- **Tickets:** B-699, B-701
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

Alle Befunde aus PS1.1–PS1.4 sind Tickets im Projekt PAP, B-701 hat keine offene Zeile, der Sprint ist abgeschlossen.

## Kontext

Liest die Ergebnisse von PS1.1–PS1.4.

## Erlaubte Dateien

`docs/backlog/` (nur über `plan_*`), `docs/sprints/…/PS1-*/`.

## Nicht-Ziele

Befunde beheben.

## Schritte

1. Je ✏️ aus PS1.1–PS1.4 ein Ticket per `plan_create` (Projekt PAP, Lane im Text, Verweis auf D-Zeile).
2. B-701: Status jeder Zeile auf `bestätigt` oder `geändert (B-…)`.
3. Abnahme der Sprint-README füllen (höchstens fünf Zeilen).
4. B-701 und B-699 schließen, wenn AC erfüllt.

## Fertig, wenn

B-701 ohne `offen`, jeder Befund hat ein Ticket.

## Prüfen

`task test -- planning` grün; `grep -c '| offen' B-701` = 0.

## Ergebnis

–
