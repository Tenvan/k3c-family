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
| Der Editor im Welt-Betrachter schreibt `data/islands.json` atomar und format-stabil und nur nach einer Stand-Prüfung (geändert seit dem Laden → nicht speichern, Entwurf bleibt). Nach jedem Speichern erscheint „REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung“. Eine gespeicherte Planung ist eine REG-Änderung durch 🧑. | Das Werkzeug schreibt REG-Daten; Golden ändert sich nur gewollt. | `data/islands.json` |
