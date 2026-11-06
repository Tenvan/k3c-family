# MG2 · PAP · Papierschnitt-Migration: Reittiere

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

Welle 2, Lane `reittiere`, Domäne PAP. RD1 (Renderer) und MC1 (Asset-MCP-Tools) sind in `feature/pap`; MG1.1 hat den Weg über MCP mit dem echten Renderer bestätigt. In `public/grafik/reittiere/` liegt seit RD1 nur `pferd` (Reittier des Monarchen). Reittiere heute aus `data/sprites.json` › `mounts`: horse/horseWalk (pferd), blackHorse (rappe), whiteHorse (schimmel), unicorn (einhorn), pegasus, elephant (elefant), stag (hirsch), wolf, whiteWolf (weisser-wolf), hellhound/hellHound (hoellenhund), nightmare (albtraum) – elf Assets.

## Ziel

Als Spec über MCP angelegt und gerendert: alle Reittiere als `vierbeiner` mit Schritt, Trab, Galopp.

Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

- Spielende: reiten sichtbar unterschiedliche Tiere.
- Claude: arbeitet nur über die Asset-MCP-Tools.
- MG1 (parallel): besitzt den Monarchen, `pferd` wird abgestimmt.
- 🧑 Ralf: Sichtprüfung in PS1.

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review`.

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1); neue Rig-Typen nur als Ticket an RD1.

## Regeln und Einschränkungen

Nur `public/grafik/reittiere/`. Kein `assets_style_set`: fehlende Papier-Materialien als Ticket für die Lane `stil` bzw. Default in B-701; erlaubt sind auch `index.paper.json` und `golden.json` der Gruppe. Jede Session prüft jedes Asset in jeder Animation mit `assets_preview`.

## Beispiele

- `assets_create {id: rappe, rig: vierbeiner, gruppe: reittiere}` + `assets_set` mit Teilen des Pferds und Körper `maehne` → gültig, `schritt.png`, `trab.png`, `galopp.png`.
- `assets_preview einhorn {animation: galopp, frame: 4}` → Horn bewegt sich mit dem Kopf.

## Ausnahme- und Fehlerfälle

- Fehlende Farbe (schwarz, grau) → Abbildung auf vorhandene Materialien (D-MG2-01), kein `assets_style_set`.
- Tierform passt nicht zum Rig `vierbeiner` → bestmögliche Annäherung, Befund als D-Zeile an RD1.
- `assets_set` mit Fehler → abgelehnt, nichts geändert.

## Akzeptanzkriterien

- **AC-01** Jedes Asset der Gruppe hat eine gültige Spec.
- **AC-02** `task gen:paper` erzeugt alle Animationen der Gruppe; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

keine (Defaults als D-MG2-Zeilen in B-701)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG2.1 | `MG2.1-pferde.md` | Umsetzung | autonom | fertig |
| MG2.2 | `MG2.2-fabelwesen-hirsch.md` | Umsetzung | autonom | offen |
| MG2.3 | `MG2.3-woelfe-elefant.md` | Umsetzung | autonom | offen |
| MG2.4 | `MG2.4-review.md` | Review | autonom | offen |

Lane `reittiere`, nacheinander; Umsetzung Sonnet, Review Opus; keine Mensch-Session (Sichtprüfung in PS1).

- **MG2.1 Pferde:** pferd prüfen, rappe, schimmel, albtraum.
- **MG2.2 Fabelwesen und Hirsch:** einhorn, pegasus, hirsch.
- **MG2.3 Wölfe und Elefant:** wolf, weisser-wolf, hoellenhund, elefant.
- **MG2.4 Review** (Opus).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
