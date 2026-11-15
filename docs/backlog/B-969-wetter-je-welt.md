# B-969 · Jede Welt hat ein eigenes Wetter-Ereignis mit Spielwirkung

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SI3
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Welten unterscheiden sich bisher über Biome und Grafik, nicht über Ereignisse.

## Ziel

Jede Welt hat ein eigenes Wetter-Ereignis mit Spielwirkung: Sengoku Taifun, Nil Flut, Polar Schneesturm. Kurz, angekündigt, verändert für eine Nacht Sicht, Tempo oder Wege.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI3), Fachprüfung durch `k3c-welten`. Freigabe der Spec, Bestätigung der Defaults und Abnahme am TV: 🧑.

## Anforderungen

Dieses Ticket trägt **Regeltext und Engine**; Darstellung steht in B-999, die Auswahl im Welten-Editor in B-1003.

- **Wirkung (🧑 2026-11-15): je Welt eine eigene.** Taifun (Sengoku) bremst alle und drückt Pfeile zur Seite; Flut (Nil) setzt tiefe Abschnitte unter Wasser und sperrt Wege; Schneesturm (Polar) nimmt die Sicht. Das Wetter trifft Spieler und Gegner gleich.
- **Zeitpunkt (🧑 2026-11-15): selten, einen Tag vorher angekündigt.** Etwa alle paar Tage, deterministisch aus dem Seed (`engine/rng`); am Vortag zeigt der Himmel Vorboten (Wolken, steigendes Wasser). Dauer eine Nacht.
- **Ablage (🧑 2026-11-15): Wetter-Katalog, die Welt wählt eins.** Ein kleiner Katalog fester Wetterarten mit Regel und Grafik; die Welt-Datei nennt nur die Art oder „keins“. Neue Welten wählen im Welten-Editor aus der Liste.
- Funktioniert mit 2 Spielern gleichzeitig, auch wenn sie auf verschiedenen Stufen sind.

Ergänzt (Defaults, siehe Offene Fragen):

- Katalog in neuer Datei `data/weather.json` (Arten `taifun`, `flut`; `schneesturm` ist im Regeltext als geplant beschrieben und kommt mit der Polar-Welt, die es noch nicht gibt). Die Welt-Datei bekommt das optionale Feld `weather` (Art aus dem Katalog; fehlt es, hat die Welt kein Wetter). `sengoku` wählt `taifun`, `nil` wählt `flut`, `standard` und `britannien` bleiben ohne.
- Eigenes kleines System neben den Nacht-Events (`data/events.json` bleibt unverändert): Der Zeitplan gilt je Insel und wird aus Insel-Seed und Tag berechnet, nichts davon steht im Spielstand.
- Wetternacht: jede `everyNights`-te Nacht mit festem Versatz aus dem Seed, frühestens Nacht `firstNight`; fällt sie auf Vollmond oder Blutmond, rückt sie eine Nacht weiter. Vorboten ab Tagesanbruch des Vortags, Wirkung von Nachtbeginn bis Tagesanbruch.
- Das Wetter wirkt nur an der Oberfläche (Stufe mit Tiefe 0); der Zeitplan ist für alle Stufen derselbe, unter Tage gibt es keine Wirkung.
- Taifun: Tempo aller Einheiten (Monarchen, Truppen, Bürger, Gegner) × `speedFactor`; Fernkampf-Reichweite aller Schützen (Spieler- und Gegnerseite) × `rangeFactor` als Abdrift der Pfeile.
- Flut: An beiden Rändern der Insel stehen `floodUnits` Units unter Wasser und sind für alle Bodeneinheiten unpassierbar; wer bei Beginn darin steht, läuft heraus und kommt nicht zurück. Gegner aus überfluteten Portalen kommen erst nach der Flut. Gebäude nehmen keinen Schaden.
- Der Zustand nennt `weather` mit Art und Phase (`forecast`, `active`) sowie bei Flut die gesperrten Bereiche, damit B-999 zeichnen kann.
- Alle Zahlen in `data/weather.json`, vorläufig (Balancing später).

## Nicht-Ziele

Wetter je Biom, frei einstellbare Wirkung je Welt, Belohnungen durch Wetter (angespülte Truhen, umgeworfene Bäume), Wetter als reine Kulisse.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/wetter.md`, `docs/rules/welten.md`, `engine/sim/**`, `data/weather.json`, `data/embed.go`, `data/worlds/sengoku.json`, `data/worlds/nil.json`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –
- Zwei Sessions: SI3.1 Regeltext (REG), SI3.2 Engine (SIM). Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`).
- Kein neues Feld im Spielstand (Zeitplan wird berechnet); braucht die Umsetzung doch eines, hält die Session an und meldet es (dann Sperre `spielstand`).
- Das Format der Welt-Datei ändert sich um ein optionales Feld; `docs/rules/welten.md` § 2 und § 3 ziehen mit. Keine Simulation, kein Balancing (gesperrt).

## Beispiele

- Sengoku, Tag 6: Am Abend ziehen dunkle Wolken auf. Nacht 7: Taifun, Monarchen, Truppen und Gegner sind langsamer, Pfeile treiben ab. Am Morgen ist er vorbei.
- Nil, Wetternacht: Die Ränder der Insel stehen unter Wasser; ein Portal am Rand schickt seine Gegner erst am Morgen.
- Monarch 1 an der Oberfläche, Monarch 2 in Stufe 2: Nur Monarch 1 und die Einheiten der Oberfläche sind betroffen.

## Ausnahme- und Fehlerfälle

- Welt-Datei nennt eine unbekannte Wetterart → Laden schlägt mit klarer Meldung fehl (`docs/rules/welten.md` § 3).
- Welt ohne Feld `weather` → kein Wetter, Verhalten wie heute.
- Wetternacht trifft Vollmond oder Blutmond → rückt eine Nacht weiter.
- Insel schmaler als zwei Flutzonen plus Hub → die Flutzone endet vor dem Burg-Radius (`castleRadiusUnits`).
- Inselwechsel in der Vorboten-Phase → der Zeitplan der neuen Insel gilt, nichts wird übernommen.
- Speichern und Laden mitten in der Wetternacht → das Wetter läuft nach dem Laden weiter (aus Seed und Tag berechnet).

## Akzeptanzkriterien

- **AC-01** `docs/rules/wetter.md` (neu) beschreibt Katalog, Zeitplan, Vorboten, Taifun, Flut und den geplanten Schneesturm in der Tabellenform der Regeldateien; Werte als vorläufig markiert.
- **AC-02** `data/weather.json` enthält `taifun` und `flut` mit `everyNights`, `firstNight` und den Wirkungswerten, ist in `data/embed.go` eingebunden; ein Go-Test lädt und prüft sie.
- **AC-03** Die Welt-Datei kennt `weather`; Go-Test: unbekannte Art wird beim Laden abgelehnt, `sengoku` hat `taifun`, `nil` hat `flut`, `standard` und `britannien` keins; `docs/rules/welten.md` § 2 und § 3 nennen das Feld.
- **AC-04** Go-Test Zeitplan: gleicher Seed → gleiche Wetternächte; Vorboten genau am Vortag; Dauer eine Nacht; nie in einer Vollmond- oder Blutmond-Nacht.
- **AC-05** Go-Test Taifun: Tempo von Monarch, Truppe und Gegner × `speedFactor`, Reichweite der Schützen beider Seiten × `rangeFactor`, nur in der Wetternacht.
- **AC-06** Go-Test Flut: Die Randzonen sind für Spieler und Gegner unpassierbar, Einheiten darin laufen heraus, Portale darin halten ihre Gegner bis zum Morgen.
- **AC-07** Go-Test Stufen: gleicher Zeitplan für alle Stufen, Wirkung nur bei Tiefe 0; zwei Spieler auf verschiedenen Stufen.
- **AC-08** Go-Test: Der Zustand nennt Art, Phase und gesperrte Bereiche; Speichern und Laden in der Wetternacht ändert nichts am Ablauf.
- **AC-09** `check_run task:check:diff` grün; `testdata/golden/` unverändert (Welt `standard` hat kein Wetter) oder Änderung im Commit begründet.

## Offene Fragen

Alle mit Default entschieden (vorläufig, 🧑 bestätigt):

- Eigenes System oder Event-Art? → Eigenes kleines System mit `data/weather.json`; `data/events.json` beschreibt Nacht-Events mit Gegnern und gilt für alle Welten gleich (geprüft: Vollmond, Blutmond, Händler-Überfall).
- Ganze Insel oder je Stufe? → Zeitplan je Insel, Wirkung nur an der Oberfläche.
- Stärke und Abstand? → `everyNights` 5, `firstNight` 3; Taifun `speedFactor` 0,7 und `rangeFactor` 0,7; Flut `floodUnits` 24 (vorläufig, Balancing später).
- Flut auf Inseln ohne tiefe Abschnitte? → Die Flut nimmt immer die beiden Randzonen der Insel; Höhen kennt die Welt nicht.
- Polar und Schneesturm? → Die Welt Polar gibt es nicht (`data/worlds/`: standard, sengoku, nil, britannien). Der Schneesturm steht als geplant im Regeltext und wird mit der Polar-Welt umgesetzt; `britannien` und `standard` bleiben ohne Wetter.
- Pfeile „zur Seite drücken“? → Als kürzere Reichweite der Schützen; eine echte Abdrift der Flugbahn wäre ein Eingriff in den Kampf.

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt. 2026-11-15 von L zerlegt in B-969 (Regel und Engine), B-999 (Darstellung) und B-1003 (Welten-Editor).
