# B-752 · Monarch und Reittier getrennt rendern und im Client zusammensetzen

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-08 🧑 Chat ("ok, dann umsetzen")

## Ausgangslage

`figuren/monarch` rendert je Kombination aus Reittier, Gangart und Spielerfarbe ein eigenes Sheet: 39 Animationen mal 5 Varianten = 195 Ausgaben, 67 MB PNG, gut 2 Minuten Renderzeit (`tools/paper/rig/reiten.ts`, `public/grafik/figuren/monarch.paper.json`). Neue Monarchen mit anderen Kopfbedeckungen vervielfachen das weiter (Reittiere x Gangarten x Monarche x Kopfbedeckungen x Farben).

## Ziel

Monarch, Reittier und Kopfbedeckung werden einzeln gerendert und im Client zur gewünschten Kombination zusammengesetzt; die Anzahl der Sheets wächst additiv statt multiplikativ.

## Beteiligte und Zielgruppen

Entwickler (🧑); Spieler sehen ihren Monarchen mit gewähltem Reittier und Kopfbedeckung.

## Anforderungen

- Reittier-Sheets je Gangart (gibt es schon unter `reittiere/`), getrennt in hintere und vordere Ebene, wo der Reiter davor oder dahinter sitzt.
- Reiter-Sheets je Gangart, bezogen auf einen Sattel-Anker; das Manifest nennt je Frame Anker (Sattel, Kopf) mit Position und Drehung.
- Kopfbedeckungen als eigene kleine Sheets am Kopf-Anker.
- Der Client setzt die Kombination eines Spielers beim Beitritt in eine Canvas-Textur zusammen (nur benutzte Kombinationen) und nutzt sie wie heute.
- Spielerfarben bleiben als Varianten des Reiters (Palettentausch `tausch`), nicht des Reittiers.
- Deterministisch, Goldens für die Teile.

## Nicht-Ziele

Kein SVG im Spiel (Entscheidung 🧑 2026-11-08: Variante 1, PNG-Teile im Client zusammensetzen, reicht); kein Live-Vektor-Rendering; keine neuen Monarchen oder Kopfbedeckungen selbst (nur die Struktur).

## Regeln und Einschränkungen

Entscheidung Umhang (🧑 2026-11-08): Der Umhang gehört zum Monarchen (Reiter-Ebene), nicht zum Reittier. Er wird mit dem Reiter gerendert und liegt über dem Reittier. Sonst: Entscheidung 005 (Papierschnitt-Grafik); Specs, Manifest-Vertrag und Goldens unter `tools/paper`; Xbox-Edge als Zielplattform (Ladezeit, Speicher).

## Beispiele

Reittier Schimmel, Reiter Standard, Krone A, Farbe Spieler 2 → Client setzt `schimmel-trab` (hinten), `reiter-trab@spieler2`, `krone-a`, `schimmel-trab` (vorn) zu einem Sheet zusammen.

## Ausnahme- und Fehlerfälle

Fehlt ein Teil oder ein Anker: Fallback auf Standard-Reittier ohne Kopfbedeckung, Log mit 🤒.

## Akzeptanzkriterien

- **AC-01** Die Sheet-Anzahl für Monarch plus Reittiere sinkt gegenüber den 195 Sheets bei gleicher Optik (Vergleich Pixel/Auge je Gangart).
- **AC-02** Eine neue Kopfbedeckung kostet nur ihr eigenes Sheet, kein Neurendern der Reittiere.
- **AC-03** Die Zusammensetzung dauert für einen Spieler auf der Xbox unter einer Sekunde (Messung am Gerät).

## Offene Fragen

keine

## Notizen

SVG im Spiel verworfen (🧑 2026-11-08). Zur Information die Messung: Streifen `trab-schimmel` (8 Frames) rastert im Browser in 190 ms (PNG 437 KB, SVG 190 KB), resvg 734 ms; Xbox nicht gemessen.

Verworfen 2026-11-12 (Entscheidung Ralf, Einordnung Claude): durch Entscheidung 008 und 009 überholt – das native Spiel setzt den Monarchen als Rig aus Teilen zur Laufzeit zusammen (NV2, NV3), Xbox-Edge als Ziel entfällt. Stufe 1 (Ebenen für Monarch und Reittier) ist mit #522 und #525 umgesetzt.
