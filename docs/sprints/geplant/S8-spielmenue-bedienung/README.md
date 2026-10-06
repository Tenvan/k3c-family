# S8 · CLI · Spielmenü „Spiel verlassen“, Y-Belegung und Glyphen-Entscheidung

- **Status:** geplant
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-293, B-205, B-294
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Spielmenü hat nur „Weiter“ (B-293); S3 belegt Y noch mit einem Bau-Menü (B-205); Münze und „Nacht naht“ haben in der ersten Nacht keine Glyph (B-294).

## Ziel

Spielende verlassen das Spiel aus dem Menü mit jedem Eingabegerät, Y folgt dem Beschluss. Am Ende sichtbar: Spielmenü verlässt ins Lobby, Y ohne Bau-Menü, Glyph-Entscheidung umgesetzt.

## Beteiligte und Zielgruppen

🧑 entscheidet B-294; Agent baut in `src/scenes/`.

## Anforderungen

B-293 › Anforderungen; B-205 › Anforderungen; B-294 › Anforderungen.

## Nicht-Ziele

Optionen und Pause (S5), Lobby-Umbau (LB1).

## Regeln und Einschränkungen

CLI; Spielstand wird beim Verlassen gespeichert (S2).

## Beispiele

Menü → „Spiel verlassen“ → Lobby, Spielstand gespeichert.

## Ausnahme- und Fehlerfälle

Verbindung weg beim Verlassen → Lobby trotzdem, Meldung 👋.

## Akzeptanzkriterien

- **AC-01** Das Spielmenü hat neben „Weiter“ einen Eintrag „Spiel verlassen“ (B-293/AC-01, B-293/AC-02, B-293/AC-03).
- **AC-02** Die Y-Belegung in S3 folgt dem Beschluss „kein Bau-Menü“ (B-205/AC-01, B-205/AC-02, B-205/AC-03).
- **AC-03** Münze und „Nacht naht“ zeigen in der geführten ersten Nacht keine Glyph (B-294/AC-01).

## Offene Fragen

- B-294: Glyph oder Bild für Münze und „Nacht naht“, entscheidet 🧑.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- S8.1 „Spiel verlassen“ im Spielmenü (AC-01).
- S8.2 Y-Belegung nach Beschluss, Glyph-Entscheidung umsetzen (AC-02, AC-03).
- S8.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
