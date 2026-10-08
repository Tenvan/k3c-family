# B-190 · Der Client erfährt zuverlässig, wie viele Ereignisse verworfen wurden

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NT1
- **Projekt:** LST
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

F3.2 hat `World.EventsDropped` (`engine/sim/types.go`, JSON `eventsDropped,omitempty`) eingeführt: Zahl der Ereignisse, die die Obergrenze je Tick verworfen hat. Mit `omitempty` fehlt das Feld bei 0, damit die Protokoll-Beispiele (`TestFormWieBeispiele`, `engine/net/ws_test.go`) gültig bleiben. Das Delta (`engine/net/delta.go`) überträgt aber nur Schlüssel, die im neuen Zustand stehen: Fällt der Wert von n auf 0, behält der Client den alten Wert.

## Ziel

Der Client sieht je Tick die richtige Zahl verworfener Ereignisse (für Anzeige und Messung in F4).

## Beteiligte und Zielgruppen

Entwickler von F4 (Protokoll, Benchmark B-140), Debug-Overlay.

## Anforderungen

- `eventsDropped` steht im Protokoll beschrieben; ein Rückgang auf 0 erreicht den Client (z. B. Feld immer senden oder im `stateOf` setzen), Beispiele in `testdata/protocol/` passen dazu.

## Nicht-Ziele

Obergrenze und Priorität selbst (F3.2), Anzeige im Client.

## Regeln und Einschränkungen

Domäne SRV; Protokolländerung nur in einer eigenen Session, beide Enden gemeinsam (`docs/arbeitsweise.md` › Protokoll).

## Beispiele

Tick 100: 5 verworfen → Client zeigt 5; Tick 101: 0 verworfen → Client zeigt 0.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Übertragung eines Zählers.

## Akzeptanzkriterien

- **AC-01** Ein Test in `engine/net` zeigt: Nach einem Tick mit verworfenen Ereignissen und einem ohne hat der aus `snap` und `delta` zusammengesetzte Zustand `eventsDropped` 0; `docs/protocol.md` beschreibt das Feld.

## Offene Fragen

keine

## Notizen

Entstanden in F3.2 (2026-10-03); passt in F4 (B-140).
