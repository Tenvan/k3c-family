# SI5.3 · Assets Hütte, Dojo, Schrein, Schwertschmied, Priesterin (B-1001)

- **Status:** blockiert
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si5
- **Abhängig von:** SI5.2
- **Tickets:** B-1001
- **Kriterien:** AC-03

## Ziel

Die Papierschnitt-Assets Hütte, Dojo, Schrein, Schwertschmied und Priesterin sind angelegt, zugeordnet und geprüft (B-1001/AC-01).

## Kontext

Papierschnitt nach Entscheidung 005: Assets nur über die MCP-Tools `assets_*` (`assets_guide` zuerst lesen), nie Specs von Hand. Hub-Gebäude als Vorbild in `public/grafik/gebaeude-paper/` (z. B. `schmiede`, `taverne`, `heilplatz`; Gerüste `geruest-*`), Figuren in `public/grafik/figuren/` (Monarch) und `public/grafik/truppen/` (mit `assets_list` ansehen, Bürger-Figuren als Vorbild für Größe und Posen Stehen und Gehen). Zuordnung in `data/paper.json`; die Schlüssel der Engine (`dojo`, `shrine`, `swordsmith`, `priestess`, Hütte) liefert SI5.2 (Ergebnis dort lesen). PNG und `manifest.json` sind nicht eingecheckt (B-737); Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742).

## Erlaubte Dateien

- `public/grafik/gebaeude-paper/**`
- `public/grafik/figuren/**`
- `data/paper.json`
- `data/asset-prompts.json`

## Nicht-Ziele

Walfänger und Tranhütte, Code im Spiel (SI5.4), Sounds.

## Schritte

1. `assets_guide` und `assets_style_get` lesen, Vorbilder ansehen.
2. `einsiedler-huette`, `dojo`, `schrein` in Gruppe `gebaeude-paper` anlegen (Bauwerke in der Größe eines Hub-Gebäudes, Hütte kleiner).
3. `schwertschmied` und `priesterin` in Gruppe `figuren` anlegen (Posen Stehen und Gehen).
4. `data/paper.json`: Zuordnung zu den Schlüsseln der Engine.
5. `assets_preview`, Fachprüfung durch Agent `k3c-grafik`, schwere Befunde beheben.

## Fertig, wenn

- [ ] AC-03: Alle fünf Assets existieren, `data/paper.json` ordnet sie zu, `k3c-grafik` ohne schweren Befund.
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

Pausiert (Ralf, 2026-11-17): Welt-Assets zum Schluss.

**Fertig (eingecheckt, Commit 22c9c7f66):**
- `gebaeude-paper/einsiedler-huette`, `dojo`, `schrein` (Rig gebaeude, `stand`), `figuren/schwertschmied`, `figuren/priesterin` (Rig figur, vom Bauer abgeleitet, `idle`/`lauf`), alle gültig.
- `data/paper.json` Block `einsiedler` (Schlüssel `swordsmith`, `priestess`, `dojo`, `shrine`, `huette`), nur vollständige Assets.
- Geprüft: assets_preview, `k3c-grafik` ohne schweren Befund, `check_run task:check:diff` grün.

**Fehlt beim Fortsetzen:**
- Engine-Schlüssel: SI5.2 verworfen, die Schlüssel sind nach Spec B-972 gewählt und gegen die Engine abzugleichen; das Spiel liest den Block noch nicht (SI5.4).
- Maßstab Hütte (90 px) neben Figuren (~130 px) vermutlich zu klein, Augenschein im Spiel.
- Gen/Golden im Abnahme-Ticket.
