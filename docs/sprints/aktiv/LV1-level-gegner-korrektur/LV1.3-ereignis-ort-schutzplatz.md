# LV1.3 · Ereignisse mit Ort, Schutzplatz ohne Entity-ID

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
