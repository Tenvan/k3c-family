# S8 · CLI · Spielmenü „Spiel verlassen“, Y-Belegung und Glyphen-Entscheidung

- **Status:** geplant
- **Projekt:** –
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-293, B-205, B-294
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

„Spiel verlassen“ steht schon im Code (Commit `f07d6fae`, PR #153: `leave` in `src/scenes/optionsLogic.ts`, Test in `optionsLogic.test.ts`, `GameScene.leaveRoom()`), aber ohne Nachweis von B-293/AC-02 und AC-03. Die Aktion `build` auf Y ist im Code schon entfernt (S3.1, `src/input/slotBindings.ts`: „Y ist frei“), die Session-Datei S3.1 nennt in der Belegung aber noch „Bau-Menü Y“ (B-205). Münze und „Nacht naht“ zeigen in der geführten ersten Nacht nur Text (B-294).

## Ziel

Spielende verlassen das Spiel aus dem Menü mit jedem Eingabegerät, Y folgt dem Beschluss. Am Ende sichtbar: Spielmenü verlässt ins Lobby, Y ohne Bau-Menü, Münze und „Nacht naht“ mit Bild vor dem Text.

## Beteiligte und Zielgruppen

🧑 nimmt im Browser ab (S8.4); Agent baut in `src/scenes/`.

## Anforderungen

B-293 › Anforderungen; B-205 › Anforderungen; B-294 › Anforderungen.

## Nicht-Ziele

Optionen und Pause (S5), Lobby-Umbau (LB1), neue Belegung für Y, Änderungen in `src/input/` (PLAT).

## Regeln und Einschränkungen

CLI; Spielstand wird beim Verlassen gespeichert (S2).

- Beschluss 🧑 2026-10-06 (Chat): B-294 – Bild statt Glyph: Münzsymbol bzw. Mond vor dem Text, aus vorhandenen Packs unter `public/` (Zuordnung in `docs/assets/`).
- Beschluss 🧑 2026-10-06 (Chat): B-205 ist redaktionell, ohne neue S3-Revision. S8 korrigiert Code und den S3-Text (S3-Dateien sind erlaubte Planungs-Dateien der Session S8.2).

## Beispiele

Menü → „Spiel verlassen“ → Lobby, Spielstand gespeichert. Erste Nacht: Münze liegt vor dem Monarchen → über ihr Münzbild + „Hinlaufen: Münze aufheben“.

## Ausnahme- und Fehlerfälle

Verbindung weg beim Verlassen → Lobby trotzdem, Meldung 👋.

## Akzeptanzkriterien

- **AC-01** Das Spielmenü hat neben „Weiter“ einen Eintrag „Spiel verlassen“ (B-293/AC-01, B-293/AC-02, B-293/AC-03).
- **AC-02** Die Y-Belegung in S3 folgt dem Beschluss „kein Bau-Menü“ (B-205/AC-01, B-205/AC-02, B-205/AC-03).
- **AC-03** Münze und „Nacht naht“ zeigen in der geführten ersten Nacht ein Bild (Münzsymbol bzw. Mond) vor dem Text, aus einem Pack unter `public/`, mit Zuordnung in `docs/assets/` (B-294/AC-01, Beschluss 🧑 2026-10-06).
- **AC-04** `task check` grün; keine Datei über 400 Zeilen, keine Funktion über 60.

## Offene Fragen

- Mond-Bild: Unter `public/` ist per Dateiname kein Mond zu finden (`find public -iname "*moon*"` leer; ungeprüft in Sammel-Sheets wie `kyrises-free-16x16-rpg-icon-pack`). Findet S8.2 keinen, entscheidet 🧑 über Ersatz (anderes Pack-Bild oder neues Pack). Nicht blockierend für S8.1; blockiert nur den Mond-Teil von S8.2.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| S8.1 | `S8.1-spiel-verlassen.md` | Umsetzung | autonom | offen |
| S8.2 | `S8.2-y-und-hinweisbilder.md` | Umsetzung | autonom | offen |
| S8.3 | `S8.3-review.md` | Review | autonom | offen |
| S8.4 | `S8.4-browser-abnahme.md` | Workshop | Mensch | offen |

## Abnahme

–
