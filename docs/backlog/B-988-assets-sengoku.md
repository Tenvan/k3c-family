# B-988 · Sengoku bekommt die Assets Torii, Pagode, Ninja, Samurai und Samurai-Lager

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE7
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sengoku (WE3) nutzt nur vorhandene Assets: Tannen, Standard-Portal, Wolf, Goblin. Papierschnitt-Assets entstehen aus Specs unter `public/grafik/<gruppe>/` und nur über die MCP-Tools `assets_*` (Entscheidung 005).

## Ziel

Sengoku hat eigene Bilder: Torii, Pagode, Ninja, Samurai und Samurai-Lager liegen als Papierschnitt-Assets vor und sind als Skins nutzbar.

## Beteiligte und Zielgruppen

Spieler sehen die Welt. Claude legt die Assets über `assets_*` an, Agent `k3c-grafik` prüft, 🧑 nimmt im Bereich Assets der Workbench ab.

## Anforderungen

- Assets: `objekte/torii` (Ersatz für `objekte/ausgang`), `gebaeude-paper/pagode` (Ersatz für den Turm), `gegner/ninja` (Basis `gegner/wolf`), `gegner/samurai` (Basis `gegner/hoehlentroll`), `objekte/samurai-lager` (Ersatz für `objekte/portal`).
- Jedes Skin-Asset hat dieselben Zustände, Animationen, Ankerpunkte und etwa dieselbe Größe wie sein Basis-Asset, damit das Spiel es ohne Code tauscht (B-994).
- Stil nach `assets_guide` und `assets_style_get`; Texturbudget eingehalten.
- Angelegt und geändert nur über `assets_create`, `assets_set`, `assets_render`; Specs und `index.paper.json` sind eingecheckt, PNG nicht (B-737).

## Nicht-Ziele

Zuordnung in der Welt (B-996), Grafik-Golden und `gen:paper` (Abnahme-Ticket des Sprints), neue Animationen.

## Regeln und Einschränkungen

Entscheidung 005. Gen-Läufe nur im Abnahme-Ticket (B-742).

- **Bereich:** `public/grafik/objekte/**`, `public/grafik/gegner/**`, `public/grafik/gebaeude-paper/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

`assets_get gegner/ninja` zeigt dieselben Animationen wie `gegner/wolf`. `assets_preview objekte/torii` zeigt ein rotes Tor in Papierschnitt-Optik.

## Ausnahme- und Fehlerfälle

Basis-Asset hat eine Animation, die die Variante nicht sinnvoll füllt → Animation trotzdem anlegen (kann die Basis-Bewegung übernehmen), nie weglassen. Texturbudget überschritten → Asset verkleinern, nicht das Budget ändern.

## Akzeptanzkriterien

- **AC-01** `assets_list` zeigt die fünf Assets; Specs liegen unter `public/grafik/`.
- **AC-02** Je Skin-Asset: Zustände und Animationen deckungsgleich mit dem Basis-Asset (Vergleich über `assets_get`, im Ergebnis der Session notiert).
- **AC-03** Agent `k3c-grafik` ohne schwere Befunde.
- **AC-04** Sichtprüfung 🧑 im Bereich Assets (Review).

## Offene Fragen

- Basis je Asset → Default: Torii = Ausgang, Pagode = Turm, Ninja = Wolf, Samurai = Höhlentroll, Samurai-Lager = Portal (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-960 (Assets und Gegner-Varianten Sengoku).
