# Regelwerk: Wetter je Welt

Beschlossen von 🧑 in der Ideenrunde k3c-07 am 2026-11-15 (Ticket B-969; Rahmen: [`bosse.md`](bosse.md) § 2, [`welten.md`](welten.md), [`stufen.md`](stufen.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. **Alle Zahlen sind vorläufig** (Defaults, von 🧑 bestätigt; Feintuning später im Balancing). Jede Regel gilt für 2+ Spieler.
Umsetzung: Regeltext SI3.1, Engine SI3.2 (`engine/sim`), Darstellung B-999, Auswahl im Welten-Editor B-1003.

## 1. Grundsatz und Katalog

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| Es gibt einen kleinen **Katalog fester Wetterarten**, jede mit Regel und Grafik. Eine Welt wählt **genau eine Art oder keine**; das Wetter trifft **Spieler und Gegner gleich**. | Welten unterscheiden sich bisher nur über Biome und Grafik; je Welt ein eigenes Ereignis gibt Abwechslung, ohne die Spielregeln je Welt frei einstellbar zu machen. | `data/weather.json` (Katalog); Welt-Datei: optionales Feld `weather` (`welten.md` § 2) | – |
| Das Wetter ist ein **eigenes kleines System** neben den Nacht-Events. `data/events.json` bleibt unverändert (Vollmond, Blutmond, Händler-Überfall gelten für alle Welten). | Nacht-Events beschreiben Gegner und Belohnungen; Wetter verändert Sicht, Tempo oder Wege. | `data/weather.json`, `data/events.json` | – |
| Fehlt `weather` in der Welt-Datei, hat die Welt **kein Wetter** (Verhalten wie heute). Eine **unbekannte Art** lässt das Laden mit klarer Meldung scheitern (`welten.md` § 3). | Kein stilles Ignorieren von Tippfehlern. | Welt-Datei | – |
| Das Wetter hat keine Belohnung (keine angeschwemmten Truhen, keine umgeworfenen Bäume) und gilt nicht je Biom. | Schlank halten; Nicht-Ziele von B-969. | – | – |

### 1.1 Zuordnung der Welten

| Welt | Wetterart | Stand |
|---|---|---|
| `sengoku` | `taifun` | Rust RU62.5 (Kern, Zeitplan, Taifun) |
| `nil` | `flut` | Rust: Zeitplan RU62.5, Wirkung RU62.6 (`weather/flood.rs`) |
| `standard` | keins | – |
| `britannien` | keins | – |
| Polar (Welt gibt es noch nicht) | `schneesturm` | **geplant**, kommt mit der Polar-Welt (§ 4) |

## 2. Zeitplan und Vorboten

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Wetternacht:** jede `everyNights`-te Nacht mit **festem Versatz aus dem Seed**, frühestens Nacht `firstNight`. Startwerte: `everyNights` 5, `firstNight` 3. | Selten genug, um besonders zu sein; oft genug, um in jeder Kampagne vorzukommen. | `data/weather.json` › `everyNights`, `firstNight` | Eine Wetternacht etwa alle 5 Tage (Startziel) |
| **Deterministisch:** Der Zeitplan wird aus Insel-Seed und Tag **berechnet** (`engine/rng`, nie `math/rand`); gleicher Seed ergibt gleiche Wetternächte. **Nichts davon steht im Spielstand.** | Kein neues Spielstand-Feld; Laden mitten in der Wetternacht ergibt denselben Ablauf. | `engine/sim` (SI3.2) | Go-Test: gleicher Seed, gleiche Nächte |
| **Mond-Nächte weichen aus:** Fällt die Wetternacht auf Vollmond (jede 7.) oder Blutmond (jede 13.), **rückt sie eine Nacht weiter**. | Zwei Nacht-Ereignisse zugleich wären unlesbar und würden die Burg überfordern (`bosse.md` § 2). | `data/events.json` (nur gelesen) | Nie Wetter in einer Vollmond- oder Blutmond-Nacht |
| **Vorboten:** Ab **Tagesanbruch des Vortags** zeigt der Himmel Vorboten (Phase `forecast`: Wolken, steigendes Wasser). **Wirkung** (Phase `active`) von **Nachtbeginn bis Tagesanbruch**; Dauer **eine Nacht**. | Angekündigt, damit Spieler sich vorbereiten können (Spielgefühl: kein unfairer Überfall). | Zustand: `weather` mit Art und Phase | Vorboten genau am Vortag |
| **Zeitplan je Insel:** Jede Insel rechnet ihren eigenen Plan aus ihrem Seed; alle Stufen einer Insel haben **denselben** Plan. Bei **Inselwechsel** (auch in der Vorboten-Phase) gilt der Plan der neuen Insel, nichts wird übernommen. | Spielstruktur 003: Stufen laufen im selben Takt, Inseln sind getrennte Welten. | `stufen.md` | – |

## 3. Wirkung und Geltung

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Wirkung nur an der Oberfläche** (Stufe mit Tiefe 0). Unter Tage gibt es keine Wirkung, der Zeitplan läuft trotzdem mit. Mit zwei Spielern auf verschiedenen Stufen sind nur Monarch und Einheiten der Oberfläche betroffen. | Wetter gehört unter freien Himmel; Spieler in Höhlen bleiben unberührt. | – | Go-Test: zwei Spieler auf verschiedenen Stufen |
| **Zustand für die Darstellung:** Der Zustand nennt `weather` mit **Art** und **Phase** (`forecast`, `active`) sowie bei Flut die **gesperrten Bereiche** (`floodZones`, Liste von `{from, to}` in Units, nur in der aktiven Flut an der Oberfläche; berechnet, nicht im Spielstand), damit B-999 zeichnen kann. | Darstellung (B-999) braucht keine eigene Rechnung. | `docs/protocol.md` | – |

### 3.1 Taifun (`sengoku`)

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Tempo:** Alle Einheiten (Monarchen, Truppen, Bürger, Gegner) bewegen sich mit Tempo × `speedFactor` (Start 0,7). | „Bremst alle“; trifft beide Seiten gleich. | `data/weather.json` › `taifun.speedFactor` | – |
| **Abdrift der Pfeile:** Die Fernkampf-Reichweite aller Schützen (Spieler- und Gegnerseite) × `rangeFactor` (Start 0,7). | „Drückt Pfeile zur Seite“ als kürzere Reichweite; eine echte Abdrift der Flugbahn wäre ein Eingriff in den Kampf (`kampf.md`). | `data/weather.json` › `taifun.rangeFactor` | – |

### 3.2 Flut (`nil`)

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Randzonen unter Wasser:** An **beiden Rändern der Insel** stehen `floodUnits` Units (Start 24) unter Wasser und sind für **alle Bodeneinheiten** unpassierbar. Die Flut nimmt immer die beiden Randzonen; Höhen kennt die Welt nicht. | „Setzt tiefe Abschnitte unter Wasser und sperrt Wege“, ohne neues Höhenmodell. | `data/weather.json` › `flut.floodUnits` | – |
| **Wer bei Beginn in der Zone steht, läuft heraus** (mit `runUnits` Units je Sekunde, Start 20, schneller als jedes Bodentempo) und kommt nicht zurück, bis die Flut vorbei ist. Gegner mit `ignoresWalls` (Flieger) ignorieren die Flut. | Niemand wird eingeschlossen oder verdrängt. | `data/weather.json` › `flut.runUnits` | – |
| **Portale in der Zone** halten ihre Gegner bis zum Morgen: Gegner aus überfluteten Portalen kommen erst nach der Flut: ihre Spawns bleiben in der Warteschlange, solange das Portal im Wasser steht; bei Tagesanbruch gilt die übliche Regel (die Nachtwelle endet, Reste verfallen). | Sonst liefen Gegner durch gesperrte Zonen. | – | – |
| **Gebäude nehmen keinen Schaden.** | Wetter soll Wege verändern, nicht Burgen zerstören. | – | – |
| **Schmale Insel:** Ist die Insel schmaler als zwei Flutzonen plus Hub, endet die Zone vor dem Burg-Radius (`castleRadiusUnits`). | Die Burg bleibt immer erreichbar. | `data/hub.json` | – |

## 4. Geplant: Schneesturm (Polar)

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Schneesturm** nimmt die **Sicht** (Darstellung und Wahrnehmung, genaue Wirkung offen). Er wird zusammen mit der **Polar-Welt** umgesetzt, die es noch nicht gibt (`data/worlds/`: standard, sengoku, nil, britannien). Zeitplan, Vorboten und Geltung gelten wie in §§ 2 und 3. | Jede Welt ein eigenes Wetter; die Wirkung folgt dem Spielgefühl der Welt. | nicht in `data/weather.json` bis zur Polar-Welt | – |

## 5. Offen und Annahmen

- **Alle Zahlen** (`everyNights` 5, `firstNight` 3, `speedFactor` 0,7, `rangeFactor` 0,7, `floodUnits` 24) sind vorläufige Defaults; 🧑 hat die Defaults bestätigt, einzelne Zahlen nicht gesondert. Balancing ist gesperrt, Feintuning später.
- Pfeile „zur Seite drücken“ gilt als kürzere Reichweite, nicht als Flugbahn-Abdrift.
- Schneesturm: Wirkung im Detail und Datenfelder offen bis zur Polar-Welt.
- Kennzahlen, die fehlen: Wetterereignisse im Bericht (`weatherStarted`, `weatherEnded`), Zielkorridore je Wetterart.
