# BAL5 · SIM · Balancing-Tester: Profil „Mauern zuerst“ und Sensitivität ohne Wirkung

- **Status:** geplant
- **Projekt:** –
- **Domäne:** SIM
- **Prio:** mittel
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-300, B-301
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Profil „Mauern zuerst“ unterscheidet sich kaum von „sparsam“ (B-300); ein Sensitivitäts-Pfad ohne Wirkung fällt nicht auf (B-301).

## Ziel

Die Berichte des Balancing-Testers zeigen nur wirksame Variationen und unterscheidbare Profile. Am Ende sichtbar: Profil „Mauern zuerst“ spielt messbar anders, Sensitivitäts-Pfad ohne Wirkung ist ein Fehler.

## Beteiligte und Zielgruppen

REG nutzt die Berichte; Agent baut in `tools/k3c-dev/internal/balance/`.

## Anforderungen

B-300 › Anforderungen; B-301 › Anforderungen.

## Nicht-Ziele

Neue Kennzahlen, Wertänderungen in `data/`.

## Regeln und Einschränkungen

Einschiebbar; deterministisch, nur `engine/rng`.

## Beispiele

Pfad `castle.hp` mit ±10 % → Kennzahlen ändern sich; Pfad ohne Wirkung → Fehler.

## Ausnahme- und Fehlerfälle

Pfad unbekannt → Fehler mit Pfad, kein leerer Bericht.

## Akzeptanzkriterien

- **AC-01** Das Profil „Mauern zuerst“ spielt messbar anders als „sparsam“ (B-300/AC-01).
- **AC-02** Ein Sensitivitäts-Pfad ohne Wirkung ergibt einen Fehler (B-301/AC-01).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BAL5.1 Profil „Mauern zuerst“ schärfen, Sensitivität prüft Wirkung (AC-01, AC-02).
- BAL5.2 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
