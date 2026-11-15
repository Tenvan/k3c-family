# NA1.4 · Xbox-Tasten und Tastenkappen als Papierschnitt-Icons

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/na1
- **Abhängig von:** NA1.3
- **Tickets:** B-911
- **Kriterien:** AC-05

## Ziel

In der Gruppe `icons` liegen die Papierschnitt-Icons der Xbox-Tasten und zwei leere Tastenkappen, eingetragen in `data/paper.json` (B-911, Teil Grafik).

## Kontext

Papierschnitt nach Entscheidung 005: Assets nur über die MCP-Tools `assets_*` (`assets_guide` zuerst lesen, dann `assets_create`, `assets_set`, `assets_render`, `assets_preview`, `assets_review`); Standards `docs/standards/paper-spec.md`, `paper-mcp.md`. Vorbilder: `public/grafik/icons/icon-muenze.paper.json`, `icon-krone.paper.json`; Eintrag in `data/paper.json` wie `"icon-eisen": { "gruppe": "icons", "frame": [32, 32], "frames": 1, "rahmen": [-16, -32, 32, 32] }`. PNG und `manifest.json` sind nicht eingecheckt (B-737); eingecheckt werden Specs, `index.paper.json`, `golden.json`.

Gebraucht (B-911 › Anforderungen): `xbox-a`, `xbox-x`, `xbox-y`, `xbox-lb`, `xbox-rb`, `xbox-lt`, `xbox-rt`, `xbox-rs`, `xbox-up`, `xbox-down`, `xbox-menu` (Namen wie `native/k3c-game/hud/glyphs.go` › `IconAsset`: `pad:A` → `icons/xbox-a`) und `taste-kappe`, `taste-kappe-breit` (leer, die Beschriftung setzt das HUD). Farben vorläufig: A grün, X blau, Y gelb, Rest hell, in Papiertönen des Stils (`public/grafik/stil.paper.json`). Diese Session ist das Feature: `task gen:paper` und das Grafik-Golden der Gruppe laufen hier.

## Erlaubte Dateien

- `public/grafik/icons/**`
- `data/paper.json`

## Nicht-Ziele

HUD-Code (NA1.5); je Taste ein eigenes Icon mit Beschriftung; andere Gruppen; Stil-Datei ändern.

## Schritte

1. `assets_guide` und zwei bestehende Icon-Specs lesen.
2. Die 13 Icons mit `assets_create`/`assets_set` anlegen, je mit `assets_preview` ansehen.
3. Einträge in `data/paper.json` ergänzen.
4. `task gen:paper`, Golden der Gruppe `icons` aktualisieren (Begründung in den Commit), `task test -- paper`.
5. Agent `k3c-grafik` prüfen lassen, Befunde beheben; `task check:diff`.

## Fertig, wenn

- [ ] AC-05 (Teil Grafik): `assets_list` zeigt die 13 Icons in der Gruppe `icons`; jedes steht in `data/paper.json` (B-911/AC-01).
- [ ] `task test -- paper` und `task check:diff` grün; `k3c-grafik` ohne schweren Befund.

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
