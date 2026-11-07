# MG7 · PAP · Papierschnitt-Migration: Hintergründe je Biom

- **Status:** erledigt
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** e3c95f7f
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Welle 2, Lane `hintergruende`, Domäne PAP. Biome aus `data/biomes/*.json` (forest, cave, mine, ironhold, crystal). Heute liegen die Hintergründe als Pixel-Bausteine in `public/grafik/gebaeude/hg-<biom>-<fern|mitte|nah>[-v<n>].json` (24 × 15 Kacheln, 768 × 480 px in Dichte 32, Biom-Namen wald, hoehle, mine, eisenstollen, kristall; Schema in `tools/buildings/hires.go`). Motive je Biom: Tannenhang, Tropfsteinhalle, Schachtwand mit Gerüst, Schmiedehalle mit Glut, Kristallhalle.

## Ziel

Als Spec über MCP angelegt und gerendert: Hintergründe je Biom als Rig `hintergrund` mit den Ebenen fern, mitte und nah, je zwei Varianten (D-MG7-01), in der neuen Gruppe `public/grafik/hintergruende/`.

Am Ende sichtbar: 30 Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

- Spieler: sehen später (US1) Papierschnitt-Hintergründe mit Parallax je Biom.
- Claude: legt die Specs an, prüft jede Ebene und Variante per Preview.
- Opus-Reviewer: getrennte Prüfung in MG7.3.
- 🧑 Ralf: Sichtprüfung in PS1, entscheidet die D-Zeilen D-MG7-xx.

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review`.

- IDs `hg-<biom>-<ebene>-v<n>` wie heute (D-MG7-02), Rahmen 768 × 480 Spec-Einheiten, `hoehe_px` 480.
- Ebenen sind bis auf ihre Motive transparent; fern ganz in Ton `schatten`, mitte Grundton mit Schatten-Auflagen, nah kräftig mit Licht- und Schattenkanten (D-MG7-04).
- Profile am linken und rechten Rand gleich hoch, Einzelmotive mit Abstand zum Rand, damit die Ebenen waagrecht kacheln (D-MG7-03).

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1); Parallax-Logik, Himmel und Abdunkelung je Ebene bleiben im Client. Keine neuen Materialien, keine Rig-Erweiterung.

## Regeln und Einschränkungen

Nur `public/grafik/hintergruende/`. Kein `assets_style_set`: fehlende Papier-Materialien oder Rig-Fähigkeiten als Default in B-701 (D-MG7-xx), keine neuen Tickets. Erlaubt sind auch `index.paper.json` und `golden.json` der Gruppe. Nach `task gen:paper` fremde Gruppen zurücksetzen. Jede Session prüft jede Ebene und Variante mit `assets_preview`.

## Beispiele

- `hg-wald-fern-v1`: zwei Hügelketten mit Tannen-Silhouetten, ganz in `smaragd` Ton `schatten`, oben transparent.
- `hg-mine-mitte-v2`: Stützgestelle aus `holz`, Laternen in `gold`, Erzadern als Falz.
- Zwei Kopien von `hg-hoehle-fern-v1` nebeneinander ergeben ein durchgehendes Profil ohne Sprung.

## Ausnahme- und Fehlerfälle

- Ein Motiv ragt über den Rahmen: Rahmen bleibt 768 × 480, das Motiv wird verschoben.
- Ein Biom braucht einen Ton, den kein Material hat (Lava, Leuchten): nächstes vorhandenes Material, Default als D-Zeile.
- Ungültige Spec: `assets_set` lehnt ab, nichts ändert sich; Patch korrigieren.

## Akzeptanzkriterien

- **AC-01** Jede Ebene und Variante hat eine gültige Spec.
- **AC-02** `task gen:paper` erzeugt alle Ebenen; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

Defaults in B-701 (D-MG7-01 bis D-MG7-04), Entscheidung durch 🧑 in PS1.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG7.1 | `MG7.1-wald-hoehle.md` | Umsetzung | autonom | fertig |
| MG7.2 | `MG7.2-mine-eisenstollen-kristall.md` | Umsetzung | autonom | fertig |
| MG7.3 | `MG7.3-review.md` | Review | autonom | fertig |

Lane `hintergruende`, nacheinander; Umsetzung Sonnet, Review Opus; keine Mensch-Session (Sichtprüfung in PS1).

- **MG7.1:** `hg-wald-*` und `hg-hoehle-*`, je fern, mitte, nah in v1 und v2 (12 Assets).
- **MG7.2:** `hg-mine-*`, `hg-eisenstollen-*`, `hg-kristall-*` (18 Assets).
- **MG7.3 Review** (Opus).

## Abnahme

- 2026-11-07, Review MG7.3 (Opus-Reviewer, Claude): AC-01 bis AC-03 erfüllt (Nachweise MG7.1–MG7.3); Sichtprüfung durch 🧑 in PS1 offen.
- Behoben: Kerben an Streben (Mine, Eisenstollen), Schatten der Höhlen-Blöcke, Löcher am Säulenfuß (Höhle mitte).
- Offen als D-MG7-01 bis D-MG7-06 in B-701. Keine neuen Tickets.
- `task check:diff` lief in die Zeitgrenze (Workbench-Go-Tests des feature/pap-Stands); `task test` 3173/3173 und `task test -- paper` 187/187 grün.
