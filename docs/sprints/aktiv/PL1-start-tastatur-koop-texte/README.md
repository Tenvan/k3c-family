# PL1 · PLAT · Neues Spiel, zwei Spieler an einer Tastatur, Overlay auf der Xbox, zentrale Texte

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** PLAT
- **Reife:** bereit
- **Tickets:** B-316, B-195, B-215
- **Start-Commit:** edf3f1b1
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-09, 🧑 im Chat (inkl. Grenzfall Domäne PL1.2 in GameScene)

## Ausgangslage

„Neues Spiel“ lädt bei vorhandenem Spielstand den alten (B-292); an einer Tastatur spielt nur einer (B-316); das Debug-Overlay ist auf der Xbox nicht erreichbar (B-195); Texte von Touch-Overlay und Shell stehen im Code (B-215).

## Ziel

Jede Mechanik ist am PC mit zwei Spielern prüfbar, „Neues Spiel“ verhält sich wie erwartet. Am Ende sichtbar: „Neues Spiel“ startet immer neu, zwei Tastatur-Spieler, Overlay per Controller.

## Beteiligte und Zielgruppen

🧑 testet am PC und an der Xbox (PL1.5); Agent baut in `src/input/`, `src/landing/`, `src/core/` und bindet die zweite Tastatur in `src/scenes/GameScene.ts` ein.

## Anforderungen

B-316 › Anforderungen; B-195 › Anforderungen; B-215 › Anforderungen.

## Nicht-Ziele

Controller-Prüfungen nachholen (HW1). Werkzeug-Seiten `src/tools/` übersetzen (eigenes Ticket, siehe Beschluss). Server oder Protokoll ändern. Hilfe, Glyphen und Overlay in `src/scenes/` umbauen (bei Bedarf Ticket, Domäne CLI).

## Regeln und Einschränkungen

PLAT; View + Menu reserviert, B nicht belegen; reservierte Tasten Pos1, F, Esc, Ö, Ä bleiben.

- Beschluss 🧑 2026-10-06 (Chat): Tastatur-Belegung laut Vorschlag in B-316. Spieler 1 links: A/D, Shift, Leertaste, E, Q, R, T, Z, K. Spieler 2 rechts: Pfeiltasten, Strg rechts, Enter, Ziffernblock. Spieler 2 tritt mit seiner Bestätigen-Taste bei.
- Beschluss 🧑 2026-10-06 (Chat): Die Werkzeug-Seiten (`src/tools/`) werden auch übersetzt. Weil PL1 damit größer als 4 Sessions würde, plant PL1 nur Touch-Overlay und Shell; die Werkzeug-Seiten bekommen ein eigenes Ticket.
- Beschluss 🧑 2026-10-06 (Chat): Die Rückfrage aus B-195 (Seite, `?dev=0`) bleibt offen und blockiert nicht; PL1.1 reproduziert das Problem zuerst.
- B-292 ist im Code schon umgesetzt (Commit `f07d6fae`); der Nachweis läuft seit 2026-10-07 in LP1 (LP1.2), nicht mehr in PL1.

## Beispiele

Tastatur: Spieler 1 drückt Leertaste, Spieler 2 drückt Enter → zwei Monarchen im Split-Screen; A/D bewegt nur Spieler 1, Pfeile nur Spieler 2.

## Ausnahme- und Fehlerfälle

Belegung kollidiert mit Dev-Tasten → Dev-Taste weicht, Hinweis in der Doku.

## Akzeptanzkriterien

- **AC-01** entfällt: B-292 ist am 2026-10-07 nach LP1 gewechselt (Beschluss 🧑, LP1/AC-05).
- **AC-02** Zwei Spieler spielen an einer Tastatur im Split-Screen (B-316/AC-01, B-316/AC-02).
- **AC-03** Das Debug-Overlay lässt sich auf der Xbox öffnen (B-195/AC-01, B-195/AC-02).
- **AC-04** Die Texte von Touch-Overlay und Shell kommen aus den zentralen Textdateien (B-215/AC-01, B-215/AC-02).

## Offene Fragen

- B-195: Auf welcher Seite und ob mit `?dev=0` versucht wurde, klärt 🧑 (nicht blockierend, PL1.1 stellt alle Fälle nach).
- Grenzfall Domäne: PL1.2 ändert in `src/scenes/GameScene.ts` (CLI) nur Erzeugen, `update()` und Eingabeliste der zweiten Tastatur-Instanz. Vorschlag der Planung, 🧑 bestätigt mit der Spec-Freigabe (nicht blockierend).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PL1.1 | `PL1.1-neues-spiel-overlay-ursache.md` | Umsetzung | autonom | fertig |
| PL1.2 | `PL1.2-zwei-spieler-tastatur.md` | Umsetzung | autonom | fertig |
| PL1.3 | `PL1.3-texte-touch-shell.md` | Umsetzung | autonom | offen |
| PL1.4 | `PL1.4-review.md` | Review | autonom | offen |
| PL1.5 | `PL1.5-abnahme-pc-xbox.md` | Umsetzung | Mensch | offen |

## Abnahme

–
