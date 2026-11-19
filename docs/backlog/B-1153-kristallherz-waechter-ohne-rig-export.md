# B-1153 · kristallherz-waechter hat keinen Rig-Export, gen:rig schreibt ihn nicht

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** LA1
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`public/grafik/bosse/kristallherz-waechter.paper.json` (Spec `version` 1) hat keinen Rig-Export unter `public/rig/bosse/`; `task gen:rig` schreibt ihn nicht (RU61.6, 2026-11-19: Export enthält nur die fünf Bosse goblin-anfuehrer, lava-golem, ratten-koenigin, splitter-titan, troll-koenig). Ursache ungeprüft: vermutlich fehlt der Eintrag im Katalog `tools/rigexport/catalog.ts` oder die Spec ist (noch) nicht rigfähig.

## Ziel

Geklärt ist, ob der Kristallherz-Wächter einen Rig-Export bekommt; entweder steht er unter `public/rig/bosse/kristallherz-waechter/` oder der Verzicht ist begründet dokumentiert.

## Beteiligte und Zielgruppen

Agent PAP (Agent `k3c-grafik` prüft); Entscheidung bei 🧑, falls der Spec-Umbau (B-720-Linie) nötig wäre.

## Anforderungen

- Ursache klären (Katalog `tools/rigexport/catalog.ts`, Spec-Teile, Pivots).
- Ist der Export mit kleinem Aufwand möglich: aufnehmen, `task gen:rig:check` für die Gruppe grün, `k3c-rig`-Tests (`boss_tests`) um den Boss erweitern.
- Sonst: Verzicht im Ticket begründen und schließen.

## Nicht-Ziele

Boss-Spec neu bauen (Spec-Umbau wäre ein eigenes Ticket); die fünf bereits exportierten Bosse (B-885).

## Regeln und Einschränkungen

Gen-Lauf am Feature (B-742).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `tools/rigexport/**`, `public/rig/bosse/**`, `rust/crates/k3c-rig/src/rig/boss_tests.rs`
- **Abhängig von:** B-885
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Katalog nimmt den Boss auf → `task gen:rig` schreibt `public/rig/bosse/kristallherz-waechter/rig.json`.

## Ausnahme- und Fehlerfälle

Spec ohne Pivot-Teile → nicht an der Spec drehen, Spec-Ticket vorschlagen.

## Akzeptanzkriterien

- **AC-01** Entweder liegt `public/rig/bosse/kristallherz-waechter/rig.json` vor und `task gen:rig:check` ist für die Gruppe grün, oder der Verzicht steht begründet im Ticket.

## Offene Fragen

Bekommt der Kristallherz-Wächter überhaupt einen Rig? → Default: nur wenn der Export ihn mit kleinem Aufwand schreibt.

## Notizen

Gefunden in RU61.6 (2026-11-19), zu B-885 offene Frage.
