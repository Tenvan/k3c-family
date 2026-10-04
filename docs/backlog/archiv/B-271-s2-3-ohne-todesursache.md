# B-271 · Der Spielmetrik-Report wartet auf B-182 oder startet ohne Todesursache

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** S2
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Session S2.3 (Spielmetrik-Report, B-150) hängt von B-182 ab: „ohne sie ist ‚Tod durch was‘ nicht erfüllbar, dann nicht beginnen“. B-182 ist offen, `playerDown` (`engine/sim/common.go`, `applyDamage`) trägt nur `player`, keine Ursache. S2.3 steht deshalb auf `blockiert` (2026-10-04). S2 ist ein SRV-Sprint und darf `engine/sim/` nicht ändern.

## Ziel

🧑 entscheidet, wie S2.3 weitergeht, damit S2 abgeschlossen werden kann.

## Beteiligte und Zielgruppen

🧑 (Entscheidung), Entwickler (Server, Sim).

## Anforderungen

- Eine der Möglichkeiten unten ist gewählt und in S2.3 bzw. im Sprint vermerkt.

## Nicht-Ziele

Die Umsetzung von B-182 selbst (SIM-Sprint).

## Regeln und Einschränkungen

Ein Sprint bleibt in seiner Domäne (`docs/arbeitsweise.md`); Beschluss Q12 verlangt „Tod durch was“.

## Beispiele

nicht relevant: Entscheidungsfrage.

## Ausnahme- und Fehlerfälle

nicht relevant: Entscheidungsfrage.

## Akzeptanzkriterien

- **AC-01** S2.3 nennt die gewählte Möglichkeit und ist wieder `offen` oder aus S2 herausgenommen.

## Offene Fragen

Möglichkeiten (🧑): (1) B-182 als kleinen SIM-Sprint vor S2.3 ziehen (empfohlen, Q12 bleibt vollständig); (2) S2.3 jetzt mit `cause: null` umsetzen und das Feld mit B-182 nachziehen (Schema 1 bleibt, AC-05 nur teilweise); (3) S2.3 in einen späteren Sprint verschieben, S2 ohne B-150 abschließen.

## Notizen

Überholt (2026-10-04, S2.3): B-182 ist mit W0 erledigt (#127), `playerDown` trägt `cause`. S2.3 lief damit wie geplant (Möglichkeit 1 ohne eigenen Sprint), AC-01 erfüllt.
