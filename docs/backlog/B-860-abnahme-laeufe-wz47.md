# B-860 · Abnahme-Läufe WZ47

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** WZ47
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Sprint WZ47 (Ticket-Relationen, Index-Verdichtung, `plan_digest`) ist erledigt; er ändert nur Workbench-Code und Planungsdateien.

## Ziel

Die Abnahme-Läufe von WZ47 sind grün.

## Beteiligte und Zielgruppen

Claude autonom; 🧑 liest das Ergebnis.

## Anforderungen

Läufe laut `docs/arbeitsweise.md` › Sim, Balance und Gen, nur über `check_run`.

## Nicht-Ziele

Nachbessern im Ticket.

## Regeln und Einschränkungen

Sim und Balancing gesperrt.

- **Bereich:** – (nur Prüfläufe)
- **Abhängig von:** WZ47
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Prüfläufe.

## Ausnahme- und Fehlerfälle

Rot → Problem-Ticket.

## Akzeptanzkriterien

- **AC-01** Sim und Balance verschoben (Sperre 2026-11-11).
- **AC-02** Gen: `check_run task:test` grün.
- **AC-03** Race: `check_run task:check:race` (lokal ohne gcc als übersprungen nennen).

## Offene Fragen

keine

## Notizen

Vorschlag aus WZ47.4 (2026-11-14).
