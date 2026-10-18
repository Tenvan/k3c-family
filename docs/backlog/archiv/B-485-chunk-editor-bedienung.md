# B-485 · Chunk-Editor im Welt-Betrachter komfortabler bedienen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ17
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** freigegeben
- **Revision:** 5
- **Freigabe:** 2026-10-18, Ralf im Chat (Revision 5: Inselgrößen 16/24/40/64/100/160)

## Ausgangslage

Rückmeldung von 🧑 aus der Abnahme WZ8.7 (Punkt 2, 2026-10-18): Das Verhalten des Chunk-Editors im Welt-Betrachter (`tools/k3c-studio`) stimmt, die Bedienung ist aber unkomfortabel. Das Dropdown ist zu klein, der Schnellfilter zeigt die übrigen Auswahlmöglichkeiten nicht, und Einträge haben kein Icon zum Text.

## Ziel

Der Chunk-Editor lässt sich flüssig bedienen: durchsuchbare Auswahllisten mit Emoji-Icons, Chunk-Anzahl je Stufe über Inselgrößen einstellbar (ändert die Level-Länge), getrennter Zoom der Vorschauen bei synchronem Bildlauf und Umwandlung eines Seed-Chunks in feste Objekte.

## Beteiligte und Zielgruppen

🧑 (Ralf) plant und bearbeitet Chunks im Asset-Studio und gibt die Spec frei und nimmt am PC ab. Agenten setzen REG-, SIM- und DEV-Sessions um, ein getrennter Reviewer prüft.

## Anforderungen

Entschieden von 🧑 am 2026-10-18 (Chat, Revision 2 im Workshop WZ17.1):

**Auswahllisten und Icons (DEV)**
- Art und Objekt wählt man in einer durchsuchbaren Liste: ein Feld mit großem Dropdown, alle Einträge sichtbar. Tippen sortiert Treffer nach oben und blendet nicht passende nur grau ab, statt sie zu verbergen.
- Jeder Eintrag trägt ein Emoji vor dem Text (ohne neue Dateien), auch im Feld und in der Chunk-Liste.

**Chunk-Anzahl je Stufe mit Inselgrößen (REG, SIM, DEV)**
- Die Chunk-Anzahl einer Stufe ist einstellbar und ändert die Level-Länge im Spiel (nicht nur die Anzeige im Editor). Eine Vorauswahl nennt feste Chunk-Anzahlen: sehr klein 16 · klein 20 · mittel 24 · groß 32 · sehr groß 40 · mega groß 48 (alle gerade, der Hub liegt mittig). Die Tabelle gilt für alle Biome und steht als Wert in den Daten (`data/island-sizes.json`), nicht im Code. Grund für 16 als kleinste Größe (Workshop WZ17.1): Portale brauchen Platz ab ±3 Chunks vom Hub; die kürzeste Stufe der Eisenfeste und Kristallhöhle (Chunk 25 Units, Hub 4 Chunks, 3 Portale) hat 16 Chunks. Die Länge in Units hängt von der Chunk-Breite des Bioms ab (16 Chunks = 800 Units im Wald, 400 in der Kristallhöhle).
- Die Anzahl steht je Insel im Feld `chunkCounts` neben `plannedChunks` (Tiefe als Schlüssel, z. B. `{"0": 24}`) in `data/islands.json`. Gilt je Stufe; fehlt der Eintrag, bleibt die Länge aus `lengthUnits` des Bioms (heutiges Verhalten, Golden-Daten byte-gleich, wie WZ8 AC-02).
- Die Planung (Chunk-Plätze, Mauerlinien, Biom-Summen) wird gegen die eingestellte Anzahl geprüft; passt ein geplanter Chunk nicht mehr hinein, wird der Entwurf abgelehnt und der Fehler steht am Platz.
- Chunk- und Live-Vorschau zeigen die eingestellte Länge ohne Speichern (Entwurf), aus derselben Quelle wie bisher.

**Zoom getrennt, Bildlauf synchron (DEV)**
- Alt + Mausrad über der Chunk-Vorschau oder der Live-Vorschau stellt nur deren Zoom ein, der andere bleibt.
- Beide Vorschauen zeigen dieselbe Weltposition in der Bildmitte, auch bei verschiedenem Zoom; Scrollen verschiebt beide um dieselbe Welt-Strecke.
- Beide Zoom-Faktoren bleiben pro Fenster erhalten (auch beim Wechsel von Insel und Stufe), nicht dauerhaft gespeichert.

**Seed-Chunk anpassen (DEV, SRV)**
- Ein Chunk mit Chunk-Seed lässt sich in feste Objekte umwandeln: Der Inhalt aus dem Seed (Bäume, Felsen, Truhe …) wird übernommen und ist dann einzeln verschiebbar, löschbar und ergänzbar. Den Inhalt liefert das Vorschau-Level (der Server rechnet das Level mit dem Entwurf und gibt die Objekte des Chunks mit Art und Lage zurück), kein neuer Generator-Code.
- Die Umwandlung ist verlustfrei: Chunk- und Live-Vorschau zeigen unmittelbar danach dasselbe wie vorher.

## Nicht-Ziele

- Änderung bereits gespeicherter Welten (Spielstand-Format bleibt).
- Ruinen und Gegnerlager (B-468).
- Dauerhaftes Speichern des Zooms.
- Die Sondergegend selbst (bleibt Entwurf, wird später gezielt geplant).

## Regeln und Einschränkungen

- Ändert die Level-Länge je Stufe, ist das eine REG-Änderung (Regeln unter `docs/rules/` zuerst, Glossar: „Inselgröße“ als neuer Begriff); Golden-Daten bleiben ohne Einstellung byte-gleich, ein neuer Golden-Fall nur als eigener Commit mit Begründung.
- Eine Session hat genau eine Domäne: REG (Workshop, Format), SIM (Generator, Prüfung), SRV (Vorschau-Option, falls nötig), DEV (Editor, Zoom, Listen). Spielstand-Format und Protokoll bleiben unverändert.
- Determinismus: nur `engine/rng`; kein `Math.random()` im Client.
- Keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- Das Schreiben nach `data/islands.json` folgt den Regeln aus WZ8 (atomar, format-stabil, Stand-Prüfung); B-486 (Stand-Prüfung schweigt) wird vorher behoben.

## Beispiele

- Stufe 1 (Wald) auf „klein“ stellen → 20 Chunks (1000 Units), Hub in der Mitte; Chunk- und Live-Vorschau zeigen die Stufe sofort als Entwurf.
- Planungs-Chunk auf Platz +11 bei 20 Chunks → Fehler am Platz +11, Speichern gesperrt.
- Alt + Mausrad über der Live-Vorschau → nur diese zoomt; Bildlauf verschiebt beide Vorschauen um dieselbe Welt-Strecke.
- Chunk mit Seed „lichtung“ → „In feste Objekte umwandeln“ → die Vorschau sieht gleich aus, die Objekte stehen einzeln in der Liste.

## Ausnahme- und Fehlerfälle

- Chunk-Anzahl, die nicht in der Größen-Tabelle steht (ungerade, kleiner als 16 oder sonst nicht eine der sechs Größen) → beim Laden abgelehnt mit Insel, Tiefe und den gültigen Werten (Freitext im Editor gibt es nicht, nur die Vorauswahl).
- Verkleinern der Stufe mit bestehender Planung, die nicht mehr passt → Fehler am Platz, Entwurf bleibt.
- Datei von außen geändert → Stand-Prüfung wie WZ8 (B-486).
- Seed-Chunk ohne Seed oder ungültiger Chunk → Umwandlung gesperrt, Fehler am Platz.

## Akzeptanzkriterien

- **AC-01** Frontend-Test: Die Art-/Objekt-Liste zeigt alle Einträge, Tippen sortiert Treffer nach oben, nicht passende sind abgeblendet aber vorhanden; jeder Eintrag hat ein Emoji.
- **AC-02** Go-Test (Generator): Ohne Chunk-Anzahl sind `level-*.json` und `sim-*.json` byte-gleich; mit Anzahl 16, 20, 24, 32, 40, 48 hat das Level (Einzeltest je Größe, ein Biom, ein Seed) genau diese Chunks (gerade, Hub mittig); erzeugt eine Größe in einem Biom kein gültiges Level, meldet die Erzeugung den Fehler sauber (kein Panic). Ein Nachweis über viele Seeds und alle Biome entfällt (Entscheidung von 🧑 am 2026-10-18); wie viele Chunks sinnvoll sind, ermittelt ein eigenes Projekt.
- **AC-03** Go-Test: Die Prüfung der Planung lehnt einen Chunk-Platz außerhalb der eingestellten Anzahl ab und nennt den Platz; `chunkCounts` mit einer Anzahl außerhalb der Tabelle wird beim Laden mit Insel und Tiefe abgelehnt.
- **AC-04** Frontend-Test: Die Größen-Vorauswahl setzt die Anzahl; Chunk- und Live-Vorschau zeigen die Länge des Entwurfs aus derselben Quelle; ist die Anzahl für das Biom nicht erzeugbar, steht der Fehler am Entwurf und Speichern ist gesperrt.
- **AC-05** Frontend-Test: Alt + Mausrad ändert nur den Zoom der Vorschau unter dem Zeiger; Scrollen verschiebt beide um dieselbe Welt-Strecke (gleiche Weltposition in der Bildmitte); Zoom bleibt beim Wechsel von Insel und Stufe.
- **AC-06** Test: Die Umwandlung eines Seed-Chunks in feste Objekte liefert in Chunk- und Live-Vorschau denselben Inhalt wie vorher (gleiche Objekte und Orte); der Server liefert die Objekte eines Chunks aus dem Vorschau-Level.
- **AC-07** Abnahme von 🧑 am PC im laufenden Studio (Watch-Modus, direkt nach der Umsetzung): Bedienung der Listen, Größen-Auswahl, getrennter Zoom und Umwandlung.

## Offene Fragen

Entschieden von 🧑 am 2026-10-18 (Chat und Workshop WZ17.1, Revision 3): Chunk-Anzahl ändert die Level-Länge je Stufe; Größen 16 · 20 · 24 · 32 · 40 · 48 (ersetzen die ersten Annahmen 8 · 12 · 16 · 24 · 32 · 48), Feld `chunkCounts` neben `plannedChunks`, Tabelle in `data/island-sizes.json`; Seed-Inhalt aus dem Vorschau-Level; Zoom mit gleicher Weltposition, pro Fenster gemerkt; Seed-Chunk wird in feste Objekte übernommen; durchsuchbare Liste mit Emoji-Icons; keine weiteren Mängel.

**Revision 3 (2026-10-18, 🧑):** Aufwendige Simulations- und Nachweisläufe entfallen (100 Seeds je Größe und Biom, Benchmark, neuer Golden-Fall). Der Editor legt an, was anlegbar ist; was der Generator für ein Biom nicht erzeugen kann, meldet die Prüfung als Fehler am Entwurf. Wie viele Chunks je Stufe und wie viele Inseln je Welt sinnvoll sind, ermittelt später ein eigenes Projekt.

Offen: nichts Blockierendes. Im Spielstand gespeicherte Welten ändern sich nicht (Spielstand-Format bleibt); die Länge gilt nur für neu erzeugte Stufen.

## Notizen

**Revision 5 (2026-10-18, 🧑 nach dem Live-Test von WZ17.5): Die Inselgrößen sind sehr klein 16 · klein 24 · mittel 40 · groß 64 · sehr groß 100 · mega groß 160 Chunks** (der Abstand 16 zu 48 war zu gering). Diese Tabelle ersetzt jede Nennung von 16 · 20 · 24 · 32 · 40 · 48 in diesem Ticket (Anforderungen, Beispiele, AC-02, Offene Fragen); maßgeblich sind `docs/rules/geplante-chunks.md` § 7 und `data/island-sizes.json`. Beispiel „Wald auf klein“ = 24 Chunks (1200 Units).

**Nachträge von 🧑 aus der Live-Abnahme von WZ17.4 (2026-10-18, Revision 4), gelten als Teil von AC-01 und AC-05:**
- Die Auswahllisten zeigen **deutsche Namen** als Text (Glossar-Begriffe, wo es sie gibt); der technische Wert (`kind`) bleibt in den Daten. Der Filter findet deutschen Namen und Wert. (Abgenommen: passt.)
- Statt der Positions-Leiste gibt es einen **Bereichs-Schieber**: Die Spur steht für die ganze Stufe, der Griff für den sichtbaren Bereich (Breite folgt dem Zoom); Ziehen und Klick auf die Spur verschieben beide Vorschauen um dieselbe Welt-Strecke. Der gelbe Bereich für die Live-Vorschau passt zum sichtbaren Bereich (Fehler behoben, abgenommen).
- **Manuell geplante Chunks** sind in der Chunk-Vorschau mit einem farbigen, dickeren unteren Rand hervorgehoben (blau, bei Prüffehler rot); der Mechanismus ist allgemein. (Abgenommen: passt.)
- **Chunk-Infos im Bodenstreifen der Live-Vorschau** (nur Overlay im Welten Editor, nicht im Renderer): Farbe, Art (deutsch), Platz, Chunk-Seed, Anzahl fester Objekte; bei starkem Zoom lesbar, bei weitem Herauszoomen kompakt. **Klick auf einen Chunk** (Streifen, Bodenstreifen, Planungsliste) springt dorthin. (Abgenommen: passt.)
