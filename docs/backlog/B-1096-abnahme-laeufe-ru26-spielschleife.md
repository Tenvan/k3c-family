# B-1096 · Abnahme-Läufe RU26: Sim, Balance, Gen und Race für die Rust-Spielschleife

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** RU26
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

**Lane: zweit** (Second Worker fährt die Läufe; der Hauptrechner führt keine Sim-, Balancing-, Gen- oder Race-Läufe aus). RU26 hat die Spielschleife (Zustände, Split-Screen, Zeichnen) von Go nach Rust portiert (`rust/crates/k3c-game/src/game/**`). Die Sim-Sperre vom 2026-11-11 gilt; AC-01 und AC-02 heißen daher `verschoben (Sperre 2026-11-11)`.

## Ziel

Die Abnahme-Läufe für RU26 sind gelaufen oder mit Grund verschoben; Ralf hat die Spielschleife im Spiel geprüft, sobald RU30 sie startbar macht.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** Sim: verschoben (Sperre 2026-11-11), keine `sim_test`-Läufe.
- **AC-02** Balance: verschoben (Sperre 2026-11-11).
- **AC-03** Gen samt Golden: `check_run task:test` (ruft `gen:paper` auf, Grafik-Golden `tests/paperAssets.test.ts`, `tools/paper/golden`, Sprite-Tests) grün; `testdata/golden` bleibt unverändert.
- **AC-04** Race: `check_run task:check:race` grün (ohne C-Compiler lokal als übersprungen nennen).
- **AC-05** Prüfliste für Ralf im Spiel, sobald RU30 die Schleife startbar macht: (1) Zwischenbilder: Bewegung der Einheiten und der Kamera bei 60 Hz ruhig, kein Zittern, kein Gleiten beim Stufenwechsel (`JUMP_UNITS`); Vergleich mit dem Go-Spiel (`--pacing sync`) auf demselben Rechner; (2) zwei Spieler: Zelle oben und unten, jede Kamera folgt dem eigenen Monarchen, jedes Gerät steuert nur seinen Spieler, Beitritt mit Bestätigen, verlorenes Pad wird ersetzt; (3) Menü: B geht in jedem Menü zurück, im Spiel ist B unbelegt, Pause öffnet und schließt; (4) Vollbild-Taste F schaltet Vollbild und Fenster (Größe und Lage setzt Rust nicht, B-1094); (5) Leistungsanzeige (F2/F3) zeigt FPS und Zeiten.

## Nicht-Ziele

Neue Funktionen, Änderungen am Go-Code, Golden neu schreiben.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

nicht relevant: Abnahme-Läufe ohne neues Verhalten.

## Ausnahme- und Fehlerfälle

nicht relevant: Ein roter Lauf wird behoben oder als Ticket erfasst.

## Akzeptanzkriterien

- **AC-01** Sim: `verschoben (Sperre 2026-11-11)`.
- **AC-02** Balance: `verschoben (Sperre 2026-11-11)`.
- **AC-03** Gen: `check_run task:test` grün, Golden unverändert.
- **AC-04** Race: `check_run task:check:race` grün.
- **AC-05** Ralf hat die Prüfliste (Takt, Eingabe, Kamera, siehe Anforderungen) im Spiel abgehakt.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
