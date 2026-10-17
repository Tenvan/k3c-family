# B-461 · Chunks einer Insel-Stufe lassen sich planen, per Seed oder mit festen Objekten

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ8
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Level-Generator (`engine/level/level.go`) baut jede Stufe aus Biom und Seed: `frame()` würfelt die Länge (Wald 900–1100 Units → 18–22 Chunks) und setzt Hub (Mitte), Ausgang und Rand; danach folgen Portale, `eventChunks` (`chest`, `recruitCamp`; Camps bevorzugt nah am Hub und auf Abstand zu den Linien, `clearOfLines`, B-261), die übrigen Chunks nach `chunkWeights`, Ressourcen je Chunk-Art (`resourcesPerChunk`), Skill-Punkte und zuletzt die Adern. Alle diese Würfe kommen nacheinander aus dem Haupt-Strom `<biom>:<seed>`, ihre Reihenfolge ist Vertrag (`testdata/golden/level-*.json`). Die Mauerlinien streuen aus `<biom>:<seed>:sites`, die Sim würfelt aus `<biom>:<seed>:sim`.

`level.Validate` (`engine/level/validate.go`) prüft Länge, Tiefen-Eingang, Portale (Hub-Abstand, außerhalb des äußersten Tors), den Bau des Endbosses sowie die Zahl der Ereignis-Chunks, Adern und Skill-Punkte. `sim.CreateWorld` übernimmt die Level-Objekte (`placeEntities`: Ressourcen, Truhen, Skill-Punkte, Camps, Portale); `createIsland` erzeugt alle Stufen einer Insel mit demselben Seed. `data/islands.json` kennt je Insel nur `depths` und `depthScaling`. Gespeichert wird nur der Seed.

Feste Gegenden, also ein bestimmter Chunk mit bestimmtem Inhalt an fester Stelle, lassen sich nicht planen.

## Ziel

Je Insel und Stufe lassen sich einzelne Chunks planen: Die Chunk-Art ist fest, der Inhalt kommt aus einem eigenen Chunk-Seed oder steht als feste Objekte (Ressourcen, Adern, Truhen, Camps, Portale) an fester Lage. Der Generator setzt geplante Chunks deterministisch an ihren Platz, alles andere bleibt zufällig. Geplant und geprüft wird im Welt-Betrachter des Asset-Studios mit sofortiger Chunk- und Live-Vorschau, auch für ungespeicherte Entwürfe.

Vorgebaute Gebäude (Ruinen) und feste Gegner (Gegnerlager) sind nicht Teil dieses Tickets, sie folgen in **B-468** (Entschieden 4); das Datenformat lässt dafür Platz.

## Beteiligte und Zielgruppen

- 🧑 Ralf: entscheidet Regeln und Datenformat (Workshop), plant Sondergegenden im Editor, nimmt in Vorschau und Spiel ab und gibt die Spec frei.
- Spielende (Couch-Koop, 2+ Spieler): erleben feste Gegenden an bekannter Stelle; der Rest jeder Stufe bleibt je Spielstand zufällig.
- Agenten: setzen Regeltext (REG), Datenfeld, Generator und Prüfung (SIM) und den Editor (DEV) um.

## Anforderungen

- **Ort:** Geplante Chunks stehen in `data/islands.json` je Insel und Tiefe (Entschieden 1). Sie gelten für jeden Spielstand dieser Insel. Level ohne Planung (andere Inseln oder Tiefen, Biom-Level ohne Insel wie `level_generate`) bleiben rein zufällig.
- **Chunk-Platz:** Ein geplanter Chunk sitzt auf einem Platz relativ zu den Hub-Chunks (`-3` = dritter Chunk links davon, `+1` = erster rechts), damit er bei jeder gewürfelten Länge gleich weit vom Hub liegt (Annahme, WZ8.1 bestätigt). Die Hub-Chunks decken Hub ±50 Units ab (Hub-Breite 100, `count/2-1` und `count/2` bei 50-Unit-Chunks, vier Hub-Chunks bei 25-Unit-Chunks). Platz +1 beginnt also bei Hub +50 und Platz −1 endet bei Hub −50; in 50-Unit-Biomen beginnt Platz +n bei Hub +50·n und Platz −n bei Hub −50·(n+1). Allgemein: Anfang von +n = Hub + 50 + (n−1)·Chunk-Breite, Anfang von −n = Hub − 50 − n·Chunk-Breite. Die Lage `at` eines Objekts zählt ab Chunk-Anfang.
- **Inhalt (Teil 1):** je geplantem Chunk die Chunk-Art (Pflicht), optional ein Chunk-Seed, optional feste Objekte (Art und Lage in Units ab Chunk-Anfang) aus Ressourcen laut `resourcesPerChunk` des Bioms, Ader (`veins.kind` des Bioms), Truhe, Camp und Portal. Ohne Objektliste würfelt der Inhalt der Chunk-Art aus dem eigenen Strom; mit Objektliste ist der Chunk vollständig fest (Annahme).
- **Format (Annahme, WZ8.1 legt fest):**

  ```json
  { "id": "island1", "depths": [0, 1, 2, 3, 4],
    "plannedChunks": {
      "0": [ { "slot": -3, "kind": "clearing", "seed": "lichtung" } ],
      "1": [ { "slot": 3, "kind": "cavern", "objects": [ { "kind": "stoneVein", "at": 40 }, { "kind": "chest", "at": 45 } ] } ]
    } }
  ```

- **Determinismus:** Gleicher Seed und gleiche Planung ergeben auf jedem Rechner dasselbe Level. Der Inhalt eines geplanten Chunks würfelt nur aus seinem eigenen Strom (Regeln), nie aus dem Haupt-Strom.
- **Biom-Summen:** Was das Biom zählt, gilt inklusive geplanter Inhalte: Portale genau `portals.count`, Truhen und Camps `eventChunks` min–max, Adern genau `veins.count`, Skill-Punkte min–max. Feste Portale, Truhen, Camps und Adern ersetzen gewürfelte, statt sie zu ergänzen.
- **Prüfung unabhängig vom Seed:** Eine Planung wird beim Laden der Daten und im Editor geprüft, gegen die kürzeste mögliche Länge des Bioms und jede Streuung der Linien. Ist sie dort gültig, scheitert kein Seed zur Laufzeit. Fehler nennen Insel, Tiefe, Platz und Grund. Die Mauerlinien (`wallLines`, `data/hub.json`) nimmt die Prüfung als Parameter (Standard: eingebettete Daten), damit B-462 dieselbe Prüfung gegen eine Kandidaten-`hub.json` laufen lassen kann.
- **Planung und Linien als Parameter:** Generator, Prüfung der Planung und Insel-Erzeugung nehmen die **Planung und die Mauerlinien** optional als Parameter, ohne globalen Zustand und ohne Paket-Variable zum Umschalten; ohne Parameter gelten die eingebetteten Daten (`data/` ist per `go:embed` fest im Binary). Heute lädt `engine/level/lines.go` die Linien einmal in die Paket-Variable `wallLines`, die `clearOfLines` und `generateLines` lesen; sie dient künftig nur noch als Standard. Zuschnitt (Namen Annahme, WZ8.2 und WZ8.3 legen sie fest):
  - `engine/level`: Options-Struktur (`level.Options` mit Planung und Linien) und `level.GenerateWith(b, seed, opts)`; `level.Generate(b, seed)` bleibt für bestehende Aufrufer und ruft sie mit dem Nullwert auf. `clearOfLines` und `generateLines` nehmen die Linien als Argument, ebenso die Prüfung der Planung (WZ8.2). Der Typ der Linien-Daten (heute unexportiert `wallLineData`) wird exportiert, damit `engine/sim` und die Vorschau-Funktion ihn füllen können.
  - `engine/sim`: `sim.CreateIsland` bekommt eine Options-Struktur (`sim.CreateOptions`: Planung je Tiefe; WZ9.3 ergänzt den Kandidaten aus B-462 als weiteres Feld, ohne die Signatur zu ändern). **`sim.CreateOptions` ist eine neue Struktur nur für die Erzeugung** (Planung, Kandidaten-`hub.json`) und **getrennt vom vorhandenen `sim.IslandOptions`** (`engine/sim/island_options.go`: Grad, Ziel, Niederlage, im Spielstand gespeichert). **`CreateOptions` wird nicht gespeichert:** Spielstand-Format und `IslandSaveVersion` bleiben unverändert; Planung und Kandidat wirken nur bei der Erzeugung (Preview, Laden eines Spielstands nutzt weiter die eingebetteten Daten). **Alle Aufrufer werden in WZ8.3 angepasst** (Nullwert = heutiges Verhalten): `engine/room/fullsave.go`, `engine/room/manager.go`, `tools/k3c-dev/internal/balance/run.go` und die Testdateien (heute 16); `createIsland` (Laden, Inselwechsel) gibt den Nullwert weiter. `sim.CreateWorld` reicht die Options-Struktur des Levels an `level.GenerateWith` durch.
  - Vorschau-Funktion (WZ8.4): Die Option „geplante Chunks“ ihrer Options-Struktur (WZ7.1 hat sie ohne globalen Zustand angelegt) reicht die Planung in `sim.CreateOptions` durch; es gibt keinen globalen Zustand für den Entwurf.
  - **Vorrang:** Sind in der Vorschau-Funktion sowohl „geplante Chunks“ (WZ8.4) als auch „Kandidaten-`hub.json`“ (WZ9.4, B-462) gesetzt, gelten die Linien des Kandidaten, und die Planung wird gegen diese Linien geprüft, nicht gegen die eingebetteten.
- **Sim:** Feste Objekte verhalten sich wie gewürfelte gleicher Art (abbauen, markieren, aufheben, rekrutieren, Wellen aus Portalen), auch mit 2 Spielern; keine neue Sim-Logik.
- **Editor:** Im Welt-Betrachter (B-460, WZ7; Asset-Studio `tools/k3c-studio`) Insel und Tiefe wählen, Chunk-Platz wählen, Art, Chunk-Seed und Objekte setzen, verschieben und löschen. Chunk- und Live-Vorschau rechnen aus dem Entwurf im Speicher und zeigen jede Änderung sofort (≤ 1 s, angenommen). Prüffehler stehen am Platz, gespeichert wird nur eine gültige Planung nach `data/islands.json`.
- **Chunk-Vorschau auf Insel und Stufe:** Mit der Planung wählt die Chunk-Vorschau des Welt-Betrachters nicht mehr Biom und Seed, sondern Insel und Stufe (Seed bleibt wählbar). Sie liest ihr Level aus derselben Quelle wie die Live-Vorschau (Level-Teil des Vorschau-Snapshots, gleiche Erzeugung mit derselben Planung), damit beide Vorschauen nie verschiedene Level zeigen. Die Warnungen der Spielbarkeits-Prüfung gelten für dieses Level.
- **Schreiben (Werkzeug schreibt REG-Daten, gleiche Regeln wie B-462/B-459):**
  - atomar (temporäre Datei, dann umbenennen); Absturz beim Schreiben lässt die alte Datei vollständig;
  - format-stabil: Schlüssel-Reihenfolge, Zeilenumbrüche, übrige Felder und Inseln bleiben erhalten; Laden und Speichern ohne Änderung ist byte-gleich, eine Änderung ändert nur den betroffenen Eintrag;
  - Stand-Prüfung: Hat sich `data/islands.json` seit dem Laden geändert, wird nicht gespeichert (Meldung „`data/islands.json` geändert, bitte neu laden“, der Entwurf bleibt erhalten);
  - nach jedem Speichern der Hinweis „REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung“.
- **Leistung (messbar):** Ein Benchmark der Level-Erzeugung (`engine/level`, in WZ8.3 angelegt, Stand vor WZ8 auf demselben Rechner als Baseline) zeigt für alle Biome ohne Planung höchstens +10 % Laufzeit gegenüber der Baseline und mit der Test-Planung höchstens +10 % gegenüber demselben Level ohne Planung.

## Nicht-Ziele

- Vorgebaute Gebäude/Ruinen und feste Gegner/Gegnerlager → **B-468** (Teil 2, Projekt WZG, Folge-Sprint nach WZ8). Das Format reserviert nur den Platz dafür.
- Planung je Biom in `data/biomes/*.json` (verworfen, Entschieden 1).
- Neue Chunk-Arten, neue Objekt-Arten oder Objekte mit eigener Regel; feste Skill-Punkte (Offene Frage 7).
- Balancing von `chunkWeights` und Biom-Summen; ganze Level von Hand bauen. Hub, Tiefen-Eingang, Rand und Mauerlinien bleiben Sache des Generators.
- Echte Planungen und geänderte Golden-Daten ausliefern: Der Sprint liefert das leere Feld, eine Test-Planung unter `testdata/` und den Editor. Eine Planung in `data/islands.json` zu speichern und einzuchecken ist eine eigene REG-Entscheidung von 🧑 (Regeln).
- Mehrere Inseln und Inselwechsel (B-103); Freischaltung je Hub-Stufe (B-462); Umgang mit `leveltest.html` (B-460, WZ7.5).
- Migration bestehender Spielstände (keine Rücksicht auf alte Spielstände, Beschluss 🧑 bis auf Widerruf).

## Regeln und Einschränkungen

- **Determinismus (`CLAUDE.md`, Glossar):** Zufall nur über `engine/rng` (`rng.New`), nie `math/rand` oder `Math.random()`. Jeder geplante Chunk würfelt aus einem eigenen Strom: ohne Chunk-Seed `<biom>:<seed>:chunk:<platz>` (Art fest, Inhalt wechselt mit dem Level-Seed), mit Chunk-Seed `<biom>:chunk:<chunk-seed>` (Inhalt in jedem Spielstand gleich). Die Namen sind eine Annahme; WZ8.1 legt sie fest, danach sind sie Vertrag wie die Reihenfolge des Haupt-Stroms.
- **Haupt-Strom:** Ohne Planung ändern sich Reihenfolge und Zahl der Würfe nicht. Mit Planung reserviert der Generator die geplanten Plätze direkt nach `frame()`; Portale, Ereignisse, Chunk-Arten, Skill-Punkte und Adern würfeln nur über freie, nicht geplante Chunks. Dadurch dürfen sich ihre Würfe gegenüber dem Level ohne Planung verschieben (gewollt, eigener Golden-Fall). Übrige Portale verteilt der Generator abwechselnd, beginnend auf der Seite mit weniger festen Portalen (Annahme).
- **Golden (`arbeitsweise.md` › Golden aktualisieren):** `testdata/golden/level-*.json` und `sim-*.json` bleiben ohne Planung byte-gleich. Der Golden-Fall mit Planung nutzt eine Test-Planung unter `testdata/`, nicht `data/islands.json`, damit spätere Planungen von 🧑 ihn nicht brechen. Er entsteht in der Session WZ8.3 (neue Datei, `task golden:update` nur dafür) als **eigener Commit innerhalb der Session**, nach dem Code-Commit und mit Begründung im Commit-Text; `arbeitsweise.md` verlangt Golden-Änderungen in eigenem Commit, das ist die Ausnahme von „ein Commit je Session“. Der Code-Commit der Session enthält keine Golden-Dateien (gleiche Regel wie B-462). Ändert später eine echte Planung in `data/islands.json` Golden-Daten, ist das eine gewollte Datenänderung (REG): `task golden:update` in eigenem Commit, Begründung im Commit-Text, Prüfung im Review.
- **Domänen (`arbeitsweise.md` › Domänen):**
  - REG: Regeltext und Begriffe (`docs/rules/`, `docs/glossar.md`).
  - SIM: neues Feld in `data/islands.json`, Laden (`engine/sim/island_data.go`), Generator und Prüfung (`engine/level/`, mit Planung und Mauerlinien als Parameter), Options-Struktur in `sim.CreateIsland` samt Anpassung aller Aufrufer (WZ8.3; die Anpassung in `engine/room/` und `tools/k3c-dev/` ist rein mechanisch, Nullwert, ohne Verhaltensänderung, und gehört als Grenzfall zur Session). SIM legt das Feld leer an; echte Planungen sind Werte (REG).
  - SRV: Option „geplante Chunks“ der Vorschau-Funktion aus WZ7.1 (`engine/room/`, `engine/net/`) reicht die Planung durch.
  - DEV: Editor (`tools/k3c-studio/`).
  - Feature-Kette REG → SIM → SRV → DEV.
- **Grenzfall „Werkzeug schreibt REG-Daten“** (wie B-462): Der Code des Editors ist DEV; das Speichern zur Laufzeit ist eine REG-Änderung durch 🧑. Sessions des Sprints committen keine geänderte `data/islands.json`; Tests schreiben nur in Kopien (`t.TempDir()` bzw. Fixtures). Die Sondergegend der Abnahme bleibt Entwurf (ungespeichert) oder wird als REG-Ticket festgehalten; ein späterer Commit mit gespeicherter Planung trägt Begründung (welche Insel, Tiefe, Platz, warum) und, nur bei gewollter Änderung, `task golden:update`.
- **Schichten:** `engine/level` importiert nichts aus `engine/sim`, `engine/room` oder `engine/net`; Planung und Mauerlinien kommen als Parameter in Generator und Prüfung, ohne globalen Zustand. `level.Generate(b, seed)` bleibt für bestehende Aufrufer unverändert (Aufruf mit dem Nullwert der Options-Struktur). `engine/sim/lines.go` baut Mauer, Turm und Tor weiter aus `w.Level.Lines`; Linien aus einem Kandidaten wirken also über das Layout.
- **Daten eingebettet:** `data/*.json` ist per `go:embed` im Binary (`data/embed.go`). Eine gespeicherte Planung wirkt im Spiel erst nach dem Neubau des Spielservers; die Vorschau nutzt bis dahin den Entwurf als Parameter.
- **Verträge:** Kein Protokollwechsel, es gibt nur bestehende Chunk- und Objekt-Arten. Der Spielstand speichert weiter nur Seeds; Level-Objekte werden über `kind@x` zugeordnet, deshalb muss `kind@x` je Stufe eindeutig bleiben. Eine geänderte Planung ergibt für bestehende Spielstände ein anderes Level (keine Migration).
- **Lage fester Objekte:** wie gewürfelte gleicher Art. Ressourcen, Truhen und Adern halten ≥ 2 Units Abstand zur Chunk-Grenze (wie `inside()`); Camps dürfen innerhalb der Linien liegen (Glossar › Camp); Portale halten `minDistanceFromHubUnits` zum Hub und liegen außerhalb des äußersten Tors (Q49, Q57). Adern sind Zahlziele (Markieren) und halten zusätzlich ≥ 4 Units zu jedem Bauplatz der Linien inklusive Streuung (+0 bis +4) und zum Farm-Weltplatz (Q57; Annahme, Offene Frage 8).
- **Budget:** Datei ≤ 400, Funktion ≤ 60 Zeilen. `level.go` (274 Zeilen) nicht weiter aufblähen; die Planung kommt in eine eigene Datei (z. B. `engine/level/planned.go`).
- **Begriffe:** laut Glossar. Neue Begriffe (geplanter Chunk, Chunk-Platz, Chunk-Seed; für Teil 2 Ruine, Gegnerlager) trägt WZ8.1 ein, bevor die erste SIM-Session beginnt.

## Beispiele

Plätze und Längen gerechnet aus `data/biomes/*.json` und `engine/level/level.go` (`frame()`) bei der kürzesten Länge; Platz +n beginnt in 50-Unit-Biomen bei Hub +50·n (Annahme Format und Stromnamen wie in Anforderungen und Regeln).

- **Wald, Tiefe 0, `{slot: -3, kind: "clearing", seed: "lichtung"}`** → In jedem Spielstand ist der dritte Chunk links von den Hub-Chunks (Hub −200 … −150) eine Lichtung mit immer denselben Bäumen und Büschen an derselben Lage (Strom `forest:chunk:lichtung`); alle anderen Chunks wechseln mit dem Seed.
- **Wald, `{slot: 5, kind: "rocks"}` ohne Seed** → Chunk +5 (Hub +250 … +300) ist immer „Felsen“; Zahl und Lage der Felsen wechseln mit dem Level-Seed (Strom `forest:<seed>:chunk:5`).
- **Höhle, Tiefe 1, `{slot: 3, kind: "cavern", objects: [stoneVein bei 40, chest bei 45]}`** → Platz +3 beginnt bei Hub +150: Ader bei Hub +190, Truhe bei Hub +195, sonst nichts im Chunk. Der Generator würfelt nur noch 1 Ader (`veins.count` 2) und rechnet die Truhe auf `eventChunks.chest` (1–3) an.
- **Wald, `{slot: 6, kind: "portal"}`** → Platz +6 liegt bei Hub +300 … +350, das Portal sitzt fest rechts (Mitte, Hub +325); das zweite würfelt der Generator links.
- **Höhle, `{slot: 2, objects: [stoneVein bei 40]}`** → Platz +2 beginnt bei Hub +100, die Ader steht bei Hub +140: ≥ 4 Units hinter dem Tor der Linie 5 (bis Hub +132 bei voller Streuung), also gültig.
- **Insel 1 ohne `plannedChunks`** → `task golden:update` ändert nichts, `git diff testdata/golden` ist leer.
- **Editor:** 🧑 verschiebt eine feste Truhe um 5 Units → Chunk- und Live-Vorschau zeigen sie sofort an der neuen Stelle, beide aus demselben Level (Insel und Stufe gewählt). Speichern schreibt nur diesen Eintrag in `data/islands.json`, zeigt den REG-Hinweis, und im Spiel steht sie dort nach dem Neubau des Spielservers.
- **Editor, Datei extern geändert:** `data/islands.json` wurde nach dem Laden per `git checkout` zurückgesetzt → Speichern wird abgelehnt, der Entwurf bleibt, 🧑 lädt neu und speichert erneut.
- **2 Spieler** in einer Stufe mit fester Truhe → der erste, der sie aufhebt, bekommt das Gold, danach ist sie für beide weg (wie eine gewürfelte).

## Ausnahme- und Fehlerfälle

Jede ungültige Planung wird abgelehnt und benannt (Insel, Tiefe, Platz, Grund). Beim Laden schlägt der Start fehl wie heute bei einer ungültigen `islands.json` (`loadIslands`); im Editor bleibt Speichern gesperrt und der Fehler steht am Platz.

- **Hub:** Platz `0` gibt es nicht; Hub-Chunks sind nicht planbar, Chunk-Art `hub` ist verboten.
- **Länge:** Platz liegt bei der kürzesten Länge auf Tiefen-Eingang, Rand oder außerhalb. Wald (18 Chunks) erlaubt ±1 bis ±7, ±8 ist dort Tiefen-Eingang bzw. Rand; Höhle (14) ±1 bis ±5; Mine (12) ±1 bis ±4. Eisenstollen (20 Chunks à 25 Units, 4 Hub-Chunks) ±1 bis ±7 und Kristallhöhle (16) ±1 bis ±5 sind gerechnet, WZ8.2 bestätigt sie gegen den Generator.
- **Sonder-Chunks:** `exit` und `edge` sind nicht planbar. `portal`, `chest` und `recruitCamp` sind als Art erlaubt und zählen auf die Biom-Summen.
- **Doppelt:** zwei Einträge mit demselben Platz in derselben Tiefe; zwei Objekte gleicher Art an gleicher Lage (`kind@x` nicht eindeutig).
- **Unbekannt:** Tiefe nicht in `depths` der Insel; Chunk-Art weder in `chunkWeights` des Bioms noch erlaubter Sonder-Chunk; Objekt-Art nicht im Biom (z. B. `ironVein` im Wald); Felder für Teil 2 (Gebäude, Gegner) → „noch nicht unterstützt“ (kommen mit B-468).
- **Lage:** Objekt näher als 2 Units an der Chunk-Grenze; Ader näher als 4 Units an Mauer, Turm oder Tor einer Linie bei irgendeiner Streuung oder am Farm-Weltplatz. Beispiele Höhle: `{slot: 2, objects: [stoneVein bei 14]}` → Hub +114 liegt 2 Units vom Tor der Linie 4 (bis +112) und 2 Units vom Turm der Linie 5 (ab +116), also abgelehnt; `{slot: 1, objects: [stoneVein bei 14]}` → Hub +64 ist der Mauer-Platz der Linie 2.
- **Mauerlinien und Portale:** festes Portal näher am Hub als `minDistanceFromHubUnits` (Wald 150, also erst ab Platz ±3, Chunk-Mitte Hub ±175) oder innerhalb des äußersten Tors. Portal-Plätze: Wald ±3 bis ±7, Höhle ±3 bis ±5, Mine ±3 und ±4; Platz ±1 und ±2 sind für Portale gesperrt. In den 25-Unit-Biomen beginnt Platz ±4 bei Hub ±125; ein festes Portal als Objekt braucht dort `at` ≥ 10 (siehe Notizen).
- **Summen:** mehr feste Portale als `portals.count`, mehr Truhen oder Camps als `eventChunks.max`, mehr Adern als `veins.count`. Der Wald hat keine Adern, jede feste Ader dort wird abgelehnt.
- **Kein Platz mehr:** Nach der Reservierung bleiben bei der kürzesten Länge zu wenig freie Plätze für die übrigen Portale, die Mindestzahl der Ereignisse (Truhen und Camps) oder die Adern. Jede nicht feste Ader braucht einen eigenen Ressourcen-Chunk, der weder Hub, Rand, Ausgang, Portal, Ereignis noch vollständig fest ist (`veins.count` minus feste Adern; Annahme, Offene Frage 7). Skill-Punkte brauchen nur einen Chunk außerhalb von Hub, Rand und vollständig festen Chunks und dürfen sich einen Chunk teilen: `skillPoints.min` ist nie die Engstelle, feste Skill-Punkte gibt es nicht (Frage 7). Beispiel Mine mit geplanten Chunks `{slot: -4, kind: "shaft"}`, `{slot: -3, kind: "shaft"}` und `{slot: 3, kind: "shaft"}`: Bei 12 Chunks sind nur ±3 und ±4 portaltauglich, übrig bleibt Platz +4, Portal 2/2 findet keinen Platz → abgelehnt, statt später bei einem Seed zu scheitern.
- **Bau des Endbosses** (Kristallhöhle) liegt an der inneren Kante des Ausgangs-Chunks; der 2-Units-Abstand zur Chunk-Grenze hält Objekte davon frei.
- **Leer:** Feld fehlt, `{}` oder leere Liste = keine Planung (byte-gleich). Leerer Chunk-Seed = kein Chunk-Seed. Umlaute und Emoji im Chunk-Seed sind erlaubt (`rng` hasht UTF-16, `TestUmlauteUndEmoji`).
- **Daten ändern sich später** (REG kürzt eine Biom-Länge, streicht eine Chunk-Art oder verschiebt eine Mauerlinie, etwa über den Gebäude-Planer B-462): Die Prüfung beim Laden meldet betroffene Planungen, `task check` wird rot, bis die Planung angepasst ist. Der Gebäude-Planer prüft die Planung vor dem Schreiben mit (B-462, Linien als Parameter).
- **Vorschau mit ungültigem Entwurf:** Die Vorschau-Funktion lehnt eine ungültige Planung mit demselben Fehler ab; die Vorschau zeigt den Fehler und behält das letzte gültige Bild.
- **Speichern:** `data/islands.json` seit dem Laden extern geändert (Editor, `git checkout`, anderer Prozess) → nicht speichern, Meldung „`data/islands.json` geändert, bitte neu laden“, der Entwurf bleibt erhalten. Datei nicht lesbar oder kein gültiges JSON → Fehler mit Pfad, Bearbeiten und Speichern gesperrt. Prüfung nicht erreichbar oder abgebrochen → kein Schreiben (fail closed). Absturz beim Schreiben → alte Datei bleibt vollständig (atomares Ersetzen).
- **Golden bricht nach dem Speichern:** erwartet bei einer gespeicherten Planung; nicht automatisch aktualisieren, 🧑 entscheidet (REG).

## Akzeptanzkriterien

- **AC-01** Ein REG-Beschluss von 🧑 (Workshop WZ8.1) legt Format und Regeln fest: Ort `data/islands.json` je Insel und Tiefe, Chunk-Platz, Chunk-Art, Chunk-Seed, feste Objekte mit Lage, Stromnamen, Anrechnung auf die Biom-Summen, Lage-Regeln. Der Regeltext steht in `docs/rules/` (Regel · Begründung · Daten); das Glossar enthält „geplanter Chunk“, „Chunk-Platz“ und „Chunk-Seed“ (dazu „Ruine“ und „Gegnerlager“ als geplant, B-468) und die korrigierte Chunk-Breite (25 Units in Eisenstollen und Kristallhöhle), bevor die erste SIM-Session beginnt.
- **AC-02** Ohne Planung (Feld fehlt oder leer) bleiben `testdata/golden/level-*.json` und `sim-*.json` byte-gleich: `task golden:update` erzeugt keinen Diff, `task check:go` ist grün.
- **AC-03** Mit Planung setzt der Generator jeden geplanten Chunk mit seiner Art an seinen Platz; nicht geplante Chunks bleiben zufällig. Go-Test: gleiche Eingaben → gleiches Layout (neuer Golden-Fall mit Test-Planung unter `testdata/`, als eigener Commit der Session WZ8.3 mit Begründung). Ein Chunk mit Chunk-Seed hat über 100 Level-Seeds denselben Inhalt (Lage relativ zum Chunk-Anfang); ohne Chunk-Seed bleibt nur die Art gleich. Linien als Parameter: Werden die eingebetteten Linien explizit übergeben, ergibt sich dasselbe Layout wie ohne Parameter; abweichende Linien (z. B. Linie 5 verschoben) ändern Lage und Abstände der Linien im Layout, ohne globalen Zustand (zwei Aufrufe mit verschiedenen Linien nacheinander liefern je ihr Layout).
- **AC-04** Der Inhalt geplanter Chunks kommt nie aus dem Haupt-Strom. Go-Test: Ändert sich Chunk-Seed oder Ressourcen-Objekt eines geplanten Chunks (gleiche Art, gleiche Biom-Summen), bleiben alle nicht geplanten Chunks und ihre Objekte gleich.
- **AC-05** Die Biom-Summen gelten inklusive geplanter Inhalte: `level.Validate` ist für jedes Biom mit einer Beispiel-Planung über 500 Seeds leer (Länge, Tiefen-Eingang, Portale, Bau, Ereignisse, Adern, Skill-Punkte). Eine Planung, die die Prüfung beim Laden besteht, scheitert bei keinem dieser 500 Seeds (Generator, Session WZ8.3).
- **AC-06** Jede ungültige Planung aus „Ausnahme- und Fehlerfälle“ wird beim Laden abgelehnt und mit Insel, Tiefe, Platz und Grund benannt; je Fall ein Go-Test (darunter Platz ±3 als erster erlaubter Portal-Platz im Wald, die Mine mit nur noch einem Portal-Platz, die Adern ohne freien Ressourcen-Chunk und Platz ±4 mit `at` < 10 in den 25-Unit-Biomen). Die Prüfung nimmt die Mauerlinien als Parameter: Eine Ader bei Hub +136 (Höhle, Platz +2, Lage 36) ist mit den eingebetteten Linien gültig und mit Linie 5 bei 126 statt 124 abgelehnt. Der Nachweis über 500 Seeds steht bei AC-05.
- **AC-07** Feste Objekte wirken in der Sim wie gewürfelte gleicher Art. Go-Test mit `CreateIsland` (Options-Struktur mit Test-Planung) und 2 Spielern: fester Baum abbaubar, feste Truhe aufhebbar, festes Camp liefert Landstreicher, aus dem festen Portal kommt die Welle. Alle bisherigen Aufrufer von `CreateIsland` laufen mit dem Nullwert der Options-Struktur unverändert (AC-02).
- **AC-08** Die Vorschau-Funktion aus WZ7.1 nimmt die Planung als Option „geplante Chunks“ ihrer Options-Struktur an. Go-Test: gleiche Planung → gleicher Snapshot; ohne Option gilt die eingebettete Planung; eine ungültige wird mit dem Fehler aus AC-06 abgelehnt; kein globaler Zustand (zwei Aufrufe mit verschiedener Planung nacheinander liefern je ihren Snapshot). Der Test mit gleichzeitig gesetzter Kandidaten-`hub.json` (Vorrang der Kandidaten-Linien) liegt in WZ9.4 (B-462/AC-09), weil das Feld erst dort entsteht.
- **AC-09** Der Welt-Betrachter (B-460) bearbeitet geplante Chunks: Insel und Tiefe wählen, Chunk-Platz wählen, Art, Chunk-Seed und Objekte setzen, verschieben und löschen. Chunk- und Live-Vorschau zeigen jede Änderung des Entwurfs sofort; Prüffehler aus AC-06 stehen am Platz; Speichern schreibt nur eine gültige Planung nach `data/islands.json`.
- **AC-10** 🧑 plant eine Sondergegend im Editor und nimmt sie in Chunk- und Live-Vorschau ab: gleicher Ort und gleicher Inhalt bei zwei verschiedenen Seeds. Die Gegend bleibt ein ungespeicherter Entwurf oder wird als REG-Ticket festgehalten; der Sprint committet keine Planung in `data/islands.json` und ändert keine Golden-Daten außer dem neuen Fall aus AC-03. Im Spiel weist AC-07 das Verhalten mit der Test-Planung nach.
- **AC-11** Die Chunk-Vorschau des Welt-Betrachters wählt Insel und Stufe und liest ihr Level aus derselben Quelle wie die Live-Vorschau. Test: Für denselben Entwurf, dieselbe Insel, Stufe und denselben Seed zeigen beide Vorschauen dieselben Chunks und Objekte (Layout aus der Quelle gleich dem Level-Teil des Snapshots).
- **AC-12** Schreibregel (Werkzeug schreibt REG-Daten): Laden und Speichern ohne Änderung ist byte-gleich; eine Änderung ändert nur den betroffenen Eintrag; Schreiben ist atomar; bei extern geänderter Datei wird nicht gespeichert, neu geladen und gemeldet, der Entwurf bleibt; nach dem Speichern erscheint der REG-Hinweis „REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung“. Tests mit Kopie in `t.TempDir()`; `git status` zeigt nach den Testläufen keine Änderung an `data/islands.json`.
- **AC-13** Leistung: Der Benchmark der Level-Erzeugung (Baseline vor WZ8, gleicher Rechner) zeigt ohne Planung höchstens +10 % Laufzeit je Biom und mit der Test-Planung höchstens +10 % gegenüber demselben Level ohne Planung; Messwerte stehen im Commit-Text des Code-Commits der Session WZ8.3.

## Offene Fragen

**Entschieden** (🧑, 2026-10-17, Chat):

1. **Ort:** Geplante Chunks liegen je Insel und Stufe in `data/islands.json`, nicht je Biom. Andere Level bleiben zufällig; ohne Planung bleiben `testdata/golden/level-*.json` byte-gleich.
2. **Inhalt gewünscht:** alle vier Arten. (a) Feste Objekte: Ressourcen, Adern, Truhen, Camps, Portale. (b) Feste Gebäude/Ruinen: vorgebaute Bauten, braucht neue Regel (REG) und Sim-Anteil. (c) Seed je Chunk: Chunk-Art fest, Inhalt aus eigenem Strom. (d) Feste Gegner/Gegnerlager: betrifft die Wellen-Regeln (REG). Die Aufteilung auf Tickets regelt Entscheidung 4.
3. **Editor:** im Welt-Betrachter des Asset-Studios (WZ7) mit sofortiger Chunk- und Live-Vorschau.
4. **Teilung angenommen** (2026-10-17, Chat): WZ8 setzt feste Objekte und den Seed je Chunk um (Teil 1, dieses Ticket); Ruinen/Gebäude und Gegnerlager (b, d) wandern in **B-468** (Projekt WZG, Folge-Sprint nach WZ8, abhängig von WZ8).
5. **SRV-Session aufgenommen** (2026-10-17, Chat): WZ8.4 erweitert die Vorschau-Funktion aus WZ7.1 um die Planung als Option, damit die Live-Vorschau ungespeicherte Entwürfe zeigt; der Sprint hat 7 Sessions.

**Offen, im Workshop WZ8.1** (Teil von AC-01, blockiert die Freigabe nicht):

6. Chunk-Platz relativ zu den Hub-Chunks (Annahme) oder relativ zu Tiefen-Eingang/Rand?
7. Ersetzt eine Objektliste die gewürfelten Ressourcen der Chunk-Art (Annahme) oder ergänzt sie sie? Feste Skill-Punkte erlaubt (Annahme: nein)?
8. Lage-Regeln: 4 Units Abstand zu Bauplätzen nur für Adern (Annahme) oder auch für Truhen und Camps? Stromnamen wie in Regeln?
9. Verteilung übriger Portale bei festen Portalen (Annahme: abwechselnd, beginnend auf der Seite mit weniger festen) und Anrechnung fester Truhen/Camps auf die gewürfelte Zahl der Ereignis-Chunks.

## Notizen

**Teilung (entschieden, Offene Fragen 4):** Alle vier Inhalte passen nicht in einen Sprint. Teil 1 allein braucht REG → SIM → SIM → SRV → DEV plus getrenntes Review und Abnahme, also 7 Sessions (wie WZ7 über dem Richtwert 3–6).

- **Teil 1 – WZ8 (dieses Ticket):** Format, Chunk-Seed, feste Objekte, Generator, Prüfung, Planung und Linien als Parameter (Option der Vorschau-Funktion), Editor, Chunk-Vorschau auf Insel und Stufe.
- **Teil 2 – B-468** „Geplante Chunks tragen Ruinen und Gegnerlager“ (Domänen REG → SIM → SRV → CLI → DEV, abhängig von WZ8): Ausgangslage, Regelfragen und Aufwand stehen dort; hier nur die Schnittstelle: Das Format reserviert den Platz, der Generator lehnt die Felder bis dahin mit „noch nicht unterstützt“ ab.

**Verworfen:** Geplante Chunks erst nach der Erzeugung überlagern, damit der Haupt-Strom unverändert bleibt. Portale und Ereignisse landen dann abhängig vom Seed auf geplanten Plätzen; Konflikte wären erst zur Laufzeit sichtbar und die Biom-Summen verletzt. Mit Reservierung lässt sich der Haupt-Strom aber nicht ganz unverschoben halten, weil `rng.Shuffle` einen Wurf je freiem Chunk verbraucht. Deshalb gilt AC-04 nur für den Inhalt geplanter Chunks.

**Linien als Parameter (Review-Befund, 2026-10-17):** Ohne Linien-Parameter in `engine/level` könnten Snapshot und Prüfung nur die eingebetteten `wallLines` sehen; die Kandidaten-`hub.json` des Gebäude-Planers (B-462, WZ9) bliebe wirkungslos. Deshalb nehmen Generator und Prüfung die Linien als Parameter (Standard eingebettet, kein globaler Zustand), `sim.CreateIsland` bekommt eine Options-Struktur (WZ8.3, alle Aufrufer mit Nullwert angepasst), und bei gleichzeitig gesetzter Planung und Kandidat gelten die Kandidaten-Linien. WZ9.3 ergänzt die Linien-Parameter in `engine/level`, falls WZ8 noch nicht erledigt ist.

**Messwerte** (kürzeste Länge, `data/biomes/*.json`, gegengerechnet mit `engine/level/level.go` › `frame()`, `eligible()`; Hub = `count/2 · Chunk-Breite`, Hub-Chunks decken Hub ±50, Platz +1 beginnt bei Hub +50):

| Biom | Chunks | Hub-Chunks | planbar | portaltauglich (Chunk-Mitte ≥ Mindestabstand) |
|---|---|---|---|---|
| Wald | 18 × 50 | 2 | ±1…±7 | ±3…±7 (≥ 150, Mitte ab ±175) |
| Höhle | 14 × 50 | 2 | ±1…±5 | ±3…±5 |
| Mine | 12 × 50 | 2 | ±1…±4 | ±3, ±4 |
| Eisenstollen | 20 × 25 | 4 | ±1…±7 (gerechnet, WZ8.2 prüft) | ±4…±7 (≥ 135, Mitte ab ±137,5; gerechnet) |
| Kristallhöhle | 16 × 25 | 4 | ±1…±5 (gerechnet, WZ8.2 prüft) | ±4, ±5 (gerechnet) |

Hinweis zu den 25-Unit-Biomen (Eisenstollen, Kristallhöhle): Platz ±4 beginnt bei Hub ±125, sein Chunk gilt als Portal-Chunk (Mitte ±137,5), aber ein festes Portal als **Objekt** in einem anderen Chunk auf ±4 braucht `at` ≥ 10 (Hub ±135 = Mindestabstand); ab Platz ±5 (Hub ±150) genügt jedes `at` (Objekt-Abstand zur Chunk-Grenze ≥ 2 Units). In den 50-Unit-Biomen beginnt der erste portaltaugliche Platz ±3 bei Hub ±150, dort gilt jedes `at`.

Ort der Bauplätze (Hub-Abstand, Streuung der Linien +0 bis +4, Turm 8 innen, Tor 4 außen, Linie 1 ohne Streuung): Linie 1 (Turm ±36, Mauer ±44, Tor ±48) liegt in den Hub-Chunks. Die Plätze ±1 und ±2 (50-Unit-Biome, Hub ±50 … ±150) enthalten die Linien 2–5 (Turm ab ±56, Tor bis ±132) und den Farm-Weltplatz ±52. Frei von Mauer, Turm und Tor um mindestens 4 Units (Regel der Adern) ist dort nur der Bereich ab Hub ±136, also Platz ±2 ab Lage 36.

**Glossar-Befund:** „Chunk … 50 Units breit“ stimmt für Eisenstollen und Kristallhöhle nicht (25 Units); WZ8.1 korrigiert (AC-01).

**Bezug:** B-460 (Welt-Betrachter, WZ7.1 Vorschau-Funktion SRV mit Options-Struktur, Studio `tools/k3c-studio` aus WZ6), B-462 (Gebäude-Planer, WZ9; gleiche Schreibregeln, Kandidaten-`hub.json` als Option der Vorschau-Funktion), B-468 (Teil 2), B-103 (mehrere Inseln).
