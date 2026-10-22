# RM1.3 · Review RM1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** rm1/3-review
- **Abhängig von:** RM1.2, RM1.4
- **Tickets:** B-285, B-495
- **Kriterien:** AC-02, AC-03

## Ziel

Alle Kriterien von RM1 sind geprüft, die Abnahme steht in der Sprint-README.

## Kontext

Diff der Sessions RM1.2 und RM1.4 (`engine/net/actions.go`, `engine/sim/monarch.go`, `docs/protocol.md`, `src/scenes/hud/skillMenuLogic.ts`). Review-Session: `task check`, `task check:go`, `task check:race` nacheinander; Reviewer-Agent getrennt (SRV, Protokoll).

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`
- Fixes unter 30 Zeilen in den Dateien von RM1.2 und RM1.4

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Prüfläufe nacheinander.
2. Reviewer-Agent (schwere Befunde).
3. Abnahme in der Sprint-README (höchstens fünf Zeilen).

## Fertig, wenn

- [ ] AC-02, AC-03 mit Nachweis.
- [ ] Prüfläufe grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

2026-10-23, autonom (Claude).

- `task check` grün (44 s), `task check:go` grün (3:46), `task check:race` grün (264 ms, vermutlich aus dem Cache; ungeprüft, ob neu gelaufen).
- AC-02 geprüft: RM1.2 (Go- und Vitest-Nachweise). AC-03 geprüft: RM1.4 (`TestAktionslisteRespec`).
- Reviewer (Sonnet, getrennt): keine schweren Befunde. Hinweis ohne Fix: `respec` fehlt auch bei gefallenem Monarchen (wie alle Aktionen am Ort, dokumentiert); die Nachricht `respec` prüft "lebt" nicht (unverändert, B-495 Nicht-Ziel).
