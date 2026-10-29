# B-665 · Im Modus Stufe bearbeiten stehen eine große Live-Vorschau, eine schmale Chunk-Vorschau und ein Inspektor statt vier Steuerzeilen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Über der Live-Vorschau stehen vier Steuerzeilen: Seed, Laden, Zufälliger Seed, Verkleinern (`ChunkPreview.tsx`); Hub-Stufe, Tageszeit, Fortschritt, Schichten (per Portal aus `stage/LivePreview.tsx` in `controlsHost`); Zoom mit acht Sprungknöpfen und Positions-Schieber (`ViewBar.tsx`, `RangeSlider.tsx`). Chunk-Vorschau (ca. 110 px), Positions-Schieber und Band (`LiveChunkBand.tsx`) zeigen denselben Ausschnitt. Die Live-Vorschau ist bei 100 % nur ca. 200 px Spielwelt hoch. Editor für geplante Chunks (`PlanEditor.tsx`) und Gebäude-Filter (`stage/BuildingFilter.tsx`, per Portal in `treeHost`) stehen weit unten; den Chunk-Platz tippt man als Zahl ein. Gemessen am 2026-10-29: 71 Knöpfe, 10 Eingabefelder, 8 Auswahllisten, Seite 1.421 px hoch bei 900 px Fenster.

## Ziel

Die Live-Vorschau ist der größte Teil der Seite, ihre Steuerung sitzt an ihr, und was man bearbeitet (Chunk, Gebäude, Ansicht), steht daneben im Inspektor.

## Beteiligte und Zielgruppen

- 🧑 Ralf plant Chunks und prüft Stufen im Studio.
- Sessions der Domäne DEV setzen um.

## Anforderungen

- Modus Stufe bearbeiten (B-664): links Chunk-Vorschau, Werkzeugleiste und Live-Vorschau, rechts der Inspektor (ca. 380 px); unter ca. 1100 px Seitenbreite steht der Inspektor darunter (B-663).
- Chunk-Vorschau als schmale Leiste: je Chunk Chunk-Platz und Chunk-Art, Marken für geplant, Prüffehler und Hub. Ein gelber Rahmen zeigt den Ausschnitt der Live-Vorschau; Ziehen am Rahmen oder Klick verschiebt ihn. Die Sprungpunkte (Hub, Mauer, Portale, Ausgang) stehen als kleine Marken über der Leiste. Das ersetzt Positions-Schieber und Sprungknopf-Reihe; die Legende wird eine kleine Zeile.
- Eine Werkzeugleiste direkt über der Live-Vorschau: Hub-Stufe und Tageszeit als Segment-Knöpfe, Fortschritt als Regler, Hintergrund-Ebenen als Menü, Zoom −/Wert/+, „Springe zu“ als Menü, Live-Vorschau auf ganze Fläche (Taste M wie heute).
- Seed, Laden und Zufälliger Seed wandern in den Kopf zur Auswahlzeile (B-664). „Verkleinern/Maximieren“ entfällt; die Live-Vorschau zeigt immer die volle Spielhöhe und füllt den freien Platz.
- Inspektor mit den Reitern Chunk, Gebäude, Ansicht (Glossar: Inspektor).
- Reiter Chunk: Ein Klick auf einen Chunk in Chunk-Vorschau oder Band wählt dessen Chunk-Platz und öffnet den Reiter. Ungeplant: Knopf „Chunk planen“ (Art aus `chunkWeights` des Bioms wie heute `defaultKind`). Geplant: Chunk-Art (`KindPicker`), Chunk-Seed, feste Objekte, Ruine, „In feste Objekte umwandeln“, „Planung entfernen“. Darunter die geplanten Chunks der Stufe als Chips, Prüffehler am Chip. Kein Eingabefeld „Neuer Platz“ mehr.
- Reiter Gebäude: zunächst der heutige Gebäude-Filter, bis B-666 ihn ersetzt.
- Reiter Ansicht: Hintergrund-Ebenen, Band über der Live-Vorschau ein/aus, Hinweis auf getrennten Zoom (Alt + Rad bleibt).
- Tasten (Pfeile, +/−, M; nicht in Feldern, `OWN_KEYS`) und Mausrad (`wheelAction`) wie heute. Gemeinsame Mitte und Zoom-Logik (`viewState.ts`, `viewHooks.ts`) bleiben, nur die Bedienelemente ändern sich.

## Nicht-Ziele

- Modi, Auswahlzeile und Speicher-Stelle (B-664).
- Neue Bedienung der Gebäude-Zustände (B-666).
- Klick auf Bauplätze in der Live-Vorschau (B-667).
- Objekte geplanter Chunks per Ziehen in der Live-Vorschau verschieben (bei Bedarf eigenes Ticket).

## Regeln und Einschränkungen

- Domäne DEV, nur `tools/k3c-studio/frontend/src/world/`, `src/stage/` (Bedienelemente, nicht die Phaser-Bühne), `styles/world.css` und Bausteine in `tools/webkit`.
- `LivePreview` hängt Steuerung und Gebäude-Filter heute per Portal ein (`controlsHost`, `treeHost`); die neuen Orte dürfen weiter Portale nutzen, wenn sonst die Phaser-Bühne umgebaut werden müsste.
- `LivePreview` dient auch der Seite Gebäude-Planer: Deren Darstellung und Tests bleiben unverändert.
- Radix nur über `@webkit`, Atomic Design; keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- Bestehende Tests (`viewState`, `ChunkJump`, `ChunkFocus`, `RangeSlider`, `LiveChunkBand`, `PlanEditor`, `ConvertButton`, `RuinField`, `WorldPage`) bleiben grün oder werden mit gleicher Aussage umgestellt.
- Baut auf B-663 und B-664 auf.

## Beispiele

- Klick auf den Chunk am Chunk-Platz +2 → die Live-Vorschau zentriert ihn, der Inspektor zeigt den Reiter Chunk mit „+2 · Ruine, geplant“.
- Klick auf den ungeplanten Chunk-Platz −3 → „Chunk planen“ → neuer Eintrag am Platz −3 mit der ersten Art aus `chunkWeights`.
- Fenster 1440 × 900, Chat offen → Live-Vorschau mindestens 400 px hoch, über der Chunk-Vorschau steht nur der Kopf.

## Ausnahme- und Fehlerfälle

- Hub-Chunks und Rand lassen sich nicht planen → „Chunk planen“ gesperrt, der Grund steht sichtbar im Inspektor (`rules/geplante-chunks.md`).
- Prüffehler an einem Chunk außerhalb des Ausschnitts → rote Marke in der Chunk-Vorschau und am Chip; Klick springt hin.
- Level lädt noch oder Laden scheitert → Meldung in der Live-Vorschau statt leerer Fläche (heute `note`).

## Akzeptanzkriterien

- **AC-01** Im Modus Stufe bearbeiten steht über der Chunk-Vorschau keine Steuerzeile; Hub-Stufe, Tageszeit, Ebenen, Zoom und „Springe zu“ stehen in einer Werkzeugleiste direkt über der Live-Vorschau (Vitest).
- **AC-02** Positions-Schieber und Sprungknopf-Reihe sind ersetzt: Ziehen und Klick in der Chunk-Vorschau verschieben den Ausschnitt, die Sprungpunkte stehen als Marken über der Leiste (`ChunkJump`-Test mit gleicher Aussage grün).
- **AC-03** Ein Klick auf einen Chunk in Chunk-Vorschau oder Band öffnet den Reiter Chunk mit diesem Chunk-Platz; es gibt kein Eingabefeld für den Platz mehr (Vitest).
- **AC-04** „Chunk planen“, Chunk-Art, Chunk-Seed, Objekte, Ruine, Umwandeln und „Planung entfernen“ ergeben nach dem Speichern dieselbe Welt-Datei wie die heutigen Bedienwege (`PlanEditor`-, `ConvertButton`-, `RuinField`-Tests mit gleicher Aussage grün).
- **AC-05** Browser-Pane (Port 5196, `?mock=1`), Fenster 1440 × 900, Chat offen: Die Live-Vorschau ist mindestens 400 px hoch und ohne Scrollen der Seite ganz sichtbar, der Inspektor steht rechts daneben.
- **AC-06** Pfeiltasten, +/−, M und Mausrad (Strg, Alt, Umschalt) wirken wie vorher (`viewState`-Tests grün, Tasten-Test in `WorldPage.test.tsx`).
- **AC-07** Manuelle Abnahme durch 🧑 im Studio: einen Chunk planen, umwandeln, speichern und „Im Spiel öffnen“.

## Offene Fragen

- Startet die Live-Vorschau mit „Einpassen“ (Spielhöhe füllt den freien Platz) statt mit 100 %? Default: Einpassen. Entscheidet 🧑.
- Objekte geplanter Chunks per Ziehen in der Live-Vorschau verschieben: eigenes Ticket? Default: ja, später. Entscheidet 🧑.

## Notizen

- Mock: https://claude.ai/artifact/8EESxobV9HwKvqudPmDgbi (Tafel „Modus Stufe bearbeiten“, Reiter Chunk und Ansicht klickbar).
- Glossar-Eintrag „Inspektor“ am 2026-10-29 angelegt.
