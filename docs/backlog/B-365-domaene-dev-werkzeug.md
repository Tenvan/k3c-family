# B-365 · Das Entwickler-Werkzeug k3c-dev hat eine eigene Domäne statt SRV

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** DV1
- **Projekt:** WZG
- **Erstellt:** 2026-10-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Wunsch von 🧑 (2026-10-08): Arbeit an `tools/k3c-dev/` ist eher Tooling/Dev als Server. Heute gehört k3c-dev laut `docs/arbeitsweise.md` › Domänen zu **SRV** (zusammen mit `engine/room/`, `engine/net/`, `engine/store/`, `cmd/`, Docker). Folge: Sprints am Werkzeug (M1–M11, PJ2, TR1, B-360, B-362, B-363, B-364) sperren die SRV-Domäne gegen Server-Arbeit und umgekehrt, obwohl sie keine Dateien teilen.

## Ziel

Eine eigene Domäne (Vorschlag `DEV`) für das Entwickler-Werkzeug: Sessions daran sperren SRV nicht mehr, und Regeln, Glossar, Planungstest und plan-Tools kennen sie.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `docs/arbeitsweise.md` › Domänen: Zeile `DEV` (Dateien `tools/k3c-dev/`, ggf. Werkzeug-Binaries unter `cmd/`), SRV ohne k3c-dev; Glossar-Eintrag.
- Planungstest (`tests/planningDocs.ts`, Vorlagen) und plan-Tools (Domänenliste, Filter) kennen `DEV`; Filter-Chip auf der Planungsseite.
- Offene Tickets und geplante Sprints am Werkzeug auf `DEV` umstellen (B-360, B-362, B-363, PL2? nach Dateien prüfen); erledigte bleiben.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die Domänen-Tabelle nennt `DEV` mit `tools/k3c-dev/`; SRV nennt k3c-dev nicht mehr; Glossar hat den Begriff.
- **AC-02** `plan_create`/`plan_set` akzeptieren `Domäne: DEV`, `task test -- planning` ist grün mit einem Ticket in `DEV`.
- **AC-03** Offene Tickets am Werkzeug tragen `DEV`.

## Offene Fragen

- Kürzel: `DEV` oder `TOOL`? (Vorschlag `DEV`, passt zu k3c-dev.)
- Gehören `cmd/k3c-load`, `cmd/k3c-tui` und die Werkzeug-Seiten unter `src/tools/` (heute PLAT) mit dazu?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
