# tools/k3c-studio/frontend/src/assetpage/

## Responsibility
Seite Assets des Asset-Studios (B-677, WZ33): ersetzt die Seiten Grafik, Sound und Quellen. Eine Seite für Slots und Dateien beider Arten: Kopf mit Fokus-Chips und Suche, Facetten links, Sichten Slots, Dateien und Szene, rechts Inspektor oder Werkzeuge, unten eine Abspielleiste.

## Design
- `AssetsPage.tsx`: Seite; hält Auswahl, Werkzeug-Umschalter und Teilung (`useSplit`, gemerkt als `assets.split`); verbindet Daten, Filter und Sichten, meldet die Art an den Rahmen (`onArt`) und an den Chat.
- `PageHeader.tsx`: Fokus-Chips, Suche, Sicht, Gruppierung, Neu laden, Werkzeuge.
- `SlotsView.tsx`: Sicht Slots; Grafik als Raster von Karten, Sound als Liste mit ▶.
- `FilesView.tsx`: Sicht Dateien mit Stufe (Netz, Vorrat, Spiel, Präf. n), Lizenz-Marke und Größe; Netz-Dateien sind wählbar.
- `SceneView.tsx`: Sicht Szene; bindet `ScenesView` (Grafik) bzw. `SoundScenes` (Sound) ein.
- `Inspector.tsx`: Reiter Belegung | Kandidaten | Details (Reiter gemerkt als `assets.inspector`).
- `Belegung.tsx`: Präferenzen eines Slots mit Ziehen, ↑, ✕ und ▶; bei Grafik Ausschnitt (`CutEditor`) und Mini-Szene (`MiniScene`).
- `CandidatesTab.tsx`, `CandidateRow.tsx`: Kandidaten eines Slots (Zuordnungen, Spiel, Vorrat, Netz) mit Vorschau, Lizenz-Marke und „Zuordnen“; Netz-Dateien brauchen eine Bestätigung der Größe.
- `candidateLogic.ts`: reine Logik der Kandidaten (Stufe, Ton, Tooltips, Bestätigungstext, Fehlertext).
- `useCandidates.ts`: lädt Kandidaten bei Slot- und Suchwechsel und bei `assets:changed` neu.
- `ToolsPanel.tsx`: Werkzeuge statt Inspektor: Proben (`sound/Proben`), Recherche und Aufräumen (`sources/SourceDialogs`).
- `transport.ts`: Zustand der einen Abspielleiste; `playOn` (Zuordnung), `playFile` (Datei ohne Slot), `stopPlayback`, `changeVolume`.
- `usePageKeys.ts`: Tastatur der Seite (↑↓, Enter/Leertaste, X, S, +/−), nicht während der Eingabe in ein Feld.
- `model.ts`: `buildRows` macht aus Slot-Register, Vorschlägen und Katalog die Zeilen beider Sichten; `filter.ts`: Facetten (je Gruppe „oder“, Gruppen untereinander „und“), Gruppen und Sichten.
- `viewState.ts`: Ansicht (Facetten, Sicht, Gruppierung) gemerkt über `lib/prefs`; eigene Fokus-Chips getrennt; die Suche wird nicht gemerkt.
- `usePageModel.ts`: Chips mit Zählern, Facetten und sichtbare Gruppen beider Sichten.
- `pageLogic.ts`: Hilfen der Seite (Art der Auswahl, Schritt durch Slots, Vorschlags-IDs).
- `useAssetData.ts`: Slot-Register, Vorschläge und Katalog beider Arten zu den Zeilen.
- `useDownloads.ts`: Auswahl, Bestätigung und Download der Netz-Dateien („Nur laden“, „Laden und zuordnen“).
- `jump.ts`: Sprung zur Seite mit einem Slot (Ereignis `studio:goto-assets`) aus Planer, Monarch, Gegner, Chat und Ticket.
- Tests: `*.test.ts(x)` zu Filter, Ansicht, Modell, Kandidaten, Werkzeugen, Sprung und Seite.

## Flow
1. `useAssetData` lädt das Slot-Register (`assets/useAssets`), die Vorschläge (`sources/useSources`) und den Katalog beider Arten; `buildRows` macht daraus die Zeilen.
2. `usePageModel` leitet Chips, Facetten und Gruppen aus Zeilen und Ansicht ab; `filter.ts` filtert.
3. Die gewählte Sicht zeigt die Gruppen; die Auswahl setzt den Slot für Inspektor und Abspielleiste.
4. Inspektor → Reiter Kandidaten → `useCandidates` → RPC `AssetCandidates(slot, query, limit)`. „Zuordnen“ in `CandidateRow` ruft `AssetCandidateAssign(slot, id, stand, confirmedBytes)` mit dem `stand` aus der Antwort auf.
5. Belegung speichert über `assets.save` (`AssetsSet`). Eine Änderung von außen kommt als `assets:changed` und lädt neu.
6. ▶ → `transport.playOn` bzw. `playFile` → Audio-Kern (`sound/audio`). Verlassen der Seite stoppt alles (`stopAll`).

## Integration
- Eingehängt in `../App.tsx` (Seite `assets`, `AssetsPage` mit `onArt`).
- Backend: `tools/k3c-studio/backend/app_candidates.go` (RPC `AssetCandidates`, `AssetCandidateAssign`), Logik in `tools/assetkit/candidates.go`; Mock in `../api/mockCandidates.ts`.
- Abhängigkeiten: `../assets/` (Zuordnung, `useAssets`, `CutEditor`, `MiniScene`, `SlotInfo`), `../graphics/ScenesView`, `../sound/` (`audio`, `playback`, `Proben`, `SoundScenes`), `../sources/` (`useSources`, `SourceDialogs`, `sourcesLogic`), `../chat/` (`chatBridge`, `assignContext`), `../api` (`backend`, Typen), `../lib/prefs`, `@game/tools/soundtestLogic` (`stepVolume`), `@webkit/atoms|molecules|organisms`, `@webkit/lib/split`, `@webkit/lib/ticketContext`.
- Styles: `../styles/assetpage.css`, `assetInspector.css`, `assetTools.css`.
