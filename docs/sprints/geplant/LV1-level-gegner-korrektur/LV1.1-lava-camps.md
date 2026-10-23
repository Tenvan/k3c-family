# LV1.1 · Lava und Camps im Level-Generator

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** lv1/1-lava-camps
- **Abhängig von:** –
- **Tickets:** B-291, B-262
- **Kriterien:** AC-01, AC-02

## Ziel

Kein Lava-Streifen liegt auf einem Linien-Platz; die Camp-Regel bleibt wie beschlossen und ist vermerkt.

## Kontext

Lava-Chunks: `engine/level/level.go` (Chunk-Wahl), Streifen 8 Units in der Chunk-Mitte (`engine/sim/world_lava.go`); Linien-Plätze `engine/level/lines.go` (`clearOfLines`). Default (LV1 › Offene Fragen): Ein Lava-Chunk, dessen Streifen eine Linie überlappt, wird ohne zusätzlichen Würfel zu einem gewöhnlichen Chunk. B-262: „so lassen“ (🧑 2026-10-04), Camp-Test `camps_test.go`.

## Erlaubte Dateien

- `engine/level/`, `engine/sim/world_lava*.go`
- `testdata/golden/` (nur per `task go:golden`, eigener Commit)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Lava-Schaden an Gegnern, Camp-Regel ändern.

## Schritte

1. Test über 500 Seeds: kein Lava-Streifen auf einem Linien-Platz (rot).
2. Regel im Generator.
3. Golden in eigenem Commit, B-262 vermerken.

## Fertig, wenn

- [ ] AC-01 (B-291/AC-01): Test 500 Seeds grün.
- [ ] AC-02 (B-262/AC-01): Regel vermerkt, Camp-Tests grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
