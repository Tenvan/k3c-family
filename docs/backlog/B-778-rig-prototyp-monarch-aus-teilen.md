# B-778 · Ein dritter Prototyp setzt den Monarchen in Ebitengine zur Laufzeit aus Rig-Teilen zusammen

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** NV2
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat („Danach NV2 freigegeben“, mit Reittier-Rig, Plätzen für Waffen und Rüstungen, Gen-Skript bleibt)

## Ausgangslage

Entscheidung 008: Das native Spiel entsteht in Go + Ebitengine. Der Monarch ist heute der größte Posten der Grafik: `figuren/monarch` rendert je Kombination aus Animation, Reittier und Spielerfarbe ein eigenes Sheet, 195 Sheets (39 Animationen × 5 Varianten), 68 MB PNG, 363 von 580 MiB RGBA aller Sheets (gemessen 2026-11-11 über die PNG-Köpfe in `public/grafik`). Jeder weitere Monarch, jede Kopfbedeckung und jedes Reittier vervielfacht das. B-752 hat dafür getrennte Ebenen aus Sheets vorgesehen (Stufe 1 umgesetzt); SVG im Spiel wurde dort verworfen.

Das Rig steht schon in `tools/paper`: 12 starre Teile mit Pivot und Eltern (`public/grafik/figuren/monarch.paper.json`), Posen `idle` und `lauf` als Funktionen (`rig/figur.ts`), Keyframes (`render/pose.ts`), Matrix-Kette (`render/baum.ts`), Umhang als Verlet-Kette mit 8 Segmenten (`rig/umhang.ts`), Reiten über Sattel-Anker und Gangart (`rig/reiten.ts`, `rig/gangarten.ts`).

## Ziel

Ein Prototyp in Go + Ebitengine zeichnet den Monarchen **und sein Reittier** zur Laufzeit aus einzelnen Teil-Texturen nach dem Rig, ohne Sheets, und belegt mit Messwerten und Bildvergleich, ob dieser Weg die Sheets ersetzen kann.

Gründe und Richtung (🧑 2026-11-11):
- Die Leistung der Go-Engine ist gegenüber WebGL mehr als ausreichend; Rechnen der Pose je Bild ist bezahlbar.
- Freie Kombinationen aus Monarch, Reittier, Rüstung und Waffe werden möglich; mit Sheets wächst die Zahl multiplikativ.
- **Zielbild:** Später sind möglichst alle Figuren Rig-Modelle statt PNG-Sheets (mehr Variationen). Das kann die Zahl gleichzeitiger Gegner begrenzen und wird mit weiteren Prototypen getestet (B-780).
- **Gen-Skript bleibt:** `gen:paper` rendert weiter alle Sheets, die noch gebraucht werden, und schrumpft Figur für Figur. Für Rig-Figuren liegt alles Relevante im Repo.
- **Waffen und Rüstungen** kommen später als optionale Elemente bei Monarch, Gegnern und Bossen dazu; das Rig-Modell sieht sie von Anfang an vor.

## Beteiligte und Zielgruppen

🧑 Ralf (Sichtprüfung, Messung am Mini-PC, Entscheidung über den Weg für alle Figuren), Claude (Bau, Referenzmessung).

## Anforderungen

- **Rig-Modell mit Plätzen:** Die Rig-Datei beschreibt eine Figur als Teile-Baum (Pivot, Eltern, Zeichenreihenfolge) plus benannte **Plätze** für optionale Elemente (mindestens `waffe`, `ruestung`, `kopfbedeckung`, `sattel`). Ein Platz hängt an einem Teil und nennt Position, Drehung und Zeichenebene. Das Format gilt für `figur` und `vierbeiner` und ist so gebaut, dass Gegner und Bosse es später nutzen können.
- **Teile-Export:** Jedes Teil von Monarch und Pferd wird einmal in Ruhelage als PNG gerastert (Papierstruktur wie im Sheet). Der Export liest Spec und Stil nur; `tools/paper` und `gen:paper` bleiben unverändert.
- **Alles im Repo:** Teil-PNGs und Rig-Dateien sind eingecheckt (klein); Bauen und Starten des Prototyps braucht keinen Gen-Lauf.
- **Rig in Go:** Figur: Posen `idle` und `lauf`, Keyframe-Animationen (`give`, `hit`, `death`, `respawn`), Matrix-Kette, Umhang. Vierbeiner: Gangarten `schritt`, `trab`, `galopp` und Stand, Beine als Papierstück entlang der Stützpunkte (`beinpfad`). Gleiche Formeln und Konstanten wie `tools/paper`.
- **Kombination:** Der Reiter sitzt am Platz `sattel` des Reittier-Rigs, in der richtigen Ebene zwischen hinteren und vorderen Teilen des Reittiers; Zügel wie im Sheet. Kein Sheet je Kombination.
- **Optionale Elemente:** Der Prototyp zeigt die Plätze an einer zweiten Waffe statt `schwert` und einem Rüstungs-Teil; ohne Element bleibt der Platz leer.
- **Zeichnen:** Teile in Listenreihenfolge mit Ebitengine, Umhang und Beine als Dreiecksband, vier Spielerfarben.
- **Flüssig:** Die Pose wird je Bild gerechnet (nicht 8 Frames je Zyklus), der Umhang folgt der echten Geschwindigkeit.
- **Vergleich:** Rig und Sheet nebeneinander, Messbericht wie B-777 (FPS, GPU-Speicher, RAM), Abweichung je Bild gegen das Sheet.

## Nicht-Ziele

Gegner, Truppen und Bosse als Rig (B-780); weitere Reittiere außer dem Pferd; fertige Waffen- und Rüstungs-Grafik (nur Platzhalter-Teile für den Prototyp); Simulation, HUD, Netz; Änderungen an `tools/paper`, `gen:paper`, `src/`, `engine/`; Entfernen von Sheets aus dem Spiel.

## Regeln und Einschränkungen

Entscheidung 005 (Papierschnitt aus Specs) und 008 gelten. Dateien nur unter `native/`; Befehle nur über `task native:*`. Kein SVG und kein Vektor-Rendering zur Laufzeit (B-752), nur PNG-Teile. Wurzel-`go.mod` bleibt unberührt, kein cgo, kein `math/rand`. Logging mit Emojis, Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Die heutigen Sheets sind der Schiedsrichter für die Optik.

## Beispiele

`task native:rig` öffnet ein Fenster: links der Monarch aus dem Sheet, rechts aus Rig-Teilen, beide laufen; Tasten wechseln Animation, Spielerfarbe und Reiten. `task native:rig:measure` schreibt `reports/native-rig-<rechner>-<zeit>.json`.

## Ausnahme- und Fehlerfälle

Teil-PNG oder Rig-Datei fehlt (Export nicht gelaufen) → Abbruch mit klarer Meldung, kein leeres Bild. Animation ohne Pose und ohne Keyframes → Meldung wie in `tools/paper` (`animation-unbekannt`).

## Akzeptanzkriterien

- **AC-01** `task native:rig:export` schreibt Teil-PNGs (Monarch je Spielerfarbe, Pferd) und je Figur eine Rig-Datei mit den Plätzen `waffe`, `ruestung`, `kopfbedeckung`, `sattel`; zweimal ausgeführt entstehen byte-gleiche Dateien. Sie sind eingecheckt, `task native:rig` läuft auf einem frischen Checkout ohne Gen-Lauf.
- **AC-02** Die Rig-Rechnung in Go liefert für Monarch (`idle`, `lauf`) und Pferd (`trab`) je Frame dieselben Teil-Matrizen und Stützpunkte wie `tools/paper` (Test gegen exportierte Sollwerte, Toleranz 0,01).
- **AC-03** Das Fenster zeigt Rig und Sheet nebeneinander: Monarch in `idle`, `lauf` und einer Keyframe-Animation, Monarch auf dem Rig-Pferd in Schritt, Trab und Galopp; der Umhang bewegt sich; Waffe und Rüstung lassen sich per Taste tauschen und abnehmen.
- **AC-04** Bildvergleich je Frame gegen das Sheet für `idle`, `lauf` und `trab` (Reiter mit Pferd): Abweichung als Zahl im Bericht, dazu ein Differenzbild; Schatten und Schärfe sind in den Notizen bewertet.
- **AC-05** Messbericht vom Entwicklungs-PC: FPS und Grafikspeicher mit 2, 50 und 200 Rig-Figuren (Reiter mit Pferd) gegen dieselbe Zahl aus Sheets; die Werte sind die erste Grundlage für B-780.
- **AC-06** Sichtprüfung und Messwerte vom Mini-PC stehen unter Notizen (Abnahme 🧑).

## Offene Fragen

- **Schatten:** Im Sheet hat der Schlagschatten je Lage eine feste Lichtrichtung; in einer Teil-Textur dreht er sich mit dem Teil. Default: Schatten ins Teil backen und die Abweichung messen; zweite Variante (Schatten-Textur je Teil mit festem Versatz) nur, wenn der Vergleich es verlangt.
- **Schärfe:** Gedrehte Bitmaps werden weicher. Default: Teile in doppelter Dichte rastern, linear filtern.
- **Beine des Pferds:** `beinpfad` zeichnet Ober- und Unterbein als ein gebogenes Papierstück, kein starres Teil. Default: Dreiecksband aus den Stützpunkten mit einer Bein-Textur, wie beim Umhang.
- **Farbvarianten:** Default eigene Teil-PNGs je Farbe (klein); Einfärben per Shader erst bei Bedarf.
- **Platzhalter:** Zweite Waffe und Rüstungs-Teil legt Claude nur für den Prototyp unter `native/` an (keine Spec in `public/grafik`).

## Notizen

Angelegt 2026-11-11 nach Ralfs Entscheidung (008). Analyse dazu: 12 Teile (oberarm-h, unterarm-h, oberschenkel-h, unterschenkel-h, schwert, oberschenkel-v, unterschenkel-v, rumpf, kopf, krone, oberarm-v, unterarm-v), Umhang 8 Segmente, zu portieren etwa 500 Zeilen TypeScript (Schätzung). Verwandt: B-752 (Ebenen aus Sheets).
