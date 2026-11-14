# S6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Onboarding „Erste Nacht geführt“ und Controller-Glyphen
- **Projekt:** –

## Ziel

Die erste Nacht läuft geführt mit Hinweisen über den Objekten, der Grad „Leicht“ verzeiht Verluste, und alle Hinweise zeigen Controller-Glyphen statt Tasten-Text. Am Ende sichtbar: Ein Kind spielt die erste Nacht ohne Erklärung.

## Ergebnis

2026-10-05, Review S6.5 (leicht, nur schwere Befunde): keine schweren Befunde, nichts zu beheben. tsc, oxlint (0 Fehler), Vitest (1333), `go test`, golangci-lint (0 issues) grün. AC-01, AC-02, AC-04 mit Tests belegt (`guideHints.test.ts`, `difficulty_protect_test.go` samt Gegenprobe, `glyphs.test.ts`); AC-03, AC-05 umgesetzt, Tests grün, Sicht angenommen, Validierung offen (S6.4; Browser-Pane-Lauf nicht freigegeben). AC-06 angenommen, Validierung offen (S6.4); S6.4 steht im Fahrplan unter „Offen am Gerät“, der Sprint bleibt aktiv, B-148 und B-149 bleiben eingeplant. Neues Ticket: B-294 (Frage: …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Die Hinweis-Funktion liefert den nächsten Hinweis aus Snapshot-Daten und gesehenen Hinweis … |
| AC-02 | – | Im Grad „Leicht“ verliert die erste Nacht weder Gold noch Gebäude (B-148/AC-02). |
| AC-03 | – | Hinweise erscheinen über Münze, Bauplatz und beim Nahen der Nacht und verschwinden nach de … |
| AC-04 | – | Aktion + Gerät ergibt einen Glyph-Schlüssel, Unbekanntes den Text-Rückfall (B-149/AC-01). |
| AC-05 | – | Hinweise zeigen Glyphen für Controller, Tastatur und Touch, jede Glyph-Quelle hat einen Cr … |
| AC-06 | – | 🧑 hat Führung und Glyphen am TV abgenommen, die erste Nacht mit mindestens einem Kind (B-1 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-148 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-06 – | Die erste Nacht wird mit kontextuellen Hinweisen geführt, der Grad „Leicht“ kost … |
| B-149 | erledigt | AC-04 –, AC-05 –, AC-06 – | Hinweise zeigen Controller-Glyphen statt Tasten-Text |
