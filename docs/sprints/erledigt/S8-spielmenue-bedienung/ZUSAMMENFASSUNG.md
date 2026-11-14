# S8 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Spielmenü „Spiel verlassen“, Y-Belegung und Glyphen-Entscheidung
- **Projekt:** BED

## Ziel

Spielende verlassen das Spiel aus dem Menü mit jedem Eingabegerät, Y folgt dem Beschluss. Am Ende sichtbar: Spielmenü verlässt ins Lobby, Y ohne Bau-Menü, Münze und „Nacht naht“ mit Bild vor dem Text.

## Ergebnis

Review 2026-10-09 (S8.3, eigener Review-Agent): keine schweren Befunde, `task check` und `task check:go` grün. - AC-01 geprüft (S8.1: Test „Leave game“, Rechnung 938 < 1006 px); B-293/AC-02 im Browser abgenommen 2026-10-10 (S8.4, 🧑: Tastatur, Controller, Touch; Server-Log 👋). AC-02 geprüft (S8.2), B-205 erledigt; AC-04 geprüft. - AC-03: **nicht erfüllt**. Münzbild per Test belegt (S8.2), im Browser aber nicht zu sehen (S8.4, 🧑) → B-393; Mond verschoben nach B-370 (kein Bild unter `public/`, Entscheidung 🧑). - Neue Tickets: B-370, B-392 (Split-Screen-Kamera, aus S8.4), B-393. - Version: v0.15.1 …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Das Spielmenü hat neben „Weiter“ einen Eintrag „Spiel verlassen“ (B-293/AC-01, B-293/AC-02 … |
| AC-02 | geprüft | Die Y-Belegung in S3 folgt dem Beschluss „kein Bau-Menü“ (B-205/AC-01, B-205/AC-02, B-205/ … |
| AC-03 | geprüft | Münze und „Nacht naht“ zeigen in der geführten ersten Nacht ein Bild (Münzsymbol bzw. Mond … |
| AC-04 | geprüft | `task check` grün; keine Datei über 400 Zeilen, keine Funktion über 60. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-293 | erledigt | AC-01 geprüft | Das Spielmenü hat neben „Weiter“ einen Eintrag „Spiel verlassen“ |
| B-205 | erledigt | AC-02 geprüft | Die Y-Belegung in S3 folgt dem Beschluss „kein Bau-Menü“ |
| B-294 | verworfen | AC-03 geprüft | Münze und „Nacht naht“ zeigen in der geführten ersten Nacht keine Glyph |
