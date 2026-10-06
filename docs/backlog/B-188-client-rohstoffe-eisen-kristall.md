# B-188 · Der Client kennt alle fünf Rohstoffe des Servers

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** eingeplant
- **Sprint:** W8
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`ResourceKind` in `src/model/data.ts` kennt nur `wood`, `stone`, `copper`; der Server führt seit SP13 fünf Materialien (`engine/sim/island_storage.go` › `stockField`: auch `iron`, `crystal`). DBG2.1 hat deshalb für die Dev-Aktion `material` einen eigenen Typ `DevResource` in `src/online/clientProtocol.ts` angelegt. Anzeigen für Eisen und Kristall (Vorrat, HUD) fehlen im Client.

## Ziel

Ein Typ für Rohstoffe im ganzen Client, passend zum Server; `DevResource` fällt weg.

## Beteiligte und Zielgruppen

Entwickler (Client); Spieler sehen später Eisen und Kristall im HUD (W6).

## Anforderungen

- `ResourceKind` nennt alle Rohstoffe, die der Server im Zustand schickt; `Stock` und `DevMessage` nutzen ihn.

## Nicht-Ziele

Darstellung im HUD (W6), Sim-Regeln für Eisen und Kristall (B-115, W2).

## Regeln und Einschränkungen

Domäne CLI; der Client rechnet nichts (`src/scenes/noSim.test.ts`).

## Beispiele

Ein Snapshot mit `stock.iron` → der Typ `Stock` kennt das Feld, kein `as`-Cast nötig.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Typänderung.

## Akzeptanzkriterien

- **AC-01** `ResourceKind` enthält `iron` und `crystal`; `DevResource` ist entfernt; `task check` grün.

## Offene Fragen

keine

## Notizen

Entstanden in DBG2.1 (2026-10-03).
