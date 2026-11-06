# MG4 · PAP · Papierschnitt-Migration: Gegner

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** e09e8f5b
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Welle 2, Lane `gegner`, Domäne PAP. RD1, MC1, RV1 und MG3 sind in `feature/pap` (e09e8f5b). 17 Gegner aus `data/enemies.json`: greed, wolf, alphaWolf, goblin, goblinArcher, skeleton, bat, caveTroll, zombie, ratSwarm, mineGhost, lavaSlime, ironBeetle, fireSpirit, crystalSpider, shardling, crystalGuardian. Die alten Sheets (`data/sprites.json` › `enemies`) sind teils Platzhalter (z. B. Fledermaus = Feuerschädel, Rattenschwarm = Pilz); gezeichnet wird, was der Name sagt. Die Gruppe `gegner` gibt es noch nicht.

## Ziel

Als Spec über MCP angelegt und gerendert: alle Gegner (Rig-Typ je Gegner `figur`, `vierbeiner` oder `requisite`).

Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

- Spielende: erkennen Gegnerart und Gefahr auf einen Blick.
- Claude: arbeitet nur über die Asset-MCP-Tools (Umsetzung Sonnet empfohlen, Review Opus).
- Haupt-Session: merged nach `feature/pap`.
- 🧑 Ralf: Sichtprüfung in PS1.

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review`.

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1); fehlende Rig-Typen (z. B. Spinne, Golem) nur als Ticket an RD1.

## Regeln und Einschränkungen

Nur `public/grafik/gegner/` (Specs, PNGs, Manifeste, `index.paper.json`, `golden.json`). Kein `assets_style_set`; fehlende Materialien als D-Zeile. Jede Session prüft jedes Asset in jeder Animation mit `assets_preview`. Nach Änderungen `task gen:paper`, fremde Gruppen zurücksetzen. Rig je Gegner: `figur` (Goblins, Gier, Skelett, Zombie, Troll, Kristallwächter), `vierbeiner` (Wolf, Alphawolf), `requisite` mit Keyframes (übrige). Asset-IDs deutsch.

## Beispiele

- `assets_create {id: wolf, rig: vierbeiner, gruppe: gegner}` → Vorlage mit Beinteilen, danach `assets_set` mit Kopf, Schweif, `bein` und Gangarten.
- `assets_preview fledermaus {animation: angriff, frame: 3}` → Fledermaus im Sturzflug.

## Ausnahme- und Fehlerfälle

- Vierbeiner-Animation ohne `gangart` → `animation-unbekannt`; Vierbeiner bewegen sich nur über Gangarten.
- `requisite`/`figur` ohne Keyframes für `angriff` → `animation-unbekannt`.
- Unbekanntes Material → `material-unbekannt`, nichts geändert.

## Akzeptanzkriterien

- **AC-01** Jedes Asset der Gruppe hat eine gültige Spec (`assets_list {gruppe: gegner}` ohne `ungültig`).
- **AC-02** `task gen:paper` erzeugt alle Animationen der Gruppe; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

keine (Defaults als D-MG4-Zeilen in B-701)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG4.1 | `MG4.1-goblins-untote.md` | Umsetzung | autonom | fertig |
| MG4.2 | `MG4.2-grosse-woelfe.md` | Umsetzung | autonom | fertig |
| MG4.3 | `MG4.3-kreaturen-a.md` | Umsetzung | autonom | offen |
| MG4.4 | `MG4.4-kreaturen-b.md` | Umsetzung | autonom | offen |
| MG4.5 | `MG4.5-review.md` | Review | autonom | offen |

Reife Entwurf, Lane `gegner`; Zuschnitt beim Bereitmachen (je Session 3–6 Assets), dazu ein Review (Opus); keine Mensch-Session (Sichtprüfung in PS1). Umsetzung Sonnet.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
