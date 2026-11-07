# MG1 · PAP · Papierschnitt-Migration: Monarch und Spielerfarben

- **Status:** erledigt
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** 44f2d30c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Welle 2, Lane `monarch`, Domäne PAP. RD1 (Renderer) und MC1 (Asset-MCP-Tools) sind in `feature/pap` (44f2d30c). Der Monarch liegt seit RD1 als Spec mit Ausgabe unter `public/grafik/figuren/` (Animationen idle, lauf, schritt, trab, galopp; Reiten mit `reittiere/pferd`). Spieler-Sheets heute aus `data/sprites.json` › `players`: P1 König, P2 König-2, P3 König hellblau, P4 König-2 rosa. Die Gruppe `figuren` hat nur ein Asset (Monarch); die Spielerfarben werden Varianten, keine eigenen Assets.

## Ziel

Als Spec über MCP angelegt und gerendert: Monarch als `figur` mit allen Animationen und Reiten; Spielerfarben als Varianten `spieler1` bis `spieler4`.

Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

- Spielende: erkennen ihren Monarchen an der Farbe (vier Spieler).
- Claude (Sonnet): arbeitet nur über die Asset-MCP-Tools; MG1.1 testet die Werkzeuge für MG2–MG8.
- Haupt-Session: entscheidet nach MG1.1 über den Start von MG2–MG8, merged nach `feature/pap`.
- 🧑 Ralf: Sichtprüfung in PS1.

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review`.

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1); Änderungen am Renderer oder neue Rig-Typen nur als Ticket an RD1.

## Regeln und Einschränkungen

Nur `public/grafik/figuren/` (Specs, gerenderte PNGs, Manifeste, `index.paper.json`, `golden.json` der Gruppe). Kein `assets_style_set`: fehlende Papier-Materialien als Ticket für die Lane `stil` bzw. Default in B-701. Jede Session prüft jedes Asset in jeder Animation mit `assets_preview`, Befunde im Ergebnis.

## Beispiele

- `assets_set monarch {varianten: {spieler2: {tausch: {koenigsblau: koenigsrot}}}}` → gültig, `idle@spieler2.png` … entstehen; Golden nach `task gen:paper` (B-707).
- `assets_preview monarch {animation: galopp, frame: 3, variante: spieler3}` → Bild mit Reiter auf dem Pferd in den Farben von Spieler 3.

## Ausnahme- und Fehlerfälle

- Patch mit unbekanntem Material → `assets_set` lehnt ab (`material-unbekannt`), Spec bleibt unverändert.
- Pferd fehlt im Nachbar-Ordner → `reittier-unbekannt`, nichts wird geschrieben.
- Getönte Farben (hellblau, rosa) gibt es nicht als Material → Abbildung auf vorhandene Materialien (D-Zeile), Ticket an Lane `stil` nur bei Bedarf.

## Akzeptanzkriterien

- **AC-01** Jedes Asset der Gruppe hat eine gültige Spec (`assets_list` ohne `ungültig`).
- **AC-02** `task gen:paper` erzeugt alle Animationen der Gruppe; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

keine (Defaults als D-MG1-Zeilen in B-701)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG1.1 | `MG1.1-integration-monarch-mcp.md` | Umsetzung | autonom | fertig |
| MG1.2 | `MG1.2-spielerfarben-varianten.md` | Umsetzung | autonom | fertig |
| MG1.3 | `MG1.3-review.md` | Review | autonom | fertig |

Lane `monarch`, Umsetzung Sonnet, Review Opus; keine Mensch-Session (Sichtprüfung in PS1). Nur ein Asset in der Gruppe, daher statt 3–6 Assets je Session:

- **MG1.1 Integration:** Monarch durchgängig über MCP (alle Animationen, Reiten), Befunde als Tickets im Projekt PAP.
- **MG1.2 Spielerfarben:** Varianten `spieler1`–`spieler4`.
- **MG1.3 Review** (Opus).

## Abnahme

2026-11-06, Review MG1.3 (Opus): AC-01–AC-03 erfüllt (MG1.1, MG1.2, MG1.3 › Ergebnis).
Behoben: Doku-Lücken aus dem Review; Vorgabe Ralf für den Umhang (D-MG1-05) umgesetzt.
Neue Tickets: B-707–B-711 (Projekt PAP). Offen: Sichtprüfung der Spielerfarben durch 🧑 in PS1.
