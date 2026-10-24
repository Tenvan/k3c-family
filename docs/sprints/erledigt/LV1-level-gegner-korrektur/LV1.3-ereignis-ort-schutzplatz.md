# LV1.3 · Ereignisse mit Ort, Schutzplatz ohne Entity-ID

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** lv1/3-ereignis-ort
- **Abhängig von:** –
- **Tickets:** B-217, B-260
- **Kriterien:** AC-05, AC-06

## Ziel

`built` und `playerDown` tragen `x`; der Schutzplatz einer Truppe hängt nicht an ihrer Entity-ID.

## Kontext

B-217: Ereignisse in `engine/sim` (`emit`, `events.go`), Client-Ersatzlogik `src/scenes/effects/effects.ts` (`BuildSpots`, `playerX`), Typen `src/model/types.ts`, Doku `docs/protocol.md` › Ereignisse. Feld zusätzlich, keine neue Version; Client nutzt `x` und behält den Rückfall. B-260: `stepTroop` (`engine/sim/unit_common.go`) nutzt `w.HubX + t.ID%5 - 2`; neu: Index unter den Truppen. Golden ändern sich (eigener Commit).

## Erlaubte Dateien

- `engine/sim/`
- `src/model/types.ts`, `src/scenes/effects/effects.ts` (+Test), `docs/protocol.md`, `testdata/protocol/`
- `testdata/golden/` (nur per `task go:golden`, eigener Commit)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Neue Ereignistypen, andere Truppen-Verhalten.

## Schritte

1. Go-Test: `built` und `playerDown` mit `x`; emit ergänzen.
2. Client: `effectFor` nutzt `x`, Rückfall bleibt; Typ, Doku.
3. Schutzplatz über den Index der Truppe, Test: neuer unbezahlter Platz verschiebt keine Schutzplätze.
4. Golden in eigenem Commit.

## Fertig, wenn

- [ ] AC-05 (B-217/AC-01, AC-02): Go-Test und `task check` grün.
- [ ] AC-06 (B-260/AC-01): Test grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-05 (B-217/AC-01) geprüft:** `built` (`engine/sim/unit_common.go`) und `playerDown` (`engine/sim/common.go`) tragen `x` (Units, `unitX`); Tests `TestEventBuiltMitOrt`, `TestEventPlayerDownMitOrt` grün. Client: `GameEvent` mit optionalem `x`, `effectFor` nutzt `x`, Rückfall (`BuildSpots`, `playerX`) bleibt für ältere Server; Vitest `effects.test.ts` grün. `docs/protocol.md`: Zusatzfeld, keine neue Version.
- **AC-06 (B-260/AC-01) geprüft:** Schutzplatz `shelterX` aus dem Index in `w.Troops` (5 Plätze um den Hub), Test `TestSchutzplatzOhneEntityID` (ID-Verschiebung um 3 ändert keinen Platz). Golden: 8 Dateien neu (`task go:golden`, eigener Commit) – Ursache allein die geänderte Schutzplatz-Zuordnung.
- Review (Sonnet): Index -1 abgesichert, Protokoll-Satz korrigiert; durchgereichter Index statt Suche verworfen (nur bei Gefahr ohne Auftrag, wenige Dutzend Truppen). Bekannt: Stirbt eine Truppe, rücken die Plätze der folgenden nach (deterministisch).
