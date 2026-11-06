# MG5 · PAP · Papierschnitt-Migration: Bosse

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** a4f66f28
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Welle 2, Lane `bosse`, Domäne PAP. RD1 und MC1 sind in `feature/pap`; MG2 (Reittiere) und MG3 (Truppen) sind gemergt. Bosse aus `data/bosses.json`: fünf Minibosse (goblinLeader, trollKing, ratQueen, lavaGolem, shardTitan) und ein Endboss (crystalHeartWarden, steht still). Die Gruppe `public/grafik/bosse/` gibt es noch nicht.

## Ziel

Als Spec über MCP angelegt und gerendert: alle Bosse mit größerer `hoehe_px`.

Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

- Spielende: erkennen Bosse an Größe und Silhouette.
- Claude: arbeitet nur über die Asset-MCP-Tools.
- 🧑 Ralf: Sichtprüfung in PS1.

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review`.

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1); fehlende Rig-Typen nur als Ticket an RD1.

## Regeln und Einschränkungen

Nur `public/grafik/bosse/`. Kein `assets_style_set`: fehlende Papier-Materialien als Ticket für die Lane `stil` bzw. Default in B-701; erlaubt sind auch `index.paper.json` und `golden.json` der Gruppe. Jede Session prüft jedes Asset in jeder Animation mit `assets_preview`.

## Beispiele

- `assets_create {id: troll-koenig, rig: figur, gruppe: bosse}` + `assets_set` mit Krieger-Teilen, Haut `stein`, Krone `gold` → gültig, `idle.png`, `lauf.png`, `angriff.png` in 288 px Höhe.
- `assets_preview kristallherz-waechter {animation: idle, frame: 4}` → Herz mit ausgeschwungenen Splittern.

## Ausnahme- und Fehlerfälle

- Fehlende Farbe → vorhandenes Material (D-MG5-02), kein `assets_style_set`.
- Form passt nicht zum Rig → bestmögliche Annäherung, D-Zeile an RD1.
- `assets_set` mit Fehler → abgelehnt, nichts geändert.

## Akzeptanzkriterien

- **AC-01** Jedes Asset der Gruppe hat eine gültige Spec.
- **AC-02** `task gen:paper` erzeugt alle Animationen der Gruppe; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

keine (Defaults als D-MG5-Zeilen in B-701)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG5.1 | `MG5.1-minibosse-humanoid.md` | Umsetzung | autonom | fertig |
| MG5.2 | `MG5.2-golems-kristall.md` | Umsetzung | autonom | fertig |
| MG5.3 | `MG5.3-review.md` | Review | autonom | fertig |

Lane `bosse`, nacheinander; Umsetzung Sonnet, Review Opus; keine Mensch-Session (Sichtprüfung in PS1).

- **MG5.1:** goblin-anfuehrer, troll-koenig, ratten-koenigin (Rig `figur`).
- **MG5.2:** lava-golem, splitter-titan (`figur`), kristallherz-waechter (`requisite`).
- **MG5.3 Review** (Opus).

## Abnahme

- 2026-11-07, Review MG5.3 (Opus-Reviewer, Claude): AC-01, AC-02, AC-03 erfüllt (Nachweise in MG5.1–MG5.3); Sichtprüfung durch 🧑 in PS1 offen.
- Behoben: Sockel des kristallherz-waechter kippte im angriff mit; Beschreibung troll-koenig (Holzschwert).
- Offen als D-MG5-01 bis D-MG5-04 in B-701 (Größe, Farben, Rig der Golems, Lesbarkeit). Keine neuen Tickets.
- `task check:diff`: nur Workbench-Frontend-Wackel-Tests unter Last rot, einzeln 432/432 grün (D-MG2-07).
