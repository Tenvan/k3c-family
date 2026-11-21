# ECO1.2 · Siegel in Auswahl, Spielstand-Liste und HUD

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/2-siegel-anzeige
- **Abhängig von:** ECO1.1
- **Tickets:** B-1192
- **Kriterien:** AC-02

## Ziel

Das Siegel des gewählten Grades steht in der Auswahl, in der Spielstand-Liste und im HUD bei der Welt-Anzeige.

## Kontext

Ticket B-1192, Siegel-Assets aus ECO1.1. Der Grad liegt schon im Spielstand; fehlt er (alter Stand), gilt der Standardgrad ohne Fehler. Frame-Budget: Textur einmal laden, kein Allokieren je Bild; 1 und 2 Ansichten.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/menu/**`
- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/local/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Wo Schwierigkeit heute gezeigt wird, per grep finden (menu, Spielstand-Liste, hud Welt-Zeile).
2. Siegel-Asset je Grad laden und neben Text zeichnen.
3. Alter Stand ohne Grad: Standardgrad.
4. Tests (Anzeige, fehlender Grad).

## Fertig, wenn

- [ ] AC-02: Auswahl, Spielstand-Liste und HUD zeigen das Siegel; alter Stand ohne Fehler.
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
