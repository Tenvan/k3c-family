# KB1.2 · Lage des Endboss-Baus im Ausgangs-Chunk entscheiden und umsetzen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** kb1/2-endboss-bau-lage
- **Abhängig von:** KB1.1
- **Tickets:** B-343
- **Kriterien:** AC-02

## Ziel

Die Lage des Endboss-Baus ist vorentschieden (innere Kante des Ausgangs-Chunks), im Generator dokumentiert und durch Tests belegt.

## Kontext

`engine/level/level.go` › `lairX`, Regeln `docs/rules/bosse.md` § 1. Tests `TestLairNurInDerKristallhoehle`, `TestCampAbstandZuLinien500Seeds`, `TestBreiteUndDichte` (engine/level). Default KB1 › Offene Fragen: keine Codeänderung, Frage an 🧑 über KR6.

## Erlaubte Dateien

- `engine/level/*_test.go`, `engine/level/level.go` (nur Kommentar)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Eigenes Chunk für den Bau, Mindestabstand zu Portalen.

## Schritte

1. Die drei Tests gezielt laufen lassen.
2. Vorentscheidung in B-343 vermerken, Frage-Ticket in KR6.

## Fertig, wenn

- [ ] AC-02 (B-343/AC-01): drei Tests grün, Vorentscheidung dokumentiert, Bestätigung durch 🧑 über KR6.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
