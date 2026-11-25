# B-1285 · Abnahme RU40: Sim, Balance, Gen und Fensterläufe der Spiel-EXE am PC

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU40
- **Projekt:** RSU
- **Erstellt:** 2026-11-25
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU40 (Balancing-Läufe im Game-Client) ist umgesetzt: `k3c-game sim balance` headless und mit `--window`, `sim save`, `sim_test`/`replay_run` über die EXE, `engine/simcli` und `cmd/k3c-sim` entfernt. Sim, Balance, Gen und `check:race` laufen wegen der Sim-Sperre (Beschluss 🧑 2026-11-11) erst hier (B-742); die Fensterläufe brauchen 🧑 am PC.

## Ziel

RU40 ist abgenommen: Fensterläufe von 🧑 bestätigt, die gesperrten Läufe einmal gelaufen.

## Beteiligte und Zielgruppen

🧑 prüft am PC (Gerät: PC); ein Agent fährt nach Widerruf der Sim-Sperre die gesperrten Läufe.

## Anforderungen

1. Nach `task build` (Gerät: PC): `k3c-game sim balance --window --speed 8 --seed-list k3c-dev --bots saver --players 1 --depths 0 --days 1` läuft sichtbar; Fenster schließen ergibt Exit 130 ohne Bericht; `--players 2` zeigt zwei Zellen.
2. Studio: `sim_test start {head:true}` zeigt das Fenster, `stop` schließt es, danach läuft keine `k3c-game.exe` mehr; `replay_run {head:true}` liefert denselben Hash wie headless.
3. Studio: „Im Spiel ansehen“ (voll-<seed> über `sim save`), Meldung bei ungültigem Seed, Welten-Override speichern.
4. Nach Widerruf der Sim-Sperre: `sim_test` (AC-01 Sim), ein Balancing-Lauf `task balance:run -- --seeds 2` über `k3c-game sim balance` (AC-02 Balance), `check_run task:test` samt `gen:paper` und Golden (AC-03 Gen), `check_run task:check:race` (AC-04 Race; ohne gcc lokal übersprungen, dann nennen).

## Nicht-Ziele

Neue Funktionen; Wertänderungen aus dem Balancing (nur Lauf).

## Regeln und Einschränkungen

Sim und Balancing bis auf Widerruf gesperrt (CLAUDE.md); Balancing schreibt nur unter `data/worlds/`.

- **Bereich:** `reports/**`
- **Abhängig von:** RU40
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Fenster schließen während `sim balance --window` → Zeile `aborted`, Exit 130, kein Bericht.

## Ausnahme- und Fehlerfälle

Kein gcc lokal → `check:race` als übersprungen nennen.

## Akzeptanzkriterien

- **AC-01** Sim: `sim_test` läuft über die Spiel-EXE und endet ohne Fehler (nach Widerruf der Sperre).
- **AC-02** Balance: `task balance:run -- --seeds 2` schreibt einen Bericht über `k3c-game sim balance`.
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Grafik-Golden grün.
- **AC-04** Race: `check_run task:check:race` grün oder als übersprungen genannt.
- **AC-05** 🧑 hat die Fensterläufe und die Studio-Prüfungen (Anforderungen 1 bis 3) am PC bestätigt.

## Offene Fragen

keine

## Notizen

Ergebnisse RU40.2 bis RU40.4 nennen die Befehle im Einzelnen.
