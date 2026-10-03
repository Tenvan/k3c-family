# BAL4 · REG · Abgleich Spielmetrik und Simulator

- **Status:** geplant
- **Domäne:** REG
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-160
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Simulator misst mit Bots (BAL1–BAL3), echte Abende liefern Spielmetrik (B-150) und Fragebogen (B-151, P1). Ein Vergleich fehlt. Details in B-160.

## Ziel

Simulatorwerte und Werte echter Abende stehen nebeneinander, Abweichungen haben Ursache und Beschluss. Am Ende sichtbar: Abgleichtabelle in `docs/rules/`, beschlossene Änderungen in `data/`, `task balance` grün.

## Beteiligte und Zielgruppen

🧑 spielt die Abende, beschließt Toleranzen und Wertänderungen; der Agent wertet aus.

## Anforderungen

B-160 › Anforderungen.

## Nicht-Ziele

Erfassen der Spielmetrik (B-150), Fragebogen (B-151), Spielspaß-Bewertung.

## Regeln und Einschränkungen

Werte ändert nur REG mit Beschluss (`docs/arbeitsweise.md`); Metrik nur aus Berichten in `reports/`. Voraussetzung: B-150, BAL2, mindestens ein Spieleabend (P1).

## Beispiele

Simulator: Überleben Welle 3 median 82 %, echter Abend 55 % → Ursache und Beschluss in der Tabelle.

## Ausnahme- und Fehlerfälle

Zu wenige Sitzungen → „nicht aussagekräftig“, keine Wertänderung.

## Akzeptanzkriterien

- **AC-01** Die Abgleichtabelle enthält jede gemeinsame Kennzahl mit Simulatorwert, Wert echter Abende und Abweichung (B-160/AC-01).
- **AC-02** Zu jeder Abweichung über der Toleranz stehen Ursache und Beschluss von 🧑 (B-160/AC-02).
- **AC-03** Beschlossene Wertänderungen sind in `data/` umgesetzt, `task balance` und `task check:go` sind grün (B-160/AC-03).
- **AC-04** Kennzahlen mit zu wenigen Sitzungen sind als „nicht aussagekräftig“ markiert (B-160/AC-04).

## Offene Fragen

- Toleranz je Kennzahl und Definition „echter Abend“: Entscheidet 🧑 (`docs/fragenkatalog.md` Q12, Q24); blockiert die Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BAL4.1 Abgleichtabelle aus Reports und Tester-Läufen erstellen, Markierung „nicht aussagekräftig“ (AC-01, AC-04).
- BAL4.2 🧑 Workshop (Agent: Mensch): Abweichungen durchgehen, Ursachen und Beschlüsse festhalten (AC-02).
- BAL4.3 Beschlossene Werte in `data/` umsetzen, `task balance` und `task check:go`; schließt den Sprint ab (AC-03).

## Abnahme

–
