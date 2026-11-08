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

### Abnahme-Läufe 2026-11-08 (Workbench mit RS2-Stand, CPU-Last vor jedem Lauf gering)

Vorab: `sim_test` online lehnte ab, `bin/k3c-sim.exe` war älter als `engine/net/protocol.go`; `task k3c-sim:build --force` in der Shell (Task-Schloss in der Workbench zu).

- **AC-01 Sim – rot (B-754):** online perf 1 × 4 Geräte, 5 min: run-29 Tick p99 max 6,8 ms (über Abbruchgrenze 5 ms), Wiederholung run-30 3,2 ms; Referenz RS2.3 run-28 2,5 ms. Keine Trennungen, Fehler 0, Monarchen 4/4. Lauf „offline mit Clients“ lehnt `sim_test` ab (Client braucht den Spielserver), daher online headless 1 × 2 Geräte (Bots über `k3c-sim`), 10 min, stability+perf: run-31 Pass, Tick p99 max 1,6 ms, Fehler 0, Trennungen 0.
- **AC-02 Balance – rot (B-753):** BAS-Standardlauf (coop2, 101 Seeds) run-32 fertig in 7 min 39 s über `bin/k3c-sim`, Ziele: 1 von 5, 4 von 8 Korridoren (Werte im Fluss, B-698; kein vergleichbarer Lauf vor RS2, kein Vergleichswert). Meilenstein-Lauf (coop2, standard, 40 Tage, 101 Seeds) run-33: Abbruch nach 1 h 0 min 2 s „Zeitlimit erreicht“, ohne Bericht; vor RS2 38 min 41 s (RS1.1) → mindestens +55 %, Grenze +5 % verfehlt.
- **AC-03 Gen – grün:** `check_run task:test` exit 0, 4 min 27 s (mit `gen:paper`); keine Golden-Änderung im Arbeitsbaum.
- **AC-04 Race – übersprungen:** `check_run task:check:race` exit 0 in 468 ms, Ausgabe „go:race übersprungen: kein C-Compiler“. Hinweis: `go:race` deckt nur `./engine/...` ab, `tools/k3c-workbench/internal/simexe` wäre auch mit gcc nicht im Race-Lauf; die CI prüft `-race`.

Status bleibt offen, bis B-753 und B-754 geklärt sind.
