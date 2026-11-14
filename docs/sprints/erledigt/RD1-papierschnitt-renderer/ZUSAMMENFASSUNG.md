# RD1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Papierschnitt-Renderer: Rig-Typen, IK, Physik, resvg-Export, Golden-Tests
- **Projekt:** PAP

## Ziel

`bun tools/paper/bin/paper.ts` erfüllt den CLI-Vertrag vollständig (`validate`, `render`, `frame`, `diff`, `pixeldiff`) und rendert Monarch und Pferd aus ihren Specs wie der Prototyp; `task gen:paper` baut alle Gruppen unter `public/grafik/`. Am Ende sichtbar: `public/grafik/figuren/monarch/galopp.png` aus der Spec, Golden-Tests grün in der CI.

## Ergebnis

2026-11-06 (RD1.5, Claude): AC-01–AC-05 erfüllt, Nachweise in RD1.1–RD1.4 (Golden auf Linux bestätigt, CI-Run 42279806963 grün). Sprint-Review (Opus): drei wichtige Befunde behoben (Golden-Regel im Vergleich, `validate` mit Render-Regeln, Aufräumen nur toter Prozesse); leichte Befunde als offene Punkte im Ergebnis von RD1.5. Entscheidungen: B-701 D-RD1-01 bis D-RD1-08; Vertrags-Nachträge nötig für Reit-Rahmen, renderKey beim Reiten, Saum, neue CLI-Regeln. Offen für 🧑: Sichtvergleich mit dem Prototyp (Monarch idle/lauf/galopp, Pferd) beurteilen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | `paper validate` erfüllt alle Regeln aus `paper-spec.md` (005/AC-01). |
| AC-02 | erfüllt | Golden-Tests: Spec → PNG-Hash fest für Monarch (5 Animationen), Pferd (3 Gangarten), Mauer … |
| AC-03 | erfüllt | Monarch rendert Idle, Lauf, Schritt, Trab, Galopp mit IK-Hufen, Rückengelenk und Umhang-Ph … |
| AC-04 | erfüllt | Materialänderung in `stil.paper.json` ändert die Hashes aller betroffenen Assets ohne Spec … |
| AC-05 | erfüllt | `frame`, `diff`, `pixeldiff` liefern die Antworten aus dem Vertrag; die Attrappe aus PV1.4 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
