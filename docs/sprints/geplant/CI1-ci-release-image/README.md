# CI1 · INF · CI-Nachweis, Release-Image ohne Dev-Mode, Test-Abdeckung

- **Status:** geplant
- **Projekt:** REL
- **Domäne:** INF
- **Reife:** Entwurf
- **Tickets:** B-273, B-053, B-019
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Release-Image startet mit Dev-Mode (B-273), der CI-Nachweis für SP01 fehlt (B-053), die Test-Abdeckung ist nicht sichtbar (B-019).

## Ziel

Release-Images sind sicher, die CI belegt die Prüfungen, Abdeckungslücken sind sichtbar. Am Ende sichtbar: CI grün mit SP01-Prüfungen, Release-Image lehnt Dev-Aktionen ab, Abdeckung im CI-Bericht.

## Beteiligte und Zielgruppen

🧑 veröffentlicht Releases; Agent ändert CI und Docker.

## Anforderungen

B-273 › Anforderungen; B-053 › Anforderungen; B-019 › Anforderungen.

## Nicht-Ziele

Neue Prüfungen über die Abdeckung hinaus.

## Regeln und Einschränkungen

INF; Docker-Datei gehört SRV, Änderung als eigene Session mit Grenzfall-Hinweis.

## Beispiele

Release-Image starten, Dev-Aktion senden → 🚫 abgelehnt.

## Ausnahme- und Fehlerfälle

CI rot wegen Flake → Ticket (NT1), nicht wiederholen bis grün.

## Akzeptanzkriterien

- **AC-01** Das Release-Image startet den Server ohne Dev-Mode (B-273/AC-01, B-273/AC-02, B-273/AC-03).
- **AC-02** Die CI hat die Prüfungen aus SP01 einmal grün durchlaufen (B-053/AC-01, B-053/AC-02).
- **AC-03** Test-Abdeckung der Engine ist sichtbar (B-019/AC-01).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- CI1.1 Release-Image ohne Dev-Mode (AC-01).
- CI1.2 CI-Nachweis SP01 und Abdeckung im Bericht (AC-02, AC-03).
- CI1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
