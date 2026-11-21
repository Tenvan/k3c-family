# B-1154 · Abnahme-Läufe RU62: Sim, Balance, Gen samt Golden und Race

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU62
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU62 (Welt-Features SI1 bis SI6 in Rust) ist umgesetzt; Sim, Balance und Gen liefen bewusst nicht in den Sessions (Sperre, B-742). Dieses Ticket bündelt die Abnahme-Läufe. Die Rust-Tests der Pakete sind in den Sessions grün; vorbestehend rot: 6 Vitest-Fälle in `tools/paper/golden` und `render.test.ts` (Windows EPERM), fehlender `golangci-lint` im PATH.

## Ziel

Sim, Balance und Gen des Sprints RU62 sind gemessen und grün, das Grafik-Golden stimmt, der Race-Detector läuft sauber.

## Beteiligte und Zielgruppen

Claude (autonom, nachts); Ralf nur für die manuelle Abnahme am PC (separate Prüfpunkte der Sessions RU62.2 bis RU62.13).

## Anforderungen

- Alle Messungen deterministisch, ohne Schwellen anzuheben, ohne `testdata/` oder Golden zu verändern, außer bei gewollter Regeländerung mit Begründung.

## Nicht-Ziele

Manuelle Abnahme am PC; Balancing-Änderungen.

## Regeln und Einschränkungen

Nur über `sim_test` und `check_run`; Sim-Code nur im eigenen Worktree (B-925).

- **Bereich:** `reports/**`
- **Abhängig von:** RU62
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden

## Beispiele

nicht relevant: Abnahme-Ticket.

## Ausnahme- und Fehlerfälle

Rot: neues Ticket mit Messwerten, nicht die Schwelle anheben.

## Akzeptanzkriterien

- **AC-01** Sim: `sim_test` für die Welten mit Fracht, Koop-Truhen, Wetter, Welt-Reittier, Einsiedler und Fest läuft ohne Abbruch und deterministisch.
- **AC-02** Balance: Zielkorridore der Werte aus `data/cargo.json`, `coop.json`, `weather.json`, `mounts.json`, `hermits.json`, `festival.json` gemessen.
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Grafik-Golden grün (neue Sheets der Assets `ladeplatz-*`, `fracht-*`, `doppel-truhe*`, `wetter-wolkenbank`, `gatter`, `einsiedler-huette`, `dojo`, `schrein`, `schwertschmied`, `priesterin`, `fest-*`); Bild-Budget und Texturspeicher der zusätzlich geladenen Sheets messen.
- **AC-04** Race: `check_run task:check:race` grün.

## Offene Fragen

keine

## Notizen

Lane `zweit`: Der Second Worker fährt dieses Ticket (Sim, Balance, Gen und Race).
Siehe Ergebnis der Session RU62.14 (Befunde für den Hauptrechner).
