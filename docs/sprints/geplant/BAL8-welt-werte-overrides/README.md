# BAL8 · REG · Welt-Werte: Basis erben, Overrides je Welt, Simulation rechnet mit dem Wertesatz der Welt

- **Status:** geplant
- **Projekt:** BAL
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-680, B-681
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Alle Werte kommen heute aus globalen Dateien in `data/`; eine Welt kann nichts übersteuern (B-680), die Simulation liest globale Datei-Variablen (B-681).

## Ziel

Eine Welt-Datei erbt die Basis-Werte und übersteuert nur, was sie ändern will; jeder Raum rechnet mit dem aufgelösten Wertesatz seiner Welt. Am Ende sichtbar: zwei Räume mit verschiedenen Welten und verschiedenen Kosten beeinflussen sich nicht; Go-Tests und Golden grün.

## Beteiligte und Zielgruppen

Wer Welten und Balancing plant; Agenten REG und SIM; 🧑 bestätigt die Regel (Coop-Multiplikatoren bleiben die einzige Stelle für coop2/4).

## Anforderungen

`B-680 › Anforderungen`, `B-681 › Anforderungen`.

## Nicht-Ziele

Editor und Chat für Overrides (WZ37, B-682, B-683); neue Balancing-Werte.

## Regeln und Einschränkungen

Deterministisch; kein `math/rand`; Golden nur mit Begründung; Protokoll- oder Spielstand-Änderung nur nach Entscheidung von 🧑; keine Rücksicht auf alte Spielstände; Basiswerte sind Solo, coop2/4 nur über Multiplikatoren.

## Beispiele

Welt „schwer“ übersteuert Gegner-HP → nur Räume dieser Welt rechnen damit.

## Ausnahme- und Fehlerfälle

Override für unbekannten Wert → Welt wird beim Laden abgelehnt mit Pfad des Werts.

## Akzeptanzkriterien

- **AC-01** Regeldatei beschreibt Erben und Übersteuern der Basis-Werte, 🧑 bestätigt (`B-680/AC-01` bis `AC-05`).
- **AC-02** Die Simulation rechnet je Raum mit dem Wertesatz seiner Welt, Welten beeinflussen sich nicht (`B-681/AC-01` bis `AC-03`).

## Offene Fragen

Welche Basis-Werte sind übersteuerbar (Umfang laut `B-680`)? Entscheidet 🧑 mit der Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BAL8.1 (REG) Regeln und Datenformat der Overrides (AC-01).
- BAL8.2 (SIM) Wertesatz je Raum statt globaler Variablen, Go-Tests, Golden (AC-02).
- BAL8.3 Review, alle Kriterien, schließt ab.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
