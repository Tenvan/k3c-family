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
| Jede Welt ist **für sich vollständig**: Sie verweist auf keine andere Welt und erbt nichts; was zwei Welten gemeinsam haben, steht in beiden Dateien. Entschieden von Claude (Delegation 🧑, 2026-10-19). | Eine Welt-Datei lässt sich einzeln prüfen, kopieren und löschen. | – |

## 2. Format

Entschieden von Claude (Delegation 🧑, 2026-10-19), nach B-488 › Anforderungen und Empfehlung 1 der Session WZ18.1.

**Welt** (oberste Ebene, Reihenfolge der Felder beim Schreiben wie in der Tabelle):

| Feld | Pflicht | Typ | Regel |
|---|---|---|---|
| `id` | ja | Text | Welt-ID (§ 4), **gleich dem Dateinamen** ohne `.json`. |
| `name` | ja | Text | Anzeigename, nicht leer (z. B. „Standard“, „Nordmeer“); muss nicht eindeutig sein. |
| `notes` | nein | Text | Freie Notiz für Entwurf und REG-Begründung; das Spiel liest sie nicht. Fehlt sie, gilt `""`. |
| `islands` | ja | Liste | Inseln in Spielreihenfolge (Insel 1 zuerst), mindestens eine. |

**Insel** (Eintrag in `islands`, Felder wie heute in `data/islands.json`, Reihenfolge beim Schreiben wie in der Tabelle):

| Feld | Pflicht | Typ | Regel |
|---|---|---|---|
| `id` | ja | Text | Insel-ID (§ 4), eindeutig in der Welt. |
| `name` | ja | Text | Anzeigename, nicht leer (z. B. „Insel 1“). |
| `depths` | ja | Liste Zahlen | Tiefen der Stufen, jede ein bekanntes Biom (`data/biomes/*.json`), jede höchstens einmal; Lücken erlaubt (z. B. `[0, 4]`). |
| `depthScaling` | ja | Objekt | `hp`, `damage`, `speed`: Basen der Potenz je Tiefe (`stufen.md` § 1, Skalierung), alle > 0. |
| `plannedChunks` | nein | Objekt | Geplante Chunks je Tiefe (`geplante-chunks.md` § 1–5); fehlt oder `{}` = reiner Zufall. |
| `chunkCounts` | nein | Objekt | Chunk-Anzahl je Tiefe (`geplante-chunks.md` § 7); ohne Eintrag gilt die Länge des Bioms. |

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
| Je Insel: jede Tiefe hat einen **Miniboss**, die **tiefste** Tiefe hat einen **Endboss**. Heute gibt es den Endboss nur in Tiefe 4: jede Insel braucht Tiefe 4. | Jede Stufe hat ihr Ziel, jede Insel ihr Ende (`stufen.md` § 3). | `data/bosses.json` |
| Je Insel: `depthScaling` mit `hp`, `damage`, `speed` jeweils > 0. | Keine Gegner mit 0 HP oder Stillstand. | – |
| `chunkCounts`: Schlüssel ist eine Tiefe der Insel (Dezimalzahl ohne führende Null), Wert eine Inselgröße aus der Tabelle. | `geplante-chunks.md` § 7. | `data/island-sizes.json` |
| `plannedChunks`: Schlüssel ist eine Tiefe der Insel; die Planung besteht die Prüfung gegen die Mauerlinien und die Chunk-Anzahl der Tiefe, unabhängig vom Seed (`level.ValidatePlanFor`). | `geplante-chunks.md` § 5. | `data/biomes/*.json` |

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
| kein Endboss | `data/worlds/nordmeer.json: Insel insel3: kein Endboss in der tiefsten Stufe 2` |
| Tiefe unbekannt | `data/worlds/nordmeer.json: Insel eisinsel: unbekannte Tiefe 5` (vor dem Endboss geprüft, siehe oben) |
| kein Miniboss | `data/worlds/nordmeer.json: Insel eisinsel: kein Miniboss in Tiefe 3` (heute hat jede Tiefe 0–4 einen) |
| Faktor ≤ 0 | `data/worlds/nordmeer.json: Insel eisinsel: depthScaling braucht Faktoren > 0` |
| Größe falsch | `data/worlds/nordmeer.json: Insel eisinsel: chunkCounts Tiefe 0: die Chunk-Anzahl 12 ist keine Inselgröße (gültig: 16, 24, …)` |
| Planung falsch | `data/worlds/nordmeer.json: Insel eisinsel: plannedChunks Tiefe 0: auf Platz -9: …` |

Entschieden von Claude (Delegation 🧑, 2026-10-19): Muster und neue Fälle; die Gründe aus `checkIsland` bleiben wörtlich.

## 4. IDs

Entschieden von Claude (Delegation 🧑, 2026-10-19), Empfehlung 2 der Session WZ18.1; Zeichen nach B-488 › Ausnahme- und Fehlerfälle.

| Regel | Begründung | Daten |
|---|---|---|
| **Welt-ID** und **Insel-ID** folgen dem Muster `^[a-z0-9][a-z0-9-]{0,31}$` (Kleinbuchstaben, Ziffern, Bindestrich, 1–32 Zeichen, nicht mit Bindestrich beginnend), wie im Asset-Studio (`tools/k3c-studio/backend/app_world_files.go` › `worldID`). | Die Welt-ID ist Dateiname: kein Pfadzugriff über die ID, keine Groß-/Kleinschreibungs-Falle unter Windows. | – |
| Die Welt-ID ist eindeutig, weil sie der Dateiname ist; `standard` ist die Standard-Welt und wird nicht neu angelegt. | Eine Datei je Welt. | `data/worlds/` |
| Die Insel-ID ist **eindeutig je Welt**, über Welten hinweg frei: `island1` darf in jeder Welt vorkommen und meint jeweils die Insel dieser Welt. | Jede Welt ist für sich vollständig (§ 1); eine Kopie der Standard-Welt behält ihre IDs. | – |
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
| **Weitere Welten** liest der Server nur im **Dev-Mode** (`K3C_DEV`) **frisch von der Platte** (`<K3C_WORLDS_DIR>/<id>.json`, Standard `data/worlds` relativ zum Arbeitsordner, nach dem Muster von `K3C_SAVES_DIR`; `task start`/`task dev` laufen im Repo-Stamm. Das Asset-Studio ist ein eigenes Fenster ohne Spielserver (Entscheidung 004) und schreibt in denselben Ordner `data/worlds/` des Checkouts; `tools/k3c-studio/backend` › `worldFile` zeigt für `standard` dann auf `data/worlds/standard.json`; der Server beobachtet `data/` und startet nach einer Änderung neu), bei jedem Anlegen und jedem Laden eines Raums, mit der Prüfung aus § 3. Ohne Dev-Mode gibt es nur `standard`. Achtung: Der Server startet heute standardmäßig **mit** Dev-Mode (`K3C_DEV` leer oder `1`), aus ist er nur mit `K3C_DEV=0` (`cmd/k3c-server/main.go`). | Eine im Studio gespeicherte Welt startet ohne Neubau; Spielende sehen keine Entwürfe. | `cmd/k3c-server`, `engine/room/` |
| Der Raum wählt die Welt **beim Anlegen** (Standard `standard`, Wahl nur im Dev-Mode) und behält die geladene Welt, solange er läuft; Änderungen an der Datei gelten erst für neu angelegte oder neu geladene Räume. Unbekannte oder ungültige Welt beim Anlegen → Raum wird nicht angelegt, der Fehler nennt die Welt (`Welt "nordmeer" unbekannt` bzw. die Meldung aus § 3). | Ein laufender Raum ändert seine Regeln nicht unter den Spielern. | `engine/room/` |
| **Inselwechsel:** Nach dem Endboss der Insel geht es zur **nächsten Insel der Welt-Datei** (`stufen.md` § 1); das Ziel des Raums (Siegvariante) gilt wie heute nur auf der letzten Insel der Welt (`engine/sim/island_victory.go`). Der Seed der nächsten Insel bleibt aus Spiel-ID und Listenplatz gebildet (Golden unverändert). Gilt mit 2 Spielern gleichzeitig wie mit einem. | Reihenfolge der Inseln = Reihenfolge der Liste. | `islands` |
| Das Protokoll trägt die Welt-ID dort, wo der Client die Insel kennt (Werte wie im Spielstand: `world`, `islandId`); Felder und Version legt WZ18.5 in `docs/protocol.md` fest. | Client und Server benennen Welt und Insel gleich. | `docs/protocol.md` |

## 7. Git

Entschieden von Claude (Delegation 🧑, 2026-10-19), Empfehlung 5 der Session WZ18.1.

| Regel | Begründung | Daten |
|---|---|---|
| Eingecheckt ist nur `data/worlds/standard.json`. Eigene Welten bleiben lokal: `.gitignore` bekommt `data/worlds/*` und `!data/worlds/standard.json` (umgesetzt in WZ18.2). | Entwürfe gelangen nicht versehentlich in Commits, Builds oder die CI (B-488/AC-10: keine eingecheckte Welt). | `.gitignore` |
| Eine weitere Welt einchecken entscheidet 🧑; dann bekommt `.gitignore` eine eigene Ausnahme `!data/worlds/<id>.json`, und die Welt ist eine REG-Änderung mit Begründung im Commit. | Bewusste Entscheidung statt Nebeneffekt. | `.gitignore` |
| Eine geänderte Standard-Welt ist eine REG-Änderung durch 🧑 (`task go:golden` nur bei gewollter Änderung), wie die Planung (`geplante-chunks.md` § 6). Quelle: B-488 › Regeln. | Golden ändert sich nur gewollt. | `data/worlds/standard.json` |

## 8. Begriffe im Code

Entschieden von Claude (Delegation 🧑, 2026-10-19), Empfehlung 6 der Session WZ18.1. „Welt“ hat drei Bedeutungen
([`../glossar.md`](../glossar.md)):

- **Welt** (dieses Regelwerk): Sammlung von Inseln, eine Welt-Datei. Im Go-Code Typ **`WorldDef`** in `engine/sim`
  (`ID`, `Name`, `Notes`, `Islands []IslandDef`), Feld `world` im Spielstand und im Protokoll.
- **`World`** im Go-Code ist **eine Stufe** (bleibt so, kein Umbenennen in diesem Sprint).
- **„in der Welt“** heißt die Spielfläche einer Stufe (Weltposition).

## 9. Offen

- Obergrenze n der Inseln je Welt und Inhalt weiterer Inseln (`stufen.md` § 7), Variante „Ebenen“.
- Endbosse in anderen Tiefen als 4 (heute muss jede Insel Tiefe 4 haben, § 3); neue Bosse sind Nicht-Ziel von B-488.
