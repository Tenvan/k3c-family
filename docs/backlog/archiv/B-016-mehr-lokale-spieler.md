# B-016 · Layout für mehr als zwei lokale Spieler ist entschieden

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** SP08
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-01 🧑 Chat („bei drei Spieler ein 2x1 Raster“, mit Auswahl: zwei oben, einer breit unten), Revision 3

## Ausgangslage

Heute gibt es höchstens 2 lokale Spieler im Split-Screen.

## Ziel

Layout für mehr als zwei lokale Spieler ist entschieden. Nutzen: Mit mehreren lokalen Spielern pro Gerät (Entscheidung 001) wird das nötig.

## Beteiligte und Zielgruppen

Spieler am TV (Edge auf der Xbox) und am Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Layout für 3–4 lokale Spieler an einem Gerät: flache Streifen, 2×2-Raster oder gemeinsame Kamera.

## Nicht-Ziele

Anzeige für Online-Spieler an anderen Geräten.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts; Seiten-Regeln aus `CLAUDE.md`; Taste B nicht belegen; 2 Spieler gleichzeitig (Split-Screen). Jeder lokale Spieler steuert seinen eigenen Monarchen (Entscheidung 001).

## Beispiele

3 Controller an der Xbox → jeder sieht seinen Monarchen.

## Ausnahme- und Fehlerfälle

Spieler weit auseinander → das Verhalten ist Teil der Entscheidung.

## Akzeptanzkriterien

- **AC-01** Die Layout-Entscheidung ist getroffen und festgehalten.
- **AC-02** SP08 setzt sie um.

## Offene Fragen

keine

## Notizen

Entscheidung 2026-10-01 🧑 (Chat, beim Bereitmachen von SP08): **2×2-Raster** für 3–4 lokale Spieler, eine Kamera je Spieler. Bei 1–2 Spielern bleibt es bei Vollbild bzw. Streifen. Revision 3 (2026-10-01 🧑 Chat): bei **3 Spielern zwei oben und einer breit unten** (statt 2×2 mit freiem Feld), bei 4 bleibt es beim 2×2-Raster. AC-01 ist damit getroffen, AC-02 setzt SP08 um.
