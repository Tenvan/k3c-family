# Regelwerk: Welten und Welt-Datei

Workshop WZ18.1 am 2026-10-19: 🧑 hat die Entscheidungen im Chat an Claude delegiert. Jede Entscheidung trägt ihre Quelle:
**B-488** (von 🧑 freigegebene Spec) oder **entschieden von Claude (Delegation 🧑, 2026-10-19)** nach den Empfehlungen der
Session WZ18.1. Grundlage: B-488, Sprint WZ18; Stufen und Inseln: [`stufen.md`](stufen.md); Planung und Chunk-Anzahl einer
Insel: [`geplante-chunks.md`](geplante-chunks.md). Je Regel: **Regel · Begründung · Daten**.

Umsetzung: WZ18.2 (Laden, Prüfung, Datei verschieben), WZ18.3 (Spielstand, Inselwechsel), WZ18.4 (Raum), WZ18.5 (Protokoll),
WZ18.6 (Welten Editor). Bis WZ18.2 liegt die Standard-Welt noch als `data/islands.json` (ohne `id` und `name`).

## 1. Welt und Ort

| Regel | Begründung | Daten |
|---|---|---|
| Eine **Welt** ist eine Sammlung von n Inseln in fester Reihenfolge; ein Raum spielt genau eine Welt, die er beim Anlegen wählt. Quelle: B-488. | Inseln lassen sich als Ganzes entwerfen, tauschen und testen, ohne die Standard-Welt zu ändern. | – |
| Jede Welt ist **eine Datei** `data/worlds/<id>.json` (Welt-Datei). Die **Standard-Welt** hat die ID `standard`; `data/islands.json` wird zu `data/worlds/standard.json` und entfällt. Quelle: B-488 › Offene Fragen. | Ein Ort für alle Welten; die Standard-Welt ist eine Welt wie jede andere. | `data/worlds/standard.json` |
| Eine Welt **erbt die Basis-Werte** aus `data/` (Ressourcen, Kosten, Material-Vorrat, Gegner, Truppen, Gebäude, Monarch, Wellen, Ziele) und **übersteuert** mit `overrides` nur die Felder, die sie ändern will. Ohne `overrides` gilt die Basis unverändert (byte-gleich, Golden unverändert). Quelle: B-680, Entscheidung 🧑 2026-10-31. Ersetzt die Regel „für sich vollständig, erbt nichts“ (WZ18.1). | Eine Welt nennt nur ihre Abweichung; eine Änderung an der Basis gilt für alle Welten, die das Feld nicht übersteuern. | `data/*.json`, `overrides` |
| Eine Welt kann mit **`extends: <welt-id>`** von einer anderen Welt erben (Kette Basis → A → B). Aufgelöst wird von der Basis her: erst die Overrides der ältesten Eltern-Welt, zuletzt die der Welt selbst; das Kind gewinnt. Ohne `extends` erbt die Welt direkt von der Basis. Höchstens **4 Welten** in einer Kette über der Basis (Startwert). Quelle: B-680, Entscheidung 🧑 2026-10-31. | Varianten einer Welt (z. B. „Nordmeer schwer“) teilen ihre Abweichungen, ohne sie zu kopieren. | `extends` |
| Eine Änderung an einer Eltern-Welt wirkt **bewusst auf ihre Kinder**. Eine Welt mit Kind-Welten wird nicht gelöscht und nicht umbenannt (Meldung nennt die Kinder); eigenständig wird eine Welt über „Welt kopieren“ (flach machen: Overrides der Kette in die Kopie, ohne `extends`). Der Welten-Editor zeigt die Kind-Welten vor dem Speichern (B-682). Quelle: B-680. | Kein Kind verliert still seine Grundlage. | – |
| **Basiswerte sind Solo.** Mehr Spieler (coop2, coop4) werden nur über die Multiplikator-Felder justiert (`waves.json` › `perExtraPlayer`, `bossHpPerExtraPlayer`); eine Welt übersteuert diese Felder wie jedes andere, aber nie die Basiswerte „für Coop“. Quelle: BAL8, 🧑. | Eine Stelle für die Spielerzahl, Solo bleibt der Maßstab. | `data/waves.json` |

### 1.1 Overrides: Format und Merge

`overrides` ist ein Objekt **je Basis-Datei** (Schlüssel = Dateiname ohne `.json`), erlaubt sind `buildings`, `ship`, `troops`,
`economy`, `hub`, `monarch`, `enemies`, `waves`, `goals`, `events` (Abstand, Stärke und Lohn je Event, Abstand 0 = aus, B-982; `bosse.md` § 2). Jeder Eintrag hat **dieselbe Struktur wie die Basis-Datei** und nennt nur
die geänderten Felder. Entschieden von Claude (autonom, B-680):

| Wert in der Basis | Override | Ergebnis |
|---|---|---|
| Zahl, Text, true/false | gleicher Typ | ersetzt den Basiswert |
| Objekt (z. B. `cost`) | Objekt | je Feld gemischt; nicht genannte Felder bleiben |
| Einträge je ID (z. B. `buildings.wall`, `enemies.wolf`, `monarch.skills`) | Objekt | je Schlüssel gemischt; nur vorhandene IDs, **keine neuen Einträge** |
| Liste von Objekten (z. B. `levels`, `table`, `ship.stages`) | Objekt `{"<Schlüssel>": {…}}` | je Eintrag gemischt; Schlüssel ist die `id` des Eintrags (z. B. `"hull"`), sonst der Index ab 0 (`"1"`); **nie als Ganzes**, keine neuen Einträge |
| Liste von Werten (z. B. `gold: [3, 5]`, `traits`) | Liste | ersetzt die ganze Liste; eine feste Liste (`gold`, `chestGold`, `standard`) braucht genau ihre Länge |

Schreibweise der Felder wie in der Basis (`hp`, nicht `HP`); ein Feld, das die Basis-Datei an dieser Stelle nicht nennt, das die
Simulation aber kennt (z. B. `plantation` an einem Gebäude), ist erlaubt. `null` ist kein Wert: Felder lassen sich nicht löschen.
Beispiel „Nordmeer“ (Mauer billiger in Holz, Stufe 2 der Mauer zäher, Rumpf des Schiffs teurer, mehr Gegner je Zusatzspieler):

```json
{
  "id": "nordmeer", "name": "Nordmeer", "extends": "standard",
  "overrides": {
    "buildings": { "wall": { "cost": { "wood": 7 }, "levels": { "1": { "hp": 650 } } } },
    "ship": { "stages": { "hull": { "cost": { "gold": 9 } } } },
    "waves": { "perExtraPlayer": 0.37 }
  },
  "islands": [ … ]
}
```

### 1.2 Prüfung und Fehlermeldungen

Fail closed: Ein ungültiger Override oder eine ungültige Kette → die Welt wird nicht geladen, nichts wird repariert. Geprüft wird
beim Lesen der Welt-Datei (`extends` ist eine Welt-ID nach § 4 und nicht die eigene, jeder Schlüssel in `overrides` ist eine erlaubte
Datei) und beim Auflösen (`engine/sim` › `ResolveWorldValues`): jedes Feld gegen den Lese-Typ der Simulation, danach der gemischte
Stand mit denselben Regeln wie die Basis (Traits bekannt, Presets nennen bekannte Skills, `waves.json`-Zuschläge ≥ 0 und
`bossHpPerExtraPlayer` vorhanden, Gegner-Pools jedes Bioms mit Standard- und Elite-Gegner). Nach jeder Welt der Kette mit Overrides
wird geprüft; die Meldung nennt die Welt, deren Override den Fehler auslöst. Muster `<Welt-Datei>: overrides › <Basis-Datei> › <Pfad>: <Grund>`:

| Fall | Meldung (Beispiel) |
|---|---|
| unbekannte Datei | `data/worlds/nordmeer.json: overrides: unbekannte Datei "biomes" (erlaubt: buildings, economy, …)` |
| unbekanntes Feld | `data/worlds/nordmeer.json: overrides › data/buildings.json › wall.size: unbekanntes Feld` |
| unbekannte ID / unbekannter Eintrag | `… › data/buildings.json › palast: unbekannter Eintrag (neue Einträge sind nicht erlaubt)` |
| falscher Typ | `… › data/buildings.json › wall.cost.wood: Ganzzahl erwartet, ist Text` |
| Liste als Ganzes | `… › data/buildings.json › wall.levels: Liste von Objekten je Eintrag übersteuern ({"<id oder Index>": {…}}), ist Liste` |
| feste Liste falsch lang | `… › data/enemies.json › wolf.gold: genau 2 Werte erwartet, sind 1` |
| Schreibweise | `… › data/buildings.json › wall.HP: Schreibweise wie in der Basis: "hp"` |
| Event-Regel verletzt | `data/worlds/nordmeer.json: overrides: data/events.json: bloodMoon braucht Faktoren > 0` (ebenso: Abstand < 0, unbekannte Gegnerart) |
| Basis-Regel verletzt | `data/worlds/nordmeer.json: overrides: data/waves.json: perExtraPlayer und bossHpPerExtraPlayer dürfen nicht negativ sein` |
| Eltern-Welt unbekannt | `data/worlds/nordmeer.json: extends: Eltern-Welt "eis" unbekannt: …` |
| Zyklus | `data/worlds/b.json: extends: Zyklus b → a → b` |
| Kette zu lang | `data/worlds/e.json: extends: Kette zu lang (e → d → c → b → a, höchstens 4 Welten)` |
| erbt von sich selbst | `data/worlds/nordmeer.json: extends: Welt erbt von sich selbst` |
| Löschen/Umbenennen mit Kindern | `data/worlds/a.json: Welt hat Kind-Welten (b, c), Löschen und Umbenennen abgelehnt` |

Umsetzung: `engine/sim/world_values.go` (Auflösung, Kette, Kind-Welten), `world_values_merge.go` (Merge und Feldprüfung). Die
Simulation rechnet bis BAL8.2 (B-681) weiter mit den globalen Basis-Werten; erst dann gilt der Wertesatz je Raum.

### 1.3 Nicht übersteuerbare Felder

Der Level-Generator (`engine/level`) liest einige Werte aus den eingebetteten Basis-Dateien, nicht aus dem Wertesatz der Welt.
Ein Override darauf ließe Level (Lage der Plätze, Zahlziele, Ruinen-Planung) und Simulation auseinanderlaufen; deshalb lehnt
die Prüfung (§ 1.2) es beim Laden mit Pfad ab, Meldung `… › data/economy.json › payRangeUnits: nicht übersteuerbar (der
Level-Generator liest die Basis, welten.md § 1.3)`. Entschieden von Claude (B-692): sperren statt durchreichen, weil das
Durchreichen den Level-Generator und seine Planungsprüfung je Welt umbauen würde.

| Feld | Leser in der Basis |
|---|---|
| `economy.payRangeUnits` | Abstand der Zahlziele (`level/lines.go`) |
| `hub.sites`, `hub.islandSites` | Hub- und Insel-Plätze (`level/lines.go`, Ruinen-Lage `level/ruins.go`) |
| `hub.sitePlacement` | Mauer- und Turmplätze, Freiraum (`level/sites.go`, `sim/world_lines.go`, `level/guards.go`, `level/planned.go`) |
| `buildings.<id>.plantation.stripUnits` | Randabstand einer Ruine (`level/ruins.go`) |

Die **HP einer Ruine** folgen dagegen dem Wertesatz: Start-HP = `hpShare` (`data/ruins.json`) × HP der Art in der Welt.
Umsetzung: `engine/sim/world_values_locked.go`.

## 2. Format

Entschieden von Claude (Delegation 🧑, 2026-10-19), nach B-488 › Anforderungen und Empfehlung 1 der Session WZ18.1.

**Welt** (oberste Ebene, Reihenfolge der Felder beim Schreiben wie in der Tabelle):

| Feld | Pflicht | Typ | Regel |
|---|---|---|---|
| `id` | ja | Text | Welt-ID (§ 4), **gleich dem Dateinamen** ohne `.json`. |
| `name` | ja | Text | Anzeigename, nicht leer (z. B. „Standard“, „Nordmeer“); muss nicht eindeutig sein. |
| `notes` | nein | Text | Freie Notiz für Entwurf und REG-Begründung; das Spiel liest sie nicht. Fehlt sie, gilt `""`. |
| `extends` | nein | Text | Eltern-Welt (Welt-ID nach § 4, nicht die eigene); fehlt es, erbt die Welt von der Basis `data/` (§ 1). |
| `overrides` | nein | Objekt | Übersteuerte Basis-Werte je Basis-Datei (§ 1.1); fehlt es oder `{}`, gilt die Basis (bzw. die Eltern-Welt). |
| `combat` | nein | Objekt | Kampfvariante der Welt (`kampf.md` § 3): `mode` (`groups` Standard oder `general`) und `groupSize` (ganze Zahl ≥ 1, Standard 5, nur bei `groups`); fehlt es, gilt `groups`/5. |
| `story` | nein | Objekt | Geschichte der Welt (§ 2.2): `lexicon`, `start`, `etappen`, `end`, `bottles`; fehlt es, gibt es keine Zeilen der Welt. `extends` vererbt es nicht. |
| `islands` | ja | Liste | Inseln in Spielreihenfolge (Insel 1 zuerst), mindestens eine. |

**Insel** (Eintrag in `islands`, Felder wie heute in `data/islands.json`, Reihenfolge beim Schreiben wie in der Tabelle):

| Feld | Pflicht | Typ | Regel |
|---|---|---|---|
| `id` | ja | Text | Insel-ID (§ 4), eindeutig in der Welt. |
| `name` | ja | Text | Anzeigename, nicht leer (z. B. „Insel 1“). |
| `etappe` | nein | Ganzzahl ≥ 1 | Etappe der Insel (§ 6.1); fehlt sie, gilt Listenplatz + 1 (heute: jede Insel eine eigene Etappe, linear). |
| `story` | nein | Objekt | Geschichte der Insel (§ 2.2): `arrival`, `boss`, `done`; fehlt es, gibt es keine Zeilen der Insel. |
| `depths` | ja | Liste Zahlen | Tiefen der Stufen, jede ein bekanntes Biom (`data/biomes/*.json`), jede höchstens einmal; Lücken erlaubt (z. B. `[0, 4]`). |
| `depthScaling` | ja | Objekt | `hp`, `damage`, `speed`: Basen der Potenz je Tiefe (`stufen.md` § 1, Skalierung), alle > 0. |
| `plannedChunks` | nein | Objekt | Geplante Chunks je Tiefe (`geplante-chunks.md` § 1–5); fehlt oder `{}` = reiner Zufall. |
| `chunkCounts` | nein | Objekt | Chunk-Anzahl je Tiefe (`geplante-chunks.md` § 7); ohne Eintrag gilt die Länge des Bioms. |
| `endBoss` | nein | Text | ID eines Endbosses aus `data/bosses.json › endBosses` (B-981); fehlt das Feld, gilt der Endboss der **tiefsten** Tiefe der Insel. Der **Bau** des Endbosses liegt immer in der tiefsten Stufe, in jedem Biom (`bosse.md` § 1). |
| `nature` | nein | Objekt | Natur-Werte als Vorgabe der Insel (§ 2.1): `{traits, weights}`; gilt in jeder Stufe, deren Biom die Eigenschaft hat. |
| `natureByDepth` | nein | Objekt | Natur-Werte je Tiefe (§ 2.1), gewinnen über `nature`; nur Eigenschaften des Bioms der Tiefe. |

**Unbekannte Felder** (auf jeder Ebene, auch in `plannedChunks`) werden **abgelehnt** und mit Namen genannt, nicht still
verworfen (B-488 › Ausnahme- und Fehlerfälle). Fehlende Pflichtfelder ebenso, auch einzelne Faktoren in `depthScaling`
(Meldung „Pflichtfeld fehlt“, nicht „Faktoren > 0“). Das ist neu: `loadFrom` (`engine/sim/data.go`) liest heute nachsichtig
(`json.Unmarshal`), nur `PlannedChunk` ist schon strikt; WZ18.2 baut dafür einen strikten Decoder
(`DisallowUnknownFields`) und erkennt fehlende Felder (z. B. Zeiger-Felder oder Prüfung der Rohdaten).

Beispiel `data/worlds/nordmeer.json` (Welt „Nordmeer“, Insel 1 als Kopie der Standard-Welt, Insel 2 kurz und härter):

```json
{
  "id": "nordmeer",
  "name": "Nordmeer",
  "notes": "Entwurf: Insel 1 wie standard, Insel 2 nur Wald und Kristallhöhle.",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": [0, 1, 2, 3, 4], "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 }, "plannedChunks": {} },
    { "id": "eisinsel", "name": "Eisinsel", "depths": [0, 4], "depthScaling": { "hp": 1.6, "damage": 1.4, "speed": 1.15 },
      "plannedChunks": { "0": [ { "slot": -3, "kind": "clearing", "seed": "lichtung" } ] },
      "chunkCounts": { "0": 24 } }
  ]
}
```

Die Standard-Welt `data/worlds/standard.json` (WZ18.2) ist die bisherige `data/islands.json` mit `"id": "standard"` und
`"name": "Standard"` davor; die Liste `islands` bleibt **byte-gleich** (B-488/AC-02, Golden unverändert), `notes` darf auf die
neuen Pfade angepasst werden (das Spiel liest sie nicht). Entschieden von Claude (Delegation 🧑, 2026-10-19).

### 2.1 Natur-Eigenschaften (B-891)

Jedes Biom nennt in `data/biomes/<biom>.json` seine Eigenschaften und Gewichts-Gruppen; die Liste steht in den Daten:

- `traits`: je Eigenschaft `{name, min, max, default}` und genau eine Wirkung: `scales` (Anzahl der Ressourcen-Arten aus
  `resourcesPerChunk` oder der Adern-Art), `chunks` (Gewicht der Chunk-Arten in `chunkWeights`) oder `display: true` (nur
  Darstellung). Der Faktor ist Wert / `default`; `default` ergibt das Level wie ohne Eigenschaft. Mengen werden
  kaufmännisch auf ganze Zahlen gerundet; würden alle Chunk-Gewichte 0, bleiben die des Bioms.
- `weights`: je Gruppe `{name, kind, options}`, je Variante `{min, max}` mit Gewicht 0–5. Jedes Objekt der Art `kind`
  bekommt eine Variante mit der Wahrscheinlichkeit Gewicht / Summe; Monokultur = eine Variante > 0. Ergeben gewürfelte
  Gewichte zusammen 0, bekommt die erste gewürfelte Variante mit `max` > 0 das Gewicht 1.

Je Stufe gilt ein Wert: fest gesetzt in der Welt (`natureByDepth` vor `nature`) oder aus dem Seed gewürfelt
(`engine/rng`, Strom `<biom>:<seed>:nature`, in Datei-Reihenfolge; ein fester Wert verschiebt die übrigen Würfe nicht).
Varianten würfeln je Objekt aus dem Strom `<biom>:<seed>:variant:<gruppe>:<x>`. Geplante Chunks würfeln ihren Inhalt
ohne Natur-Faktoren. Feste Werte prüft das Laden: Bereich, bekannte Eigenschaft und Gruppe (die Vorgabe der Insel muss
in mindestens einem Biom der Insel gelten), nicht alle Varianten einer Gruppe fest auf 0. Der Spielstand speichert die
geltenden Werte je Stufe (`nature`); das Laden baut damit dieselbe Natur.

Beispiel (Insel 1, Wald als Pappel-Monokultur mit dichtem Wald, Felsen auf allen Stufen mit Felsdichte):

```json
"nature": { "traits": { "rockDensity": 3 } },
"natureByDepth": { "0": { "traits": { "forestDensity": 5 }, "weights": { "treeSpecies": { "tanne": 0, "pappel": 3, "birke": 0 } } } }
```

### 2.2 Geschichte (B-965)

Quelle: B-965 (🧑 2026-11-15, Ideenrunde). Jede Welt und jede Insel erzählt eine kurze **Geschichte** in einer
**Kunstsprache** mit eigenem Klang je Welt; kurze echte Wörter deuten die Handlung an. Die Zeilen stehen fest in der
Welt-Datei; wann sie gelten, regelt § 6.2. Alles ist optional (B-965/R5): Fehlt `story` oder ein Eintrag, gibt es dort
keine Zeilen, sonst ändert sich nichts.

**`story` der Welt:**

| Feld | Typ | Regel |
|---|---|---|
| `lexicon` | Liste Text | **Wortschatz** der Kunstsprache: Kunstwörter in Kleinbuchstaben, Apostroph und Bindestrich im Wort erlaubt (`akh'et`, `sen-hotep`). Größe höchstens die Obergrenze (§ 3). |
| `start` | Eintrag | Zeilen beim Start einer neuen Kampagne. |
| `etappen` | Objekt | Etappen-Nummer (Dezimalzahl ohne führende Null, § 6.1) → Eintrag: Zeilen beim Abschluss dieser Etappe. |
| `end` | Eintrag | Zeilen beim Kampagnensieg. |
| `bottles` | Liste Einträge | Zeilen für die Flaschenpost; hier nur Format und Prüfung, genutzt erst von B-971. |

**`story` der Insel:**

| Feld | Typ | Regel |
|---|---|---|
| `arrival` | Eintrag | Zeilen bei der Ankunft auf der Insel. |
| `boss` | Eintrag | Zeilen, wenn der Endboss der Insel ausgelöst wird. |
| `done` | Eintrag | **Abschlusszeile** nach dem Sieg über den Endboss (B-966 zeigt sie als Andenken). |

**Zeilenregel** (B-965/R3): Ein **Eintrag** ist eine Liste aus 1 bis 2 Zeilen (Text, nicht leer, höchstens so lang wie
die Obergrenze). Wörter trennt der Leerraum; Satzzeichen am Rand eines Worts zählen nicht mit, ein Stück ohne Buchstaben
(etwa ein Gedankenstrich) ist kein Wort. Ein **echtes Wort** hat mindestens zwei Buchstaben und nur Großbuchstaben
(`KRONE`); es ist deutsch und wird nicht übersetzt. Jedes andere Wort ist ein **Kunstwort**, steht klein (am Satzanfang
groß begonnen) und muss im `lexicon` stehen, Groß- und Kleinschreibung egal. Je Zeile stehen 2 bis 3 echte Wörter; eine
Zeile darf zwei kurze Sätze haben.

Beispiel (Ausschnitt `data/worlds/sengoku.json`; Texte nur als Muster, die echten schreibt B-976):

```json
"story": { "lexicon": ["oru", "kamae", "no", "sen", "shiro", "nu"],
  "start": ["Oru kamae KRONE no sen. Shiro nu BRENNT."],
  "etappen": { "1": ["Sen no MEER, oru KRONE kamae."] },
  "end": ["Shiro no FRIEDE, kamae KRONE sen."] },
"islands": [ { "id": "island1", "name": "Reisfeld-Ebene", "story": {
  "arrival": ["Oru no REIS, sen SCHIFF nu."],
  "boss": ["Nu kamae MASKE, shiro TIEF."],
  "done": ["MASKE nu, KRONE oru sen."] } } ]
```

## 3. Prüfung beim Laden

Die Prüfung gilt für **jede** Welt-Datei, gleich ob eingebettet, im Dev-Mode von der Platte gelesen oder im Welten Editor als
Entwurf (B-488 › Anforderungen). Sie ist die heutige Prüfung von `engine/sim/island_data.go` (`loadIslands`, `checkIsland`,
`checkChunkCounts`, `checkPlannedChunks`) plus die neuen Regeln der Welt. Reihenfolge der Prüfung je Insel: ID, Name, Tiefen (bekannt, einmalig), dann Bosse,
Faktoren, `chunkCounts`, `plannedChunks`; die erste verletzte Regel wird gemeldet (heute prüft `checkIsland` den Endboss
vor den Tiefen, WZ18.2 dreht das, damit eine unbekannte Tiefe als solche gemeldet wird). Fail closed: bei einem Fehler wird die Welt nicht
geladen, nichts wird repariert.

| Regel | Begründung | Daten |
|---|---|---|
| Datei ist JSON nach § 2: keine unbekannten Felder, alle Pflichtfelder da; `id` passt zum Dateinamen; `name` nicht leer; mindestens eine Insel. | Fehler in Hand- oder Prompt-Entwürfen fallen sofort auf. | `data/worlds/<id>.json` |
| Je Insel: `id` nach § 4 und eindeutig in der Welt; `name` nicht leer; `depths` nicht leer, jede Tiefe bekannt und höchstens einmal. | Die Insel-ID ist der Verweis im Spielstand (§ 5). | `data/biomes/*.json` |
| Je Insel: jede Tiefe hat einen **Miniboss**. Der **Endboss** ist `endBoss` (eine ID aus `endBosses`, kein Miniboss), ohne Feld der Endboss der **tiefsten** Tiefe (`endBosses` kennt die Tiefen 0 bis 4, B-981). Die tiefste Stufe trägt den **Bau**: Ihr Biom muss ihn samt Camp-Abstand tragen (`level.CheckLairFit`, mit der Inselgröße aus `chunkCounts`, sonst der kürzesten Länge). | Jede Stufe hat ihr Ziel, jede Insel ihr Ende (`stufen.md` § 3), in dem Biom, das zu ihrem Thema passt. | `data/bosses.json`, `data/biomes/*.json` |
| Je Insel: `etappe` ab 1, über die Liste lückenlos (die erste Insel hat Etappe 1, jede weitere dieselbe oder um 1 höher) und nicht fallend. Jede Insel braucht einen Hafen, außer der einzigen Insel der letzten Etappe (B-961). | Jede Etappe öffnet erst nach der vorigen; ein Ziel ohne Hafen ließe das Schiff stranden. | – |
| Je Insel: `depthScaling` mit `hp`, `damage`, `speed` jeweils > 0. | Keine Gegner mit 0 HP oder Stillstand. | – |
| `chunkCounts`: Schlüssel ist eine Tiefe der Insel (Dezimalzahl ohne führende Null), Wert eine Inselgröße aus der Tabelle. | `geplante-chunks.md` § 7. | `data/island-sizes.json` |
| `plannedChunks`: Schlüssel ist eine Tiefe der Insel; die Planung besteht die Prüfung gegen die Mauerlinien und die Chunk-Anzahl der Tiefe, unabhängig vom Seed (`level.ValidatePlanFor`). | `geplante-chunks.md` § 5. | `data/biomes/*.json` |
| `story` der Welt und jeder Insel (§ 2.2, B-965/R4): keine unbekannten Felder; `lexicon` höchstens so groß wie die Obergrenze; je Eintrag 1 bis 2 Zeilen, keine leer, keine länger als die Obergrenze (in Zeichen, nicht Bytes); jedes Kunstwort steht im `lexicon`; je Zeile 2 bis 3 echte Wörter; jede Etappen-Nummer in `etappen` hat mindestens eine Insel. Geprüft wird nach den Inseln, in der Reihenfolge `lexicon`, `start`, `etappen` (aufsteigend), `end`, `bottles`, dann je Insel `arrival`, `boss`, `done`; je Zeile: leer, Länge, Kunstwörter, Anzahl echter Wörter. | Die Zeilen bleiben kurz und am TV lesbar; ein Tippfehler im Kunstwort fällt beim Laden auf, nicht erst im Spiel. | `data/story.json` (Obergrenzen, vorläufig: Zeile 64 Zeichen, Wortschatz 24 Wörter; angelegt in GS1.2) |

**Fehlermeldungen** nennen Datei (und damit Welt), Insel, Tiefe und Platz, soweit vorhanden, im Muster
`<Datei>: Insel <insel-id>: <Grund>` (Grund wie heute aus `checkIsland`). Verbindlich sind die Teile, nicht jedes Wort:

| Fall | Meldung (Beispiel) |
|---|---|
| kein JSON | `data/worlds/nordmeer.json: kein gültiges JSON: …` |
| unbekanntes Feld | `data/worlds/nordmeer.json: unbekanntes Feld "size"` |
| ID passt nicht zur Datei | `data/worlds/nordmeer.json: id "nord" passt nicht zum Dateinamen` |
| Welt ohne Insel | `data/worlds/nordmeer.json: keine Insel` |
| ID ungültig | `data/worlds/nordmeer.json: Insel "Insel 2": id ungültig (a-z, 0-9, -, höchstens 32 Zeichen)` |
| Insel doppelt | `data/worlds/nordmeer.json: Insel island1 doppelt` |
| kein Endboss in der tiefsten Stufe | `data/worlds/nordmeer.json: Insel insel3: kein Endboss in der tiefsten Stufe 2` |
| `endBoss` unbekannt | `data/worlds/nordmeer.json: Insel insel3: endBoss "x" unbekannt (erlaubt: forestChieftain, caveColossus, …)` |
| `endBoss` ist ein Miniboss | `data/worlds/nordmeer.json: Insel insel3: endBoss "trollKing" ist ein Miniboss (erlaubt: …)` |
| Biom trägt den Bau nicht | `data/worlds/nordmeer.json: Insel insel3: bau in der tiefsten Stufe 2: biom mine: nur 0 Chunks mit 80 Units Abstand zum Bau, 1 Camp(s) brauchen Platz (Inselgröße 4 Chunks)` |
| Tiefe unbekannt | `data/worlds/nordmeer.json: Insel eisinsel: unbekannte Tiefe 5` (vor dem Endboss geprüft, siehe oben) |
| kein Miniboss | `data/worlds/nordmeer.json: Insel eisinsel: kein Miniboss in Tiefe 3` (heute hat jede Tiefe 0–4 einen) |
| Etappe falsch | `data/worlds/nordmeer.json: Insel eisinsel: etappe 3 lässt Etappe 2 aus (lückenlos ab 1)`, `… etappe 1 fällt unter die Etappe 2 der Insel davor (nicht fallend)`, `… etappe 0 ungültig (mindestens 1)` |
| Faktor ≤ 0 | `data/worlds/nordmeer.json: Insel eisinsel: depthScaling braucht Faktoren > 0` |
| Größe falsch | `data/worlds/nordmeer.json: Insel eisinsel: chunkCounts Tiefe 0: die Chunk-Anzahl 12 ist keine Inselgröße (gültig: 16, 24, …)` |
| Planung falsch | `data/worlds/nordmeer.json: Insel eisinsel: plannedChunks Tiefe 0: auf Platz -9: …` |
| Geschichte: unbekanntes Feld | `data/worlds/sengoku.json: story: unbekanntes Feld "intro"`, `… Insel island1: story: unbekanntes Feld "ende"` |
| Geschichte: Wortschatz zu groß | `data/worlds/sengoku.json: story.lexicon: 30 Wörter (höchstens 24)` |
| Geschichte: Anzahl Zeilen | `data/worlds/sengoku.json: story.start: 3 Zeilen (1 bis 2)` |
| Geschichte: leere Zeile | `data/worlds/sengoku.json: story.end Zeile 2: leer` |
| Geschichte: Zeile zu lang | `data/worlds/sengoku.json: Insel island1: story.arrival Zeile 1: 71 Zeichen (höchstens 64)` |
| Geschichte: Kunstwort unbekannt | `data/worlds/sengoku.json: story.start Zeile 1: Wort "kame" fehlt im lexicon` |
| Geschichte: echte Wörter | `data/worlds/sengoku.json: Insel island1: story.boss Zeile 1: 1 echtes Wort (2 bis 3)`, `… story.etappen.1 Zeile 1: 4 echte Wörter (2 bis 3)`, `… story.bottles Eintrag 2 Zeile 1: …` |
| Geschichte: Etappe ohne Insel | `data/worlds/sengoku.json: story.etappen: Etappe 3 hat keine Insel` |

Entschieden von Claude (Delegation 🧑, 2026-10-19): Muster und neue Fälle; die Gründe aus `checkIsland` bleiben wörtlich.
Meldungen der Geschichte: B-965/R4; sie nennen Welt (über die Datei), Insel, Feld und Zeile im selben Muster. Entschieden
von Claude (GS1.1, 2026-11-15, vorläufig): Wortlaut, Prüfreihenfolge und Zählung in Zeichen.

## 4. IDs

Entschieden von Claude (Delegation 🧑, 2026-10-19), Empfehlung 2 der Session WZ18.1; Zeichen nach B-488 › Ausnahme- und Fehlerfälle.

| Regel | Begründung | Daten |
|---|---|---|
| **Welt-ID** und **Insel-ID** folgen dem Muster `^[a-z0-9][a-z0-9-]{0,31}$` (Kleinbuchstaben, Ziffern, Bindestrich, 1–32 Zeichen, nicht mit Bindestrich beginnend), wie im Asset-Studio (`tools/k3c-studio/backend/app_world_files.go` › `worldID`). | Die Welt-ID ist Dateiname: kein Pfadzugriff über die ID, keine Groß-/Kleinschreibungs-Falle unter Windows. | – |
| Die Welt-ID ist eindeutig, weil sie der Dateiname ist; `standard` ist die Standard-Welt und wird nicht neu angelegt. | Eine Datei je Welt. | `data/worlds/` |
| Die Insel-ID ist **eindeutig je Welt**, über Welten hinweg frei: `island1` darf in jeder Welt vorkommen und meint jeweils die Insel dieser Welt. | Inseln erbt eine Welt nicht (`extends` gilt nur für Basis-Werte, § 1); eine Kopie der Standard-Welt behält ihre IDs. | – |
| IDs ändern sich nach dem ersten Spielstand nicht mehr; Umbenennen im Editor ändert nur `name`. Wer eine ID ändert, macht Spielstände dieser Insel unladbar (§ 5). | Die ID ist der Verweis im Spielstand. | – |

## 5. Spielstand-Verweis und Migration

Entschieden von Claude (Delegation 🧑, 2026-10-19), Empfehlung 3 der Session WZ18.1; Migration auf `standard` nach B-488 › Offene Fragen.

| Regel | Begründung | Daten |
|---|---|---|
| Der Spielstand speichert die Welt als **`world`** (Welt-ID) und die aktuelle Insel als **`islandId`** (Insel-ID) statt des Index `island`. Beide Felder sind ab der neuen Version Pflicht; `island` entfällt. Die Planung und die Erzeugungs-Optionen (`CreateOptions`) werden nicht gespeichert, sie kommen beim Laden aus der Welt. | Ein Index bricht, sobald Inseln umsortiert werden; die ID bleibt. | `engine/sim/island_save.go` |
| `IslandSaveVersion` steigt von **7 auf 8** (Fixture `v8`). | Jede Formatänderung erhöht die Version (`arbeitsweise.md` › Spielstand-Format). | – |
| **Migration** alter Stände (Version 1–7): `world` = `standard`, `islandId` = ID der Insel am alten Index `island` in der Standard-Welt (fehlt `island`: Index 0, also `island1`). Ein Index außerhalb der Liste ist ein Fehler wie heute. Die Datei wird beim Laden **nicht** umgeschrieben; Version 8 entsteht erst beim nächsten Speichern. | Alte Stände spielen ohne Verlust weiter (B-488 › Beispiele). | `saves/*.json` |
| Beim Laden eines Stands mit **unbekannter Welt** (keine Datei, ungültige Welt-Datei, oder Welt außer `standard` ohne Dev-Mode) oder **unbekannter Insel** (ID nicht in der Welt) wird nicht geladen; der Fehler nennt Welt und Insel, die Datei bleibt unverändert. Meldungen: `spielstand: Welt "nordmeer" unbekannt`, `spielstand: Welt "nordmeer" nur im Dev-Mode`, `spielstand: Insel "eisinsel" gibt es in Welt "nordmeer" nicht`. | Fail closed, kein stiller Wechsel auf eine andere Welt. | – |
| Passt der Stand nicht mehr zur geänderten Welt (z. B. Tiefen der Insel geändert), gilt die bestehende Prüfung des Stands (Hubs je Tiefe); auch dann bleibt die Datei unverändert. | Eine Welt-Datei kann sich zwischen Speichern und Laden ändern. | – |

Beispiel (Ausschnitt, Version 8): `{ "version": 8, "campaignId": "familie", "world": "nordmeer", "islandId": "eisinsel", … }`.
Ein Version-5-Stand mit `"island": 0` lädt als `"world": "standard", "islandId": "island1"`.

## 6. Laden im Spiel

Entschieden von Claude (Delegation 🧑, 2026-10-19), Empfehlung 4 der Session WZ18.1; Wahl nur im Dev-Mode nach B-488 › Offene Fragen.

| Regel | Begründung | Daten |
|---|---|---|
| **Eingebettet** ist nur die Standard-Welt: `data/embed.go` nennt `worlds/standard.json` ausdrücklich (kein Muster `worlds/*.json`). Die Standard-Welt kommt auch im Dev-Mode aus der Einbettung. | Lokale Entwürfe landen nie in einem Build; Golden und CI rechnen immer mit derselben Standard-Welt. | `data/embed.go` |
| Die Sim lädt eine Welt über einen **Parameter** (Welt-Daten), nicht über die globale Liste `islandDefs`; ohne Angabe gilt `standard`. | Mehrere Räume spielen gleichzeitig verschiedene Welten. | `engine/sim/` |
| **Weitere Welten** liest `engine/room` nur im **Dev-Mode** (`Manager.Dev`) **frisch von der Platte** aus `Manager.Worlds` (`<id>.json`; der Welt-Betrachter der Workbench setzt `data/worlds` des Checkouts, `tools/k3c-workbench/internal/studioview`), bei jedem Anlegen und jedem Laden eines Raums, mit der Prüfung aus § 3. Ohne Dev-Mode oder ohne Ordner gibt es nur `standard`. Das native Spiel setzt den Ordner heute nicht (nur `standard`); Spielserver, `K3C_DEV` und `K3C_WORLDS_DIR` sind mit dem Spielserver entfallen (B-850) | Eine im Studio gespeicherte Welt startet ohne Neubau; Spielende sehen keine Entwürfe. | `engine/room/worlds.go` |
| Der Raum wählt die Welt **beim Anlegen** (Standard `standard`, Wahl nur im Dev-Mode) und behält die geladene Welt, solange er läuft; Änderungen an der Datei gelten erst für neu angelegte oder neu geladene Räume. Unbekannte oder ungültige Welt beim Anlegen → Raum wird nicht angelegt, der Fehler nennt die Welt (`Welt "nordmeer" unbekannt` bzw. die Meldung aus § 3). | Ein laufender Raum ändert seine Regeln nicht unter den Spielern. | `engine/room/` |
| **Wertesatz je Raum:** Beim Anlegen und Laden löst der Raum die Welt einmal zu ihrem Wertesatz auf (`sim.ResolveWorld`, Kette über `extends` aus demselben Welten-Ordner); Insel und alle Stufen rechnen danach nur mit ihm, nie mit globalen Werten, auch nach dem Inselwechsel. Ohne Overrides in der Kette gilt der Basis-Wertesatz (byte-gleich, folgt dem Balancing-Tester `UseData`). Ungültige Overrides → Raum wird nicht angelegt bzw. Stand nicht geladen, Meldung aus § 1.2. Zwei Räume mit verschiedenen Welten beeinflussen sich nicht. Gegen die Basis geprüft bleiben: Spielstand-Format (Truppenarten, Berufe, Rüstungsstufen, Tier-Gating), Kandidaten-hub.json, `events.json`, Skill-Katalog. Quelle: B-681, entschieden von Claude (autonom, BAL8.2). | Mehrere Räume spielen gleichzeitig verschiedene Welten; Auflösung einmal je Raum kostet je Tick nichts. | `engine/sim/world_values_room.go`, `engine/room/worlds.go` |
| **Inselwechsel:** Nach dem Endboss der Insel geht es zur **nächsten Insel der Welt-Datei** (`stufen.md` § 1); das Ziel des Raums (Siegvariante) gilt wie heute nur auf der letzten Insel der Welt (`engine/sim/island_victory.go`). Der Seed der nächsten Insel bleibt aus Spiel-ID und Listenplatz gebildet (Golden unverändert). Abfahrt ist am Hafen der Insel mit fertigem Schiff (`stufen.md` § 1, B-637). Gilt mit 2 Spielern gleichzeitig wie mit einem. | Reihenfolge der Inseln = Reihenfolge der Liste. | `islands` |
| Das Protokoll trägt die Welt-ID dort, wo der Client die Insel kennt (Werte wie im Spielstand: `world`, `islandId`); Felder und Version legt WZ18.5 in `docs/protocol.md` fest. | Client und Server benennen Welt und Insel gleich. | `docs/protocol.md` |

### 6.1 Etappen und Zielwahl am Hafen (B-961)

Inseln einer **Etappe** sind in beliebiger Reihenfolge anfahrbar; die nächste Etappe öffnet erst, wenn alle Inseln der laufenden abgeschlossen sind. Start bleibt die erste Insel der Liste, eine Rückkehr auf abgeschlossene Inseln gibt es nicht.

- **Abgeschlossen** ist eine Insel mit besiegtem Endboss beim Ablegen. Der Spielstand trägt die abgeschlossenen Insel-IDs (`completed`) und das gewählte Ziel (`chosen`) mit (Version 9, beide optional; ältere Stände: leer).
- **Ziele** (`Island.Targets`): die nicht abgeschlossenen Inseln der Etappe der aktuellen Insel in Listenreihenfolge; sind alle abgeschlossen, die Inseln der nächsten Etappe. Keine Ziele = letzte Insel der Welt (Kampagnensieg wie bisher). Der Wechselpunkt öffnet nur, wenn es Ziele gibt.
- **Wahl:** Ein gekrönter Spieler wählt am **offenen** Wechselpunkt (vor dem Ablegen) ein Ziel (`Room.ChooseIsland`, `sim.ChooseIsland`); eine Insel außerhalb der Ziele oder ein geschlossener Wechselpunkt wird abgelehnt, das Ziel bleibt. Wählen mehrere Spieler im selben Takt, gewinnt der **höchste Spieler-Index** (unabhängig von der Aufrufreihenfolge); in späteren Takten gilt die letzte Wahl. Ohne Wahl gilt das erste Ziel in Listenreihenfolge.
- **Bau der Zielinsel:** `NextIsland` baut das Ziel; der Seed ergibt sich aus Spiel-ID und **Listenplatz** des Ziels (`<id>:<Listenplatz>`), `Number` ist der Listenplatz. Die abgeschlossenen Inseln ziehen mit.
- **Beispiel:** Etappen 1, 2, 2 (Inseln a, b, c): Nach a bietet der Hafen b und c; Wahl c, danach bietet er nur b; nach b gibt es keine Ziele, b ist die letzte (Sieg möglich).

### 6.2 Geschichte an Wendepunkten (B-965)

Quelle: B-965/R6 und R7 (🧑 2026-11-15); die Auslöser sind Defaults und **vorläufig** (§ 9). Ein **Wendepunkt** ist ein
Moment der Kampagne, an dem Zeilen der Geschichte (§ 2.2) gelten. Die Engine nennt sie über reine Funktionen in
`engine/sim`: ohne Zustand, ohne Zufall, ohne neue Ereignisse; das Spiel blendet sie nur ein (B-974).

| Wendepunkt | Auslöser | Zeilen, in dieser Reihenfolge |
|---|---|---|
| Neue Kampagne | Start einer neuen Kampagne | `start` der Welt, danach `arrival` der Startinsel |
| Ankunft | Inselwechsel (`islandSwitch`) | `arrival` der Zielinsel |
| Endboss ausgelöst | `bossSpawned` des Endbosses | `boss` der Insel |
| Endboss besiegt | `bossDefeated` des Endbosses | `done` der Insel; schließt der Sieg die Etappe ab, danach `etappen[<Nr>]`; ist es die letzte Insel der Welt, statt dessen `end` |

- **Etappe abgeschlossen** heißt: Alle übrigen Inseln der Etappe sind abgeschlossen (§ 6.1); die Insel mit dem eben
  besiegten Endboss zählt dabei schon mit, nicht erst beim Ablegen. Ohne `etappe` ist jede Insel eine eigene Etappe, die
  Etappenzeile folgt dann auf jede Abschlusszeile.
- **Letzte Insel der Welt** ist die Insel, nach der es keine Ziele mehr gibt (§ 6.1), nicht der letzte Listenplatz. Die
  Zeile ihrer Etappe in `etappen` gilt nie, an ihre Stelle tritt `end`.
- **Kein Auslösen beim Laden:** Das Laden eines Spielstands nennt keine Zeilen. Miniboss ausgelöst oder besiegt: keine Zeile.
- **Siegvariante ohne Endboss** (z. B. Tage überleben): `done` und `end` gelten beim Sieg der Variante auf der letzten
  Insel; auf früheren Inseln bleibt es beim Endboss (Bedingung des Inselwechsels, § 6).
- **Fehlender Eintrag:** Der Wendepunkt nennt dafür nichts, die übrigen Zeilen bleiben (Insel ohne `boss`: am Endboss
  keine Zeile, `done` gilt weiter).
- **Determinismus und 2 Spieler:** Dieselbe Welt und derselbe Wendepunkt ergeben immer dieselben Zeilen; sie gelten für
  alle Spieler der Insel gleich, in welcher Stufe sie auch stehen. Simulation, Golden und Spielstand ändern sich nicht.
- **Beispiel:** Etappen 1, 2, 2 (Inseln a, b, c): Sieg über den Endboss von a → `done` von a, danach `etappen["1"]`.
  Sieg auf c (b noch offen) → nur `done` von c. Sieg auf b danach, b ist die letzte Insel → `done` von b, danach `end`.

## 7. Git

Entschieden von Claude (Delegation 🧑, 2026-10-19), Empfehlung 5 der Session WZ18.1.

| Regel | Begründung | Daten |
|---|---|---|
| Eingecheckt ist nur `data/worlds/standard.json`. Eigene Welten bleiben lokal: `.gitignore` bekommt `data/worlds/*` und `!data/worlds/standard.json` (umgesetzt in WZ18.2). | Entwürfe gelangen nicht versehentlich in Commits, Builds oder die CI (B-488/AC-10: keine eingecheckte Welt). | `.gitignore` |
| Das Spielpaket (`task build`/`buildall` → `dist/k3c-game/`) enthält alle Spieldaten samt der lokalen Welten als `data/` neben der EXE; das Spiel liest diesen Ordner ohne `K3C_DATA_DIR` (`data/embed.go`). Quelle: 🧑 2026-11-15. | Das Paket ist vollständig lauffähig und lässt sich weitergeben. | `native/Taskfile.yml` › `game:dist` |
| Eine weitere Welt einchecken entscheidet 🧑; dann bekommt `.gitignore` eine eigene Ausnahme `!data/worlds/<id>.json`, und die Welt ist eine REG-Änderung mit Begründung im Commit. | Bewusste Entscheidung statt Nebeneffekt. | `.gitignore` |
| Eine geänderte Standard-Welt ist eine REG-Änderung durch 🧑 (`task go:golden` nur bei gewollter Änderung), wie die Planung (`geplante-chunks.md` § 6). Quelle: B-488 › Regeln. | Golden ändert sich nur gewollt. | `data/worlds/standard.json` |

## 8. Begriffe im Code

Entschieden von Claude (Delegation 🧑, 2026-10-19), Empfehlung 6 der Session WZ18.1. „Welt“ hat drei Bedeutungen
([`../glossar.md`](../glossar.md)):

- **Welt** (dieses Regelwerk): Sammlung von Inseln, eine Welt-Datei. Im Go-Code Typ **`WorldDef`** in `engine/sim`
  (`ID`, `Name`, `Notes`, `Islands []IslandDef`, `Extends`, `Overrides`); der aufgelöste Wertesatz heißt **`WorldValues`**, Feld `world` im Spielstand und im Protokoll.
- **`World`** im Go-Code ist **eine Stufe** (bleibt so, kein Umbenennen in diesem Sprint).
- **„in der Welt“** heißt die Spielfläche einer Stufe (Weltposition).

## 9. Offen

- Obergrenze n der Inseln je Welt und Inhalt weiterer Inseln (`stufen.md` § 7), Variante „Ebenen“.
- Eigene Grafik der Endbosse der Tiefen 0–3 (bis dahin das Asset der Basisart, B-993), mehrere Endbosse je Insel.
- Geschichte (B-965), Defaults **vorläufig**, von 🧑 bei der Freigabe bestätigt: „Abschluss einer Etappe“ ist die Etappe
  der Welt (§ 6.1); „vor dem Endboss“ ist der Moment, in dem er ausgelöst wird (`bossSpawned`); echte Wörter deutsch und
  unübersetzt; Zeilen über Funktionen statt neuer Ereignisse; Einblendung je Spielerhälfte (B-974).
- Geschichte, Obergrenzen **vorläufig**: Zeile höchstens 64 Zeichen, Wortschatz höchstens 24 Wörter je Welt
  (`data/story.json`).
- Welt „Polar“ gibt es nicht; ihr nordischer Klang gilt, sobald sie entsteht. Eine Geschichte bekommen heute `standard`,
  `sengoku`, `nil`, `britannien` (B-976). Mechanik der Flaschenpost zu `story.bottles`: B-971.
