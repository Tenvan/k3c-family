# B-666 · Gebäude-Zustände der Live-Vorschau lassen sich je Art, per Voreinstellung und je Bauplatz schnell setzen

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

`tools/k3c-studio/frontend/src/stage/BuildingFilter.tsx` gruppiert die Bauplätze nach Art, mit Haken „alle ein/aus“ und aufklappbarer Liste. Je Bauplatz gibt es zwei Auswahllisten (`SiteLookControls`: Baustufe Auto/1–n, Zustand Auto/100/75/50/25/10 %/Nicht gebaut); der Bauplatz heißt nur nach seiner x-Stelle. Mauern aufgeklappt sind 24 Bauplätze mit 48 Auswahllisten, die Seite wächst von 1.421 auf 2.386 px (gemessen 2026-10-29). Es gibt keine Sammel-Aktion je Art oder für alle, keine Markierung in der Live-Vorschau, und nichts sagt, dass die Zustände nicht gespeichert werden. Die Zustände gelten je Kontext (`usePreviewWorld` in `LivePreview.tsx`: Seed, Biom, Hub-Stufe, Planer-Stand).

## Ziel

Typische Prüfbilder (alles ausgebaut, nach einem Angriff, nichts gebaut) gehen mit einem Klick, einzelne Bauplätze lassen sich schnell ändern, und es ist klar, dass das nur Vorschau ist.

## Beteiligte und Zielgruppen

- 🧑 Ralf prüft Gebäude-Grafik und Zerstörungsbilder im Studio.
- Sessions der Domäne DEV setzen um.

## Anforderungen

- Die Bedienung steht im Reiter Gebäude des Inspektors (B-665) und ersetzt dort den heutigen Gebäude-Filter.
- Hinweis oben: „Nur Vorschau: Zustände werden nicht gespeichert“ mit „Zurücksetzen“.
- Voreinstellungen für alle Bauplätze mit einem Klick: „Wie Hub-Stufe“ (alles Auto, heutiger Start), „Voll ausgebaut“ (höchste Baustufe je Art, 100 %), „Nach Angriff“ (gemischte Trefferpunkte, fest aus der x-Stelle abgeleitet, ohne Zufall), „Nichts gebaut“.
- Je Art eine Zeile: sichtbar (Haken, gemischt wie heute), Baustufe und Zustand für alle Bauplätze der Art, „gemischt“ bei abweichenden Werten, Aufklappen.
- Aufgeklappt: Bauplätze als kleine Chips mit x-Stelle und Balken der Trefferpunkte (gestrichelt = nicht gebaut). Klick wählt den Bauplatz und zentriert die Live-Vorschau auf ihn.
- Gewählter Bauplatz: Karte mit Art und x-Stelle, Baustufe und Zustand als Segment-Knöpfe statt Auswahllisten, Knopf „Auf alle <Art> anwenden“. In der Live-Vorschau ist er mit einem Rahmen markiert (`Marks.selected` wie beim Gebäude-Planer).
- Ein Wechsel von Seed, Biom, Hub-Stufe oder Planer-Stand leert die Zustände wie heute.
- Backend unverändert: `worldPreview` mit `off` und `looks` (`SiteLook`, Glossar: Gebäude-Zustand).

## Nicht-Ziele

- Gebäude-Zustände speichern.
- Klick auf einen Bauplatz in der Live-Vorschau (B-667).
- Änderungen an der Seite Gebäude-Planer über das gemeinsame Bauteil hinaus.

## Regeln und Einschränkungen

- Domäne DEV, nur `tools/k3c-studio/frontend/src/stage/` (Bedienelemente und `stageLogic.ts`), `src/world/` und Bausteine in `tools/webkit`.
- Determinismus: „Nach Angriff“ ohne `Math.random()`, aus der x-Stelle abgeleitet; die Logik steht rein in `stageLogic.ts`.
- `BuildingFilter` steckt in `LivePreview` und erscheint auch auf der Seite Gebäude-Planer: Die neue Bedienung gilt dort mit, ohne deren Tests zu brechen.
- Radix nur über `@webkit`, Atomic Design; keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- Baut auf B-665 (Inspektor) auf.

## Beispiele

- Klick „Voll ausgebaut“ → alle Bauplätze in höchster Baustufe mit 100 %, die Zeilen zeigen die Stufe und „100 %“.
- Mauer bei x 215 gewählt, Zustand 50 → die Live-Vorschau zeigt sie beschädigt, die Zeile Mauern zeigt „gemischt“. „Auf alle Mauern anwenden“ → alle Mauern 50 %, die Zeile zeigt „50 %“.
- Klick „Nach Angriff“ zweimal mit demselben Level → beide Male dasselbe Bild.

## Ausnahme- und Fehlerfälle

- Art ohne Baustufen (`levels` leer in `data/buildings.json`) → nur Zustand wählbar.
- „Nicht gebaut“ gewählt → Baustufe gesperrt (wie heute).
- Ausgeblendeter Bauplatz → keine Zustands-Knöpfe, Chip abgeblendet.
- Antwort der Vorschau veraltet → wird verworfen (wie heute über den Schlüssel in `usePreviewWorld`).

## Akzeptanzkriterien

- **AC-01** Je Bauplatz gibt es keine Auswahllisten mehr; Baustufe und Zustand stellt man am gewählten Bauplatz mit Segment-Knöpfen ein (Vitest).
- **AC-02** Die vier Voreinstellungen setzen `looks` und `off` für alle Bauplätze wie beschrieben; „Nach Angriff“ ergibt bei gleichem Level dieselben Werte (Vitest auf `stageLogic.ts`).
- **AC-03** Baustufe und Zustand je Art wirken auf alle Bauplätze der Art; abweichende Werte zeigen „gemischt“ (Vitest).
- **AC-04** Klick auf einen Bauplatz-Chip wählt ihn, zentriert die Live-Vorschau und markiert ihn dort (Vitest mit Bühnen-Attrappe: `focusX`, `setMarks`).
- **AC-05** Browser-Pane (Port 5196, `?mock=1`): Mauern aufgeklappt (24 Bauplätze) macht den Reiter Gebäude höchstens 300 px höher als zugeklappt; die Seite selbst wird nicht höher, der Inspektor scrollt in sich.
- **AC-06** Hinweis „Nur Vorschau“ und „Zurücksetzen“ sind sichtbar; Zurücksetzen leert `off` und `looks` (Vitest).
- **AC-07** Seite Gebäude-Planer: bestehende Tests grün, der Gebäude-Filter funktioniert dort weiter.

## Offene Fragen

- Bleiben die Zustände beim Wechsel der Hub-Stufe erhalten (heute geleert)? Default: wie heute geleert. Entscheidet 🧑.
- Muster von „Nach Angriff“ (z. B. 100/50/25/75 % reihum nach x-Stelle)? Default: dieses Muster. Entscheidet 🧑.

## Notizen

- Mock: https://claude.ai/artifact/8EESxobV9HwKvqudPmDgbi (Tafel „Modus Stufe bearbeiten“, Reiter Gebäude klickbar).
- Glossar-Eintrag „Gebäude-Zustand (Vorschau)“ am 2026-10-29 angelegt.
