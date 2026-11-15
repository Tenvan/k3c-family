# SI3.3 · Asset Wolkenbank für die Vorboten (B-999)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si3
- **Abhängig von:** SI3.2
- **Tickets:** B-999
- **Kriterien:** AC-03

## Ziel

Das Papierschnitt-Asset der Wolkenbank für die Vorboten ist angelegt, zugeordnet und geprüft (B-999/AC-01).

## Kontext

Papierschnitt nach Entscheidung 005: Assets nur über die MCP-Tools `assets_*` (`assets_guide` zuerst lesen), nie Specs von Hand. Hintergründe liegen in `public/grafik/hintergruende/` (Ebenen fern, mitte, nah; mit `assets_list` und `assets_get` ansehen), Zuordnung in `data/paper.json`. Die Wolkenbank ist eine dunkle, breite Himmels-Ebene, die das Spiel am Vortag des Taifuns einblendet (Spec `B-999`); Regen, Wind und Wasserstand zeichnet das Spiel selbst. PNG und `manifest.json` sind nicht eingecheckt (B-737); Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742).

## Erlaubte Dateien

- `public/grafik/hintergruende/**`
- `data/paper.json`
- `data/asset-prompts.json`

## Nicht-Ziele

Code im Spiel (SI3.4), Schneesturm-Grafik, Sounds.

## Schritte

1. `assets_guide` und `assets_style_get` lesen, eine Himmels- oder Fern-Ebene als Vorbild ansehen.
2. `wetter-wolkenbank` in Gruppe `hintergruende` anlegen (kachelbar in der Breite, Texturbudget wie die Fern-Ebenen).
3. `data/paper.json`: Zuordnung.
4. `assets_preview`, Fachprüfung durch Agent `k3c-grafik`, schwere Befunde beheben.

## Fertig, wenn

- [ ] AC-03: Das Asset existiert, `data/paper.json` ordnet es zu, `k3c-grafik` ohne schweren Befund.
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
