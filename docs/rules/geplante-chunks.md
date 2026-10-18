# Regelwerk: Geplante Chunks

Beschlossen von 🧑 im Workshop WZ8.1 am 2026-10-17 (Chat): Die Annahmen aus B-461 gelten unverändert. Grundlage: B-461, Sprint WZ8; Stufen und Level: [`stufen.md`](stufen.md).
Je Regel: **Regel · Begründung · Daten**. Teil 1 (diese Datei) kennt feste Objekte und den Seed je Chunk; Ruinen und Gegnerlager folgen in B-468.

## 1. Ort und Platz

| Regel | Begründung | Daten |
|---|---|---|
| Geplante Chunks stehen je Insel und Tiefe in `data/islands.json` unter `plannedChunks` (Schlüssel = Tiefe, Wert = Liste). Level ohne Planung (andere Inseln oder Tiefen, Biom-Level ohne Insel) bleiben rein zufällig. Fehlt das Feld, ist es `{}` oder eine leere Liste, gibt es keine Planung. | Feste Gegenden gehören zu einer Insel, nicht zum Biom. | `data/islands.json` › `plannedChunks` |
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
| Pflicht ist die **Chunk-Art** (`kind`, aus `chunkWeights` des Bioms oder `portal`, `chest`, `recruitCamp`). Optional sind ein **Chunk-Seed** (`seed`) und **feste Objekte** (`objects`: Art und `at`). Objekt-Arten: Ressourcen laut `resourcesPerChunk` des Bioms, Ader (`veins.kind`), Truhe, Camp, Portal. | Teil 1 deckt feste Objekte und den Seed je Chunk ab. | `plannedChunks`, `data/biomes/*.json` |
| Eine **Objektliste ersetzt** die gewürfelten Ressourcen der Chunk-Art: Mit Liste ist der Chunk vollständig fest. Ohne Liste würfelt der Inhalt der Chunk-Art aus dem eigenen Strom. **Feste Skill-Punkte gibt es nicht**; Skill-Punkte würfeln weiter frei. | Fest heißt fest; kein Mischzustand, der vom Strom abhängt. | – |
| Der Inhalt eines geplanten Chunks würfelt **nur aus seinem eigenen Strom** (`engine/rng`), nie aus dem Haupt-Strom: ohne Chunk-Seed `<biom>:<seed>:chunk:<platz>` (Art fest, Inhalt wechselt mit dem Level-Seed), mit Chunk-Seed `<biom>:chunk:<chunk-seed>` (Inhalt in jedem Spielstand gleich). Ein leerer Chunk-Seed gilt als kein Chunk-Seed. Die Namen sind Vertrag wie die Reihenfolge des Haupt-Stroms. | Determinismus; die nicht geplanten Chunks bleiben von der Planung unberührt. | `engine/rng` |
| Reservierung: Der Generator reserviert die geplanten Plätze direkt nach `frame()`. Portale, Ereignisse, Chunk-Arten, Skill-Punkte und Adern würfeln nur über freie, nicht geplante Chunks. Ohne Planung ändern sich Reihenfolge und Zahl der Würfe nicht. | Konflikte werden beim Planen sichtbar, nicht erst zur Laufzeit. | – |

## 3. Biom-Summen

| Regel | Begründung | Daten |
|---|---|---|
| Was das Biom zählt, gilt **inklusive** der Planung: Portale genau `portals.count`, Truhen und Camps `eventChunks` min–max, Adern genau `veins.count`, Skill-Punkte min–max. Feste Portale, Truhen, Camps und Adern **ersetzen** gewürfelte, sie ergänzen sie nicht. Feste Truhen und Camps rechnen auf die Zahl der Ereignis-Chunks an. | Die Biom-Werte bleiben Vertrag. | `data/biomes/*.json` |
| Übrige Portale verteilt der Generator **abwechselnd**, beginnend auf der Seite mit weniger festen Portalen. | Gleichmäßig, deterministisch. | – |

## 4. Lage-Regeln

| Regel | Begründung | Daten |
|---|---|---|
| Feste Objekte liegen wie gewürfelte gleicher Art. Ressourcen, Truhen und Adern halten ≥ 2 Units Abstand zur Chunk-Grenze. Camps dürfen innerhalb der Mauerlinien liegen (Glossar › Camp). Portale halten `minDistanceFromHubUnits` zum Hub und liegen außerhalb des äußersten Tors; Portal-Plätze: Wald ±3…±7, Höhle ±3…±5, Mine ±3 und ±4; in 25-Unit-Biomen braucht ein festes Portal auf Platz ±4 `at` ≥ 10. | Gleiches Verhalten in der Sim, keine neue Logik. | `data/biomes/*.json`, `engine/level/` |
| **Nur Adern** halten zusätzlich ≥ 4 Units zu jedem Bauplatz der Mauerlinien (inklusive Streuung +0 bis +4) und zum Farm-Weltplatz. Für Truhen und Camps gilt dieser Abstand nicht. | Adern sind Zahlziele (Markieren, Q57). | `data/hub.json` |
| `kind@x` ist je Stufe eindeutig: zwei Objekte gleicher Art an gleicher Lage sind verboten. | Der Spielstand ordnet Level-Objekte über `kind@x` zu. | – |

## 5. Prüfung und Fehlerfälle

| Regel | Begründung | Daten |
|---|---|---|
| Eine Planung wird **beim Laden** geprüft, unabhängig vom Seed, gegen die kürzeste mögliche Länge des Bioms und jede Streuung der Linien. Ist sie dort gültig, scheitert kein Seed zur Laufzeit. Die Mauerlinien sind ein Parameter (Standard: eingebettete Daten). Fehler nennen Insel, Tiefe, Platz und Grund; der Start scheitert wie bei einer ungültigen `islands.json`. | Fail closed. | `engine/level/`, `engine/sim/island_data.go` |
| Abgelehnt werden: Platz `0` oder Hub; `exit`/`edge`; Platz jenseits der kürzesten Länge; doppelter Platz oder doppeltes `kind@x`; Tiefe nicht in `depths`; unbekannte Chunk-Art oder Objekt-Art im Biom; Verstöße gegen die Lage-Regeln; mehr feste Portale, Truhen, Camps oder Adern als das Biom zählt; zu wenig freie Plätze für übrige Portale, Mindestzahl der Ereignisse oder Adern. Der Wald hat keine Adern. | Jeder Fall ein Test (WZ8.2). | – |
| Felder für **Teil 2** (Gebäude, Gegner) sind im Format reserviert, der Generator lehnt sie mit „noch nicht unterstützt“ ab (B-468). | Platz im Format, ohne Verhalten. | – |

## 6. Schreiben

| Regel | Begründung | Daten |
|---|---|---|
| Der Chunk-Editor im Welten Editor schreibt `data/islands.json` atomar und format-stabil und nur nach einer Stand-Prüfung (geändert seit dem Laden → nicht speichern, Entwurf bleibt). Nach jedem Speichern erscheint „REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung“. Eine gespeicherte Planung ist eine REG-Änderung durch 🧑. | Das Werkzeug schreibt REG-Daten; Golden ändert sich nur gewollt. | `data/islands.json` |

## 7. Chunk-Anzahl und Inselgrößen

Beschlossen von 🧑 im Workshop WZ17.1 am 2026-10-18 (Chat, B-485 Revision 2).

| Regel | Begründung | Daten |
|---|---|---|
| Die **Chunk-Anzahl** einer Stufe steht je Insel im Feld `chunkCounts` (Schlüssel = Tiefe, Wert = Anzahl), neben `plannedChunks`. Sie legt die Länge der Stufe in Chunks fest und ersetzt die gewürfelte Länge aus `lengthUnits`. Fehlt das Feld oder der Eintrag der Tiefe, gilt die Länge des Bioms wie bisher (Level, Golden und Spielstände bleiben unverändert). Die Länge in Units ist Anzahl × Chunk-Breite des Bioms (50 oder 25 Units). | Inseln lassen sich gezielt größer oder kleiner planen, ohne bestehende Level zu ändern. | `data/islands.json` › `chunkCounts` |
| Erlaubt sind nur die **Inselgrößen** der Tabelle: sehr klein **16** · klein **24** · mittel **40** · groß **64** · sehr groß **100** · mega groß **160** (Revision 5, 2026-10-18: die erste Tabelle 16/20/24/32/40/48 war zu eng). Die Tabelle gilt für alle Biome; alle Anzahlen sind gerade, der Hub liegt mittig. Jede andere Anzahl wird beim Laden abgelehnt (Fehler nennt Insel, Tiefe und die gültigen Werte). | 16 ist die kleinste Anzahl, die in allen Biomen Platz für Hub, Portale mit Mindestabstand und Ausgang lässt (Eisenfeste und Kristallhöhle mit 25-Unit-Chunks und je 3 Portalen); 8 und 12 gehen nicht. | `data/island-sizes.json` |
| Die Planung wird gegen die eingestellte Anzahl geprüft: Ein geplanter Platz jenseits der Anzahl wird abgelehnt (Fehler nennt Insel, Tiefe, Platz). Die Prüfung rechnet mit der eingestellten Anzahl statt mit der kürzesten Länge des Bioms. Biom-Summen (Portale, Ereignisse, Adern, Skill-Punkte) gelten unverändert. Kann der Generator eine Größe in einem Biom nicht erzeugen, meldet er den Fehler (Editor: Fehler am Entwurf, Speichern gesperrt); einen Nachweis über viele Seeds und alle Biome gibt es nicht, wie viele Chunks sinnvoll sind, ermittelt später ein eigenes Projekt (Beschluss 🧑, 2026-10-18). | Fail closed; der Editor legt an, was anlegbar ist. | `engine/level/` |
| Der Editor bietet nur die Größen der Tabelle an (kein Freitext). Chunk- und Live-Vorschau zeigen die eingestellte Länge für ungespeicherte Entwürfe aus derselben Quelle. Eine gespeicherte Chunk-Anzahl ist wie eine gespeicherte Planung eine REG-Änderung durch 🧑 (Hinweis nach dem Speichern, Golden nur gewollt). | Gleiche Schreibregel wie § 6. | `data/islands.json` |
| Die **Umwandlung** eines Seed-Chunks in feste Objekte übernimmt dessen Inhalt aus dem Vorschau-Level des Servers (Art und Lage der Objekte); Chunk- und Live-Vorschau zeigen danach dasselbe wie vorher. | Kein neuer Generator-Code; fest heißt fest (§ 2). | – |
