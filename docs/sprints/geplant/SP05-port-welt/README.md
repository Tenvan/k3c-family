# SP05 · SIM · Port I – Welt, Zyklus, Wirtschaft

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-043
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Simulation gibt es nur in TypeScript (`src/world/sim/`); ab SP04 liegen Golden-Läufe vor.

## Ziel

Die Go-Simulation rechnet Welt, Tag/Nacht und Wirtschaft Tick für Tick wie TypeScript. Am Ende sichtbar: Golden-Läufe ohne Gegner grün.

## Beteiligte und Zielgruppen

Entwickler oder Agent; Spieler profitieren erst ab SP08.

## Anforderungen

B-043 › Anforderungen. Sprint-eigen: Zustand (`types.ts`), `createWorld`, Tag/Nacht (`cycle.ts`) und Wirtschaft (`economy.ts`) in Go.

## Nicht-Ziele

Einheiten, Gegner, Reisen (SP06).

## Regeln und Einschränkungen

`src/world/` wird nur gelesen (Grenzfall Portierung); Schichtgrenze: `engine/sim` importiert nichts aus `engine/room`, `engine/net`, `cmd/`.

## Beispiele

Golden-Lauf mit festem Seed und 2 Spielern ohne Gegner → identische Snapshots alle N Ticks.

## Ausnahme- und Fehlerfälle

Abweichung → der Test nennt Tick und Feld.

## Akzeptanzkriterien

- **AC-01** Zustand, `createWorld` und Tag/Nacht in Go bestehen die passenden Golden-Läufe.
- **AC-02** Die Golden-Läufe ohne Gegner sind grün (B-043/AC-02 für diesen Teil).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SP05.1 Zustand (`types.ts` → Go-Typen), `createWorld`, Tag/Nacht (`cycle.ts`) (AC-01).
- SP05.2 Wirtschaft (`economy.ts`) (AC-02).
- SP05.3 🔍 Review (alle).

## Abnahme

–
