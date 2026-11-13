# B-833 · Abnahme WZ41: CI auf develop mit Tests grün, check:all nachgeholt

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Sprint WZ41 hat die Tests in `ci.yml` wieder eingeschaltet (WZ41.4, B-825) und den Release-Workflow prüfen lassen (B-758). Die Befehle sind lokal grün; der Nachweis auf `develop` geht erst nach dem Merge, weil PR-Läufe nur Merge-Konflikte prüfen. `task check:all` (B-758/AC-03) lief nicht, weil es Race und die gesperrten Balance-Tests enthält.

## Ziel

Die offenen Nachweise von WZ41 sind erbracht: erster CI-Lauf auf `develop` nach dem Merge mit Tests grün, Laufzeit je Job notiert, `task check:all` nach Aufhebung der Sperre (oder ohne Sim-Pakete, B-832) mit Befunden als Tickets.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** CI-Lauf auf `develop` nach dem Merge von WZ41: alle Jobs grün, inklusive go-arm64 (B-756/AC-01, B-757/AC-03, B-758/AC-02, B-825/AC-01).
- **AC-02** Laufzeit je Job dieses Laufs im Ticket (B-825/AC-03).
- **AC-03** `task check:all` gelaufen, Befunde als Tickets (B-758/AC-03).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Abnahme-Ticket von WZ41 (WZ41.3, 2026-11-13); Sim und Balancing gesperrt, kein gen:paper-Golden nötig (kein Gen-Feature geändert).
