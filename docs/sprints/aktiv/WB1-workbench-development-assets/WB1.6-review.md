# WB1.6 · Review WB1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** ui
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wb1/6-review
- **Abhängig von:** WB1.3, WB1.4
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Sprint WB1 ist geprüft und abgenommen: Die Workbench hat die Bereiche Development und Assets ohne schreibende Controls, Prompt-Buttons funktionieren, das Studio-Programm ist vollständig entfernt.

## Kontext

Review nach `docs/arbeitsweise.md` › Review, gegen Entscheidung 005 › 5 und 7, den Vertrag `docs/standards/paper-mcp.md` › 3–4 und die Sprint-README WB1, nur den Diff des Sprints (`git diff <Start-Commit>..HEAD`). Parallel liefen RD1, MC1 und RV1; deren Ordner (`tools/paper/`, `internal/mcpsrv/tools_assets*.go`, `internal/paperassets/`, `internal/review/`, `internal/assetsui/`, `frontend/src/assets/review/`) darf der Diff von WB1 nicht berühren.

## Erlaubte Dateien

- Dateien der Sessions WB1.1–WB1.5 (nur Befund-Fixes)
- `docs/sprints/` (Abnahme, Status), `docs/backlog/` (neue Tickets)

## Nicht-Ziele

Keine neuen Funktionen; keine Vertragsänderung (nur Ticket an PAP); keine manuelle Sichtprüfung (PS1, B-701).

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander; dazu `task check:dev` und `task buildall` (AC-04).
2. Lane-Grenzen: `git diff --name-only <Start-Commit>..HEAD` enthält nur Pfade aus den erlaubten Dateien der Sessions WB1.1–WB1.5, nichts aus RV1/MC1/RD1.
3. AC-02 kritisch lesen: Jede Seite des Bereichs Assets steht in `assets/readonly.test.tsx`; der Test würde bei einem neuen schreibenden Aufruf wirklich rot (einmal lokal mit einem eingeschmuggelten Aufruf gegenprüfen, nicht committen).
4. AC-03: `fill.ts` ersetzt alle Platzhalter aus `paper-mcp.md` › 3, Pflichtwerte geprüft; RV1 kann `fill.ts` importieren (D-02).
5. AC-04: `rg -n -i "k3c-studio|check:studio|8091|8182" --glob '!docs/sprints/**' --glob '!docs/backlog/**' --glob '!docs/decisions/**'` nur noch gewollte Verweise (D-07: Kommentare für CU1 als Ticket festhalten, falls nicht schon in B-700); Sicherheitsblick auf `internal/studioview` (Loopback, `Host`-Prüfung, `os.Root` für Sound-Dateien, Spielstände außerhalb des Repos).
6. Abnahme in der Sprint-README (höchstens fünf Zeilen), Sprint auf erledigt.

## Fertig, wenn

- [ ] AC-01 bis AC-04 mit Nachweis aus den Session-Ergebnissen belegt.
- [ ] `task check`, `task check:go`, `task check:race`, `task check:dev`, `task buildall` grün (`check:race` ohne gcc: als übersprungen nennen).
- [ ] Lane-Grenzen eingehalten (Schritt 2).

## Prüfen

```bash
task check
task check:go
task check:race
task check:dev
task buildall
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
