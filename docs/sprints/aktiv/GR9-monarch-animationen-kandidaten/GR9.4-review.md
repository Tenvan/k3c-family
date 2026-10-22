# GR9.4 · Review Sprint GR9

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** gr9/4-review
- **Abhängig von:** GR9.3
- **Tickets:** B-420, B-505
- **Kriterien:** alle

## Ziel

Review des Sprints GR9, PR geöffnet.

## Kontext

Review nach `docs/arbeitsweise.md`; Diff `origin/develop...sprint/gr9`. GR9.5 (TV) ist keine Abhängigkeit (Hardware entkoppelt).

## Erlaubte Dateien

- Befundfixes in den Sprint-Domänen, `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `task check`, `task check:go`.
2. Diff lesen, nur schwere Befunde.
3. Abnahme (≤ 5 Zeilen), PR öffnen; Sprint bleibt aktiv bis GR9.5.

## Fertig, wenn

- [ ] AC-01, AC-02: Nachweis in den Session-Ergebnissen; Befunde behoben oder als Ticket.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01, AC-02 (ohne TV) geprüft durch Reviewer-Agent, keine schweren Befunde; `check` und `check:go` grün, `check:race` offen (CI).
