# BS1.4 · Review BS1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bs1/4-review
- **Abhängig von:** BS1.2, BS1.3
- **Tickets:** B-634, B-639
- **Kriterien:** alle

## Ziel

BS1 ist abgenommen, der PR für `sprint/bs1` ist offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session. Besonders: Determinismus des Seed-Modus, `sim_test`-Parameter offline, Instructions vollständig, kurze Prompts, Neustart-Hinweise erfolgt.

## Erlaubte Dateien

- Dateien aus BS1.1–BS1.3 (Befunde beheben)
- `docs/sprints/**/BS1-*/**`, `docs/backlog/B-634-*.md`, `B-639-*.md`

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Kriterien AC-01 bis AC-08 gegen die Session-Ergebnisse prüfen.
2. `check_run task:check`, `task:check:go`, `task:check:race` nacheinander.
3. Abnahme ausfüllen, Sprint abschließen, PR öffnen.
4. Befunde mit Workbench-Code → **⚠️ Workbench-Neustart nötig** melden.

## Fertig, wenn

- [ ] Alle Kriterien geprüft und in der Abnahme verwiesen.
- [ ] Checks grün, PR offen.

## Prüfen

```bash
task check
task check:go
task check:race
```

## Ergebnis

- AC-01 bis AC-08 gegen BS1.1–BS1.3 geprüft, alle erfüllt; AC-05 bis auf `world` (verschoben nach BS11, B-642).
- `task check` grün (erster Lauf: 2 Timeouts unter Last in `offlineTools`/`projectRules`, einzeln grün, Wiederholung grün), `task check:go` grün, `task check:race` **übersprungen** (lokal kein C-Compiler); die Nebenläufigkeit aus BS1.2 belegt nur `TestParallelWieNacheinander`, Race-Detector folgt in der CI.
- Diff `3e95ba48..HEAD` gelesen (Eingabeprüfung `sim_test`/`simrun`, Seed-Modus, Parallel-Läufe): keine neuen Befunde.
- Offen (niedrig, aus dem Review von BS1.1): Folgebefehle der Auswertung nennen `--seeds N` statt der Seed-Liste; Baseline-Vergleich bei anderem Seed-Modus nur als Hinweis (BS7).
