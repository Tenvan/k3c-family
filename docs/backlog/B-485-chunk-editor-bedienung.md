# B-485 · Chunk-Editor im Welt-Betrachter komfortabler bedienen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WZ17
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-18, Ralf im Chat

## Ausgangslage

Rückmeldung von 🧑 aus der Abnahme WZ8.7 (Punkt 2, 2026-10-18): Das Verhalten des Chunk-Editors im Welt-Betrachter (`tools/k3c-studio`) stimmt, die Bedienung ist aber unkomfortabel. Das Dropdown ist zu klein, der Schnellfilter zeigt die übrigen Auswahlmöglichkeiten nicht, und Einträge haben kein Icon zum Text.

## Ziel

Der Chunk-Editor lässt sich flüssig bedienen: durchsuchbare Auswahllisten mit Emoji-Icons, Chunk-Anzahl je Stufe über Inselgrößen einstellbar (ändert die Level-Länge), getrennter Zoom der Vorschauen bei synchronem Bildlauf und Umwandlung eines Seed-Chunks in feste Objekte.

## Beteiligte und Zielgruppen

🧑 (Ralf) plant und bearbeitet Chunks im Asset-Studio und gibt die Spec frei und nimmt am PC ab. Agenten setzen REG-, SIM- und DEV-Sessions um, ein getrennter Reviewer prüft.

## Anforderungen

Entschieden von 🧑 am 2026-10-18 (Chat):

**Auswahllisten und Icons (DEV)**
- Art und Objekt wählt man in einer durchsuchbaren Liste: ein Feld mit großem Dropdown, alle Einträge sichtbar. Tippen sortiert Treffer nach oben und blendet nicht passende nur grau ab, statt sie zu verbergen.
- Jeder Eintrag trägt ein Emoji vor dem Text (ohne neue Dateien), auch im Feld und in der Chunk-Liste.

**Chunk-Anzahl je Stufe mit Inselgrößen (REG, SIM, DEV)**
- Die Chunk-Anzahl einer Stufe ist einstellbar und ändert die Level-Länge im Spiel (nicht nur die Anzeige im Editor). Eine Vorauswahl nennt feste Chunk-Anzahlen: sehr klein 8 · klein 12 · mittel 16 · groß 24 · sehr groß 32 · mega groß 48 (alle gerade, der Hub liegt mittig). Die Tabelle ist eine Annahme von 🧑 und gehört als Wert in die Daten, nicht in den Code.
- Gilt je Stufe; ohne Einstellung bleibt die Länge aus `lengthUnits` des Bioms (heutiges Verhalten, Golden-Daten byte-gleich, wie WZ8 AC-02).
- Die Planung (Chunk-Plätze, Mauerlinien, Biom-Summen) wird gegen die eingestellte Anzahl geprüft; passt ein geplanter Chunk nicht mehr hinein, wird der Entwurf abgelehnt und der Fehler steht am Platz.
- Chunk- und Live-Vorschau zeigen die eingestellte Länge ohne Speichern (Entwurf), aus derselben Quelle wie bisher.

**Zoom getrennt, Bildlauf synchron (DEV)**
- Alt + Mausrad über der Chunk-Vorschau oder der Live-Vorschau stellt nur deren Zoom ein, der andere bleibt.
- Beide Vorschauen zeigen dieselbe Weltposition in der Bildmitte, auch bei verschiedenem Zoom; Scrollen verschiebt beide um dieselbe Welt-Strecke.
- Beide Zoom-Faktoren bleiben pro Fenster erhalten (auch beim Wechsel von Insel und Stufe), nicht dauerhaft gespeichert.

**Seed-Chunk anpassen (DEV, ggf. SIM)**
- Ein Chunk mit Chunk-Seed lässt sich in feste Objekte umwandeln: Der Inhalt aus dem Seed (Bäume, Felsen, Truhe …) wird übernommen und ist dann einzeln verschiebbar, löschbar und ergänzbar.
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

- Stufe 1 (Wald) auf „klein“ stellen → 12 Chunks, Hub in der Mitte; Chunk- und Live-Vorschau zeigen die kürzere Stufe sofort als Entwurf.
- Planungs-Chunk auf Platz +9 bei 12 Chunks → Fehler am Platz +9, Speichern gesperrt.
- Alt + Mausrad über der Live-Vorschau → nur diese zoomt; Bildlauf verschiebt beide Vorschauen um dieselbe Welt-Strecke.
- Chunk mit Seed „lichtung“ → „In feste Objekte umwandeln“ → die Vorschau sieht gleich aus, die Objekte stehen einzeln in der Liste.

## Ausnahme- und Fehlerfälle

- Chunk-Anzahl ungerade oder außerhalb der Tabelle → abgelehnt mit Nennung der gültigen Werte (Annahme: nur die sechs Größen, ggf. Freitext gerade Zahl ab Untergrenze, siehe Offene Fragen 2).
- Verkleinern der Stufe mit bestehender Planung, die nicht mehr passt → Fehler am Platz, Entwurf bleibt.
- Datei von außen geändert → Stand-Prüfung wie WZ8 (B-486).
- Seed-Chunk ohne Seed oder ungültiger Chunk → Umwandlung gesperrt, Fehler am Platz.

## Akzeptanzkriterien

- **AC-01** Frontend-Test: Die Art-/Objekt-Liste zeigt alle Einträge, Tippen sortiert Treffer nach oben, nicht passende sind abgeblendet aber vorhanden; jeder Eintrag hat ein Emoji.
- **AC-02** Go-Test (Generator): Ohne Chunk-Anzahl sind `level-*.json` und `sim-*.json` byte-gleich; mit Anzahl 8, 12, 16, 24, 32, 48 hat das Level genau diese Chunks (gerade, Hub mittig), `level.Validate` über 100 Seeds je Größe ohne Befund.
- **AC-03** Go-Test: Die Prüfung der Planung lehnt einen Chunk-Platz außerhalb der eingestellten Anzahl ab und nennt den Platz.
- **AC-04** Frontend-Test: Die Größen-Vorauswahl setzt die Anzahl; Chunk- und Live-Vorschau zeigen die Länge des Entwurfs aus derselben Quelle.
- **AC-05** Frontend-Test: Alt + Mausrad ändert nur den Zoom der Vorschau unter dem Zeiger; Scrollen verschiebt beide um dieselbe Welt-Strecke (gleiche Weltposition in der Bildmitte); Zoom bleibt beim Wechsel von Insel und Stufe.
- **AC-06** Test: Die Umwandlung eines Seed-Chunks in feste Objekte liefert in Chunk- und Live-Vorschau denselben Inhalt wie vorher (gleiche Objekte und Orte).
- **AC-07** Abnahme von 🧑 am PC im Studio-Fenster: Bedienung der Listen, Größen-Auswahl, getrennter Zoom und Umwandlung (Liste in der Abnahme-Session).

## Offene Fragen

Entschieden von 🧑 am 2026-10-18: Chunk-Anzahl ändert die Level-Länge je Stufe (feste Anzahlen 8/12/16/24/32/48); Zoom mit gleicher Weltposition, pro Fenster gemerkt; Seed-Chunk wird in feste Objekte übernommen; durchsuchbare Liste mit Emoji-Icons; keine weiteren Mängel.

Noch offen (für die Sprint-Planung, nicht blockierend für die Spec):
1. Wo steht die Chunk-Anzahl? Annahme: Feld je Stufe in `data/islands.json` (REG-Änderung), Größen-Tabelle in `data/`; das Format klärt ein Workshop wie WZ8.1.
2. Verträgt der Generator eine Anzahl unter den heutigen Mindestwerten (8 Chunks) mit Hub, Ausgang, Portalen und Biom-Summen? Sonst Untergrenze oder Prüfung festlegen.
3. Im Spielstand gespeicherte Welten ändern sich nicht (Spielstand-Format bleibt); die Länge gilt nur für neu erzeugte Stufen.
4. Wie holt die Umwandlung den Seed-Inhalt (Vorschau-Level oder Generator-Funktion), ohne die Vorschau-Funktion aus WZ7.1 zu verbiegen?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
