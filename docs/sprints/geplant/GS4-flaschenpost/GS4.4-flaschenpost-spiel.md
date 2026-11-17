# GS4.4 · Spiel: Flasche, Geschichtszeile, Pfeil und Leuchten (B-980)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs4
- **Abhängig von:** GS4.3
- **Tickets:** B-980
- **Kriterien:** AC-06

## Ziel

Das Spiel zeichnet die Flasche, blendet ihre Geschichtszeile ein und führt beide Spielerhälften mit Pfeil und Leuchten zum Ziel.

## Kontext

Spec: `docs/backlog/B-980-flaschenpost-im-spiel.md` (R1–R8, AC-02 bis AC-05). Die Simulation liefert (GS4.2, `engine/sim/bottle.go`): Zustand je Stufe mit Flasche (Ort) und Markierung (Zielart, Stufe, Ort), Ereignisse `bottleFound` (mit Zeilen), `bottleLost`, `bottleTargetReached`. Spiel: Level-Objekte in `native/k3c-game/props/` und `native/k3c-game/world/`, Effekte `native/k3c-game/fx/`, HUD je Spielerhälfte `native/k3c-game/hud/` (Radar `radar.go` mit Marker-Arten, Geschichtszeile `story.go` aus GS1, Layout `layout.go`), Zustand und Ereignisse über `native/k3c-game/view/`. Richtung: gleiche Stufe → links/rechts nach Kamera-Ausschnitt; andere Stufe → schräg zum nächsten Übergang (Tiefen-Eingang oder Treppe, Positionen stehen im Zustand der Stufe wie für das Radar). Alles ohne Text; das Spiel ändert keinen Sim-Zustand; mit 2 Spielern; keine Allokation je Frame.

## Erlaubte Dateien

- `native/k3c-game/props/**`
- `native/k3c-game/world/**`
- `native/k3c-game/fx/**`
- `native/k3c-game/hud/**`
- `native/k3c-game/view/**`

## Nicht-Ziele

Regeln und Zustand der Flaschenpost, Ton, Hinweis auf der Seekarte, Darstellung von Einsiedler und Gatter.

## Schritte

1. Tests zuerst: Flasche und Radar-Marker aus dem Zustand; `bottleFound` reiht Zeilen in alle Hälften; Pfeilrichtung (links, rechts, tiefer, höher, im Bild); Ende bei `bottleTargetReached`; geladener Zustand ohne Geschichtszeile; 0 Allokationen.
2. Flasche als Objekt mit Glitzern (aus der Uhr), Rückfall ohne Asset.
3. Pfeil am Bildrand je Spielerhälfte (neue Datei `hud/bottle.go`), Radar-Marker.
4. Leuchten am Ziel in der Welt.
5. Fachprüfung `k3c-hud`, `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-06: Flasche, Geschichtszeile, Pfeil und Leuchten in beiden Hälften, Ende am Ziel, 0 Allokationen (B-980/AC-02 bis AC-05).
- [ ] B-980/AC-06 (TV) bleibt offen bei 🧑.

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
