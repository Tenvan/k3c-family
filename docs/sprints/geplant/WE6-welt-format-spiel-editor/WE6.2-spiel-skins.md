# WE6.2 · Spiel: Skins der Welt für Gegner, Objekte, Gebäude und Schiff (B-994)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/we6
- **Abhängig von:** WE6.1
- **Tickets:** B-994
- **Kriterien:** AC-02

## Ziel

Das Spiel löst jedes Asset über die `skins` der aktiven Welt auf und zeigt Gegner-Namen aus dem Wertesatz der Welt (B-994).

## Kontext

Natives Spiel `native/k3c-game`: Asset-Laden `art/manifest.go`, `art/budget.go` (Texturbudget), `art/worldimages.go` (Vorbild: Bilder je Welt), Katalog `actors/catalog.go`, Objekte `props/`, Welt-Auswahl `world/`, `menu/`. Seit WE5.5 (B-984) trägt `sim.WorldDef` die Zuordnung `skins` (Schlüssel und Wert `<gruppe>/<asset>`, gleiche Gruppe). Gruppen-Indizes: `public/grafik/<gruppe>/index.paper.json`, je Asset `manifest.json`. Ersatz-Assets aus WE7 können noch fehlen: Tests nutzen Test-Assets oder vorhandene Assets als Ersatz (z. B. `gegner/wolf` → `gegner/goblin`).

## Erlaubte Dateien

- `native/k3c-game/**`

## Nicht-Ziele

Skins für Monarch und Truppen, neue Assets, Engine, Editor.

## Schritte

1. Eine Auflöse-Funktion (Basis-Pfad, Welt) → Pfad an der einen Stelle, an der Assets geladen werden; Tests zuerst.
2. Verträglichkeit prüfen (Animationen und Zustände des Ersatz-Assets decken die Basis ab), sonst Basis-Asset und ein Log je Asset.
3. Weltwechsel: Zuordnung und geladene Texturen der alten Welt freigeben.
4. Gegner-Namen im HUD aus dem Wertesatz der Welt.
5. Review `k3c-grafik` (Texturbudget) vor dem Commit.

## Fertig, wenn

- [ ] AC-02: B-994/AC-01 bis AC-03 als Tests grün.
- [ ] B-994/AC-04 (Sichtprüfung) ist an B-996 verwiesen.
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
