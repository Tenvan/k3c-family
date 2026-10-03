# B-137 · Golden-Daten und Spielstand-Formate haben einen festen Änderungsablauf

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** F2
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F2

## Ausgangslage

Die Golden-Dateien in `testdata/golden/` (`level-*.json`, `sim-*.json`, `rng.json`, `campaign-abstieg.json`) vergleichen Go-Ergebnisse Feld für Feld (`engine/internal/golden/golden.go`). Der Kommentar in `engine/sim/sim_test.go` nennt `tests/golden.test.ts` als Erzeuger, diese Datei gibt es nicht mehr (`tests/` enthält nur `nesting_test.go`, `planning.test.ts`, `projectRules.test.ts`, `sprites.test.ts`); in `Taskfile.yml` fehlt ein Task zum Aktualisieren. Wie man eine gewollte Änderung der Golden-Daten durchführt und begründet, steht nirgends. Der Spielstand des Raums hat `IslandSaveVersion = 2` (`engine/sim/island_save.go`); `ParseIslandSave` liest Version 2 und überführt Version 1 (`SaveVersion`, `engine/sim/save.go`), jede andere Version ist ein Fehler. Fixtures alter Stände und eine Migrationsregel fehlen, obwohl R2 bis R4 neue Felder bringen.

## Ziel

Eine gewollte Änderung von Golden-Daten läuft über genau einen Task mit Begründung im Commit, und jede Formatänderung des Spielstands bringt Versionssprung, Fixture und Test „alter Stand lädt“. Nutzen: Datenverlust und stilles Driften fallen in der CI auf.

## Beteiligte und Zielgruppen

Entwickler und Agenten; 🧑 bestätigt jede Golden-Änderung (Beschluss Q09).

## Anforderungen

- Task `golden:update` in `Taskfile.yml` erzeugt die Golden-Dateien neu; die Beschreibung des Tasks nennt, dass der Commit-Text die Begründung der Änderung enthält.
- `docs/arbeitsweise.md` beschreibt den Golden-Ablauf (wer bestätigt, wann ein Update erlaubt ist) und die Migrationsregel in je einem eigenen Abschnitt mit höchstens 15 Zeilen.
- Migrationsregel: Jede Formatänderung des Spielstands erhöht `IslandSaveVersion`, legt ein Fixture unter `testdata/saves/v<n>/` an und einen Test, der das alte Fixture über `ParseIslandSave` lädt; Fixtures für Version 1 und 2 entstehen jetzt aus bestehenden Ständen oder aus dem Code.
- Test: Ein Spielstand mit unbekannt neuerer Version ergibt eine klare Meldung (Text nennt gefundene und unterstützte Version) und überschreibt keine Datei.
- Schlägt `golden:update` ohne Änderung der Eingaben etwas anderes als bisher heraus, bleibt der Test rot (kein stilles Überschreiben in `task check`).

## Nicht-Ziele

Golden auf arm64 (B-071), Determinismus-Prüfungen (B-138), Migration konkreter neuer Felder (B-022), Spielstand-Anzeige (B-147).

## Regeln und Einschränkungen

Domäne INF (`Taskfile.yml`, `docs/arbeitsweise.md`, `tests/`-Regeln); Tests für Go-Fixtures liegen in `engine/sim/`, das ist eine ausdrücklich zu nennende Domänen-Ausnahme der Sprint-Spec. Deterministisch (`engine/rng`), keine neue Abhängigkeit. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Wertänderung in `data/waves.json` lässt `sim-forest-nacht.json` abweichen → `task check:go` ist rot → Entwickler führt `task golden:update` aus und schreibt die Begründung in den Commit → 🧑 sieht den Diff.

## Ausnahme- und Fehlerfälle

Spielstand mit Version 0 oder ohne Feld `version` → klare Fehlermeldung (wie heute), kein Absturz. Fixture fehlt für eine neue Version → der Test „jede Version hat ein Fixture“ ist rot.

## Akzeptanzkriterien

- **AC-01** `task --list` zeigt `golden:update`; der Lauf erzeugt die Dateien in `testdata/golden/` neu, ein folgendes `task go:test` ist grün (Befehle, Ausgabe).
- **AC-02** `docs/arbeitsweise.md` enthält die Abschnitte „Golden aktualisieren“ und „Spielstand-Format ändern“ (Suche nach den Überschriften).
- **AC-03** Ein Go-Test lädt je ein Fixture aus `testdata/saves/v1/` und `testdata/saves/v2/` mit `ParseIslandSave` und prüft die wichtigsten Felder (Seed, Spieler, Hubs); `task go:test` grün.
- **AC-04** Ein Go-Test lädt einen Spielstand mit `version` größer als `IslandSaveVersion` und erwartet einen Fehler, der die gefundene und die unterstützten Versionen nennt; die Quelldatei bleibt unverändert.
- **AC-05** Ein Go-Test stellt sicher, dass zu jeder Version von 1 bis `IslandSaveVersion` ein Verzeichnis `testdata/saves/v<n>/` existiert.

## Offene Fragen

Wer bestätigt ein Golden-Update (🧑 immer, oder reicht eine Review-Session): `docs/fragenkatalog.md` Q09, entscheidet 🧑.

## Notizen

Aus Plan Phase 0 (F2), Lücken 7 und 8. Spielstände liegen in `saves/` und werden vom Server gelesen (`engine/store/saves.go`).
