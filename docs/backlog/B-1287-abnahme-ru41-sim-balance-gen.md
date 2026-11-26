# B-1287 · Abnahme RU41: Sim, Balance, Gen und Race ohne Go-Engine

- **Domäne:** INF
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU41
- **Projekt:** RSU
- **Erstellt:** 2026-11-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU41 hat die Go-Engine entfernt (nur `conlog`, `devrun`, `internal/rules` bleiben) und Entscheidung 011 angenommen. Sim, Balance, Gen und `check:race` laufen wegen der Sim-Sperre (Beschluss 🧑 2026-11-11) erst hier (B-742).

## Ziel

RU41 ist abgenommen: die gesperrten Läufe sind einmal ohne Go-Engine gelaufen.

## Beteiligte und Zielgruppen

Ein Agent fährt nach Widerruf der Sim-Sperre die Läufe; 🧑 widerruft die Sperre.

## Anforderungen

1. Nach Widerruf der Sim-Sperre: `sim_test` über die Spiel-EXE (AC-01).
2. Balancing-Lauf `task balance:run -- --seeds 2` über `k3c-game sim balance` (AC-02).
3. `check_run task:test` samt `gen:paper` und Grafik-Golden (AC-03).
4. `check_run task:check:race` nur noch für `./engine/...` und die Werkzeuge (AC-04; ohne gcc lokal übersprungen, dann nennen).

## Nicht-Ziele

Neue Funktionen; Wertänderungen aus dem Balancing (nur Lauf); Golden neu schreiben.

## Regeln und Einschränkungen

Sim und Balancing bis auf Widerruf gesperrt (CLAUDE.md); Balancing schreibt nur unter `data/worlds/`.

- **Bereich:** `reports/**`
- **Abhängig von:** RU41
- **Braucht 🧑:** Widerruf der Sim-Sperre
- **Größe:** S
- **Sperre:** –

## Beispiele

`check_run task:check:race` → nur Pakete unter `engine/` und `tools/`, kein `engine/sim` mehr.

## Ausnahme- und Fehlerfälle

Kein gcc lokal → `check:race` als übersprungen nennen.

## Akzeptanzkriterien

- **AC-01** Sim: `sim_test` läuft über die Spiel-EXE und endet ohne Fehler.
- **AC-02** Balance: `task balance:run -- --seeds 2` schreibt einen Bericht über `k3c-game sim balance`.
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Grafik-Golden grün.
- **AC-04** Race: `check_run task:check:race` grün oder als übersprungen genannt.

## Offene Fragen

keine

## Notizen

Gemeinsam mit B-1285 (RU40) fahren; beide brauchen denselben Widerruf.
