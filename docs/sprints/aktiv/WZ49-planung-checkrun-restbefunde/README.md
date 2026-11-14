# WZ49 · DEV · Planungs-Werkzeuge und check_run: Restbefunde, Fahrplan aus Dateien, Im Spiel ansehen nativ

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-831, B-839, B-842, B-847, B-855
- **Start-Commit:** 3ea017230
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13 (Ralf: autonome Tickets abarbeiten)

## Ausgangslage

Restbefunde aus WZ42 und WZ44 und ein Teil der Ablösung des Browser-Clients: `check_run dev:test` ohne Frontend-Build und mit zu kurzem Zeitlimit (B-831), Plätze, Ordner-Pfade und laufende Sessions in `plan_next`/`plan_waves` (B-839), Fahrplan-Teile, die kein Werkzeug schreibt (B-842), „Im Spiel ansehen“ braucht noch den Browser-Client (B-847).

## Ziel

Die Planungs- und Prüf-Werkzeuge der Workbench rechnen richtig und schreiben den Fahrplan vollständig; „Im Spiel ansehen“ startet das native Spiel.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A); planende Agenten; 🧑 im Bereich Assets.

## Anforderungen

`B-831 › Ziel`, `B-839 › Ziel`, `B-842 › Ziel`, `B-847 › Ziel`.

## Nicht-Ziele

Regeländerung zu Sessions eines Sprints (B-838, Entscheidung 🧑).

## Regeln und Einschränkungen

Klasse A (Problem und Schuld, DEV). Nach WZ47 (gleicher Bereich `planning`, Sperre exe).

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `tools/k3c-workbench/internal/mcpsrv/check_targets*.go`, `tools/k3c-workbench/internal/mcpsrv/tools_planning*.go`, `tools/k3c-workbench/internal/studioview/**`
- **Abhängig von:** WZ47
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-831` Kriterien grün.
- **AC-02** `B-839` Kriterien grün.
- **AC-03** `B-842` Kriterien grün.
- **AC-04** `B-847` Kriterien grün.
- **AC-05** `B-855` Kriterien grün (Konsolenpuffer je Checkout).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ49.1 | `WZ49.1-plan-waves-restbefunde-fahrplan.md` | Umsetzung | autonom | fertig |
| WZ49.2 | `WZ49.2-checkrun-devtest-im-spiel-nativ.md` | Umsetzung | autonom | offen |
| WZ49.3 | `WZ49.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
