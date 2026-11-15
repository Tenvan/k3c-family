# SI6.2 · Assets Laterne, Blüte, Fackel für das Fest (B-1002)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si6
- **Abhängig von:** SI6.1
- **Tickets:** B-1002
- **Kriterien:** AC-02

## Ziel

Die Papierschnitt-Assets Laterne, Blüte und Fackel für das Fest sind angelegt, zugeordnet und geprüft (B-1002/AC-01).

## Kontext

Papierschnitt nach Entscheidung 005: Assets nur über die MCP-Tools `assets_*` (`assets_guide` zuerst lesen), nie Specs von Hand. Kleine Objekte als Vorbild in `public/grafik/objekte/` (z. B. `stern`, `setzling`), Lagerfeuer in `public/grafik/gebaeude-paper/lagerfeuer`. Verwendung (Spec `B-1002`): Sengoku lässt Laternen in den Himmel steigen, am Nil brennen Fackeln am Ufer und Blüten treiben auf dem Wasser; das Spiel vervielfältigt und bewegt die Einzelbilder selbst. Zuordnung in `data/paper.json`. PNG und `manifest.json` sind nicht eingecheckt (B-737); Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742). Diese Session läuft vor der Engine (SI6.3); sie braucht nur die Spec.

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `data/paper.json`
- `data/asset-prompts.json`

## Nicht-Ziele

Nordlicht, Tanz-Animationen, Code im Spiel (SI6.4), Sounds.

## Schritte

1. `assets_guide` und `assets_style_get` lesen, Vorbilder ansehen.
2. `fest-laterne`, `fest-bluete`, `fest-fackel` in Gruppe `objekte` anlegen (klein, auch in Menge im Texturbudget).
3. `data/paper.json`: Zuordnung.
4. `assets_preview`, Fachprüfung durch Agent `k3c-grafik`, schwere Befunde beheben.

## Fertig, wenn

- [ ] AC-02: Die drei Assets existieren, `data/paper.json` ordnet sie zu, `k3c-grafik` ohne schweren Befund.
- [ ] `check_run task:check:diff` grün.

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

–
