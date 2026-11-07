# MG8 · PAP · Papierschnitt-Migration: Natur, Objekte, Münzen, Icons

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** 094be01c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Welle 2, Lane `objekte`, Domäne PAP. RD1, MC1 und MG2–MG7 sind in `feature/pap` (094be01c). Liste aus `docs/assets/zuordnung-welt.md` (Natur, Truhen/Portale/Münzen, Abbau-Schilder, Icons, Materialien). Die Gruppen `objekte` und `icons` gibt es noch nicht. Gerüste (`site:scaffold:*`) gehören zu den Gebäuden und bleiben draußen.

## Ziel

Als Spec über MCP angelegt und gerendert: Natur und Objekte (Bäume, Fels, Erz, Truhe, Portal, Bauschilder), Münzen, Item- und HUD-Icons als `requisite`.

Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

Claude setzt um und prüft (autonom); 🧑 sieht in PS1 die Grafiken an.

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review`.

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1).

## Regeln und Einschränkungen

Nur `public/grafik/objekte/` und `public/grafik/icons/`. Kein `assets_style_set`: fehlende Papier-Materialien als Ticket für die Lane `stil` bzw. Default in B-701; erlaubt sind auch `index.paper.json` und `golden.json` der Gruppe. Jede Session prüft jedes Asset mit `assets_preview`.

## Beispiele

Tanne in `objekte` → `requisite` mit Stamm und Krone, `idle` wiegt die Krone leicht. Münze in `icons` → statisch oder mit kurzem Glanz-Kippen.

## Ausnahme- und Fehlerfälle

Fehlt ein Papier-Material (z. B. Kupfer, Kristall), wird ein vorhandenes genommen (Kupfer = `fell-fuchs`, Kristall = `edelstein`/`koenigsblau`) und als D-Zeile in B-701 vermerkt.

## Akzeptanzkriterien

- **AC-01** Jedes Asset der Gruppe hat eine gültige Spec.
- **AC-02** `task gen:paper` erzeugt alle Ausgaben der Gruppe; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

keine

## Sessions

Je Session 5–9 Assets, Umsetzung Sonnet, Review Opus, keine Mensch-Session.

- MG8.1 Bäume, Wurzeln, Büsche, Setzling → `objekte`
- MG8.2 Felsen, Erzbrocken, Adern → `objekte`
- MG8.3 Truhe, Portal, Ausgang, Abbau-Schilder, Stern → `objekte`
- MG8.4 Münze und Gold-Icons → `icons`
- MG8.5 Material-Icons → `icons`
- MG8.6 Review (Opus)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
