# NG6.2 · Radar, Pause-, Speicher- und Beitritts-Hinweis, Labels, Kampf-Anzeige

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** hud
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng6/2-radar-hinweise
- **Abhängig von:** NG6.1
- **Tickets:** B-799
- **Kriterien:** AC-02

## Ziel

Radar, Pause-, Speicher- und Beitritts-Hinweis, Bürger- und Wirtschafts-Labels und die Kampf-Anzeige folgen dem Frame.

## Kontext

Spec: B-799. Vorbild: `src/scenes/hud/radar.ts`, `radarView.ts`, `pauseBanner.ts`, `pauseButton.ts`, `savedHint.ts`, `joinHint.ts`, `citizenLabels.ts`, `economyLabels.ts`, `src/scenes/kampfAnzeige.ts`, `kampfView.ts` samt Tests. Vertrag und Importregel wie NG6.1.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- Planung NG6, B-799 (über MCP)

## Nicht-Ziele

Pause-Menü (NG7.4); Diagnose-Anzeige.

## Schritte

1. Fälle aus den TS-Tests als Go-Tests (rot).
2. Radar.
3. Hinweise und Labels, Kampf-Anzeige.

## Fertig, wenn

- [ ] AC-02: Tests grün; Screenshot mit Radar und Kampf-Anzeige im Ergebnis.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 umgesetzt: Radar mit Kamera-Ausschnitt, Pause-Band, „gesichert“, Beitritts-Hinweis aus `Frame.Unseated`, Boss-Leiste, Nacht-Events, Berufs- und Händler-Labels. Geprüft mit `kampf_test.go`, `state_test.go` und am Bild `ng6-kampf/shot-01.png` (Agent und Claude: Nacht 1, Radar mit Gegnern, „Welle 1: 5 Gegner“, Meldung, beide Zellen getrennt). Pause-Band und Beitritts-Hinweis nur per Test belegt.
