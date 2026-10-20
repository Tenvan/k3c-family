# TS1.6 · Review: Trennung Server und Studio

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** ts1/6-review
- **Abhängig von:** TS1.5
- **Tickets:** B-507
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Ein getrennter Reviewer prüft den ganzen Sprint gegen die Kriterien AC-01 bis AC-06, Befunde sind behoben oder als Ticket angelegt, die Abnahme in der Sprint-README ist ausgefüllt, der Sprint-PR ist offen.

## Kontext

Ablauf laut `docs/arbeitsweise.md › Review-Session (Sprint-Abnahme)`. Maßstab sind `B-507 › Akzeptanzkriterien` und die Sprint-README TS1. Schwerpunkte: Import-Richtung (nie `engine/` → `tools/`), keine Reste der Studio-Seite im Server, Watch (Entprellen, ungültige Datei, Neustart-Hinweis, Server schreibt nie in beobachtete Ordner), Pages ohne Build, Chat-Sicherheit (Loopback, keine Werkzeuge) nach dem Umzug unverändert.

## Erlaubte Dateien

- Planungsdateien von TS1 und B-507, neue Tickets für Befunde, `docs/sprints/README.md`, `docs/projekte/WZG-werkzeuge-testlaeufe.md`
- Kleine Korrekturen an Code nur für Befunde, die die Review-Session laut Arbeitsweise selbst beheben darf

## Nicht-Ziele

Neue Funktionen; Umsetzung im selben Lauf wie die Sessions TS1.1 bis TS1.5; Abnahme am PC (TS1.7).

## Schritte

1. Alle Session-Ergebnisse und den Diff gegen den Start-Commit lesen; `task check:all` ausführen (über `check_run`).
2. Unabhängiger Code-Review der Änderungen; schwere Befunde beheben, übrige als Ticket.
3. Abnahme-Abschnitt der Sprint-README ausfüllen, Version vorschlagen (Arbeitsweise), Sprint-PR öffnen und Auto-Merge laut Arbeitsweise.

## Fertig, wenn

- [ ] AC-01 bis AC-06 mit Nachweis im Ergebnis; `task check:all` grün.
- [ ] Befunde behoben oder als Ticket; Abnahme in der Sprint-README ausgefüllt.

## Prüfen

```bash
task check:all
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
