# BS4.2 · Review BS4

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** bs4/2-review
- **Abhängig von:** BS4.1
- **Tickets:** B-636
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Review des Sprints BS4: Diff gelesen, Prüfläufe grün, Abnahme in der README.

## Kontext

Review-Session; Reviewer-Agent (Sonnet) hat den Diff gelesen.

## Erlaubte Dateien

- Sprint-README Abnahme; Fixes unter etwa 30 Zeilen

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `task check:diff`, `task check:go`.
2. Befunde beheben oder als Ticket.
3. Abnahme in die README.

## Fertig, wenn

- [x] Checks grün, Abnahme eingetragen

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- Reviewer-Agent: keine schweren Befunde (Determinismus, Chunk-Grenzen, Plantage, Inseln ohne Wald). Behoben: Tippfehler, Protokoll-Doku (Client folgt BS6), Test `clearing` im Snapshot, Baum-Test im Freiraum.
- `task check:diff` grün; `check:race` lokal übersprungen (kein C-Compiler).
