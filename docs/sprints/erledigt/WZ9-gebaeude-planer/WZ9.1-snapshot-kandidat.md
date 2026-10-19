# WZ9.1 · Snapshot-Option Kandidaten-hub.json

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** wz9/1-snapshot-kandidat
- **Abhängig von:** WZ15.3, WZ7.1 (WZ8.4 nur, falls schon erledigt)
- **Tickets:** B-462
- **Kriterien:** AC-04, AC-08, AC-09, AC-10

## Ziel

Die Snapshot-Funktion aus WZ7.1 nimmt eine Kandidaten-`hub.json` als Option, prüft sie mit der Funktion aus WZ15.3 und baut den Snapshot daraus, ohne EXE-Neubau und ohne globalen Zustand.

## Kontext

- Voraussetzung: WZ7.1 (Snapshot-Funktion mit Options-Struktur) und WZ15.3 (`ValidateHub`, `sim.CreateOptions` mit Kandidat). Ist WZ7.1 nicht fertig, Session `blockiert`. Fehlt die Options-Struktur oder hat sie globalen Zustand, ein Ticket für WZ7 anlegen, nicht in WZ9.1 umbauen.
- Ist zugleich „geplante Chunks“ (WZ8.4) gesetzt, gelten die Kandidaten-Linien und die Planung wird gegen sie geprüft, nicht gegen die eingebetteten (Beispiel: Linie 5 bei 126 macht die Ader bei Hub +136 ungültig).
- `engine/room` importiert nichts aus `engine/net`. Ohne Option gilt die eingebettete `hub.json`; gleiche Eingabe ergibt byte-gleichen Snapshot.

## Erlaubte Dateien

- `engine/room/` (Snapshot-Funktion, Options-Struktur, Tests)
- `engine/net/` nur, wenn die Option ins Protokoll der Studio-Anbindung muss (dann Protokoll-Regeln aus `arbeitsweise.md` beachten, sonst Ticket)
- `testdata/hub/` (Fixture nur lesen)

## Nicht-Ziele

Kein Studio-Code (WZ9.2), keine Änderung an `engine/sim` über WZ15.3 hinaus, keine Änderung an `data/hub.json` oder `testdata/golden/`, kein globaler Zustand, kein Umbau von WZ7.1.

## Schritte

1. Stand von WZ7.1, WZ8.4 und WZ15.3 prüfen; fehlt WZ7.1 oder WZ15.3, `blockiert`.
2. Option „Kandidaten-hub.json“ in die Options-Struktur der Snapshot-Funktion aufnehmen.
3. Mit gesetzter Option: `ValidateHub` aufrufen, bei Befunden Fehler mit Liste zurückgeben; sonst Bau über `sim.CreateOptions`.
4. Vorrang: Option „geplante Chunks“ zusammen mit Kandidat → Kandidaten-Linien.
5. Go-Tests: Determinismus (zweimal byte-gleich), Kandidat (Rüstkammer ab Stufe 3) gegen eingebettet, ungültiger Kandidat, Vorrang, 7-Stufen-Fixture.
6. `task check:go`.

## Fertig, wenn

- [ ] AC-09: Option, Prüfung, Vorrang und Tests wie oben grün.
- [ ] AC-04: Snapshot mit Kandidat zeigt die Rüstkammer in Hub-Stufe 3, ohne Option nicht.
- [ ] AC-08: Test mit der 7-Stufen-Fixture liefert 7 wählbare Stufen.
- [ ] AC-10: `data/hub.json` und `testdata/golden/` unverändert.
- [ ] `task check:go` grün.

## Prüfen

```bash
task check:go
git diff --stat origin/develop -- data/hub.json testdata/golden
```

Unter k3c-dev: `check_run` mit `checkout`. Keine manuelle Prüfung.

## Ergebnis

AC-09 teilweise geprüft: `engine/room/preview.go` (`PreviewOptions.HubCandidate []byte`, leer = eingebettete `data/hub.json`), `preview_candidate.go` (`candidateWorld`: ein `sim.ParseHub`, Parse-Fehler und Befunde als `*CandidateError`, dann `sim.CreateWorldWith`). Prüfung mit der Funktion aus WZ15.3 vor dem Bau (fail closed), kein globaler Zustand. Ohne Option bleibt der Pfad unverändert: SHA-256 über 90 Vorschauen (5 Biome × Stufen 0–5 × 3 Phasen) vor und nach der Änderung identisch. Der Vorrang „geplante Chunks“ (Kandidaten-Linien gelten, Planung wird dagegen geprüft) fehlt, weil `PreviewOptions.Planned` (WZ8.4) nicht im Branch liegt: verschoben nach B-479 (inkl. Vorrang-Test „Linie 5 bei 126“).
AC-04 geprüft: Rüstkammer ab Stufe 3 erscheint in Hub-Stufe 3, ohne Option nicht, in Stufe 2 auch mit Kandidat nicht; zweimal derselbe Aufruf byte-gleich; Kandidat gleich eingebettet ergibt für forest, cave, crystal und alle Hub-Stufen byte-gleiches JSON und gleiche `EconomyOf`-Werte.
AC-08 geprüft: `testdata/hub/hub7.json` gibt 7 wählbare Stufen (Standard 7), Schmiede ab Stufe 6, Stufe 8 → Fehler; keine feste 5.
AC-10 geprüft: `git diff --stat origin/develop -- data/hub.json testdata/golden` leer.
Review (getrennter Agent): HIGH — mit Kandidat blieb `w.island` an der Welt (`stockMax` in der Vorschau, andere Inselzweige aktiv) → behoben mit **Abweichung von der Spec (SIM-Änderung in einer SRV-Session, von Claude freigegeben):** neue exportierte Hülle `sim.CreateWorldWith(biome, seed, CreateOptions)` baut eine einzelne Welt mit Kandidaten-Kopie ohne Insel (fail closed); das Aufräumen mit Interna (Plätze, `NextID`, Vorrat) ist entfallen. MEDIUM/LOW: ein Parse statt drei, Gleichheitstest vergleicht zusätzlich `EconomyOf`, 7-Stufen-Fehlerfall an `hubLevels+1` gekoppelt, B-479 um den Snapshot-Vorrang ergänzt. Rest: `Notes` der Prüfung (`NotePlannedChunksUnchecked`) werden nicht durchgereicht (Signaturbruch von `BuildPreview`); die Vorschau zeigt keine Insel-Plätze (Lager), auch nicht mit Kandidat, damit „Kandidat = eingebettet“ byte-gleich bleibt.
Prüfung: `go:test`, `task:check:go`, `go:lint` grün; Studio-Modul `go vet`/`go test` grün.
