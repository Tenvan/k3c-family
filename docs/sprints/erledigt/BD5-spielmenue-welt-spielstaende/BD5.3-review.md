# BD5.3 · Review

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sprint/bd5
- **Abhängig von:** BD5.2
- **Tickets:** B-506
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Alle Kriterien von BD5 geprüft, Diff gegen `origin/develop` gelesen, PR offen.

## Kontext

Sprint-README BD5, Sessions BD5.1 und BD5.2. Der Branch enthält bis zum Merge von #432 (BD4) und #436 (BD3) deren Commits.

## Erlaubte Dateien

- `docs/sprints/aktiv/BD5-spielmenue-welt-spielstaende/**`
- Fixes an Dateien aus BD5.1/BD5.2 (unter etwa 30 Zeilen)

## Nicht-Ziele

Keine neuen Funktionen; BD5.4 nicht abhaken.

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander.
2. Getrennter Reviewer liest den Diff.
3. Abnahme in der Sprint-README, Sprint bleibt aktiv (BD5.4 offen).

## Fertig, wenn

- [ ] Prüfungen grün oder ehrlich als übersprungen benannt; AC-01, AC-02 mit Nachweis; AC-03 offen bei BD5.4.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- `task check` grün (2398 Vitest), `task check:go` grün; `task check:race` lief durch, der Race-Detector wurde aber übersprungen (kein C-Compiler auf dem Rechner, die CI prüft `-race`).
- AC-01 geprüft: Go-Tests aus BD5.1; AC-02 geprüft: Vitest aus BD5.2 und Browser-Pane; AC-03 offen bei BD5.4 (Ralf).
- Getrennter Reviewer (Sonnet, Diff `857adbc3..HEAD`): keine schweren Befunde; Hinweis zur Rotation (höchstens 5 Sicherungen) in `docs/protocol.md` nachgetragen.
