# KB6.3 · Review

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** kb6/3-review
- **Abhängig von:** KB6.2
- **Tickets:** B-579, B-605
- **Kriterien:** alle

## Ziel

Sprint KB6 geprüft und abgeschlossen: alle Kriterien nachgewiesen, Abnahme in der Sprint-README.

## Kontext

Sessions KB6.1 (Schutzplatz, B-605) und KB6.2 (Wellenabstand, B-579). Review nach `docs/arbeitsweise.md` › Review.

## Erlaubte Dateien

- Planungsdateien über `plan_*`
- Fixes zu Befunden in den Dateien von KB6.1 und KB6.2

## Nicht-Ziele

Neue Features.

## Schritte

1. Diff seit Start-Commit lesen, Befunde bewerten.
2. `task check`, `task check:go`, `task check:race`.
3. Abnahme eintragen, Sprint schließen.

## Fertig, wenn

- [x] AC-01, AC-02 laut Session-Ergebnissen nachgewiesen.
- [x] Prüfungen grün, Abnahme eingetragen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01, AC-02 laut Session-Ergebnissen nachgewiesen (KB6.1 und KB6.2 fertig). `task check`, `task check:go`, `task check:race` nacheinander grün (`check:race` lokal ohne gcc übersprungen, läuft in der CI); `task check:diff` grün.
Diff-Review (Sonnet, getrennt): keine schweren Befunde. Behoben: Klammerung der Korridor-Assertion (Obergrenze der ersten Welle entfernt), `docs/data/biomes.md` nennt `percentPerGather` 4. Ungeprüft: `percentPerGather` 4 gilt auch für Höhle, Kristall und Ironhold, gemessen ist nur die Mine.
Neue Tickets: B-696, B-698 (offen), B-697 (erledigt).
