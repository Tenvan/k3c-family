# ECO1.7 · Ausrüstung von oben oder vom Boden holen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/7-ausruestung-von-oben
- **Abhängig von:** ECO1.6
- **Tickets:** B-1197
- **Kriterien:** AC-07

## Ziel

Unten ohne Regal holt ein Bürger liegende Ausrüstung oder wandert nach oben, um sie zu holen.

## Kontext

Ticket B-1197, Gebäude nach Tiefe aus ECO1.6. Fallen gelassene Stücke gibt es seit B-929. Weg über die Treppen, deterministisch, 2+ Spieler. Spielstand-Version erhöhen.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/unit/**`
- `rust/crates/k3c-sim/src/building/**`
- `rust/crates/k3c-sim/src/island/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Wie Bürger heute Waffe und Hammer aus dem Regal holen (`unit/jobs.rs`, `fetch_weapon`).
2. Ohne Regal in der Stufe: liegende Ausrüstung suchen, sonst zur Oberfläche laufen.
3. Tests.

## Fertig, wenn

- [ ] AC-07: Unten ohne Regal holt ein Bürger liegende Ausrüstung oder wandert nach oben.
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

AC-07 geprüft: Unten ohne Werkstatt holt ein Bürger Ausrüstung wie bisher vom Boden der eigenen Stufe (`pickup`, unverändert) oder geht nach oben. Neu: `IslandRef::gear_above` (`island/errand.rs`) ist wahr, wenn oben im Regal einer gebauten Werkstatt ein Hammer oder (unter dem Truppen-Limit) ein Schwert liegt, das noch kein Bürger oben holt, die Stufe keine eigene Werkstatt, aber eine gebaute Treppe nach oben hat; nur mit nativen Regeln. Dann bekommt ein freier Bürger ohne Hammer den Auftrag `ascend` (`unit/jobs.rs`, nach Bau und Reparatur, vor dem Sammeln, bei Gefahr nicht), läuft zur Treppe, `step_errands` setzt ihn oben an die Treppe nach unten (neue ID, `errand_from`), dort holt er wie jeder Bürger Schwert oder Hammer, und kehrt mit der Ausrüstung (oder leer, wenn oben nichts mehr zu holen ist) an die Treppe seiner Stufe zurück. Je Stufe höchstens ein Bürger zugleich unterwegs, deterministisch (Index-Reihenfolge). Tests `island/errand_tests.rs`: Ausrüstung oben erkannt, nichts im Regal, Go-Lauf ohne Besorgung, voller Weg (Bürger geht hoch, holt das Schwert, steht unten als Krieger, Regal leer, keine Besorgung offen). `task check:diff`: nur die zwei in B-1190 AC-04 geführten Rot-Tests (`eingebettete_welten_gueltig`, `insel_nach_1000_schritten_wie_go`).

Abweichung: Spielstand-Version nicht erhöht, weil das Format gleich bleibt: `Troop::errand_from` ist ohne JSON, ein Stand mitten in der Besorgung lässt den Bürger oben (Auftrag `ascend` steht als normaler Auftrag im Stand und läuft beim Laden weiter). 2 Spieler und Koop berührt die Regel nicht (sie arbeitet auf Bürgern).

### Für den Hauptrechner
- Entscheidung 50:50 für Ralf: Die Rückkehr geschieht ohne sichtbaren Weg (der Bürger erscheint oben an der Treppe und unten wieder an der Treppe); ein Laufweg über die Treppen mit Wartezeit wäre ein eigenes Ticket. Auch: Die mitgebrachte Truppe zählt unten zum Truppen-Limit der Stufe, das Limit der Stufe 0 gilt beim Holen.
- Sichtprüfung am PC (Ralf): Bürger verschwindet oben an der Treppe, taucht oben auf und kommt als Krieger zurück.
