# TR3.2 · Review und Abnahme des Sprints TR3

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** tr3/2-review
- **Abhängig von:** TR3.1
- **Tickets:** B-353
- **Kriterien:** alle

## Ziel

TR3 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, AC-03 per `sim_test` nachgewiesen, B-353 archiviert, der PR des Sprints offen.

## Kontext

Leichtes Review: ohne `?botfeed` und `?players` unverändertes Spiel, Bot-Slots fest gebunden, `GameScene.ts` nicht über 400 Zeilen gewachsen, B-Taste und Home-Kombi unberührt.

## Erlaubte Dateien

- Dateien von TR3.1 nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, neue Funktionen.

## Schritte

1. `Status: in Arbeit`, `task check` grün.
2. Diff `origin/develop...origin/sprint/tr3` lesen, Befunde behandeln.
3. AC-03 (aus TR3.1 verschoben): `sim_test` mit `mode: online`, `clients: 1`, `players: 2` aus einem Checkout, dessen Vite den Stand von `sprint/tr3` ausliefert (`workbench_status` › `Checkout:` prüfen); Feed-Zähler und Positionen der Client-Monarchen im Bericht nachsehen. Geht das nicht, AC-03 `blockiert` mit Grund.
4. Abnahme, B-353 archivieren, Sprint nach `erledigt/`, Fahrplan, merge, push, PR.

## Fertig, wenn

- [ ] AC-01 bis AC-03 mit Nachweis; PR offen.

## Prüfen

```bash
task check
```

## Ergebnis

- Code von TR3.1 liegt schon auf `develop` (PR #217, Squash `6837fbfb`); geprüft wurde dessen Diff unter `src/scenes/` (`GameScene.ts` 399 Zeilen, ohne `?botfeed`/`?players` unverändert, B-Taste und Home-Kombi unberührt, kein `Math.random()`).
- **AC-01 geprüft:** `localSlots.test.ts` (feste Eingaben binden Slot i ohne `confirm`), `check_run task:check` grün.
- **AC-02 geprüft:** `lobbyLogic.test.ts` (`?players=n` → n Slots, ohne Parameter Slot 0), `task:check` grün.
- **AC-03 geprüft:** `sim_test` run-1 (online, 1 Client × 2 Spieler, 1 min, Repo-Wurzel auf `develop` mit TR3) → Pass: Feed verbunden 1/1, 544 Kommandos, Monarchen bewegt 1/1, 0 Trennungen, 0 Fehler; Bericht `reports/simtest-run-1/simtest.md`.
- Keine schweren Befunde. Nebenbefund: k3c-dev kannte das Token des selbst gestarteten Spielservers nicht (401), Lauf nach Neustart von k3c-dev mit `K3C_STATUS_TOKEN` durch 🧑 → B-362. Tick p99 max 260 ms im Lauf (Ziel ≤ 10 ms) ist eine Leistungsfrage, kein TR3-Befund (Projekt LST).
