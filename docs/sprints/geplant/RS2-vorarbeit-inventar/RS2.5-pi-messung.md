# RS2.5 · Pi-3-Messung nach B-731 (Gate G1)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** Mensch
- **Domäne:** INF
- **Lane:** –
- **Modell:** –
- **Umgebung:** live
- **Branch:** sprint/rs2
- **Abhängig von:** –
- **Tickets:** B-736
- **Kriterien:** AC-05

## Ziel

Ralf misst auf dem Pi 3 Tick p99 (2 Räume × 3 Spieler) und `sim_run`-Ticks/s mit dem Go-Server nach B-731 (und optional vor B-731) und trägt die Zahlen in 006 ein. Damit liegt der Entscheidungsstand für Gate G1 vor.

## Kontext

Hardware-Session (`Agent: Mensch`), entkoppelt: keine Abhängigkeit der App-Sessions und des Reviews (`docs/arbeitsweise.md` › Hardware entkoppelt). Nachholung der offenen RS1.2 (Baseline Pi). Rezept: `docs/messung/rst-baseline.md`, linux/arm64-Build des Go-Servers (`task k3c-server:build` mit `GOOS=linux GOARCH=arm64`, Details im Rezept), Zielwerte B-042 (Tick p99 < 10 ms). Handmessung 2026-10-03: 8,0–10,3 ms. Gate G1 (B-736/AC-09): p99 ≤ 5 ms nach B-731 heißt, Claude fragt Ralf, ob Etappe 2 trotzdem weiterläuft.

## Erlaubte Dateien

- `docs/decisions/006-rust-engine.md` (Pi-Zeilen)
- `docs/messung/rst-baseline.md`
- `reports/**` (Messberichte)

## Nicht-Ziele

Kein Code. Keine Rust-Messung (RS3). Kein Dauerbetrieb.

## Schritte

1. Ralf: Go-Server (arm64) vor und nach B-731 auf den Pi 3 bringen (zwei Builds: Commit vor RS2.3 und Stand nach RS2.3).
2. Ralf: `sim_test` perf online (2 Räume × 3 Bots, 5 min) und `sim_run` 100 000 Ticks je Build; Tick-Dauer p99 und Ticks/s notieren, Raumtemperatur/Throttling (`vcgencmd get_throttled`) notieren.
3. Ralf: Zahlen in 006 › Messbefunde als Tabelle ¾Pi 3« eintragen; Gate G1 bewerten (≤ 5 ms: Rustumstieg trotzdem? ja/nein).

## Fertig, wenn

- [ ] AC-05: 006 enthält die Pi-Zeilen (Tick p99, Ticks/s, vor/nach B-731, Last, Temperatur).
- [ ] AC-05: Gate-G1-Entscheidung von Ralf im Ergebnis genannt.

## Prüfen

Manuelle Messung am Gerät durch Ralf; kein `check:diff` nötig, wenn nur `docs/` geändert wird (`task check:diff` einmal vor dem Commit).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
