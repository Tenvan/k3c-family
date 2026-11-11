# NV3.1 · Fluss in der unteren Bildhälfte und feste Schicht fern

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** nv3/1-fluss-fester-hintergrund
- **Abhängig von:** –
- **Tickets:** B-789
- **Kriterien:** AC-01, AC-02

## Ziel

Der Prototyp zeigt den Fluss in der unteren Hälfte jeder Sicht als fließende Kacheln in drei Bahnen, und die Schicht `fern` steht.

## Kontext

`native/k3c-ebiten/level.go` (Schichten `layers` mit Scroll-Faktor, Boden als gefärbter Bildpunkt), `main.go` (Sicht je Spieler, `groundY`). Der Fluss liegt vor dem Ufer (z vor allen Figuren). Platzhalter-Kacheln entstehen im Code (Papierwellen in drei Blautönen), keine neuen Assets. Allokationen im Frame und Texturwechsel klein halten (ein Bild für alle Bahnen).

## Erlaubte Dateien

- `native/k3c-ebiten/**`
- `docs/glossar.md` (Begriff Fluss, über den letzten Commit)

## Nicht-Ziele

Rigs (NV3.2, NV3.3), Shader (B-790), Schiffe.

## Schritte

1. Aufteilung der Sicht: obere Hälfte Welt, untere Hälfte Fluss; Boden-Linie an der Grenze.
2. Fluss: drei Bahnen, Kacheln laufen je Bahn mit eigenem Tempo durch, unabhängig von der Kamera; Kamera-Fahrt verschiebt sie zusätzlich.
3. `fern` mit Scroll-Faktor 0 je Sicht fest.
4. Screenshots an zwei Kamera-Positionen nach `reports/`.

## Fertig, wenn

- [ ] AC-01: Fluss füllt die untere Hälfte, drei Bahnen fließen (Screenshot).
- [ ] AC-02: `fern` steht, `mitte` und `nah` scrollen (zwei Screenshots).

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
