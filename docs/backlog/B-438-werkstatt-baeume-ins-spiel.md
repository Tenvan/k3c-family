# B-438 · Werkstatt und Bäume aus tools/gebaeude erscheinen im Spiel

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** GEB6
- **Projekt:** GEB
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Sprint GEB1 hat Werkstatt, Tanne, Pappel und Birke mit `tools/gebaeude` erzeugt (`public/grafik/gebaeude/`, Regeln `docs/standards/gebaeude-grafik.md`). Ralf hat sie am 2026-10-16 in 32 und 16 px abgenommen und als Spiel-Dichte 32 px je Kachel ×1 festgelegt. Bisher erscheinen sie nur in der Asset-Vorschau; Slot `workshop` und `node:tree` in `data/assets.json` zeigen weiter die alten Packs.

## Ziel

Im Spiel zeigen die Bäume (`node:tree`) die Grafiken aus `tools/gebaeude` (Tanne, Pappel, Birke) als überlappende Reihe in 32 px je Kachel ×1; alte Baum-Grafik wird nach der Aufräum-Regel des Projekts gelöscht. Die Werkstatt ist erledigt (GEB2.3).

Eingeplant 2026-10-16 (Chat, Ralf) in GEB6 (Natur und Objekte), weil GEB6 die Bäume ohnehin behandelt.

## Beteiligte und Zielgruppen

Spieler sehen im Spiel Bäume im Stil der neuen Gebäude-Grafik; 🧑 entscheidet über die Spec und nimmt das Spiel ab; ein Agent (Domäne CLI) setzt es in Sprint GEB6 (Session GEB6.5) um.

## Anforderungen

- `node:tree` zeigt die Bäume aus `public/grafik/gebaeude/` (Tanne, Pappel, Birke und die Formen aus GEB6.1) in Maßstab 1 (32 px je Kachel); Art und Form wählt eine reine Funktion aus der Position des Baums (deterministisch, kein `Math.random()`), den Look das Biom.
- Mehrere Bäume dicht beieinander wirken als überlappende Reihe (Abstand ½ Kachel wie in der Vorschau), wo das Level sie so setzt; die Platzierung selbst bleibt unverändert.
- Plantage-Bäume bleiben die Pappel aus GEB5.
- Die alte Baum-Grafik (`gotthicvania-swamp/umgebung/trees.png`) wird nach der Aufräum-Regel des Projekts GEB gelöscht, sofern nichts anderes sie nutzt.
- Gilt mit mehreren Spielern und auf der Xbox gleich (nur Darstellung, keine Rechenlast im Client über das Auswählen hinaus).

## Nicht-Ziele

Werkstatt (erledigt in GEB2.3); Fels, Erz, Busch und die übrigen Objekte (GEB6, eigene Kriterien dort); neue Level-Regeln oder eine andere Platzierung der Bäume; Animationen; Änderungen an `engine/`.

## Regeln und Einschränkungen

Domäne CLI (`src/scenes/`, `data/assets.json`); Spiel-Dichte 32 px ×1 (`docs/standards/gebaeude-grafik.md` § 7); der Client würfelt nicht; Datei ≤ 400, Funktion ≤ 60 Zeilen; Reihenfolge: nach GEB5 (Plantage-Pappel) und nach den Baum-Formen aus GEB6.1.

## Beispiele

- Wald-Chunk mit fünf Bäumen → fünf Bäume, je nach Position Tanne, Pappel oder Birke in einer ihrer Formen, bei jedem Start gleich.
- Plantage mit Farm → die Bäume auf den Plätzen sind Pappeln (GEB5).

## Ausnahme- und Fehlerfälle

Grafik nicht geladen → bisherige Platzhalter-Form (Stamm und Dreieck); Biom ohne Look → Look des Waldes; markierter oder abgebauter Baum → Fahne und Balken sitzen über dem neuen, niedrigeren Bild.

## Akzeptanzkriterien

- **AC-01** Im Spiel zeigt `node:tree` Bäume aus `public/grafik/gebaeude/` in Maßstab 1; die Wahl von Art, Form und Look ist eine reine Funktion von Position und Biom mit Vitest (gleiche Eingabe → gleiche Wahl, Plantage-Bäume → Pappel); die alte Baum-Grafik ist gelöscht (Suche ohne Treffer außerhalb historischer Doku); `task check` grün.

## Offene Fragen

Keine eigenen; die Annahmen zu Baum-Formen und Biom-Looks stehen in GEB6 › Offene Fragen (2, 6) und gelten mit der Freigabe des Sprints.

## Notizen

2026-10-16 (Chat, Ralf): Die Werkstatt (`workshop`) wandert mit den vier Lücken-Gebäuden in Sprint GEB2 (Session GEB2.3, Maßstab 1 im Spiel). Dieses Ticket behält nur die Bäume (`node:tree`) und nutzt den Maßstab-Weg aus GEB2.3.
