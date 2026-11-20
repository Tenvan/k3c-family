# LA1.5 · Stimmung nach dem Abgleich umsetzen (Farbe, Licht, Dunst, Tiefe)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/la1
- **Abhängig von:** LA1.4
- **Tickets:** B-1164
- **Kriterien:** AC-05

## Ziel

Die ranghöchsten Abweichungen der Stimmung aus LA1.4 sind im Rust-Spiel behoben: Farbe, Licht, Dunst und Tiefe liegen sichtbar näher am K2C-Vorbild.

## Kontext

Lane zweit, Modell opus. Eingabe: `docs/messung/stimmung-abgleich.md` (LA1.4). Look-Werte in `data/look.json` (Profil k2c), Standard `docs/standards/bildaufbau.md`. Gen und Bildvergleiche darf die Lane zweit laufen lassen. Reicht die Session nicht für alle Punkte, bleiben Punkte als Liste im Ergebnis („Für den Hauptrechner“).

## Erlaubte Dateien

- `data/look.json`, `data/light.json`
- `rust/crates/k3c-game/src/world/**`, `fx/**`, `art/**`, `still/**`
- `testdata/rust/**` nur für neue Bildvergleiche der geänderten Szenen

## Nicht-Ziele

Neue Assets, Regeln, Größen (LA1.1), Waldwand (LA1.2).

## Schritte

1. Abgleich-Liste lesen, Reihenfolge nach Rang.
2. Je Punkt ändern, Szene aufnehmen und neben K2C-Referenz prüfen.
3. Tests, `task rust:lint`, einmal `task check:diff`; Vorher-Nachher-Bilder im Ergebnis.

## Fertig, wenn

- [ ] AC-05: Die Punkte aus LA1.4 mit Rang 1 bis 3 sind behoben, Vorher-Nachher-Bilder im Ergebnis.
- [ ] Nachprüfung 🧑 am PC.
- [ ] Tests und `task rust:lint` grün.

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
