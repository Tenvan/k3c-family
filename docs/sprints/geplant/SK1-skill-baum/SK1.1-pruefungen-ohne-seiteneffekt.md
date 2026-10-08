# SK1.1 · Lern- und Respec-Regeln ohne Seiteneffekt abfragbar

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** sk1/1-pruefungen-ohne-seiteneffekt
- **Abhängig von:** –
- **Tickets:** B-270
- **Kriterien:** AC-02, AC-03

## Ziel

`engine/sim` exportiert `CanLearn(w, p, id) error` und `CanRespec(w, p) error`; `LearnSkill` und `Respec` nutzen sie selbst, Tests zeigen gleiche Fehler und unveränderte Spieler.

## Kontext

- `engine/sim/monarch.go` (150 Zeilen): `LearnSkill(w, p, id)` prüft in einem `switch` unbekannte ID, schon gelernt, keine Punkte frei (`AvailablePoints`), Tier-Gating (`lineCount` < `tierNeed`) und lernt danach sofort (hängt an `p.Skills` an, belegt einen Slot). `Respec(w, p)` prüft Tag (`w.Cycle.Phase != "day"`) und Abstand zur Burg (`hub.CastleRadiusUnits`) und leert danach `p.Skills`, `p.Slots`.
- Umbau: die Prüfungen wandern unverändert (gleiche Fehlertexte, gleiche Reihenfolge) in `CanLearn`/`CanRespec`; `LearnSkill`/`Respec` rufen sie zuerst auf. So gibt es keine zweite Kopie der Regeln.
- Beschluss 🧑 2026-10-06: Respec ohne gelernte Skills ist erlaubt (`CanRespec` nil), ohne Wirkung, wie heute.
- Aufrufer: `engine/room/skills.go` (`Room.Learn`, `Room.Respec`), `engine/sim/monarch.go` › `ApplyPreset`; ihre Signaturen bleiben gleich. `engine/net/actions.go` nennt `respec` noch nicht (Kommentar verweist auf B-270); das nachzuziehen ist SRV und gehört nicht in diesen Sprint.
- Regeln: `docs/rules/monarch.md`. Bestehende Tests: `engine/sim/monarch_test.go` (216 Zeilen, u. a. `TestMonarchTierGating`, `TestMonarchRespecNurAmTagAnDerBurg`, Helfer `mustLearn`).
- Fallstrick: `monarch_test.go` nähert sich 400 Zeilen; neue Tests in eine eigene Datei `engine/sim/monarch_can_test.go`.

## Erlaubte Dateien

- `engine/sim/monarch.go`
- `engine/sim/monarch_can_test.go` (neu)
- Planungs-Dateien (`docs/sprints/`, `docs/backlog/B-270-*.md`)

## Nicht-Ziele

Protokoll und Aktionsliste (`engine/net/actions.go`), Client-Anzeige (S3), neue Regeln oder geänderte Fehlertexte.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. `CanLearn(w, p, id) error` aus dem `switch` von `LearnSkill` herauslösen, `LearnSkill` ruft sie zuerst auf.
3. `CanRespec(w, p) error` aus den beiden Prüfungen von `Respec` herauslösen, `Respec` ruft sie zuerst auf.
4. Tabellentest in `engine/sim/monarch_can_test.go`: je Fehlerfall (unbekannt, schon gelernt, keine Punkte, Tier fehlt, Nacht, fern der Burg) und je Erfolgsfall liefert `Can…` denselben Fehler (bzw. nil) wie die Aktion auf einer Kopie des Spielers, und `p.Skills`/`p.Slots` sind nach `Can…` unverändert (`slices.Equal`).
5. Fall Respec ohne gelernte Skills am Tag an der Burg: `CanRespec` nil.
6. `task check:go` grün, Golden-Daten unverändert.

## Fertig, wenn

- [ ] AC-02: `go test ./engine/sim -run Can` grün; die Tests vergleichen Fehler von `CanLearn`/`CanRespec` mit `LearnSkill`/`Respec` und prüfen, dass der Spieler unverändert bleibt.
- [ ] AC-03: Golden-Daten unverändert (kein Diff unter `testdata/`), `task check:go` grün.

## Prüfen

```bash
task check:go
```

## Ergebnis

–
