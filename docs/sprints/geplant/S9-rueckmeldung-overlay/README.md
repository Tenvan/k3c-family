# S9 · CLI · Rückmeldung für Schlag und Skills, ein Hinweis je Spieler

- **Status:** geplant
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-319, B-318
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die PC-Abnahme von S3 (S3.4) fand zwei Mängel: Das Aktionen-Overlay zeigt mehrere Hinweise übereinander und über Preisschildern (B-319), Schlag und Skills zeigen ohne Ziel keine Rückmeldung (B-318).

## Ziel

Spieler sehen je Weltposition höchstens ein Anzeige-Element und erkennen jeden Druck auf Schlag oder Skill. Am Ende sichtbar: Jeder Tastendruck auf Schlag oder Skill ist sichtbar, das Aktionen-Overlay zeigt je Spieler einen Hinweis.

## Beteiligte und Zielgruppen

Alle Spieler; 🧑 bei Abnahmen; Agent baut in `src/scenes/`.

## Anforderungen

B-319 › Anforderungen; B-318 › Anforderungen.

## Nicht-Ziele

Neue Skills, Skill-Menü-Umbau (S3).

## Regeln und Einschränkungen

CLI; nach S3. Schriftgrößen aus dem Katalog (`FONTS`), B nicht belegen.

## Beispiele

Zwei Ziele nah beieinander → nur der Hinweis des nächsten; Schlag ins Leere → kurzer Schwung sichtbar.

## Ausnahme- und Fehlerfälle

Hinweis und Preisschild an derselben Position → Preisschild gewinnt, Hinweis entfällt.

## Akzeptanzkriterien

- **AC-01** Das Aktionen-Overlay zeigt je Spieler nur einen Hinweis, 24 px, nie über einem Preisschild (B-319/AC-01, B-319/AC-02, B-319/AC-03, B-319/AC-04).
- **AC-02** Schlag und Skills zeigen auch ohne Ziel sichtbar, dass die Taste ankam (B-318/AC-01, B-318/AC-02, B-318/AC-03).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- S9.1 Ein Hinweis je Spieler und Weltposition (AC-01).
- S9.2 Rückmeldung für Schlag und Skills ohne Ziel (AC-02).
- S9.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
