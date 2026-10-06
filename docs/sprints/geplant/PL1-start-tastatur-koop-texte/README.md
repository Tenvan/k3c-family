# PL1 · PLAT · Neues Spiel, zwei Spieler an einer Tastatur, Overlay auf der Xbox, zentrale Texte

- **Status:** geplant
- **Domäne:** PLAT
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-292, B-316, B-195, B-215
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

„Neues Spiel“ lädt bei vorhandenem Spielstand den alten (B-292); an einer Tastatur spielt nur einer (B-316); das Debug-Overlay ist auf der Xbox nicht erreichbar (B-195); Texte von Touch-Overlay und Shell stehen im Code (B-215).

## Ziel

Jede Mechanik ist am PC mit zwei Spielern prüfbar, „Neues Spiel“ verhält sich wie erwartet. Am Ende sichtbar: „Neues Spiel“ startet immer neu, zwei Tastatur-Spieler, Overlay per Controller.

## Beteiligte und Zielgruppen

🧑 testet am PC und an der Xbox; Agent baut in `src/input/`, `src/landing/`, `src/core/`.

## Anforderungen

B-292 › Anforderungen; B-316 › Anforderungen; B-195 › Anforderungen; B-215 › Anforderungen.

## Nicht-Ziele

Controller-Prüfungen nachholen (HW1).

## Regeln und Einschränkungen

PLAT; View + Menu reserviert, B nicht belegen.

## Beispiele

Tastatur: Spieler 1 WASD, Spieler 2 Pfeiltasten → zwei Monarchen im Split-Screen.

## Ausnahme- und Fehlerfälle

Belegung kollidiert mit Dev-Tasten → Dev-Taste weicht, Hinweis in der Doku.

## Akzeptanzkriterien

- **AC-01** Die Kachel „Neues Spiel“ startet auch bei vorhandenem Spielstand familie (B-292/AC-01, B-292/AC-02, B-292/AC-03).
- **AC-02** Zwei Spieler spielen an einer Tastatur im Split-Screen (B-316/AC-01, B-316/AC-02).
- **AC-03** Das Debug-Overlay lässt sich auf der Xbox öffnen (B-195/AC-01, B-195/AC-02).
- **AC-04** Die Texte von Touch-Overlay, Shell und Werkzeug-Seiten kommen aus den zentralen Textdateien (B-215/AC-01, B-215/AC-02).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- PL1.1 „Neues Spiel“ startet immer neu (AC-01).
- PL1.2 Zwei Spieler an einer Tastatur (AC-02).
- PL1.3 Overlay auf der Xbox, Texte zentral (AC-03, AC-04).
- PL1.4 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
