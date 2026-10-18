# WZ8 · REG, SIM, SRV, DEV · Chunks je Insel und Stufe planen: Seed oder feste Objekte

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** REG, SIM, SRV, DEV
- **Reife:** bereit
- **Tickets:** B-461
- **Start-Commit:** 9a9bc440
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-17, Ralf im Chat (Revision 1)

## Ausgangslage

B-461 › Ausgangslage: Der Generator reiht Chunks zufällig aus dem Haupt-Strom `<biom>:<seed>`; feste Gegenden lassen sich nicht planen. Die Entscheidungen von 🧑 vom 2026-10-17 stehen in B-461 › Offene Fragen (Entschieden 1–5): Der Sprint setzt Teil 1 um (Chunk-Seed und feste Objekte, Entschieden 4), Ruinen und Gegnerlager folgen im Ticket B-468 (Projekt WZG, Folge-Sprint nach WZ8). Für die SRV- und DEV-Sessions muss WZ7 (B-460) erledigt sein: Vorschau-Funktion WZ7.1 mit ihrer Options-Struktur (ohne globalen Zustand) und Welt-Betrachter im Asset-Studio `tools/k3c-studio`.

## Ziel

Je Insel und Stufe lassen sich Chunks planen, mit fester Art und Inhalt aus Chunk-Seed oder festen Objekten. Der Generator setzt sie deterministisch ein, der Welt-Betrachter bearbeitet sie mit sofortiger Chunk- und Live-Vorschau, auch für ungespeicherte Entwürfe.

**Am Ende sichtbar:** im Asset-Studio eine geplante Sondergegend als ungespeicherter Entwurf in Chunk- und Live-Vorschau (beide aus demselben Level), bei zwei Seeds am selben Ort mit demselben Inhalt; in der CI unveränderte Golden-Daten plus ein neuer Golden-Fall mit Test-Planung; das Verhalten im Spiel weist der Sim-Test mit 2 Spielern nach; von 🧑 abgenommen. Der Sprint liefert keine echte Planung und keine Golden-Änderung aus.

## Beteiligte und Zielgruppen

B-461 › Beteiligte. Im Sprint: 🧑 im Workshop WZ8.1, in der Abnahme WZ8.7 und für die Spec-Freigabe; Agenten in WZ8.2–WZ8.5; ein getrennter Reviewer in WZ8.6.

## Anforderungen

B-461 › Anforderungen (Ort, Chunk-Platz, Inhalt Teil 1, Format, Determinismus, Biom-Summen, Prüfung unabhängig vom Seed, Planung und Linien als Parameter, Sim, Editor, Chunk-Vorschau auf Insel und Stufe, Schreiben, Leistung).

Nur für den Sprint:

- Reihenfolge REG → SIM → SRV → DEV. Die SIM-Sessions beginnen erst, wenn der Workshop-Beschluss und die Glossar-Einträge (AC-01) committet sind.
- WZ8.4 (SRV) und WZ8.5 (DEV) beginnen erst, wenn WZ7 erledigt ist (Vorschau-Funktion WZ7.1, Welt-Betrachter WZ7.2–WZ7.4). WZ8.1 bis WZ8.3 hängen nicht von WZ7 ab.
- **Linien und Planung als Parameter:** `engine/level` nimmt die Mauerlinien (`wallLines`) und die Planung optional als Parameter, ohne globalen Zustand (Standard: eingebettete Daten). WZ8.2 legt das für die Prüfung der Planung an, WZ8.3 für den Generator (`level.Options` mit `level.GenerateWith`, `level.Generate` bleibt; Namen Annahme, B-461 › Anforderungen). Nur so zeigen Snapshot und Prüfung geänderte Linien, die der Gebäude-Planer (WZ9, B-462) als Kandidaten-`hub.json` liefert.
- **`sim.CreateIsland` bekommt in WZ8.3 eine Options-Struktur** (`sim.CreateOptions`: Planung je Tiefe; WZ9.3 ergänzt den Kandidaten als weiteres Feld). **`sim.CreateOptions` ist eine NEUE Struktur nur für die Erzeugung** (Planung, Kandidaten-`hub.json`) und **getrennt vom vorhandenen `sim.IslandOptions`** (`engine/sim/island_options.go`: Grad, Ziel, Niederlage, im Spielstand gespeichert). **`CreateOptions` wird NICHT gespeichert:** Spielstand-Format und `IslandSaveVersion` bleiben unverändert. Alle Aufrufer werden in derselben Session angepasst (Nullwert = heutiges Verhalten): `engine/room/fullsave.go`, `engine/room/manager.go`, `tools/k3c-dev/internal/balance/run.go` und die Testdateien (heute 16). Die Anpassung außerhalb von `engine/sim` ist mechanisch und Grenzfall der SIM-Session.
- WZ8.4 ergänzt die Options-Struktur der Vorschau-Funktion aus WZ7.1 um die Option „geplante Chunks“ und reicht sie in `sim.CreateOptions` durch; kein globaler Zustand, keine zweite Funktion. Bei gleichzeitig gesetzter Kandidaten-`hub.json` (WZ9.4) gelten die Kandidaten-Linien, und die Planung wird gegen sie geprüft.
- WZ8.5 stellt die Chunk-Vorschau des Welt-Betrachters auf Insel und Stufe um (Level-Teil des Snapshots aus WZ8.4 als gemeinsame Quelle mit der Live-Vorschau, AC-11) und baut den Editor mit den Schreibregeln (AC-12).
- WZ8.3 legt den Benchmark der Level-Erzeugung an und misst die Baseline vor den Änderungen (AC-13).
- Das Format reserviert Felder für Teil 2 (Gebäude, Gegner, B-468), liest sie aber nicht: Der Generator lehnt sie mit „noch nicht unterstützt“ ab.

## Nicht-Ziele

B-461 › Nicht-Ziele, besonders:

- Ruinen und Gegnerlager (B-468, Folge-Sprint nach WZ8).
- Änderungen an der Vorschau-Funktion aus WZ7.1 über die Option „geplante Chunks“ hinaus; kein HTTP-Endpunkt, kein Protokoll- oder Spielstand-Wechsel.
- Echte Planungen in `data/islands.json` einchecken und Golden-Daten ändern (Werte, REG, Entscheidung von 🧑 nach dem Sprint). Der Sprint liefert das leere Feld, eine Test-Planung unter `testdata/` und den Editor; die Sondergegend der Abnahme bleibt Entwurf oder wird als REG-Ticket festgehalten.

## Regeln und Einschränkungen

B-461 › Regeln und Einschränkungen (eigener Strom je geplantem Chunk, nur `engine/rng`, Haupt-Strom ohne Planung unverändert, Golden, Domänen, Grenzfall „Werkzeug schreibt REG-Daten“, Schichten, Daten eingebettet, Verträge, Lage, Budget, Begriffe).

Nur für den Sprint:

- Jede Session hat genau eine Domäne: WZ8.1 REG, WZ8.2–WZ8.3 SIM, WZ8.4 SRV, WZ8.5–WZ8.7 DEV. Die Review-Session trägt die Domäne der letzten Umsetzung (DEV) und prüft den Diff aller vier Domänen.
- Mit 7 Sessions liegt der Sprint wie WZ7 über dem Richtwert 3–6, weil Review und Abnahme getrennt sind.
- Golden: bestehende Dateien bleiben byte-gleich; der neue Golden-Fall mit Test-Planung entsteht in WZ8.3 als **eigener Commit innerhalb der Session** (nach dem Code-Commit, Begründung im Commit-Text). Das ist die Ausnahme von „ein Commit je Session“, weil `docs/arbeitsweise.md` › Golden aktualisieren Golden-Änderungen in eigenem Commit verlangt. Der Code-Commit enthält keine Golden-Dateien. Nur dieser aktive Sprint ändert Golden-Daten, und nur diesen einen Fall.
- Der Sprint committet keine geänderte `data/islands.json` mit Planung (Feld nur leer); Editor-Tests schreiben nur in Kopien (`t.TempDir()`, Fixtures). Nach jedem Speichern im Editor erscheint der REG-Hinweis „REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung“.
- Ändert sich ein Kriterium nach der Freigabe: erst die Spec (Revision +1, `Spec: Entwurf`, neue Freigabe), dann der Code.
- Keine neue Abhängigkeit (Go oder npm). Fenster- und Browserprüfungen nur mit Freigabe von 🧑 für den Lauf.

## Beispiele

B-461 › Beispiele. Dazu für den Sprint:

- Nach WZ8.3, noch ohne Editor: Die Test-Planung unter `testdata/` erzeugt den neuen Golden-Fall; `testdata/golden/level-*.json` und `sim-*.json` zeigen sonst keinen Diff. Der Benchmark zeigt höchstens +10 % ohne Planung.
- Nach WZ8.4: Die Vorschau-Funktion liefert mit der Test-Planung als Option „geplante Chunks“ bei gleichem Seed zweimal denselben Snapshot; ein Aufruf mit anderer Planung danach liefert seinen eigenen Snapshot.
- Nach WZ8.5: 🧑 setzt im Asset-Studio auf Wald Platz +6 (Hub +300 … +350) ein festes Portal → Chunk- und Live-Vorschau zeigen es sofort an derselben Stelle (Hub +325), das zweite Portal springt nach links. Speichern zeigt den REG-Hinweis; in der Abnahme bleibt der Entwurf ungespeichert.

## Ausnahme- und Fehlerfälle

B-461 › Ausnahme- und Fehlerfälle (Hub, Länge, Sonder-Chunks, Mauerlinien und Portale ab Platz ±3, Summen, kein Platz mehr, doppelt, unbekannt, leer, Vorschau mit ungültigem Entwurf, Speichern). Dazu für den Sprint:

- WZ7 ist noch nicht erledigt → WZ8.4 und WZ8.5 `blockiert`, WZ8.1–WZ8.3 laufen weiter.
- Die Options-Struktur der Vorschau-Funktion aus WZ7.1 reicht für die Planung nicht (z. B. globaler Zustand statt Parameter) → WZ8.4 `blockiert`, Anpassung als Ticket in SRV; keine zweite Vorschau-Funktion.
- Der Editor sprengt das Budget (≈ 400 Code-Zeilen je Session) → Rest als Ticket, das Kriterium nicht umformulieren.
- Der Workshop entscheidet anders als die Annahmen in B-461 → erst die Spec anpassen (Revision), dann WZ8.2.
- Der Benchmark überschreitet +10 % (AC-13) → Ursache in WZ8.3 beheben oder die Grenze mit 🧑 neu festlegen (Spec-Revision); nicht stillschweigend ignorieren.

## Akzeptanzkriterien

- **AC-01** `B-461/AC-01` Format und Regeln als REG-Beschluss von 🧑, Regeltext in `docs/rules/`, Glossar-Einträge (inklusive korrigierter Chunk-Breite) vor der ersten SIM-Session.
- **AC-02** `B-461/AC-02` Ohne Planung sind `level-*.json` und `sim-*.json` byte-gleich.
- **AC-03** `B-461/AC-03` Der Generator setzt geplante Chunks deterministisch ein (Golden-Fall mit Test-Planung als eigener Commit der Session WZ8.3, Chunk-Seed über 100 Level-Seeds stabil); Generator und Prüfung nehmen die Mauerlinien als Parameter (Standard eingebettet, kein globaler Zustand).
- **AC-04** `B-461/AC-04` Der Inhalt geplanter Chunks kommt nur aus dem eigenen Strom.
- **AC-05** `B-461/AC-05` Die Biom-Summen gelten inklusive Planung (`level.Validate` über 500 Seeds je Biom; eine gültige Planung scheitert bei keinem Seed, WZ8.3).
- **AC-06** `B-461/AC-06` Eine ungültige Planung wird beim Laden abgelehnt und benannt, je Fall ein Test; die Prüfung nimmt die Mauerlinien als Parameter (Beispiel Ader bei Hub +136, Linie 5 bei 126).
- **AC-07** `B-461/AC-07` Feste Objekte wirken in der Sim wie gewürfelte, mit 2 Spielern; `sim.CreateIsland` nimmt eine Options-Struktur, alle Aufrufer mit Nullwert unverändert.
- **AC-08** `B-461/AC-08` Die Vorschau-Funktion aus WZ7.1 nimmt die Planung als Option „geplante Chunks“, deterministisch, ohne globalen Zustand.
- **AC-09** `B-461/AC-09` Editor im Welt-Betrachter mit sofortiger Chunk- und Live-Vorschau, speichert nur Gültiges.
- **AC-10** `B-461/AC-10` 🧑 nimmt eine geplante Sondergegend als Entwurf in Chunk- und Live-Vorschau ab; keine Planung und keine Golden-Änderung außer dem neuen Fall wird eingecheckt.
- **AC-11** `B-461/AC-11` Chunk-Vorschau auf Insel und Stufe, gleiche Quelle wie die Live-Vorschau.
- **AC-12** `B-461/AC-12` Schreibregel: atomar, format-stabil, Stand-Prüfung (veraltete Datei wird nicht überschrieben), REG-Hinweis nach dem Speichern.
- **AC-13** `B-461/AC-13` Benchmark der Level-Erzeugung: ≤ +10 % ohne Planung, ≤ +10 % mit Test-Planung gegenüber ohne.
- **AC-14** Das Review weist jedes Kriterium nach; `task check:all` und `task check:go` sind grün, Golden-Diff (nur der neue Fall) und Begründung sind geprüft, `git status` zeigt keine Änderung an `data/islands.json` über das leere Feld hinaus.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat): B-461 › Offene Fragen 1–5, darunter 4 (Teilung angenommen: WZ8 = feste Objekte und Seed je Chunk, Ruinen und Gegnerlager → B-468) und 5 (SRV-Session WZ8.4 für die Live-Vorschau ungespeicherter Planung, 7 Sessions).

Vor der Freigabe offen: nichts Blockierendes.

**Im Workshop WZ8.1 entschieden** (2026-10-17, 🧑, Annahmen aus B-461 übernommen; Teil von AC-01): B-461 › Offene Fragen 6–9 (Bezug des Chunk-Platzes, Objektliste ersetzt oder ergänzt, Lage-Regeln und Stromnamen, Verteilung übriger Portale und Ereignisse).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ8.1 | `WZ8.1-workshop-format.md` | Workshop | Mensch | fertig |
| WZ8.2 | `WZ8.2-feld-planung-pruefung.md` | Umsetzung | autonom | fertig |
| WZ8.3 | `WZ8.3-generator-create-options.md` | Umsetzung | autonom | fertig |
| WZ8.4 | `WZ8.4-vorschau-option-planung.md` | Umsetzung | autonom | fertig |
| WZ8.5 | `WZ8.5-editor-welt-betrachter.md` | Umsetzung | autonom | fertig |
| WZ8.6 | `WZ8.6-review.md` | Review | autonom | fertig |
| WZ8.7 | `WZ8.7-abnahme.md` | Review | Mensch | offen |

- WZ8.1 (REG) Workshop mit 🧑: Format, Chunk-Platz, Stromnamen, Anrechnung auf die Biom-Summen, Lage-Regeln; Regeltext in `docs/rules/`; Glossar „geplanter Chunk“, „Chunk-Platz“, „Chunk-Seed“, „Ruine“ und „Gegnerlager“ (geplant, B-468) und Chunk-Breite korrigieren (25 Units in Eisenstollen und Kristallhöhle) (AC-01).
- WZ8.2 (SIM) Feld `plannedChunks` in `data/islands.json` (leer) mit Laden und vom Seed unabhängiger Prüfung gegen die kürzeste Länge; die Prüfung nimmt die Mauerlinien als Parameter (Options-Struktur in `engine/level`, Standard eingebettet, kein globaler Zustand, Typ der Linien-Daten exportiert); benannte Fehler, je Fall ein Test, darunter Ader bei Hub +136 mit Linie 5 bei 124 gültig und bei 126 abgelehnt; Portal- und Platz-Grenzen gegen `frame()`/`eligible()` gerechnet (AC-06, AC-02).
- WZ8.3 (SIM) Generator reserviert geplante Plätze, eigener Strom je Chunk, feste Objekte rechnen auf die Biom-Summen; Planung und Mauerlinien als Parameter (`level.GenerateWith`, `level.Generate` bleibt, `clearOfLines` und `generateLines` ohne Paket-Variable); `sim.CreateIsland` bekommt `sim.CreateOptions` (Planung je Tiefe), alle Aufrufer angepasst (`engine/room/fullsave.go`, `engine/room/manager.go`, `tools/k3c-dev/internal/balance/run.go`, Testdateien; mechanisch, Nullwert); Benchmark der Level-Erzeugung mit Baseline vor den Änderungen; 500-Seed-Test, Sim-Test mit 2 Spielern. Zwei Commits in der Session: erst der Code-Commit (Messwerte des Benchmarks im Commit-Text), danach der Golden-Fall mit Test-Planung als eigener Commit mit Begründung (Ausnahme von „ein Commit je Session“, `arbeitsweise.md` › Golden aktualisieren) (AC-02, AC-03, AC-04, AC-05, AC-07, AC-13).
- WZ8.4 (SRV) Option „geplante Chunks“ in der Options-Struktur der Vorschau-Funktion aus WZ7.1 (ohne globalen Zustand), durchgereicht in `sim.CreateOptions`; Go-Tests für Determinismus, eingebettete Planung als Standard, Ablehnung und aufeinanderfolgende Aufrufe mit verschiedener Planung (AC-08).
- WZ8.5 (DEV) Editor im Welt-Betrachter (`tools/k3c-studio`): Chunk-Platz, Art, Chunk-Seed und Objekte bearbeiten; Chunk-Vorschau auf Insel und Stufe umstellen (gleiche Quelle wie die Live-Vorschau); sofortige Vorschau aus dem Entwurf, Fehler am Platz; Speichern nur gültig, atomar, format-stabil, mit Stand-Prüfung und REG-Hinweis, Tests nur auf Kopien (AC-09, AC-06, AC-11, AC-12).
- WZ8.6 (DEV) Review: `task check:all`, Diff REG, SIM, SRV und DEV, nur schwere Befunde, PR des Sprints (AC-14, alle).
- WZ8.7 (DEV) Abnahme durch 🧑 am PC (Agent: Mensch): Sondergegend als Entwurf planen, mit zwei Seeds in Chunk- und Live-Vorschau prüfen; Entwurf bleibt ungespeichert oder wird als REG-Ticket festgehalten (AC-10).

## Abnahme

2026-10-17 (Review WZ8.6): AC-01 bis AC-09, AC-11 bis AC-14 mit Nachweis aus Code, Tests und getrennten Reviewern (Ergebnisse WZ8.2–WZ8.6); AC-10 offen (WZ8.7, 🧑 am PC).
Befunde: keine schweren. Neues Ticket: B-475 (Annahmen zu festen Ereignis-Objekten und „kein Platz mehr“).
Version: v… vorgeschlagen: keine Änderung (Werkzeug- und Generator-Sprint, Golden und Spielstand-Format unverändert).
