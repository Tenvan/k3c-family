# MG3 · PAP · Papierschnitt-Migration: Truppen

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

Welle 2, Lane `truppen`, Domäne PAP. RD1 (Renderer), MC1 (Asset-MCP-Tools) und RV1 (Review) sind in `feature/pap` (44f2d30c); MG1.1 hat den Weg über MCP mit dem echten Renderer bestätigt. Truppen aus `data/troops.json`: `vagrant` Landstreicher, `peasant` Bauer, `archer` Bogenschütze, `warrior` Krieger, `eliteArcher` Elite-Bogenschütze, `eliteWarrior` Elite-Krieger. Alte Sheets (`data/sprites.json` › `troops`) haben idle, run, attack. Die Gruppe `truppen` gibt es noch nicht.

## Ziel

Als Spec über MCP angelegt und gerendert: alle Truppen als `figur` mit ihren Animationen.

Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

- Spielende: erkennen Truppenart und Elite auf einen Blick (Silhouette, Waffe, Farbe).
- Claude: arbeitet nur über die Asset-MCP-Tools (Umsetzung Sonnet empfohlen, Review Opus).
- Haupt-Session: merged nach `feature/pap`, nummeriert Tickets um.
- 🧑 Ralf: Sichtprüfung in PS1.

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review`.

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1); neue Rig-Typen nur als Ticket an RD1.

## Regeln und Einschränkungen

Nur `public/grafik/truppen/` (Specs, gerenderte PNGs, Manifeste, `index.paper.json`, `golden.json` der Gruppe). Kein `assets_style_set`: fehlende Papier-Materialien als D-Zeile in B-701. Jede Session prüft jedes Asset in jeder Animation mit `assets_preview`. Nach `assets_set` immer `task gen:paper` (B-707, D-MG1-02). Asset-IDs deutsch: `landstreicher`, `bauer`, `bogenschuetze`, `krieger`, `elite-bogenschuetze`, `elite-krieger`.

## Beispiele

- `assets_create {id: bauer, rig: figur, gruppe: truppen}` → Spec mit Pflicht-Teilen, danach `assets_set` mit Lagen und Animationen.
- `assets_preview bauer {animation: angriff, frame: 3}` → Bauer mit nach vorn gestoßener Heugabel.

## Ausnahme- und Fehlerfälle

- `angriff` ohne Keyframes → `animation-unbekannt` (figur kennt ohne Keyframes nur idle und lauf); Spec bleibt unverändert.
- Unbekanntes Material → `material-unbekannt`, nichts geändert.
- Proportion außerhalb 0,286 ± 0,15 → `proportion`, Kopf anpassen.

## Akzeptanzkriterien

- **AC-01** Jedes Asset der Gruppe hat eine gültige Spec (`assets_list {gruppe: truppen}` ohne `ungültig`).
- **AC-02** `task gen:paper` erzeugt alle Animationen der Gruppe; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

keine (Defaults als D-MG3-Zeilen in B-701)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG3.1 | `MG3.1-landvolk-krieger.md` | Umsetzung | autonom | fertig |
| MG3.2 | `MG3.2-bogen-elite.md` | Umsetzung | autonom | fertig |
| MG3.3 | `MG3.3-review.md` | Review | autonom | fertig |

Reife Entwurf, Lane `truppen`; Zuschnitt beim Bereitmachen (je Session 3–6 Assets), dazu ein Review (Opus); keine Mensch-Session (Sichtprüfung in PS1). Umsetzung Sonnet.

## Abnahme

- 2026-11-07, autonom (Claude): AC-01 bis AC-03 belegt (Ergebnisse MG3.1–MG3.3); sechs Truppen unter `public/grafik/truppen/`, nur über die Asset-MCP-Tools geschrieben.
- Behoben aus dem Review (Opus): Bogen-Angriff mit Auszug und Pfeil, Elite-Bogenschütze mit Feder und goldenem Bogen, Heugabel aus dem Gesicht.
- Defaults und Kleinbefunde: D-MG3-01 bis D-MG3-08 in B-701; keine neuen Tickets.
- Offen: Sichtprüfung durch 🧑 Ralf in PS1.
