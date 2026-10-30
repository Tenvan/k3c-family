# B-667 · Ein Klick auf einen Bauplatz in der Live-Vorschau wählt ihn im Inspektor

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-29 🧑 Chat („jetzt die Seite mit Hilfe der Tickets und Mocks umsetzen“)

## Ausgangslage

Die Live-Vorschau (`tools/k3c-studio/frontend/src/stage/PreviewStage.ts`, Phaser) kennt Ziehen (Pan), Mausrad und Markierungen (`setMarks`, `focusX`), meldet aber keinen Klick auf ein Gebäude. Bauplätze wählt man nur über die Liste im Inspektor (B-666).

## Ziel

Ralf klickt direkt auf das Gebäude, das er ändern will, statt es in einer Liste nach x-Stelle zu suchen.

## Beteiligte und Zielgruppen

- 🧑 Ralf prüft Gebäude in der Live-Vorschau.
- Sessions der Domäne DEV setzen um.

## Anforderungen

- Ein Klick ohne Ziehen auf einen Bauplatz in der Live-Vorschau wählt ihn im Reiter Gebäude des Inspektors und öffnet den Reiter.
- Umschalt + Klick erweitert die Auswahl, Doppelklick wählt alle Bauplätze der Art. Baustufe und Zustand im Inspektor wirken dann auf alle gewählten.
- Treffer über x-Stelle und Breite des Bauplatzes (`sites` der Vorschau-Welt), nicht über Pixel der Grafik. Ziehen ab einer kleinen Schwelle bleibt Pan.
- Hover zeigt Art und x-Stelle als kleinen Hinweis.
- Ein Klick auf freie Fläche hebt die Auswahl auf.

## Nicht-Ziele

- Objekte geplanter Chunks per Ziehen verschieben.
- Klick auf Gegner oder Figuren.

## Regeln und Einschränkungen

- Domäne DEV, nur `tools/k3c-studio/frontend/src/stage/` und `src/world/`.
- `PreviewStage` ist auch die Bühne der Seite Gebäude-Planer: Der neue Rückruf (z. B. `onSite`) ist optional, ohne ihn ändert sich dort nichts. Test-Attrappen der Bühne kennen ihn optional (wie heute `settle?`).
- Keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- Baut auf B-665 und B-666 auf.

## Beispiele

- Klick auf die Mauer bei x 215 → Reiter Gebäude, Karte „Mauer bei x 215“, Rahmen um die Mauer.
- Umschalt + Klick auf die Mauer bei x 239 → „2 gewählt“; Zustand 25 → beide Mauern 25 %.
- Doppelklick auf einen Turm → alle Türme gewählt.

## Ausnahme- und Fehlerfälle

- Zwei Bauplätze liegen eng beieinander → gewählt wird der, dessen Mitte der Klick-x-Stelle am nächsten liegt.
- Klick, während die Welt noch lädt → keine Wirkung.
- Ausgeblendeter Bauplatz → nicht anklickbar.

## Akzeptanzkriterien

- **AC-01** Eine reine Funktion `siteAt(sites, xUnits)` liefert den Bauplatz unter einer Welt-x-Stelle oder nichts (Vitest).
- **AC-02** Ein Klick ohne Ziehen meldet den Bauplatz an die Seite, Ziehen über der Schwelle bleibt Pan (Vitest auf der Logik der Schwelle).
- **AC-03** Umschalt + Klick erweitert die Auswahl, Doppelklick wählt alle Bauplätze der Art, Klick auf freie Fläche hebt sie auf (Vitest).
- **AC-04** Seite Gebäude-Planer: bestehende Tests grün, ohne `onSite` keine Änderung.
- **AC-05** Manuelle Abnahme durch 🧑 im Studio: drei Mauern anklicken und gemeinsam auf 25 % setzen.

## Offene Fragen

- Doppelklick = alle Bauplätze der Art? Default: ja. Entscheidet 🧑.

## Notizen

- Mock: https://claude.ai/artifact/8EESxobV9HwKvqudPmDgbi (Hinweiszeile unter der Live-Vorschau in „Modus Stufe bearbeiten“).
- Größter Brocken der Reihe, weil er in die Phaser-Bühne eingreift; deshalb eigenes Ticket mit Prio niedrig.
- **Umgesetzt 2026-10-29** (AC-01 bis AC-04 per Vitest, im Browser-Pane mit echter Bühne gesehen: Klick auf einen Turm wählt ihn, Rahmen und Karte erscheinen; AC-05 offen bei 🧑): `siteAt`, `pickKeys`, `CLICK_SLOP`, `DOUBLE_MS` in `stageLogic.ts`, `PreviewScene.click` (optionaler Rückruf `onSite`, ohne ihn unverändert). Treffer prüft x und y (Review), Klick nur nach Drücken auf der Bühne mit linker Taste.
- Nicht per Test abgedeckt: die Phaser-Szene selbst (`PreviewScene.click`), nur die reine Logik.
