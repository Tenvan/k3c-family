# S6 · CLI · Onboarding „Erste Nacht geführt“ und Controller-Glyphen

- **Status:** aktiv
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-148, B-149
- **Start-Commit:** 27b117f
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-05, Chat, durch 🧑, Revision 2; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Der Kern-Loop ist für Kinder nicht selbsterklärend, Hinweise sind Text am Rand, Tasten stehen als Text. Voraussetzung: S3 (Aktionen-Overlay) und S5 (Optionen zum Rücksetzen der Hinweise).

## Ziel

Die erste Nacht läuft geführt mit Hinweisen über den Objekten, der Grad „Leicht“ verzeiht Verluste, und alle Hinweise zeigen Controller-Glyphen statt Tasten-Text. Am Ende sichtbar: Ein Kind spielt die erste Nacht ohne Erklärung.

## Beteiligte und Zielgruppen

Kinder und Eltern am TV; 🧑 nimmt am Gerät ab.

## Anforderungen

B-148 und B-149 › Anforderungen.

## Nicht-Ziele

Vollständiges Tutorial, Ton, neue Tastenbelegung, Balancing der Nächte (B-155).

## Regeln und Einschränkungen

Die geführte erste Nacht ist optional (Beschluss Q11). `CLAUDE.md` (Client zeichnet nur, Werte in `data/`, B-Taste frei); Grad-Werte als Go-Regel mit Sim-Test. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Neues Spiel im Grad „Leicht“: Hinweis „Aufheben“ mit Glyph „A“ über der Münze; die erste Nacht endet ohne Verlust.

## Ausnahme- und Fehlerfälle

`localStorage` gesperrt → Hinweise erscheinen bei jedem Start; unbekanntes Gerät → Text-Rückfall statt Glyph.

## Akzeptanzkriterien

- **AC-01** Die Hinweis-Funktion liefert den nächsten Hinweis aus Snapshot-Daten und gesehenen Hinweisen (B-148/AC-01).
- **AC-02** Im Grad „Leicht“ verliert die erste Nacht weder Gold noch Gebäude (B-148/AC-02).
- **AC-03** Hinweise erscheinen über Münze, Bauplatz und beim Nahen der Nacht und verschwinden nach der Handlung (B-148/AC-03).
- **AC-04** Aktion + Gerät ergibt einen Glyph-Schlüssel, Unbekanntes den Text-Rückfall (B-149/AC-01).
- **AC-05** Hinweise zeigen Glyphen für Controller, Tastatur und Touch, jede Glyph-Quelle hat einen Credit-Eintrag oder ist selbst gezeichnet (B-149/AC-02, B-149/AC-03).
- **AC-06** 🧑 hat Führung und Glyphen am TV abgenommen, die erste Nacht mit mindestens einem Kind (B-148/AC-04, B-149/AC-04).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| S6.1 | `S6.1-freundlich-grad.md` | Umsetzung | autonom | fertig |
| S6.2 | `S6.2-glyphen.md` | Umsetzung | autonom | fertig |
| S6.3 | `S6.3-hinweise.md` | Umsetzung | autonom | fertig |
| S6.4 | `S6.4-abnahme-kind.md` | Workshop | Mensch | offen |
| S6.5 | `S6.5-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-05, Review S6.5 (leicht, nur schwere Befunde): keine schweren Befunde, nichts zu beheben. tsc, oxlint (0 Fehler), Vitest (1333), `go test`, golangci-lint (0 issues) grün.
AC-01, AC-02, AC-04 mit Tests belegt (`guideHints.test.ts`, `difficulty_protect_test.go` samt Gegenprobe, `glyphs.test.ts`); AC-03, AC-05 umgesetzt, Tests grün, Sicht angenommen, Validierung offen (S6.4; Browser-Pane-Lauf nicht freigegeben).
AC-06 angenommen, Validierung offen (S6.4); S6.4 steht im Fahrplan unter „Offen am Gerät“, der Sprint bleibt aktiv, B-148 und B-149 bleiben eingeplant.
Neues Ticket: B-294 (Frage: Münze und „Nacht naht“ ohne Glyph, 🧑 entscheidet bei S6.4).
Version: v0.12.0 vorgeschlagen (neues Feature: geführte erste Nacht und Glyphen; aktuell v0.11.0)
