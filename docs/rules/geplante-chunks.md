# Regelwerk: Geplante Chunks

Beschlossen von 🧑 im Workshop WZ8.1 am 2026-10-17 (Chat): Die Annahmen aus B-461 gelten unverändert. Grundlage: B-461, Sprint WZ8; Stufen und Level: [`stufen.md`](stufen.md); Welt-Datei, in der die Planung steht: [`welten.md`](welten.md) (WZ18.1).
Je Regel: **Regel · Begründung · Daten**. Teil 1 kennt feste Objekte und den Seed je Chunk; Teil 2 (§ 8, B-468, Workshop WZ14.1) die Ruinen; Gegnerlager (§ 9, B-517, WZ27) das Feld `enemies`.

## 1. Ort und Platz

| Regel | Begründung | Daten |
|---|---|---|
| Geplante Chunks stehen je Insel und Tiefe in der Welt-Datei `data/worlds/<id>.json` (Standard-Welt `data/worlds/standard.json`, [`welten.md`](welten.md)) unter `plannedChunks` (Schlüssel = Tiefe, Wert = Liste). Level ohne Planung (andere Inseln oder Tiefen, Biom-Level ohne Insel) bleiben rein zufällig. Fehlt das Feld, ist es `{}` oder eine leere Liste, gibt es keine Planung. | Feste Gegenden gehören zu einer Insel, nicht zum Biom. | `data/worlds/<id>.json` › `islands[]` › `plannedChunks` |
| Der **Chunk-Platz** `slot` zählt relativ zu den Hub-Chunks: `-n` ist der n-te Chunk links, `+n` der n-te rechts davon. Platz `0` gibt es nicht. Anfang von +n = Hub + 50 + (n−1)·Chunk-Breite, Anfang von −n = Hub − 50 − n·Chunk-Breite. Die Lage `at` eines Objekts zählt in Units ab Chunk-Anfang. | Der Chunk liegt bei jeder gewürfelten Länge gleich weit vom Hub. | – |
| Planbar sind bei kürzester Länge: Wald ±1…±7, Höhle ±1…±5, Mine ±1…±4, Eisenstollen ±1…±7, Kristallhöhle ±1…±5 (die letzten beiden gerechnet, WZ8.2 bestätigt sie gegen den Generator). Hub, `exit` und `edge` sind nicht planbar, die Art `hub` ist verboten. | Hub, Tiefen-Eingang und Rand gehören dem Generator. | `data/biomes/*.json` |

Beispiel (Reihenfolge der Schlüssel: `slot`, `kind`, `seed`, `objects`; Teil-2-Felder legt B-468 fest):

```json
{ "id": "island1", "depths": [0, 1, 2, 3, 4],
  "plannedChunks": {
    "0": [ { "slot": -3, "kind": "clearing", "seed": "lichtung" } ],
    "1": [ { "slot": 3, "kind": "cavern", "objects": [ { "kind": "stoneVein", "at": 40 }, { "kind": "chest", "at": 45 } ] } ]
  } }
```

## 2. Inhalt

| Regel | Begründung | Daten |
|---|---|---|
| Pflicht ist die **Chunk-Art** (`kind`, aus `chunkWeights` des Bioms oder `portal`, `chest`, `recruitCamp`, `harbor` (§ 10)). Optional sind ein **Chunk-Seed** (`seed`) und **feste Objekte** (`objects`: Art und `at`). Objekt-Arten: Ressourcen laut `resourcesPerChunk` des Bioms, Ader (`veins.kind`), Truhe, Camp, Portal. | Teil 1 deckt feste Objekte und den Seed je Chunk ab. | `plannedChunks`, `data/biomes/*.json` |
| Eine **Objektliste ersetzt** die gewürfelten Ressourcen der Chunk-Art: Mit Liste ist der Chunk vollständig fest. Ohne Liste würfelt der Inhalt der Chunk-Art aus dem eigenen Strom. **Feste Skill-Punkte gibt es nicht**; Skill-Punkte würfeln weiter frei. | Fest heißt fest; kein Mischzustand, der vom Strom abhängt. | – |
| Der Inhalt eines geplanten Chunks würfelt **nur aus seinem eigenen Strom** (`engine/rng`), nie aus dem Haupt-Strom: ohne Chunk-Seed `<biom>:<seed>:chunk:<platz>` (Art fest, Inhalt wechselt mit dem Level-Seed), mit Chunk-Seed `<biom>:chunk:<chunk-seed>` (Inhalt in jedem Spielstand gleich). Ein leerer Chunk-Seed gilt als kein Chunk-Seed. Die Namen sind Vertrag wie die Reihenfolge des Haupt-Stroms. | Determinismus; die nicht geplanten Chunks bleiben von der Planung unberührt. | `engine/rng` |
| Reservierung: Der Generator reserviert die geplanten Plätze direkt nach `frame()`. Portale, Ereignisse, Chunk-Arten, Skill-Punkte und Adern würfeln nur über freie, nicht geplante Chunks. Ohne Planung ändern sich Reihenfolge und Zahl der Würfe nicht. | Konflikte werden beim Planen sichtbar, nicht erst zur Laufzeit. | – |

## 3. Biom-Summen

| Regel | Begründung | Daten |
|---|---|---|
| Was das Biom zählt, gilt **inklusive** der Planung: Portale genau `portals.count`, Truhen und Camps `eventChunks` min–max, Adern genau `veins.count`, Skill-Punkte min–max. Feste Portale, Truhen, Camps und Adern **ersetzen** gewürfelte, sie ergänzen sie nicht. Feste Truhen und Camps rechnen auf die Zahl der Ereignis-Chunks an, auch wenn sie als Objekt in einem Chunk anderer Art stehen (Entscheidung 2026-10-22, B-475: ein Objekt zählt als Entity, nicht nur ein Chunk der Art `chest`/`recruitCamp`/`portal`); ein Ereignis-Chunk ohne Objektliste trägt sein Objekt in der Chunk-Mitte. | Die Biom-Werte bleiben Vertrag. | `data/biomes/*.json` |
| Übrige Portale verteilt der Generator **abwechselnd**, beginnend auf der Seite mit weniger festen Portalen. | Gleichmäßig, deterministisch. | – |

## 4. Lage-Regeln

| Regel | Begründung | Daten |
|---|---|---|
| Feste Objekte liegen wie gewürfelte gleicher Art. Ressourcen, Truhen und Adern halten ≥ 2 Units Abstand zur Chunk-Grenze. Camps dürfen innerhalb der Mauerlinien liegen (Glossar › Camp). Portale halten `minDistanceFromHubUnits` zum Hub und liegen außerhalb des äußersten Tors; Portal-Plätze: Wald ±3…±7, Höhle ±3…±5, Mine ±3 und ±4; in 25-Unit-Biomen braucht ein festes Portal auf Platz ±4 `at` ≥ 10. | Gleiches Verhalten in der Sim, keine neue Logik. | `data/biomes/*.json`, `engine/level/` |
| **Nur Adern** halten zusätzlich ≥ 4 Units zu jedem Bauplatz der Mauerlinien (inklusive Streuung +0 bis +4) und zu den Baumschul-Plätzen (`materialien-gebaeude.md` § 3.3). Für Truhen und Camps gilt dieser Abstand nicht. | Adern sind Zahlziele (Markieren, Q57). | `data/hub.json` |
| `kind@x` ist je Stufe eindeutig: zwei Objekte gleicher Art an gleicher Lage sind verboten. | Der Spielstand ordnet Level-Objekte über `kind@x` zu. | – |

## 5. Prüfung und Fehlerfälle

| Regel | Begründung | Daten |
|---|---|---|
| Eine Planung wird **beim Laden** geprüft, unabhängig vom Seed, gegen die kürzeste mögliche Länge des Bioms und jede Streuung der Linien. Ist sie dort gültig, scheitert kein Seed zur Laufzeit. Die Mauerlinien sind ein Parameter (Standard: eingebettete Daten). Fehler nennen Insel, Tiefe, Platz und Grund; das Laden der Welt scheitert wie bei jeder ungültigen Welt-Datei ([`welten.md`](welten.md) § 3). | Fail closed. | `engine/level/`, `engine/sim/island_data.go` |
| Abgelehnt werden: Platz `0` oder Hub; `exit`/`edge`; Platz jenseits der kürzesten Länge; doppelter Platz oder doppeltes `kind@x`; Tiefe nicht in `depths`; unbekannte Chunk-Art oder Objekt-Art im Biom; Verstöße gegen die Lage-Regeln; mehr feste Portale, Truhen, Camps oder Adern als das Biom zählt; zu wenig freie Plätze für übrige Portale, Ereignisse oder Adern; „zu wenig“ rechnet mit dem **Maximum** der Ereignis-Chunks (ungünstigster Wurf, Entscheidung 2026-10-22, B-475), damit kein Seed zur Laufzeit scheitert. Der Wald hat keine Adern. | Jeder Fall ein Test (WZ8.2). | – |
| Das Feld `buildings` trägt Ruinen (§ 8), das Feld `enemies` die Wachen eines Gegnerlagers (§ 9). | Platz im Format für beide Sondergegenden. | – |

## 6. Schreiben

| Regel | Begründung | Daten |
|---|---|---|
| Der Chunk-Editor im Welten Editor schreibt die Welt-Datei der gewählten Welt (`data/worlds/<id>.json`) atomar und format-stabil und nur nach einer Stand-Prüfung (geändert seit dem Laden → nicht speichern, Entwurf bleibt). Nach jedem Speichern erscheint „REG-Änderung: Commit mit Begründung, `task go:golden` nur bei gewollter Änderung“. Eine gespeicherte Planung ist eine REG-Änderung durch 🧑. | Das Werkzeug schreibt REG-Daten; Golden ändert sich nur gewollt. | `data/worlds/<id>.json` |

## 7. Chunk-Anzahl und Inselgrößen

Beschlossen von 🧑 im Workshop WZ17.1 am 2026-10-18 (Chat, B-485 Revision 2).

| Regel | Begründung | Daten |
|---|---|---|
| Die **Chunk-Anzahl** einer Stufe steht je Insel im Feld `chunkCounts` (Schlüssel = Tiefe, Wert = Anzahl), neben `plannedChunks`. Sie legt die Länge der Stufe in Chunks fest und ersetzt die gewürfelte Länge aus `lengthUnits`. Fehlt das Feld oder der Eintrag der Tiefe, gilt die Länge des Bioms wie bisher (Level, Golden und Spielstände bleiben unverändert). Die Länge in Units ist Anzahl × Chunk-Breite des Bioms (50 oder 25 Units). | Inseln lassen sich gezielt größer oder kleiner planen, ohne bestehende Level zu ändern. | `data/worlds/<id>.json` › `islands[]` › `chunkCounts` |
| Erlaubt sind nur die **Inselgrößen** der Tabelle: sehr klein **16** · klein **24** · mittel **40** · groß **64** · sehr groß **100** · mega groß **160** (Revision 5, 2026-10-18: die erste Tabelle 16/20/24/32/40/48 war zu eng). Die Tabelle gilt für alle Biome; alle Anzahlen sind gerade, der Hub liegt mittig. Jede andere Anzahl wird beim Laden abgelehnt (Fehler nennt Insel, Tiefe und die gültigen Werte). | 16 ist die kleinste Anzahl, die in allen Biomen Platz für Hub, Portale mit Mindestabstand und Ausgang lässt (Eisenfeste und Kristallhöhle mit 25-Unit-Chunks und je 3 Portalen); 8 und 12 gehen nicht. | `data/island-sizes.json` |
| Die Planung wird gegen die eingestellte Anzahl geprüft: Ein geplanter Platz jenseits der Anzahl wird abgelehnt (Fehler nennt Insel, Tiefe, Platz). Die Prüfung rechnet mit der eingestellten Anzahl statt mit der kürzesten Länge des Bioms. Biom-Summen (Portale, Ereignisse, Adern, Skill-Punkte) gelten unverändert. Kann der Generator eine Größe in einem Biom nicht erzeugen, meldet er den Fehler (Editor: Fehler am Entwurf, Speichern gesperrt); einen Nachweis über viele Seeds und alle Biome gibt es nicht, wie viele Chunks sinnvoll sind, ermittelt später ein eigenes Projekt (Beschluss 🧑, 2026-10-18). | Fail closed; der Editor legt an, was anlegbar ist. | `engine/level/` |
| **Spielstand:** Der Stand schreibt je Stufe die Planung (`planned`) und die Chunk-Anzahl (`chunkCount`) mit, mit denen die Stufe gebaut wurde; das Laden baut sie damit, nicht mit den aktuellen Daten (B-489, WZ24.2). Spätere Änderungen der Welt-Datei betreffen nur neu erzeugte Stufen. Ohne Eintrag gilt reiner Zufall in der Länge des Bioms. | Ein alter Stand verschiebt sich nicht, wenn die Planung oder Größe einer Insel später geändert wird. | `engine/sim/island_save.go` |
| Der Editor bietet nur die Größen der Tabelle an (kein Freitext). Chunk- und Live-Vorschau zeigen die eingestellte Länge für ungespeicherte Entwürfe aus derselben Quelle. Eine gespeicherte Chunk-Anzahl ist wie eine gespeicherte Planung eine REG-Änderung durch 🧑 (Hinweis nach dem Speichern, Golden nur gewollt). | Gleiche Schreibregel wie § 6. | `data/worlds/<id>.json` |
| Die **Umwandlung** eines Seed-Chunks in feste Objekte übernimmt dessen Inhalt aus dem Vorschau-Level des Servers (Art und Lage der Objekte); Chunk- und Live-Vorschau zeigen danach dasselbe wie vorher. | Kein neuer Generator-Code; fest heißt fest (§ 2). | – |

## 8. Ruinen (Teil 2)

Beschlossen von 🧑 im Workshop WZ14.1 am 2026-10-21 (Chat, Vorschläge von Claude angenommen; Nachtrag nach dem Spec-Review am selben Tag: Vorposten, Wiederaufbau, Lage, Kennung). Grundlage: B-468, Sprint WZ14.

| Regel | Begründung | Daten |
|---|---|---|
| Eine **Ruine** ist ein gebautes, stark beschädigtes **Nutzgebäude** an fester Lage eines geplanten Chunks. Erlaubte Arten: `workshop`, `storage`, `nursery` (bis BS4 `nursery`), `barracks`, `tavern`, `healer`, `smithy`, `armory`. Burg, Mauer, Turm, Tor und Treppen sind keine Ruinen. | Sondergegenden mit Geschichte; Burg, Linien und Stufenwechsel gehören dem Hub. | `data/ruins.json` (WZ14.2) |
| Ein geplanter Chunk trägt **höchstens eine** Ruine: `buildings: [ { "kind": "smithy", "at": 6 } ]` (Reihenfolge der Schlüssel `slot`, `kind`, `seed`, `objects`, `buildings`). `at` zählt wie bei Objekten in Units ab Chunk-Anfang. | Klein und lesbar; ein Gebäude je Chunk reicht für eine Sondergegend. | `plannedChunks` |
| Die Ruine startet **gebaut mit 25 %** der HP ihrer Art (aufgerundet) und wirkt ab Spielbeginn wie ein gebautes Gebäude dieser Art (z. B. Kaserne → Truppenlimit, Lager → Kapazität des Vorrats, nicht der Bestand). | Vorsprung für die Spielenden, den sie verteidigen und reparieren. | `data/ruins.json` (Anteil), `data/buildings.json` (HP) |
| Die Ruine ist **Hub-neutral**: Sie zählt nicht für die Hub-Stufe und nicht für die Freischaltung der Mauerlinien. | Die Hub-Stufe bleibt ein bezahlter Ausbau der Burg (B-462). | – |
| Die Kennung **Ruine** ist eine dauerhafte Herkunft des Platzes: Sie bleibt nach Reparatur, Zerstörung und Wiederaufbau. | Burgfall und Darstellung brauchen sie unabhängig vom Zustand. | – |
| **Reparatur** wie bei jedem beschädigten Gebäude: Bauern reparieren kostenlos zwischen den Wellen ([`materialien-gebaeude.md`](materialien-gebaeude.md)). Gegner greifen Ruinen an wie jedes Gebäude. | Keine neue Mechanik, kein neuer Zustand. | – |
| **Vorposten:** Jenseits des äußersten Tors (Lage laut Layout der Stufe, nicht die äußerste gebaute Mauer) reparieren und bauen Bauern nur **ohne Gefahr**, und Bürger holen dort keine Waffen (Bogen, Schwert) bei Gefahr (gleiche Bedingung wie für das Verlassen der Arbeit bei Gefahr); bei Gefahr kehren sie hinter die Mauern zurück. Die Regel gilt für jeden Platz außerhalb der Mauern, nicht nur für Ruinen. | Ruinen außerhalb der Mauern sind gewollte Vorposten; Bauern sollen dafür nicht nachts hinauslaufen. | – |
| **Wiederaufbau:** Zerstören Gegner die Ruine, ist der Platz leer und bezahlbar, aber erst ab der **Hub-Stufe ihrer Art** (wie der Hub-Platz derselben Art). | Eine Ruine verschafft einen Vorsprung, hebelt aber den Ausbaupfad nicht aus. | `data/hub.json` |
| **Burgfall** (`castleLosses`): Ein Ruinen-Platz kehrt in den geplanten Ausgangszustand zurück (wieder Ruine mit 25 % HP), auch wenn er vorher zerstört war, statt leer zu werden. | Die Planung bleibt die Wahrheit; die Sondergegend geht nie verloren. | – |
| **Lage:** ≥ 2 Units zur Chunk-Grenze; ≥ 4 Units zu den Baumschul-Plätzen, zu jeder Ader, jedem Portal und jedem festen Objekt des Chunks; die Lage eines Hub- oder Insel-Platzes **gleicher Art** (`data/hub.json` › `sites`, `islandSites`) ist verboten, weil die Sim eine Ruine an Art und Lage erkennt (B-523); Zufalls-Ressourcen des Chunks halten ebenfalls ≥ 4 Units zur Ruine (B-522; ohne Ruine bleibt der Strom unverändert); eine Baumschul-Ruine braucht zusätzlich Platz für ihre Plantage (±3 Units) innerhalb des Chunks; `kind@x` je Stufe eindeutig. Zu den Bauplätzen der Mauerlinien gilt **kein** Abstand: Ruinen liegen überall, innerhalb wie außerhalb der Mauern, die Lage hängt von der Verwendung ab (Nachtrag 🧑 2026-10-21). | Gebäude und Zahlziele überdecken sich nicht, außer mit Linien-Bauplätzen (Vorrang beim Bezahlen regelt die Sim, WZ14.3); der Spielstand ordnet Plätze über `kind@x` zu. Mit Abstand zu den Linien (±12 Units je Linie) blieb innerhalb der Mauern kein Platz. | `data/hub.json` |
| **Spielstand und Protokoll:** Die Ruine ist ein gewöhnlicher Platz (`SiteSave`, `kind@x`), keine neue Spielstand-Version. Das Protokoll bleibt Version 7; `sites[]` trägt additiv die Kennung `ruin`. Ein Spielstand mit einer Ruine, die die Planung nicht mehr kennt, verliert den Platz (keine Migration). | Kleinster Eingriff in die Verträge. | `docs/protocol.md` |
| **Prüfung beim Laden** wie § 5: abgelehnt werden unbekannte oder nicht erlaubte Art, mehr als eine Ruine, Verstöße gegen die Lage-Regeln und doppeltes `kind@x`; Fehler nennen Insel, Tiefe, Platz und Grund. | Fail closed. | `engine/level/` |

## 9. Gegnerlager (Teil 3)

Umgesetzt in WZ27.1 (B-517) nach den Beschlüssen aus dem Regel-Workshop WZ14.1 (2026-10-21). Die offenen Zahlen unten sind **Annahmen von Claude** (ungeprüft durch 🧑), Werte in `data/guards.json`.

| Regel | Begründung | Daten |
|---|---|---|
| Ein Chunk trägt ein **Gegnerlager** im Feld `enemies`: Liste von Wachen `{ "kind": "goblin", "at": 20 }` (Reihenfolge der Schlüssel `slot`, `kind`, `seed`, `objects`, `buildings`, `enemies`); `at` zählt in Units ab Chunk-Anfang. Höchstens **4 Wachen** je Chunk. | Klein und lesbar; Zahl der Wächter war offen (Annahme). | `plannedChunks`, `data/guards.json` › `maxPerChunk` |
| Die Art der Wache kommt aus dem **Portal-Pool** des Bioms (`enemies.portal`), nie aus dem Nacht-Pool oder von Bossen. HP und Schaden skalieren je Tiefe wie bei Wellen (`spawnScaled`, Tabelle der Insel). | Gegnerarten aus dem Biom der Stufe (Beschluss WZ14.1). | `data/biomes/*.json` |
| **Lage:** ≥ 2 Units zur Chunk-Grenze, außerhalb des äußersten Tors (wie Portale), ≥ 1,5 Units zwischen zwei Wachen. Ruinen und Objekte desselben Chunks dürfen daneben liegen (bewachte Ruine). Abgelehnt wird mit Platz und Grund wie in § 5. | Eine Wache innerhalb der Mauern würde die Burg angreifen. | `data/guards.json` › `minGapUnits` |
| **Verhalten:** Die Wache steht am Heimatpunkt `HomeX`, greift **Spieler und Truppen** (ohne Landstreicher, ohne Schützen auf Türmen) im **Aggro-Radius** (10 Units um den Heimatpunkt) an, läuft dafür auf das nächste Ziel zu, kehrt danach zurück, flieht nie und klaut kein Gold. Gebäude, Burg und Mauern greift sie nicht an. | Wache, kein Marschierer; Annahme für den Radius. | `data/guards.json` › `aggroRadiusUnits` |
| **Keine Wirkung auf Wellen:** Wachen zählen nicht für die Gefahr (Bauern bleiben bei Tag draußen), nicht für den Aggressionspool, fliehen nicht bei Tagesanbruch und bleiben nachts Wachen. | Wellen und Balancing bleiben unverändert. | – |
| **Besiegt:** normale Beute der Gegnerart (Gold, Stufen-Ressource), **kein Respawn**. Ein geräumtes Lager bleibt geräumt, auch nach Burgfall und Laden; die Wachen verschwinden beim Burgfall nicht. | Beschluss WZ14.1; Burgfall: geräumt bleibt (Annahme). | – |
| **Spielstand:** `HubSave.guardsCleared` (Liste `kind@x` der besiegten Wachen), `IslandSaveVersion` **9**. Die Wachen selbst (Position, HP) sind flüchtig: nach dem Laden stehen die nicht geräumten wieder am Heimatpunkt. **Protokoll:** Version 7 bleibt, `enemies[]` trägt additiv `guard: true`. | Kleinster Eingriff; Fortschritt bleibt erhalten. | `engine/sim/enemies_guard.go` |
| Das Lager ist **kein** Level-Objekt (`Layout.Entities`) und würfelt nichts: Level mit und ohne Lager sind bis auf `Layout.Guards` gleich, Golden ohne Planung bleiben byte-gleich. | Ströme bleiben unberührt. | `engine/level/guards.go` |

## 10. Hafen-Chunk (BAS, B-637)

Beschlossen von 🧑 am 2026-10-26 (Chat, Projekt BAS, Defaults aus B-637). Der Hafen ist der Ort, an dem die Spieler die
Insel verlassen (`stufen.md` § 1).

| Regel | Begründung | Daten |
|---|---|---|
| Jede Insel hat in **Tiefe 0** (Stufe 1, Wald) genau **einen** geplanten Chunk der Art `harbor`, in der Standard-Welt auf Platz **−7** (äußerster planbarer Wald-Platz links). Andere Tiefen haben keinen Hafen. | Das Langzeitziel ist ab dem ersten Tag sichtbar; links außen stört der Hafen keine Mauerlinie. | `data/worlds/<id>.json` › `islands[]` › `plannedChunks` › `"0"` |
| Der Hafen-Chunk trägt einen **Steg** an der Wasserkante mit dem **Schiffs-Bauplatz**; er hat keine Ressourcen, Adern, Portale, Truhen oder Camps. Eine Objektliste ist nicht erlaubt (der Inhalt ist fest). | Der Hafen ist ein Bauort, kein Sammelort. | – |
| Der Hafen-Chunk ist ein geplanter Chunk wie andere (§ 1–5): Er reserviert seinen Platz vor den Würfen; ohne Hafen in der Planung ändert sich am Level nichts. | Determinismus; Golden ohne Hafen bleiben byte-gleich. | `engine/level` |
| Fehlt der Hafen auf einer Insel, die nicht die letzte der Welt ist, lehnt die Prüfung (`-check-data`) die Welt ab. | Ohne Hafen gäbe es keinen Inselwechsel. | – |
