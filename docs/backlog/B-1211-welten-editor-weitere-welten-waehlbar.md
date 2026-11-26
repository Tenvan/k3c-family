# B-1211 · Im Welten-Editor lassen sich alle Welten anwählen, nicht nur die erste

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WK1
- **Projekt:** NAC
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Im Welten-Editor der Workbench (`tools/k3c-workbench/frontend/src/world/`) lässt sich nur die erste Welt anwählen (Meldung Ralf 2026-11-21). Ursache offen (Auswahl in `useWorld.ts`/`WorldPage.tsx`, Welten-Liste `worldList`).

## Ziel

Jede Welt der Liste ist im Editor anwählbar.

## Beteiligte und Zielgruppen

Ralf; Hauptrechner.

## Anforderungen

- **AC-01** Ursache finden und beheben; Test mit mindestens drei Welten: Auswahl wechselt Detail, Werte und Level.

## Nicht-Ziele

Neue Editor-Funktionen.

## Regeln und Einschränkungen

Atomic Design der webkit-Frontends (`tests/atomicDesign.test.ts`).

Planungsangaben für die Arbeitswellen:

- **Bereich:** `tools/k3c-workbench/frontend/src/world/**`, `tools/k3c-workbench/internal/world/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Liste standard, wueste, rom: Klick auf rom zeigt rom.

## Ausnahme- und Fehlerfälle

Welt lässt sich nicht laden: Fehlermeldung statt Stillstand.

## Akzeptanzkriterien

- **AC-01** Frontend-Test mit drei Welten (Auswahl wechselt), `check_run` grün.

## Offene Fragen

keine.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
