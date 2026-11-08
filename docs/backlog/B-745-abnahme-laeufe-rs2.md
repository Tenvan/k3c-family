# B-745 · Abnahme-Läufe RS2

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** RS2
- **Projekt:** RST
- **Erstellt:** 2026-11-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Sprint RS2 (Projekt RST) ändert den Weg der Läufe: `sim_run`, `sim_test` offline und online, `level_generate` und `replay_run` rechnen über den Kindprozess `bin/k3c-sim` (B-726, RS2.1/RS2.2), `engine/balance` ist aus der Workbench nach `engine/` gewandert, der Raum-Zustand entsteht ohne JSON-Roundtrip (`engine/net/state.go`, B-731). Nach Regel B-742 laufen Sim, Balance, Gen und Race nicht in der Review-Session RS2.6, sondern hier.

## Ziel

Die langen Läufe bestätigen RS2 nachträglich: Stabilität und Performance, Balancing, Grafik-Golden und Race-Detector grün.

## Beteiligte und Zielgruppen

Claude arbeitet das Ticket autonom ab (nachts). Ralf (🧑) liest das Ergebnis.

## Anforderungen

- Läufe nur über `sim_test` und `check_run` der Workbench (B-348), Workbench vorher mit dem Stand von RS2 neu gebaut (sonst findet die alte EXE `internal/balance/cmd` nicht mehr).
- Messwerte in die Notizen.

## Nicht-Ziele

Nachbessern im Ticket: Rot heißt neues Ticket (Typ `Problem`, Projekt RST).

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Sim, Balance und Gen (B-742); lange Läufe nur über `sim_test`/`check_run`; Testläufe mit Dauer und Abbruchkriterium.

## Beispiele

`sim_test` online perf 1 × 4 Geräte, 5 min → Tick p99 ≤ 2,5 ms wie RS2.3 (run-28).

## Ausnahme- und Fehlerfälle

Kein C-Compiler: AC-04 übersprungen, im Ergebnis so nennen. Workbench-EXE veraltet: erst `task k3c-workbench:build`, dann Läufe.

## Akzeptanzkriterien

- **AC-01** Sim: `sim_test` online `perf` (4 Geräte, 5 min, Abbruch bei Fehler oder Tick p99 > 5 ms) und offline mit Clients (Bots über `k3c-sim bot`, 2 Geräte, 10 min, Abbruch bei Bot-Fehler) grün; Tick p99 nicht schlechter als RS2.3.
- **AC-02** Balance: `sim_test` BAS-Standardlauf und Meilenstein-Lauf (coop2, standard, 40 Tage, voll 101 Seeds) über `bin/k3c-sim`, vorläufig mit Cheats (B-698), grün; Dauer des Meilenstein-Laufs ≤ +5 % gegenüber vor RS2 (B-726/AC-03, Volllauf in RS2.2 nicht gemessen).
- **AC-03** Gen: `check_run task:test` (ruft `gen:paper`, Grafik-Golden `tests/paperAssets.test.ts`, `tools/paper/golden` samt geändertem `public.test.ts`, Sprite-Tests) grün; gewollte Golden-Änderung nur nach „Golden aktualisieren“.
- **AC-04** Race: `check_run task:check:race` grün (ohne C-Compiler übersprungen, im Ergebnis so nennen); Schwerpunkt `engine/net` (globale Caches `boxed`, `words` in `state.go`) und `tools/k3c-workbench/internal/simexe`.

## Offene Fragen

keine

## Notizen

Angelegt von der Review-Session RS2.6 (2026-11-08).
