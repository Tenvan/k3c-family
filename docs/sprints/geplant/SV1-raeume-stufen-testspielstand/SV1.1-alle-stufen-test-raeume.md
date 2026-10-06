# SV1.1 · Raum mit allen Stufen, leere Test-Räume schließen sofort

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** sv1/1-alle-stufen-test-raeume
- **Abhängig von:** –
- **Tickets:** B-199, B-290, B-204
- **Kriterien:** AC-02, AC-03, AC-04, AC-06

## Ziel

Ein neu angelegter Raum hat so viele Stufen, wie es Biome gibt (heute fünf), und ein leerer Test-Raum (`test-…`) ist nach dem nächsten Sweep weg statt nach `EmptyFor`.

## Kontext

- `engine/room/manager.go` › `open` (Zeile ~161) legt eine neue Insel fest mit `sim.CreateIsland(name, []int{0, 1, 2}, 1)` an und lehnt `depth < 0 || depth > 2` mit `ErrBadRequest` ab; der Kommentar über `open` nennt „Tiefe 0, 1, 2“. Seit W2.2 gibt es `data/biomes/ironhold.json` (Tiefe 3) und `crystal.json` (Tiefe 4); die Sim kann fünf Stufen (`engine/sim/island_depths_test.go` › `TestInselFuenfStufen`).
- Die Biome lädt die Sim intern (`engine/sim/data.go` › `loadBiomes`, nach Tiefe sortiert, nicht exportiert). Der Raum darf `engine/sim/` nicht ändern (SIM). Die Tiefen also im Raum aus `data.Files` (`biomes/*.json`) mit `level.LoadBiome` lesen, wie `engine/net/level.go` › `levelBiomes`; ob es schon einen exportierten Weg gibt, vorher per Grep prüfen (ungeprüft). Lücke in den Tiefen → Fehler beim Laden, nicht beim Spielen (B-199).
- Geladene Spielstände behalten ihre Stufen (`sim.FromIslandSave`), kein Formatwechsel.
- `engine/net/level_test.go` (Zeile 63) prüft fest `{"forest", "cave", "mine"}`; darf auf alle Biome erweitert werden.
- B-204: `engine/room/manager.go` › `lockedSweep` räumt auf, wenn `Room.sweep` (`engine/room/actions.go`, Zeile ~233) `connected() == 0` und `EmptyFor` (`engine/room/room.go`, 10 min) abgelaufen meldet; danach `dropTestSave()` (`room.go`). `TestPrefix = "test-"` steht in `manager.go`. Wartende Monarchen (`Waiting`, Frist `WaitFor`) zählen als „noch nicht leer“ und halten den Test-Raum offen.
- Die Testseite (B-086, `src/tools/testScenarios.ts`, `testing.html`) startet Räume mit `fresh=1&save=test-…`. Vermutet: Sie braucht den leeren Raum nicht länger als bis zum nächsten Sweep (Schritt 6 prüft das).
- Datei ≤ 400 Zeilen (`manager.go` hat 333), Funktion ≤ 60.

## Erlaubte Dateien

- `engine/room/`
- `engine/net/level_test.go`
- Planungs-Dateien (`docs/sprints/`, `docs/backlog/`)

## Nicht-Ziele

Client-Biome (SV1.2), `data/islands.json` und Inselwechsel (B-103, K2), Protokoll-Felder, andere Fristen für normale Räume, höhere Raum-Grenze.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Test zuerst in `engine/room/`: neuer Raum hat so viele Stufen wie Biome (fünf); Startstufe 4 angenommen, 5 abgelehnt; gespeicherter Stand mit drei Stufen lädt mit drei.
3. In `open` die feste Liste durch die Tiefen aus den Biomen ersetzen (0 bis tiefste, lückenlos), die Startstufen-Prüfung folgt derselben Liste; Kommentar anpassen.
4. Test zuerst für B-204: Test-Raum ohne verbundenes oder wartendes Gerät ist nach dem nächsten `Sweep` weg und sein Spielstand gelöscht; mit wartendem Monarchen bleibt er bis `WaitFor`; normaler Raum bleibt `EmptyFor` offen (Uhr des Managers wie in `engine/room/testsave_test.go`).
5. In `Room.sweep` bzw. `lockedSweep` für Räume mit `TestPrefix` die Frist `EmptyFor` überspringen; Log-Meldung 🧹 nennt den Grund.
6. Prüfen, ob die Testseite den leeren Raum länger braucht: `src/tools/testScenarios.ts` und `testing.html` lesen (Wiederverbinden nach Trennen?). Ergebnis ins Ergebnis dieser Session; widerspricht es der Annahme, AC-04 nicht umsetzen, sondern als offen für 🧑 melden.
7. Optional `engine/net/level_test.go` auf alle Biome erweitern.

## Fertig, wenn

- [ ] AC-02: Go-Test in `engine/room/` belegt fünf Stufen, Startstufe 4 ja / 5 nein, gespeicherter Stand mit drei Stufen lädt mit drei.
- [ ] AC-03: Derselbe Test belegt den Server-Teil (B-290/AC-01: neuer Raum hat fünf Stufen).
- [ ] AC-04: Go-Test belegt: leerer Test-Raum nach dem nächsten Sweep weg, Spielstand gelöscht; wartender Monarch hält ihn offen; normaler Raum bleibt `EmptyFor`.
- [ ] Prüfung der Testseite (Schritt 6) steht im Ergebnis.
- [ ] AC-06: `task check:go` grün.

## Prüfen

```bash
task check:go
```

## Ergebnis

–
