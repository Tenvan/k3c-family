# SI1.3 · Assets Ladeplätze, Beutel, Kiste, Laderaum (B-997)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si1
- **Abhängig von:** SI1.2
- **Tickets:** B-997
- **Kriterien:** AC-03

## Ziel

Die Papierschnitt-Assets für Ladeplätze, Beutel, Kiste und die Laderaum-Stufe des Schiffs sind angelegt, zugeordnet und geprüft (B-997/AC-01).

## Kontext

Papierschnitt nach Entscheidung 005: Assets nur über die MCP-Tools `assets_*` (`assets_guide` zuerst lesen, dann `assets_create`, `assets_set`, `assets_render`, `assets_preview`), nie Specs von Hand. Vorhandene Vorbilder: Schilder `public/grafik/objekte/schild-*`, Truhe `objekte/truhe`, Hafen und Schiff `public/grafik/gebaeude-paper/hafen`, `schiff` (drei Bau-Stufen). Zuordnung Spiel ↔ Asset in `data/paper.json`. PNG und `manifest.json` sind nicht eingecheckt (B-737); eingecheckt werden Specs, `index.paper.json` und `golden.json`. Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742). Namen der Zustände für die Zuordnung liefert SI1.2 (Ergebnis dort lesen).

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `public/grafik/gebaeude-paper/**`
- `data/paper.json`
- `data/asset-prompts.json`

## Nicht-Ziele

Code im Spiel (SI1.4), neue Stil-Regeln, Sounds.

## Schritte

1. `assets_guide` und `assets_style_get` lesen, Vorbilder mit `assets_get` ansehen.
2. Anlegen: `ladeplatz-muenzen`, `ladeplatz-truppen`, `ladeplatz-material` (Pfahl mit Schild), `fracht-beutel`, `fracht-kiste` in Gruppe `objekte`.
3. Schiff um die Stufe Laderaum ergänzen (`gebaeude-paper/schiff`), Deck mit erkennbaren Plätzen.
4. `data/paper.json`: Zuordnung der neuen Assets.
5. `assets_preview` je Asset prüfen, Agent `k3c-grafik` als Fachprüfung, schwere Befunde beheben.

## Fertig, wenn

- [ ] AC-03: Alle sechs Assets existieren, `data/paper.json` ordnet sie zu, `k3c-grafik` ohne schweren Befund.
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
