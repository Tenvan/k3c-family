# MG6 · PAP · Papierschnitt-Migration: Gebäude mit Ausbaustufen

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** 44f2d30c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Welle 2, Lane `gebaeude`, Domäne PAP. RD1 (Renderer) und MC1 (Asset-MCP-Tools) sind in `feature/pap` (44f2d30c); MG1.1 hat den Weg über MCP mit dem echten Renderer bestätigt. Die alten Gebäude liegen als `tools/buildings`-JSONs mit PNGs unter `public/grafik/gebaeude/` (`gen:gebaeude`, Maße und Stufen in `docs/standards/gebaeude-grafik.md`); sie bleiben bis CU1 liegen. Die Paper-Specs kommen in die eigene Gruppe `public/grafik/gebaeude-paper/` (`<id>.paper.json`, Ausgabe in `<id>/`; D-MG6-01). Muster: `tools/paper/fixtures/specs/mauer.paper.json`.

## Ziel

Als Spec über MCP angelegt und gerendert: alle Gebäude mit Ausbaustufen als Rig `gebaeude`, Stufen als Delta (Materialkette Holz → Stein → Kupfer → Eisen → Kristall), dazu die statischen Gebäude der Wege und der Küste.

Umfang (20 Assets): Mauer, Tor, Turm, Hintergrundmauer, Werkstatt, Taverne, Heilplatz, Schmiede, Rüstkammer, Lager, Kaserne, Lagerfeuer, Burg, Burg-Ausbau, Rekrutierungslager, Farm, Treppe hoch, Treppe runter, Hafen, Schiff.

Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

- Spielende: sehen die Gebäude mit jeder Ausbaustufe edler werden (ab US1).
- Claude (Sonnet): arbeitet nur über die Asset-MCP-Tools.
- Haupt-Session: merged nach `feature/pap`, nummeriert neue Tickets um.
- 🧑 Ralf: Sichtprüfung in PS1.

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review` (mit `checkout`); danach `task gen:paper` für `golden.json` (B-707, D-MG1-02). Maß: 1 Kachel = 32 Einheiten, Ursprung unten mittig, `ausgabe.hoehe_px` = Höhe in Kacheln × 32 (D-MG6-04).

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1); `tools/buildings` und die alten JSONs bleiben bis CU1. Bäume, Fels, Erz, Truhe, Portal, Ausgang, Setzling, Abbau-Symbole und Baugerüste gehören MG8, Hintergründe MG7 (D-MG6-01).

## Regeln und Einschränkungen

Nur `public/grafik/gebaeude-paper/`: neue `*.paper.json`, ihre Ausgabe-Ordner, `index.paper.json` und `golden.json` der Gruppe; die alten JSONs und PNGs nicht anfassen. Kein `assets_style_set`: fehlende Papier-Materialien als Abbildung auf vorhandene (D-MG6-02). Jede Session prüft jedes Gebäude in jeder Stufe: Stufe 1 mit `assets_preview`, Stufen ab 2 über die gerenderte Ausgabe, solange `assets_preview` keine Stufe kennt (D-MG6-03).

## Beispiele

- `assets_set mauer {stufen: [{tausch: {holz: stein}, zusaetze: […Zinnen…]}, {tausch: {stein: fell-fuchs}}, …]}` → gültig, `stand.png`, `stand.s2.png` … `stand.s5.png` entstehen.
- `assets_preview werkstatt` → Bild der Stufe 1 mit Teil-Boxen.

## Ausnahme- und Fehlerfälle

- Zusatz mit unbekanntem `eltern` → `eltern-unbekannt`, Spec bleibt unverändert.
- Kupfer, Eisen, Kristall gibt es nicht als Material → Abbildung auf vorhandene Materialien (D-MG6-02).
- Burg-Stufen 6–13 brauchen Material zurück auf Stein (nicht kumulativ) → eigenes Asset `burg-ausbau` als Überlage (D-MG6-05).

## Akzeptanzkriterien

- **AC-01** Jedes Gebäude hat eine gültige Spec mit allen Stufen (`assets_list` ohne `ungültig`).
- **AC-02** `task gen:paper` erzeugt alle Stufen; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Stufe selbst geprüft (Stufe 1 `assets_preview`, ab Stufe 2 Ausgabe), Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

keine (Defaults als D-MG6-Zeilen in B-701)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG6.1 | `MG6.1-mauerlinie.md` | Umsetzung | autonom | fertig |
| MG6.2 | `MG6.2-hub-staende-a.md` | Umsetzung | autonom | fertig |
| MG6.3 | `MG6.3-hub-staende-b.md` | Umsetzung | autonom | fertig |
| MG6.4 | `MG6.4-burg-und-lager.md` | Umsetzung | autonom | offen |
| MG6.5 | `MG6.5-wirtschaft-und-wege.md` | Umsetzung | autonom | offen |
| MG6.6 | `MG6.6-review.md` | Review | autonom | offen |

Lane `gebaeude`, Umsetzung Sonnet, Review Opus; keine Mensch-Session (Sichtprüfung in PS1). MG6.1 legt Materialkette und Maß fest, danach sind MG6.2–MG6.5 unabhängig.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
