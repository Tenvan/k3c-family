# B-785 · Fahrplan ist aufgeräumt: fertige Sprints erledigt, Abnahmen als Gerätetermine, alte bereite Sprints geprüft

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ44
- **Projekt:** WZG
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Stand 2026-11-11: 21 aktive Sprints. RD1, MC1, RV1, RS1, RS2 und WZ40 haben keine offene Session mehr und liegen in `docs/sprints/aktiv/`. LG1 steht im Fahrplan unter „Aktiv“ und „Geplant“, die Zeile KB6 unter „Erledigt“ hat zu viele Spalten. 20 Mensch-Sessions in 16 Sprints warten einzeln auf 🧑. W6, P1 und BR1 sind seit Oktober `bereit` und nie aktiviert (W6.1 nennt `src/model/types.ts`, W5, W10, S5). Projekt RST ruht mit offenen Tickets.

## Ziel

Der Fahrplan zeigt nur, was läuft oder auf 🧑 wartet, und 🧑 hat je Gerät einen Termin statt eines Postens je Sprint.

## Beteiligte und Zielgruppen

🧑 (Abnahmen), Agenten (`plan_next`).

## Anforderungen

- Sprints ohne offene Session wandern nach `erledigt/` (über `plan_set`).
- Fahrplan: LG1 nur einmal, Zeile KB6 mit richtiger Spaltenzahl.
- Gerätetermine: je Gerät (PC, Xbox/TV, Pi, Mini-PC, Workshop, Veröffentlichung) eine Liste der offenen Mensch-Sessions im Abschnitt „Offen am Gerät“ des Fahrplans; die Sessions bleiben in ihren Sprints.
- W6, P1, BR1: Spec-Check (Pfade, Abhängigkeiten, Begriffe) oder zurück auf `Reife: Entwurf`.
- RST: offene Tickets nach Beschluss 🧑 2026-11-11 mit B-779 (Rust entfernen, Nummer aus PR #546) schließen oder umhängen.

## Nicht-Ziele

Code; neue Regeln (stehen in `docs/arbeitsweise.md` › Parallelität und Wellen).

## Regeln und Einschränkungen

Nur Planungsdateien, alles über `plan_*`. Ein Sprint mit offener Mensch-Session bleibt aktiv.

- **Bereich:** `docs/sprints/**`, `docs/projekte/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

RD1 (fünf Sessions fertig, Abnahme vom 2026-11-06) liegt danach in `erledigt/`.

## Ausnahme- und Fehlerfälle

Sprint mit offenem PR: erst nach dem Merge verschieben.

## Akzeptanzkriterien

- **AC-01** In `docs/sprints/aktiv/` liegt kein Sprint ohne offene Session; `check_run task:test` mit Muster `planning` grün.
- **AC-02** „Offen am Gerät“ nennt jede offene Mensch-Session genau einmal unter ihrem Gerät.
- **AC-03** W6, P1, BR1 sind geprüft oder `Reife: Entwurf`; LG1 und KB6 im Fahrplan korrigiert.

## Offene Fragen

Gerät von BD10.3, W15.3 und KB5.6 ist aus dem Titel nicht eindeutig; Default (Claude, 2026-11-12): Session-Datei lesen, sonst 🧑 fragen. Teile der Anforderungen (W6, P1, BR1) sind mit der Bereinigung nach Entscheidung 009 am 2026-11-12 erledigt oder entfallen; WZ44.4 belegt den Rest.

## Notizen

Zahlen aus der Planungsanalyse 2026-11-11 (Session k3c-03).
