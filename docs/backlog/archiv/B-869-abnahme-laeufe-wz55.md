# B-869 · Abnahme-Läufe WZ55

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** WZ55
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint WZ55 ist erledigt; die Paper-CLI hat den neuen Befehl `batch`, die Render-Logik ist unverändert.

## Ziel

Die Abnahme-Läufe von WZ55 sind grün.

## Beteiligte und Zielgruppen

Claude autonom; 🧑 liest das Ergebnis.

## Anforderungen

Läufe laut `docs/arbeitsweise.md` › Sim, Balance und Gen, nur über `check_run`.

## Nicht-Ziele

Nachbessern im Ticket.

## Regeln und Einschränkungen

Sim und Balancing gesperrt.

- **Bereich:** `public/grafik/**`, `tools/paper/golden/**`
- **Abhängig von:** WZ55
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden

## Beispiele

nicht relevant: Prüfläufe.

## Ausnahme- und Fehlerfälle

Rot → Problem-Ticket.

## Akzeptanzkriterien

- **AC-01** Sim und Balance verschoben (Sperre 2026-11-11).
- **AC-02** Gen: `check_run task:test` samt `gen:paper` und Grafik-Golden grün.
- **AC-03** Race: `check_run task:check:race` (lokal ohne gcc als übersprungen nennen).

## Offene Fragen

keine

## Notizen

Vorschlag WZ55.2 (2026-11-14).

2026-11-14 (Claude, auf `sprint/welle6`): AC-01 verschoben (Sperre). AC-02 `check_run task:test` samt `gen:paper` und Grafik-Golden grün (4 min 4 s), Golden ohne Inhaltsänderung. AC-03 `check_run task:check:race` lokal ohne gcc übersprungen.
