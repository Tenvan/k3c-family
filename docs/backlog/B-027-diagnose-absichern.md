# B-027 · Diagnose-Schnittstelle ist abgesichert

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** SP03
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (Pauschalauftrag „beide komplett autonom fertig stellen“; N = 5, /api/health neu, ohne Token Diagnose aus)

## Ausgangslage

Die geplante Diagnose-Schnittstelle `/api/status` (SP03.2) und die TUI (B-002) können Spieler trennen; ohne Schutz könnte das jedes Gerät im Heimnetz.

## Ziel

Diagnose-Schnittstelle ist abgesichert. Nutzen: Auch im Heimnetz soll nicht jedes Gerät Spieler trennen können.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit Xbox und Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- `/api/status` und die TUI verlangen ein Token aus einer Umgebungsvariable.

## Nicht-Ziele

Benutzerkonten, TLS-Pflicht.

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

Ein Handy ruft `/api/status` ohne Token auf → 401.

## Ausnahme- und Fehlerfälle

Umgebungsvariable nicht gesetzt → Diagnose abgeschaltet (fail-closed, `/api/status` antwortet 404), der Spielbetrieb läuft weiter.

## Akzeptanzkriterien

- **AC-01** Ohne gültiges Token antwortet `/api/status` mit 401 (Test).
- **AC-02** Mit dem Token aus der Umgebungsvariable liefert es den Status (Test).

## Offene Fragen

keine (Verhalten ohne Variable von 🧑 bestätigt, 2026-09-30).

## Notizen

–
