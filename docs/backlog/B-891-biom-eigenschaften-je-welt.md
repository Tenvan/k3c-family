# B-891 · Biome haben einstellbare Eigenschaften (Walddichte, Steindichte, Baumarten-Mischung), aus dem Seed gewürfelt oder im Welten-Editor fest gesetzt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 (Defaults der offenen Fragen)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Biome (`data/biomes/*.json`) legen die Natur nur über `resourcesPerChunk` fest (z. B. `"tree": [0, 1]`, `"rock": [2, 4]` je Chunk-Typ). Es gibt eine Baumart `tree`, keine Dichte- oder Mischungsregler. Zwei Welten mit anderem Seed sehen deshalb im selben Biom fast gleich aus. Der Welten-Editor (B-879) zeigt Chunks, setzt aber keine Natur-Eigenschaften.

## Ziel

Jedes Biom bietet Eigenschaften (Walddichte, Steindichte, Baumarten-Mischung usw.), deren Wert je Welt aus dem Seed gewürfelt oder im Welten-Editor fest gesetzt wird. So reicht die Spanne vom lichten Monokultur-Wald bis zum dichten Mischwald.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

### Eigenschaften (Vorschlag)
- **Walddichte** 1–5 (Biome mit Bäumen): Faktor auf die Baumanzahl je Chunk.
- **Steindichte** 0–3 (Biome mit Steinen): Faktor auf Felsen und Adern.
- **Baumarten-Mischung**: je Baumart ein Gewicht 0–5 (0 = kommt nicht vor, 5 = sehr häufig). Die Wahrscheinlichkeit einer Art ist Gewicht / Summe aller Gewichte. Monokultur = eine Art > 0, alle anderen 0; Mischwald = mehrere Arten mit ähnlichen Gewichten. Mindestens eine Art muss > 0 sein.
- Weitere je Biom-Art, z. B. Buschdichte, Lichtungen (Anteil leerer Chunks), Kristalldichte (crystal), Pilze (cave). Die Liste steht je Biom in den Daten, nicht im Code.

### Datenmodell
- `data/biomes/<biom>.json` bekommt `traits`: je Eigenschaft `{min, max, default}`, bei Baumarten je Art `{min, max}` als Gewicht.
- Welt (`data/worlds/<welt>.json`) kann je Insel/Stufe einen Wert **fest setzen**; ohne Eintrag würfelt der Level-Generator den Wert aus dem Seed (`engine/rng`, deterministisch) innerhalb `min..max`.
- Der Level-Generator (`engine/level`) rechnet `resourcesPerChunk` mit den Faktoren um und wählt die Baumart je Baum nach den Gewichten.

### Welten-Editor
- Zeigt je Insel/Stufe die gewürfelten Werte, erlaubt Festsetzen und Zurücksetzen auf „aus Seed“; die Chunk-Vorschau zeigt das Ergebnis.

### Grafik
- Jede Baumart braucht ein Papierschnitt-Asset; fehlende Arten als eigenes PAP-Ticket.

## Nicht-Ziele

Balancing der Erträge (gesperrt bis auf Widerruf); neue Biome.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- AC-01: Gleicher Seed und gleiche Welt ergeben dieselben Werte und dieselbe Natur (Golden-Test).
- AC-02: Zwei Seeds ergeben im selben Biom messbar verschiedene Walddichte oder Mischung (Test).
- AC-03: Ein fest gesetzter Wert in der Welt gilt unabhängig vom Seed (Test).
- AC-04: Monokultur (eine Art Gewicht > 0) erzeugt nur diese Art; Gewichte 0 überall lehnt die Datenprüfung ab.
- AC-05: Der Welten-Editor zeigt und setzt die Werte; Sichtprüfung durch 🧑.

## Offene Fragen

- Welche Baumarten soll es geben (Vorschlag: Eiche, Tanne, Birke, Buche, tote Bäume)? Default: die vorhandenen Papierschnitt-Bäume als Arten nehmen.
- Gilt ein Wert je Insel oder je Stufe? Default: je Stufe, mit Vorgabe je Insel.
- Ändert die Dichte auch Erträge (Holz, Stein)? Default: nur Anzahl der Objekte; Ertrag je Objekt bleibt. Golden-Werte ändern sich trotzdem (task go:golden mit Begründung).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
