# WE4.2 · Seekarte am Hafen im Spiel (B-962)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/we4
- **Abhängig von:** WE4.1
- **Tickets:** B-962
- **Kriterien:** AC-02

## Ziel

Seekarten-Overlay am offenen Wechselpunkt mit Wahl per Links/Rechts/A (B-962).

## Kontext

HUD `native/k3c-game/hud/`, Zustand `native/k3c-game/view/`, Eingabe `native/k3c-game/input/`; Ziele und Wahl kommen aus B-961.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- `native/k3c-game/view/**`
- `native/k3c-game/game/**`
- `native/k3c-game/input/**`

## Nicht-Ziele

Grafik-Assets, Editor.

## Schritte

1. Zustand der Ziele in `view`.
2. Overlay zeichnen.
3. Eingabe auf Wahl-Aktion.
4. Tests mit 2 Spielern.

## Fertig, wenn

- [ ] AC-01, AC-02 aus B-962 (AC-03 TV durch 🧑)

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

AC-01/AC-02 (B-962) geprüft: `hud/seekarte_test.go` (2 Spieler, A verbraucht am Steg, keine Allokationen). Review k3c-hud: Befunde behoben. AC-03 TV-Abnahme offen bei 🧑. Offen: gehaltenes A zahlt ab dem Folgetakt wieder.
