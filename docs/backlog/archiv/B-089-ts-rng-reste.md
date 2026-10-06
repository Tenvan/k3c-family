# B-089 · Der Client enthält keinen RNG-Rest der alten TS-Simulation mehr

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** –
- **Erstellt:** 2026-10-01
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach SP09 (TS-Simulation gelöscht) blieb `src/core/rng.ts` liegen. Es wurde nur noch als Typ `Rng` im Feld `World.rng` (`src/model/types.ts`) genutzt, das der Client mit `undefined as never` füllte (`src/online/clientWorld.ts`), dazu ein Test dafür in `tests/projectRules.test.ts`. `CLAUDE.md` nannte noch `createRng(seed)`.

## Ziel

Der Client kennt keinen RNG mehr; die Regel zur Determinismus-Pflicht verweist auf `engine/rng`.

## Beteiligte und Zielgruppen

Entwickler und Agenten; 🧑 hat das Aufräumen im Chat verlangt („ja, mach das“).

## Anforderungen

- `src/core/rng.ts` und `World.rng` entfallen, `clientWorld.ts` setzt kein `rng` mehr.
- `CLAUDE.md` nennt `engine/rng`.

## Nicht-Ziele

Der Protokoll-Schlüssel `rng` in `STATIC_KEYS` (`src/online/protocol.ts`, `clientProtocol.ts`, `docs/protocol.md`) bleibt; eine Protokoll-Änderung braucht eine eigene Session (Arbeitsweise › Protokoll).

## Regeln und Einschränkungen

Domäne CLI (`src/model`, `src/online/client*`); `tests/projectRules.test.ts` und `CLAUDE.md` sind INF-Dateien und nur für diese Folgeänderung angepasst.

## Beispiele

nicht relevant: reine Aufräumarbeit.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** `grep -rn "core/rng" src tests` liefert nichts, `task check` ist grün.

## Offene Fragen

keine

## Notizen

Rückwirkend aus der erledigten Arbeit abgeleitet. Offen bleibt der Schlüssel `rng` im Protokoll (`StaticKey`), siehe Nicht-Ziele.
