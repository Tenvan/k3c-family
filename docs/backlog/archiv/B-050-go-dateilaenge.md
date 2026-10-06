# B-050 · Die Dateilänge von Go-Code wird wie bei TypeScript geprüft

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** SP01
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (mit SP01 Revision 2)

## Ausgangslage

Das Komplexitäts-Budget (`docs/arbeitsweise.md`) gilt für TypeScript und Go: Datei-Ziel 300 Zeilen, harte Grenze 400.
SP01 prüft die Dateilänge nur für TypeScript: Oxlint `max-lines` (SP01.1) und der Regel-Test mit Ratsche in
`tests/projectRules.test.ts` (SP01.3, nur `.ts`). Für Go richtet SP01.2 `funlen`, `gocyclo`, `nestif` und `depguard` ein;
in der Linter-Liste von golangci-lint (golangci-lint.run, Stand 2026-09-30) gibt es keinen Linter für die Dateilänge.

## Ziel

Die Dateilänge von Go-Code wird wie bei TypeScript geprüft. Nutzen: Das Budget gilt für beide Werkzeugketten
tatsächlich, nicht nur auf dem Papier; große Go-Dateien fallen vor dem Review auf.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die Go-Code schreiben (ab SP03/SP04); Review-Session.

## Anforderungen

- Jede Go-Datei außer `*_test.go` hat höchstens 300 Zeilen oder steht mit ihrem Wert in der Ausnahmeliste (Ratsche).
- Harte Grenze 400 Zeilen, auch für Tests (wie in `docs/arbeitsweise.md` › Komplexitäts-Budget).
- Eine Ausnahme, die nicht mehr nötig ist, lässt die Prüfung scheitern (wie bei TypeScript).

## Nicht-Ziele

Neuer Go-Linter oder eigenes Go-Programm nur für diese Prüfung; Dateien verkleinern.

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.
Vorschlag: den vorhandenen Regel-Test aus SP01.3 erweitern (Endung `.go`, Ordner `engine/`, `cmd/`, `data/`) und
dieselbe Ausnahmeliste `tests/complexity-baseline.json` nutzen, statt ein zweites Werkzeug einzuführen.

## Beispiele

- `engine/sim/world.go` mit 320 Zeilen ohne Eintrag in der Ausnahmeliste → `npm test` scheitert.
- `engine/sim/world_test.go` mit 380 Zeilen → erlaubt (Tests nur harte Grenze).

## Ausnahme- und Fehlerfälle

- Go-Datei mit mehr als 400 Zeilen → scheitert immer, auch mit Ausnahme-Eintrag.
- Noch kein Go-Code im Repo → die Prüfung läuft ohne Treffer grün.

## Akzeptanzkriterien

- **AC-01** Eine Go-Datei (außer `*_test.go`) über 300 Zeilen ohne passenden Eintrag in der Ausnahmeliste lässt `npm test` scheitern (Probe, danach zurücknehmen).
- **AC-02** Eine Go-Datei über 400 Zeilen lässt die Prüfung scheitern, auch als Test-Datei (Probe, danach zurücknehmen).
- **AC-03** Ein überflüssiger Go-Eintrag in der Ausnahmeliste lässt `npm test` scheitern.

## Offene Fragen

keine. Entschieden (🧑, 2026-09-30): Umsetzung als Erweiterung des Regel-Tests in SP01.3, SP01 dafür auf Revision 2.

## Notizen

Entstanden bei der Abwägung Oxlint gegen Biome (Chat 2026-09-30): Beide TS-Werkzeuge prüfen die Dateilänge, auf der Go-Seite fehlt sie.
