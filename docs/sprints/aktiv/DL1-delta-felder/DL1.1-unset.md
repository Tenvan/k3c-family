# DL1.1 · Delta mit `unset` für verschwundene Felder (Protokoll, beide Enden)

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** dl1/1-unset
- **Abhängig von:** –
- **Tickets:** B-297
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Das `delta` trägt eine Liste `unset` mit den Namen der Felder, die seit dem letzten Zustand fehlen; der Client entfernt
sie. Der Delta-Test deckt `castle` weiter ab, auch mit der Verlust-Kaskade aus W4.3a.

## Kontext

- Server: `engine/net/delta.go` (`deltaOf`, `idLists`); Zustand aus `stateOf`. Test: `engine/net/delta_test.go`
  (`apply` = Client-Nachbau, `TestDeltaErgibtJedenVollenZustand` spielt `testdata/golden/sim-cave-belagerung.json`
  ab Tick 2400 nach und verlangt Änderungen an `players`, `coins`, `troops`, `enemies`, `projectiles`, `events`,
  `cycle`, `castle`).
- Client: `src/online/clientDelta.ts` (`applyDelta`), Test `src/online/clientDelta.test.ts`.
- Protokoll: `docs/protocol.md` › „Zustand und Delta“ (`null` ist ein Wert, kein Löschen). Beispiele
  `testdata/protocol/s2c-snapshot-delta*.json`, Form-Prüfung `TestFormWieBeispiele` (`engine/net`) und die Vitest-Prüfung
  der Beispiele.
- Betroffene optionale Felder heute: `storms`, `merchant` (W4.2, liegt auf `sprint/w4`); mit W4.3a `drops`.
- Der W4.3a-Stand ohne Tests liegt auf `origin/wip/w4.3a-krieger`. Er macht heute zwei Fehler im Delta-Test:
  „Tick 3057: Zustand weicht ab“ (fehlendes Entfernen) und „Feld castle hat sich im Lauf nie geändert“ (Fenster).

## Erlaubte Dateien

- `engine/net/delta.go`, `engine/net/delta_test.go`
- `src/online/clientDelta.ts`, `src/online/clientDelta.test.ts`
- `docs/protocol.md` (Abschnitt „Zustand und Delta“), `testdata/protocol/` (nur ein Beispiel mit `unset`, falls nötig)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Verlust-Kaskade, Änderungen an `engine/sim/`, Protokoll-Version, Anzeige.

## Schritte

1. Server: In `deltaOf` jedes Feld aus `prev`, das in `cur` fehlt (außer `events`), sortiert in `d["unset"]`; fehlt
   keins, kein `unset`.
2. Go-Test (AC-01): `prev` mit `merchant` und `drops`, `cur` ohne → `unset` = `["drops", "merchant"]`, `apply` (um
   `unset` ergänzt) ergibt `cur`; ein Feld auf `null` steht nicht in `unset`; `events` nie in `unset`.
3. Client (AC-02): `applyDelta` löscht die Felder aus `unset`; Vitest dazu, bestehende Tests bleiben grün.
4. Testfenster (AC-03): Lauf oder Fenster in `TestDeltaErgibtJedenVollenZustand` so wählen, dass `castle` sich ändert,
   ohne die Abdeckung der anderen Felder zu verlieren. Gegenprobe lokal auf einem Wegwerf-Branch:
   `git merge origin/wip/w4.3a-krieger`, dann `go test ./engine/net -run TestDelta`; kein „Zustand weicht ab“ und
   `castle` abgedeckt (ein Golden-Abweichen aus dem fehlenden W4.3a-Golden-Update zählt nicht). Nicht einchecken.
5. `docs/protocol.md` ergänzen (AC-04); Beispiel in `testdata/protocol/` nur, wenn die Form-Prüfung es verlangt.
6. `task check` und `task check:go`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-01: Go-Test für `unset` grün.
- [ ] AC-02: Vitest für `applyDelta` mit `unset` grün.
- [ ] AC-03: `TestDeltaErgibtJedenVollenZustand` grün, `castle` abgedeckt; Gegenprobe mit W4.3a-Stand im Ergebnis.
- [ ] AC-04: Protokoll beschrieben; `task check` und `task check:go` grün; keine Datei > 400 Zeilen, keine Funktion > 60.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

–
