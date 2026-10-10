# B-385 · Das Ereignis `islandSwitch` erreicht beim Inselwechsel keinen Client

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-10
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`Room.Tick` (`engine/room/actions.go`) ruft `switchIsland` (`engine/room/island_switch.go`) direkt nach `sim.StepIsland` auf,
also vor `pushState`. Dabei werden die Ströme der Geräte geleert, und jedes Gerät bekommt `level` und `snap` der neuen Insel.
Die Ereignisse des letzten Ticks der alten Insel gehen deshalb nie raus, darunter `islandSwitch`, das in
`docs/protocol.md` › Ereignisse steht. Gefunden im Review K4.3. Heute ohne Wirkung, weil `data/` nur eine Insel kennt.

## Ziel

Ein Client erfährt über das Ereignis, dass die Insel gewechselt hat, und kann eine Meldung zeigen (K5).

## Beteiligte und Zielgruppen

Entwickler Server und Client (K5).

## Anforderungen

- Das Ereignis `islandSwitch` (und andere Ereignisse des letzten Ticks der alten Insel) kommt bei jedem Gerät an, oder das Protokoll beschreibt ausdrücklich, woran der Client den Wechsel erkennt.

## Nicht-Ziele

Darstellung des Wechsels (K5).

## Regeln und Einschränkungen

Protokoll-Grenzfall (`docs/arbeitsweise.md`): Protokoll und beide Enden in einer Session.

## Beispiele

Zwei Geräte, Endboss besiegt, alle stehen am Wechselpunkt → jedes Gerät erhält `islandSwitch`, danach `level` und `snap` der neuen Insel.

## Ausnahme- und Fehlerfälle

Letzte Insel: kein Wechsel, kein Ereignis.

## Akzeptanzkriterien

- **AC-01** Test in `engine/room/`: Beim Inselwechsel erhält jedes Gerät das Ereignis `islandSwitch` (oder den im Protokoll beschriebenen Ersatz) vor oder mit dem ersten `snap` der neuen Insel.

## Offene Fragen

Ereignis noch mit einem letzten `delta` der alten Insel senden oder im ersten `snap` der neuen mitliefern? Entscheidet 🧑 beim Einplanen.

## Notizen

–
