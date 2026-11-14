# B-839 · plan_next und plan_waves zählen Plätze, Ordner-Pfade und laufende Sessions ohne Domäne richtig

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ49
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

Review WZ44.5 (2026-11-13) fand in `tools/k3c-workbench/internal/planning` Restbefunde (der schwere Befund, nur der erste Pfad je Zeile in `allowedFiles`, ist im Review-Commit behoben):

- `next_lanes.go` (`maxParallel`): `plan_next` schlägt bis zu drei weitere Sessions vor, auch wenn schon Sessions `in Arbeit` sind; Regel 5 (höchstens vier Agenten) kann so überschritten werden. `plan_waves` zählt richtig.
- `waves_spec.go` (Überlappung): Ein Ordner ohne Wildcard und ohne `/` am Ende (`tools/paper`) überlappt nicht mit `tools/paper/cli/**` (vermutet, heute kein solcher Eintrag).
- `next.go` (`x.Domain != ""`): Sessions `in Arbeit` ohne Domäne blocken mit ihren Dateien und Sperren nichts.
- `next.go` vs. `next_origin.go`: Branch-Zeile aus der gewählten Session, origin-Overlay aus der ersten Session mit Branch (vermutet).

## Ziel

`plan_next` und `plan_waves` rechnen Plätze und Kollisionen wie `docs/arbeitsweise.md` › Wellen berechnen.

## Beteiligte und Zielgruppen

Planende Agenten.

## Anforderungen

- Vorschläge in `plan_next` höchstens `4 − in Arbeit`.
- Pfad ohne Wildcard gilt auch als Ordner-Präfix.
- Sessions `in Arbeit` ohne Domäne zählen mit Dateien und Sperren.
- Branch-Zeile und Overlay aus derselben Session.

## Nicht-Ziele

Regel für Sessions desselben Sprints (B-838).

## Regeln und Einschränkungen

- **Bereich:** `tools/k3c-workbench/internal/planning/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Drei Sessions in Arbeit → `plan_next` nennt höchstens eine weitere parallel.

## Ausnahme- und Fehlerfälle

Vier oder mehr Sessions in Arbeit → `plan_next` nennt keine weitere parallel.

## Akzeptanzkriterien

- **AC-01** Je Befund ein Test in `tools/k3c-workbench/internal/planning`, grün: (a) drei Sessions `in Arbeit` → `plan_next` nennt höchstens eine weitere parallel; (b) `tools/paper` überlappt mit `tools/paper/cli/**`; (c) Session `in Arbeit` ohne Domäne blockt mit Dateien und Sperre; (d) Branch-Zeile und origin-Overlay stammen aus derselben Session.

## Offene Fragen

keine

## Notizen

k3c-reviewer, Review WZ44.5.
