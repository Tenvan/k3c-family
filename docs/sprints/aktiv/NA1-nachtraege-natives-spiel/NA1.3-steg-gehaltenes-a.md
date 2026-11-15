# NA1.3 · Gehaltenes A am Steg zahlt erst nach dem Loslassen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/na1
- **Abhängig von:** NA1.2
- **Tickets:** B-1006
- **Kriterien:** AC-04

## Ziel

Nach einer Zielwahl auf der Seekarte bleibt die Eingabe des Spielers verbraucht, bis er A loslässt (B-1006).

## Kontext

Seekarte am Hafen (B-962): `native/k3c-game/hud/seekarte.go` (`seaView`, `step` Zeile 185 liefert `seaResult` mit `moved`, `pick`), `hud/seekarte_input.go` › `updateSea` (gibt `true` nur im Update des Bestätigens zurück), `hud/overlay.go` › `Update` (Zeile 25–27 setzt `out.Consumed[p]`). `native/k3c-game/game/play.go` › `overlays` gibt für verbrauchte Spieler einen leeren Befehl an die Simulation (`view/roles.go` › `OverlayStep`). A ist im Spiel Bezahlen (gehalten); nach dem Bestätigen geht ein weiter gehaltenes A ab dem Folgetakt an die Simulation und der Spieler zahlt am Steg. Tests: `hud/seekarte_test.go` (2 Spieler, A verbraucht am Steg, 0 Allokationen). Zustand je Spieler liegt in `h.seats[p].sea`.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- `native/k3c-game/game/**`

## Nicht-Ziele

`engine/sim` (Bezahlen bleibt, wie es ist); andere Taste für die Seekarte; Skill-Menü; Grafik.

## Schritte

1. Roten Test schreiben: bestätigen, A weiter halten → `Consumed` bleibt; ein Update ohne A → `Consumed` endet.
2. Merker je Spieler im Seekarten-Zustand (gesetzt beim Bestätigen, gelöscht beim Loslassen, beim Verlassen des Stegs, bei Kronenverlust und getrenntem Gerät); `updateSea` meldet verbraucht, solange er gilt.
3. Test mit 2 Spielern: Sperre von Spieler 1 trifft Spieler 2 nicht.
4. Allokations-Test mit aktiver Sperre.
5. Agent `k3c-hud` prüfen lassen, Befunde beheben; `go test ./hud/ ./game/`; `task check:diff`.

## Fertig, wenn

- [ ] AC-04: Tests in `native/k3c-game/hud` für Halten und Loslassen, 2 Spieler und 0 Allokationen grün (B-1006/AC-01, AC-02, AC-03).
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
