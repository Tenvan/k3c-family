# WZ9.3 · Prüfung und Bau aus einer Kandidaten-hub.json ohne globalen Zustand

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** wz9/3-pruefung-kandidat
- **Abhängig von:** WZ9.2
- **Tickets:** B-462
- **Kriterien:** AC-02, AC-09, AC-10

## Ziel

`engine/sim` bietet eine exportierte Prüfung und einen Bau-Weg für eine Kandidaten-`hub.json` als Parameter (kein globaler Zustand); Mauerlinien sind Parameter von Generator und Planungs-Prüfung in `engine/level`; die 7-Stufen-Fixture besteht die Prüfung; Goldens bleiben byte-gleich.

## Kontext

- Heutige Platz-Regeln stehen nur als Go-Tests in `engine/sim/site_data_test.go` (darin `hubLevelRule`, Zeile ~185, wird entfernt). `engine/sim` liest `data/hub.json` per `go:embed`.
- `sim.CreateOptions` ist eine **neue** Struktur nur für die Erzeugung (nicht gespeichert), getrennt von `sim.IslandOptions` (`engine/sim/island_options.go`). Spielstand-Format und `IslandSaveVersion` bleiben unverändert. Hat WZ8.3 `CreateOptions` schon angelegt, darauf aufbauen und den Kandidaten als Feld ergänzen; sonst hier anlegen und alle Aufrufer von `sim.CreateIsland` anpassen.
- Mauerlinien als Parameter von Generator und Planungs-Prüfung in `engine/level` (Standard eingebettet): wenn WZ8.2/WZ8.3 sie angelegt haben, darauf aufbauen, sonst hier anlegen (Zuschnitt wie B-461 › Anforderungen). `engine/level` importiert nichts aus `engine/sim`, `engine/room`, `engine/net`.
- Geplante Chunks (WZ8, `data/islands.json`) werden mitgeprüft, wenn WZ8 erledigt ist; sonst Hinweis „geplante Chunks nicht geprüft“ und ein Ticket zur Nachrüstung.
- Ein brechender Golden-Test ist ein Fehler im Umbau; kein `task golden:update`. Wird die Session zu groß (Budget), teilen: Prüfung / Bau aus Kandidat, Kriterien nicht umformulieren.
- **Regel „genau ein Platz je Bau“ wird geteilt:** `TestSiteDataGenauEinPlatzJeBau` (`site_data_test.go:98-118`) verlangt heute für jede Art aus `data/buildings.json` (außer Burg, Mauer, Turm, Tor) genau einen Platz, `farm` je Seite einen. Mit entfernbaren Arten gilt in der Prüfung: **Pflichtarten genau ein Platz** (`farm` je Seite einer), **optionale Arten höchstens einen**; mehr als einen Platz derselben Art bleibt Befund.

### Pflichtplätze (festgelegt aus dem Code, Abnahmepunkt für 🧑 in WZ9.8)

Der Planer darf diese Plätze weder entfernen noch ganz aus `sites`/`islandSites` nehmen lassen; die Prüfung meldet ihr Fehlen als Befund. Grundlage ist der Code-Stand zur Planung (Heute-Stand `data/hub.json`):

| Pflicht | Art | Grund im Code |
|---|---|---|
| ja | `workshop` | Bogenschützen und Krieger entstehen nur mit Bogen und Schwert aus der Werkstatt (`engine/sim/upgrades.go`, `warrior.go`, `economy.go`); ohne Werkstatt gibt es keine Kämpfer (Bauern und Landstreicher entstehen auch ohne). |
| ja | `stairsDown` | Stufenwechsel nach unten (`engine/sim/travel.go`); ohne Treppe kein Weg in die tiefere Stufe. |
| ja | `stairsUp` | Rückweg nach oben (`travel.go`, `fromDepth`); in der Tiefe 0 gibt es sie nicht. |
| ja | `farm`, je Seite genau einer | Holz-Nachwuchs (Plantage); `TestSiteDataGenauEinPlatzJeBau` verlangt je Seite genau einen. |
| nein | `barracks`, `tavern`, `storage` (`islandSites`), `healer`, `smithy`, `armory` | Ausbau und Komfort (Limit, Nachwuchs, Lager, Heilung, Elite, Rüstung); das Spiel läuft ohne sie weiter, nur die Funktion fehlt. Entfernen ist erlaubt, der Planer weist darauf hin. |
| nicht in `sites` | Burg, Mauern, Türme, Tore, Händler | kommen aus Hub-Mitte, `wallLines` und `merchant`; nicht entfernbar, nur bearbeitbar (mindestens eine Mauerlinie, `gateHubLevel` im Bereich). Die Burg ist kein Pflichteintrag in `sites`. |

Die Tabelle ist die Festlegung aus dem Code-Stand dieser Planung. WZ9.3 prüft sie am dann aktuellen Code (neue Pflichtarten, die seither entstanden sind, kommen dazu), trägt die Liste als Konstante in die Prüfung ein und nennt sie im Ergebnis. 🧑 bestätigt die Liste bei der Abnahme (WZ9.8).

## Erlaubte Dateien

- `engine/sim/` (Prüfung, Kandidaten-Bau, `CreateOptions`, Tests, `site_data_test.go`)
- `engine/level/` (Linien als Parameter, Tests)
- Aufrufer von `sim.CreateIsland` (nur Anpassung der Signatur)
- `testdata/hub/` (7-Stufen-Fixture, falls WZ9.2 sie nicht schon angelegt hat)

## Nicht-Ziele

Kein `sim.UseData` und keine globale Umschaltung. Keine Änderung an `data/hub.json`, `testdata/golden/` oder dem Spielstand-Format. Keine neuen Gebäudearten oder Felder. Kein Snapshot-Anschluss (WZ9.4).

## Schritte

1. Stand von WZ8 prüfen (`CreateOptions`, Linien-Parameter in `engine/level`) und danach den Zuschnitt wählen.
2. Regeln aus `site_data_test.go` in eine exportierte Funktion `ValidateHub(candidate, ...)` verlegen (Kandidat als Parameter, Befunde als Liste); mit WZ8 geplante Chunks gegen die Kandidaten-Linien mitprüfen.
3. Zusätzliche Regeln für hinzugefügte und entfernte Plätze (B-462 Entscheidung 5): nur bekannte `kind`-Arten (Liste aus Simulation und `data/buildings.json`); die Pflichtplätze aus der Tabelle im Kontext (Werkstatt, Treppe runter, Treppe hoch, Farm je Seite) dürfen nicht fehlen; Regel „ein Platz je Bau“ geteilt: Pflichtarten genau einen, optionale höchstens einen. Liste am aktuellen Code prüfen und im Ergebnis nennen.
4. `engine/level`: Mauerlinien als Parameter von Generator und Planungs-Prüfung, Standard eingebettet.
5. `sim.CreateOptions` um das Feld Kandidat ergänzen; Plätze-, Stufen- und Linien-Berechnung als Variante für den Bau aus einem Kandidaten (Grundlage für WZ9.4).
6. Tests auf die Funktion umstellen, `hubLevelRule` entfernen.
7. 7-Stufen-Fixture (Farm, Linie 1, `payRangeUnits`, Inset, Streuung frei wählbar) besteht die Prüfung; Test dazu.
8. `task check:go`; Golden-Diff muss leer sein. Wird die Session zu groß, in zwei Sessions teilen (Prüfung / Bau aus Kandidat), Kriterien nicht umformulieren.

## Fertig, wenn

- [ ] AC-02: Prüfung über eine Funktion in `engine/sim` ohne globalen Zustand; Linien als Parameter in `engine/level`; geplante Chunks mitgeprüft, sofern WZ8 erledigt (sonst Hinweis und Ticket).
- [ ] AC-02: Pflichtplätze (Tabelle im Kontext) als Konstante in der Prüfung; Go-Test je Pflichtart: Entfernen → Befund, optionale Arten entfernen → kein Befund; die Liste steht im Ergebnis (Abnahmepunkt für WZ9.8).
- [ ] AC-09: 7-Stufen-Fixture besteht die Prüfung; im Code keine feste 5.
- [ ] AC-10: `testdata/golden/` und `data/hub.json` unverändert; `hubLevelRule` entfernt.
- [ ] `go test ./engine/...` und `task check:go` grün.

## Prüfen

```bash
task check:go
git diff --stat origin/develop -- data/hub.json testdata/golden
```

Unter k3c-dev: `check_run` mit `checkout`. Keine manuelle Prüfung.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
