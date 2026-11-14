# B-726 · Workbench ruft Sim und Balancing als Task auf statt engine/sim zu importieren

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** verworfen
- **Sprint:** RS2
- **Projekt:** WZG
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-07, Chat (Ralf), Revision 1, durch 🧑

## Ausgangslage

Die Workbench importiert die Engine direkt: `internal/balance` (14 Dateien), `botdev` (2), `enginetools` (2), `botfeed`, `mcpsrv`, `studioview` (je 1) nutzen `k3c/engine/sim`, `room`, `net`, `store` oder `level`. Damit hängt jede Umstellung der Engine auf Rust (Entscheidung 006) an der Workbench. Ralf (2026-11-07): Die Importe dürfen wegfallen, Sim und Balancing laufen per Task.

## Ziel

Die Workbench startet Simulation, Balancing und Level-Erzeugung als Kindprozess über eine feste Schnittstelle (JSON rein, Bericht/JSON raus) und importiert kein `k3c/engine/*` mehr außer reinen Typen, die bis zur Umstellung bleiben dürfen. Danach ist es für die Workbench egal, ob die Engine Go oder Rust ist.

## Beteiligte und Zielgruppen

Ralf (🧑) entscheidet; Nutzer der Tools `sim_run`, `sim_test` (offline), `level_generate`, Balancing-Seite, „Im Spiel ansehen“.

## Anforderungen

- Ein Engine-CLI (z. B. `cmd/k3c-sim`) mit Unterbefehlen `run`, `balance`, `level`; Eingabe JSON auf stdin oder Datei, Ausgabe JSON/Bericht unter `reports/`.
- Fortschritt als JSON-Zeilen auf stdout, damit `sim_test` `status`/`wait` weiter funktioniert.
- Abbruch über Prozess-Ende (Job-Objekt wie `engine/devrun`).
- Dauer eines `sim_test`-Laufs höchstens 5 % langsamer als heute.

## Nicht-Ziele

- Keine Rust-Implementierung (B-725).
- `studioview` (eingebettete Engine für „Im Spiel ansehen“) darf vorerst den laufenden Spielserver nutzen statt eines eigenen CLI.
- Generatoren (`tools/buildings`, `ruletable`, `atlas`) entfallen mit PAP.

## Regeln und Einschränkungen

`engine/` importiert nichts aus `tools/`. Lange Läufe nur über `sim_test`/`check_run`. Feste EXE-Pfade unter `bin/` (keine Firewall-Abfrage, kein `go run`). Befehle über `task`.

## Beispiele

- `sim_test {action: start, bots: coop2, world: standard, days: 40}` → Workbench startet `bin/k3c-sim balance …`, Bericht wie heute unter `reports/simtest-<id>/`.
- `sim_run {seed: k3c-dev, ticks: 1000}` → gleiche Ausgabe wie heute.

## Ausnahme- und Fehlerfälle

- CLI fehlt oder ist veraltet → klare Fehlermeldung mit `task k3c-sim:build`.
- CLI stürzt ab → Lauf `fehlgeschlagen`, letzte stderr-Zeilen im Status.

## Akzeptanzkriterien

- **AC-01** `go list -deps ./tools/k3c-workbench/...` enthält `k3c/engine/sim`, `room`, `net`, `level` nicht mehr (Ausnahme `studioview` laut Nicht-Ziele, falls nötig).
- **AC-02** `sim_run`, `sim_test` offline (BAS-Standardlauf und Meilenstein-Lauf) und `level_generate` liefern dieselben Ergebnisse wie vorher (Bericht-Diff leer bis auf Zeitstempel).
- **AC-03** Ein Meilenstein-Lauf ist höchstens 5 % langsamer als vorher.

## Offene Fragen

keine

## Notizen

Voraussetzung für Etappe 2 aus 006 (Rust-Server); kann parallel zu RS1 laufen.

Verworfen 2026-11-12 (Entscheidung Ralf): Sim und Balancing gesperrt (Beschluss 2026-11-11); Wiederaufnahme mit neuen Tickets.
