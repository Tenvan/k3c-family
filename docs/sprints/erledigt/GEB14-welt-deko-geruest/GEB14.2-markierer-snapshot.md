# GEB14.2 · Snapshot nennt den Markierer eines Knotens

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** geb14/2-markierer-snapshot
- **Abhängig von:** –
- **Tickets:** B-532
- **Kriterien:** AC-03

## Ziel

Der Snapshot nennt den Markierer eines Knotens (`markedBy`).

## Kontext

B-532; `engine/sim/economy.go`, `save.go`.

## Erlaubte Dateien

- `engine/sim/node_mark*.go`, `economy.go`, `save.go`, `types.go`, `docs/protocol.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. `markNode`. 2. Spielstand additiv. 3. Protokoll nachtragen.

## Fertig, wenn

- [x] AC-03.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-03 umgesetzt: `ResourceNode.MarkedBy`, Spielstand `nodesMarkedBy` additiv (Version bleibt), `node_mark_test.go` (Snapshot, Roundtrip, alter Stand); Protokoll nachgetragen (v8 bleibt). Review (Sonnet) siehe Sprint-Abnahme.
