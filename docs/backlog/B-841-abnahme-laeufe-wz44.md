# B-841 · Abnahme-Läufe WZ44

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** WZ44
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Sprint WZ44 (Planungs-Werkzeuge der Workbench, Sim-Sperre im Werkzeug) ist abgenommen (WZ44.5, 2026-11-13). Sim, Balance, Gen und Race laufen nach `docs/arbeitsweise.md` › Sim, Balance und Gen hier.

## Ziel

Die Abnahme-Läufe des Sprints sind gefahren oder begründet ausgelassen.

## Beteiligte und Zielgruppen

Claude (autonom, nachts).

## Anforderungen

Nur die Läufe, die WZ44 braucht.

## Nicht-Ziele

Nachbessern; Rot wird ein neues Ticket.

## Regeln und Einschränkungen

Sim und Balancing gesperrt (Beschluss 🧑 2026-11-11).

- **Bereich:** –
- **Abhängig von:** WZ44
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

–

## Ausnahme- und Fehlerfälle

Ohne C-Compiler läuft `check:race` still durch: im Ergebnis als übersprungen nennen.

## Akzeptanzkriterien

- **AC-01** Sim: verschoben (Sperre 2026-11-11); die Sim-Sperre selbst ist mit Tests belegt (WZ44.1).
- **AC-02** Balance: verschoben (Sperre 2026-11-11).
- **AC-03** Gen: nicht relevant (WZ44 ändert keine Specs, Renderer oder Grafik).
- **AC-04** Race: `check_run task:check:race`, grün (Workbench-Pakete `planning`, `mcpsrv`).

## Offene Fragen

keine

## Notizen

–
