# TR1.4 · Review und Abnahme des Sprints TR1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** tr1/4-review
- **Abhängig von:** TR1.3
- **Tickets:** B-348
- **Kriterien:** alle

## Ziel

TR1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, B-348 archiviert, der eine PR des Sprints offen.

## Kontext

Leichtes Review, nur schwere Befunde im Diff. Besonders prüfen: Sicherheit (keine Shell, Parameter als Positivliste, Bot-Feed und Browser nur auf Loopback, Token nie in Ausgabe oder Bericht), Läufe enden sicher (`stop`, Zeitlimit, Workbench-Ende), Ausgabe von `status` ≤ 10 Zeilen, Läufe nur im Checkout der Session.

## Erlaubte Dateien

- Dateien der Sessions TR1.1 bis TR1.3 nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, neue Funktionen.

## Schritte

1. `Status: in Arbeit`. `task check`, `task check:go`, `task check:dev` grün.
2. `git diff origin/develop...origin/sprint/tr1` lesen, Befunde behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 prüfen (AC-01, AC-02, AC-06: TR1.1; AC-03: TR1.2; AC-04: TR1.3; AC-05: alle).
4. Abnahme (≤ 5 Zeilen, `Version: v… vorgeschlagen`), B-348 erledigt und archiviert, Sprint nach `erledigt/`, Fahrplan, `git merge origin/develop`, push, PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-06 haben Nachweis oder sind mit Grund und Ticket verschoben.
- [x] Schwere Befunde behoben oder als Ticket.
- [x] Checks grün, PR offen.

## Prüfen

```bash
task check
task check:go
task check:dev
```

## Ergebnis

- Checks: `task check`, `task check:go`, `task check:dev` grün (2026-10-07, im Worktree per Shell; MCP-Header kam nicht an, B-341).
- Diff `edb2a8f..origin/develop` gelesen (TR1 gemergt über #209 und #212, kein Branch `sprint/tr1` mehr): keine Shell, Parameter als Positivliste, Bot-Feed nur Loopback (Listener 127.0.0.1 plus Prüfung), Browser ohne Shell mit eigenem Profil, kein Token in Status oder Bericht; Läufe enden über `stop`, Zeitlimit (offline 60 min, online ≤ 2 h) und Workbench-Ende (`cancelAll`, Job Object); `status` ≤ 10 Zeilen; Läufe im Checkout der Session. `coder/websocket` ist dieselbe Version wie im Hauptmodul.
- Schwere Befunde: keine offen; die zwei aus TR1.3 (doppeltes `Wait`, Beobachter nicht gestoppt) wurden dort vor dem Merge behoben.
- Kriterien: AC-01, AC-02, AC-06 TR1.1; AC-03 TR1.2; AC-04 TR1.3 (Browser-Nachweis verschoben: B-349/TR2.1, B-351); AC-05 TR1.1–TR1.3.
- Hinweis: Review lief im selben Lauf wie TR1.3, auf ausdrücklichen Wunsch von 🧑; TR1.3 hatte vorher ein unabhängiges Review (code-reviewer).
