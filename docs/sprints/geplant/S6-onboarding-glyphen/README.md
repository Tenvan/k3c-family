# S6 · CLI · Onboarding „Erste Nacht geführt“ und Controller-Glyphen

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-148, B-149
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Kern-Loop ist für Kinder nicht selbsterklärend, Hinweise sind Text am Rand, Tasten stehen als Text. Voraussetzung: S3 (Aktionen-Overlay) und S5 (Optionen zum Rücksetzen der Hinweise).

## Ziel

Die erste Nacht läuft geführt mit Hinweisen über den Objekten, der Freundlich-Grad verzeiht Verluste, und alle Hinweise zeigen Controller-Glyphen statt Tasten-Text. Am Ende sichtbar: Ein Kind spielt die erste Nacht ohne Erklärung.

## Beteiligte und Zielgruppen

Kinder und Eltern am TV; 🧑 nimmt am Gerät ab.

## Anforderungen

B-148 und B-149 › Anforderungen.

## Nicht-Ziele

Vollständiges Tutorial, Ton, neue Tastenbelegung, Balancing der Nächte (B-155).

## Regeln und Einschränkungen

`CLAUDE.md` (Client zeichnet nur, Werte in `data/`, B-Taste frei); Grad-Werte als Go-Regel mit Sim-Test. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Neues Spiel im Freundlich-Grad: Hinweis „Aufheben“ mit Glyph „A“ über der Münze; die erste Nacht endet ohne Verlust.

## Ausnahme- und Fehlerfälle

`localStorage` gesperrt → Hinweise erscheinen bei jedem Start; unbekanntes Gerät → Text-Rückfall statt Glyph.

## Akzeptanzkriterien

- **AC-01** Die Hinweis-Funktion liefert den nächsten Hinweis aus Snapshot-Daten und gesehenen Hinweisen (B-148/AC-01).
- **AC-02** Im Freundlich-Grad verliert die erste Nacht weder Gold noch Gebäude (B-148/AC-02).
- **AC-03** Hinweise erscheinen über Münze, Bauplatz und beim Nahen der Nacht und verschwinden nach der Handlung (B-148/AC-03).
- **AC-04** Aktion + Gerät ergibt einen Glyph-Schlüssel, Unbekanntes den Text-Rückfall (B-149/AC-01).
- **AC-05** Hinweise zeigen Glyphen für Controller, Tastatur und Touch, jede Glyph-Quelle hat einen Credit-Eintrag oder ist selbst gezeichnet (B-149/AC-02, B-149/AC-03).
- **AC-06** 🧑 hat Führung und Glyphen am TV abgenommen, die erste Nacht mit mindestens einem Kind (B-148/AC-04, B-149/AC-04).

## Offene Fragen

Umfang der Führung und Freundlich-Grad: 🧑, `docs/fragenkatalog.md Q11`; Zeichenstil der Glyphen: `docs/fragenkatalog.md Q13`.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- S6.1 Freundlich-Grad in `data/difficulty.json` und Sim-Test (AC-02).
- S6.2 Glyph-Funktion und Zeichnung, Credits (AC-04, AC-05).
- S6.3 Hinweis-Funktion und Darstellung über den Objekten, „Gesehen“-Merkung (AC-01, AC-03).
- S6.4 🧑 Abnahme mit einem Kind am TV (AC-06).
- S6.5 Review des Sprints (Code-Sprint) (AC-01, AC-02, AC-03, AC-04, AC-05, AC-06).

## Abnahme

–
