# B-670 · Der Gebäude-Planer zeigt Bühne, Matrix und Inspektor auf einen Blick, ohne dass Filter oder Regeltabelle die Fläche schlucken

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-30 🧑 Chat („specs sind alle freigegeben“)

## Ausgangslage

Seite Gebäude-Planer des Studios (`tools/k3c-studio/frontend/src/planner/`). Von oben nach unten: Quellzeile, Befunde, Regeltabellen-Diff als volle Tabelle (`RulesDiff`), Speichern, Matrix (`HubMatrixView`), Editor (`RowEditor`) und das immer offene „Platz hinzufügen“ (`AddSite`) in einem scrollenden Mittelteil (`.pl-middle`). Darunter die Live-Vorschau mit dem alten `BuildingFilter` unter der Bühne (`LivePreview` ohne `treeHost`).

Gemessen am 2026-10-30 im Browser-Pane (1600×1000, `?mock=1`): Zugeklappt hat der Mittelteil 367 px, die Matrix steht unter dem Diff und ist erst nach dem Scrollen zu sehen. Wird „Mauern“ aufgeklappt (24 Plätze, je zwei Auswahllisten), schrumpft der Mittelteil auf wenige Pixel und Matrix und Editor verschwinden. Ein Klick auf ein Gebäude in der Bühne wählt keine Zeile der Matrix. Der Welten Editor hat seit B-665 bis B-667 einen Inspektor (Reiter, Segment-Knöpfe, Klick in die Bühne), der Planer nutzt ihn nicht.

## Ziel

Bühne, Matrix und Bearbeiten sind ohne Scrollen gleichzeitig sichtbar. Der Planer bedient sich wie der Welten Editor: Wählen in Bühne, Matrix oder Filter-Chip zeigt überall dasselbe, bearbeitet wird in einer festen Inspektor-Spalte.

## Beteiligte und Zielgruppen

- 🧑 Ralf plant Bauplätze je Hub-Stufe im Studio.
- Sessions der Domäne DEV setzen um.

## Anforderungen

- **Werkzeugleiste in einer Zeile:** Quelle, Hub-Stufe als Segment-Knöpfe (1 … n), Tageszeit, Status-Chips „Prüfung“ (ok / n Befunde) und „Regeltabelle n“ (nur bei Abweichung), rechts Änderungsanzeige, „Verwerfen“, „Speichern“. Die Hub-Stufen-Spalten der Matrix und die Auswahl der Vorschau bleiben dieselbe Wahl.
- **Regeltabellen-Diff nur auf Klick:** Der Chip öffnet ein Panel mit Hinweis, Pfad, Task und der Diff-Tabelle; standardmäßig zu. Befunde der Prüfung genauso über ihren Chip.
- **Filter als Chip-Leiste über der Bühne:** je Art ein Chip mit Anzahl; Klick blendet die Art aus und ein, ▾ öffnet die Plätze der Art als kleine Chips. Keine Auswahllisten je Platz mehr. Gebäude-Zustände setzt man wie im Welten Editor am gewählten Platz (Bausteine aus B-666 wiederverwenden).
- **Matrix und Inspektor nebeneinander unter der Bühne:** Matrix links, Zellen als farbige Kästchen (ab Stufe kräftig, danach hell, vorher leer); Platz und Liste (`sites`, `islandSites`, Baugrund, Händler) klein unter dem Namen statt eigener Spalte „Herkunft“. Inspektor rechts als feste Spalte (etwa 210 px) mit kurzen Labels („ab Hub-Stufe“, „Platz (Units)“, „ab Tiefe“, „nur mit tieferer Stufe“), Vorschau-Zustand als Segment-Knöpfe, „Entfernen“.
- **Eine Auswahl für alles:** Klick auf ein Gebäude in der Bühne, eine Matrix-Zeile oder einen Platz-Chip wählt denselben Bauplatz; Rahmen in der Bühne und Markierung in der Matrix folgen (`onSite`, `Marks.selected`).
- **„+ Platz“ im Matrix-Kopf:** legt eine neue Zeile mit Art, Liste `sites`, Platz 0 und höchster Hub-Stufe an und wählt sie; Art und Liste ändert man im Inspektor. Das Formular `AddSite` entfällt.
- Die Bühne behält ihre Höhe aus `stageStyle`, die Seite selbst scrollt nicht; wenn nötig scrollen Matrix und Inspektor je für sich.

## Nicht-Ziele

- Änderungen am Backend (`hub.json`-Prüfung, Speichern, `worldPreview`).
- Neue Felder in `data/hub.json`.
- Umbau des Welten Editors über gemeinsame Bausteine hinaus.

## Regeln und Einschränkungen

- Domäne DEV: `tools/k3c-studio/frontend/src/planner/`, `src/stage/`, `styles/planer.css`, Bausteine in `tools/webkit`.
- `LivePreview` ist mit dem Welten Editor geteilt: dessen Tests bleiben grün, der Planer bindet `treeHost` und `onPickSite` an oder bekommt eine eigene Chip-Leiste aus denselben Bausteinen.
- Radix nur über `@webkit`, Atomic Design (`tests/atomicDesign.test.ts`); keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- Baut auf B-665 bis B-667 auf.

## Beispiele

- Seite öffnen → Werkzeugleiste, Chip-Leiste, Bühne, darunter Matrix und Inspektor („Bauplatz wählen“) ohne Scrollen bei 1600×1000.
- Klick auf die Taverne in der Bühne → Zeile Taverne in der Matrix markiert, Inspektor zeigt ihre Felder.
- Chip „Mauern 24“ anklicken → alle Mauern ausgeblendet, Chip durchgestrichen; erneuter Klick blendet sie ein.
- Platz im Inspektor ändern → „● geändert“ in der Werkzeugleiste, Prüfung läuft, „Speichern“ wird möglich.

## Ausnahme- und Fehlerfälle

- Regeltabelle stimmt → kein Chip „Regeltabelle“.
- Prüfung nicht erreichbar oder Befunde → Chip rot mit Anzahl, „Speichern“ gesperrt mit Grund als Tooltip.
- Datei extern geändert → Hinweis mit „Neu laden“ wie heute.
- Zeile ohne Platz (Baugrund, Händler) → Inspektor zeigt deren Felder (`zone1WallUnits`, `buyOffsetUnits` …) wie heute `RowFields`.

## Akzeptanzkriterien

- **AC-01** Browser-Pane (Port 5196, `?mock=1`, 1600×1000): Bühne, Matrix-Kopf und Inspektor sind ohne Scrollen sichtbar, auch mit aufgeklappter Art „Mauern“.
- **AC-02** Regeltabellen-Diff und Befundliste erscheinen erst nach Klick auf ihren Chip (Vitest).
- **AC-03** Je Bauplatz gibt es im Planer keine Auswahllisten mehr; Ausblenden geht über die Chips (Vitest).
- **AC-04** Klick in der Bühne, auf eine Matrix-Zeile oder einen Platz-Chip wählt denselben Bauplatz; Matrix und Bühne markieren ihn (Vitest mit Bühnen-Attrappe).
- **AC-05** „+ Platz“ legt eine Zeile an und wählt sie; das bisherige Formular „Platz hinzufügen“ gibt es nicht mehr (Vitest).
- **AC-06** Bestehende Tests von Planer und Welten Editor sind grün oder an die neue Bedienung angepasst, ohne Verlust an geprüftem Verhalten.

## Offene Fragen

- Inspektor rechts neben der Matrix oder rechts neben der Bühne über die ganze Höhe? Default: neben der Matrix (Mock). Entscheidet 🧑.
- Bleibt `BuildingFilter` danach noch irgendwo in Gebrauch? Default: nein, er wird entfernt, wenn der Planer die neuen Bausteine nutzt. Entscheidet die umsetzende Session nach Prüfung.

## Notizen

- Mock aus dem Chat vom 2026-10-30 (Widget „studio_planer_monarch_gegner_mock“, Reiter Gebäude-Planer).
- Geschwister-Tickets: B-671 (Monarch), B-672 (Gegner); gemeinsames Muster Werkzeugleiste + Bühne + Inspektor.
- **Umgesetzt 2026-10-30:** `planner/PlannerPage.tsx`, `PlannerToolbar.tsx` (neu), `PlannerEditor.tsx` (Inspektor), `HubMatrixView.tsx`, `plannerLogic.ts`, `plannerEdit.ts`, `stage/BuildingChips.tsx` (neu), `stage/LivePreview.tsx`, `stage/stageLogic.ts`, `styles/planer.css`; `BuildingFilter` entfernt.
- AC-01 Browser-Pane 5196 `?mock=1` 1600×1000: Seite scrollt nicht, auch mit aufgeklappten Mauern. AC-02 bis AC-05 Vitest (`PlannerPage.test.tsx`, `PlannerSelect.test.tsx`, `LivePreview.test.tsx`). AC-06 alle Studio-Tests grün, `check:diff` grün.
- Entschieden nach Default: Inspektor neben der Matrix; `BuildingFilter` entfernt. Abweichungen: Ausblenden nur je Art (nicht je Platz); Hintergrund-Schichten im Inspektor; Klick auf ein Gebäude ohne Zeile hebt die Auswahl auf. Im Mock fehlt nach „+ Platz“ die neue Zeile, weil `mockHubCheck` den Kandidaten ignoriert (nur Mock).
- Offen: Abnahme durch 🧑 im echten Studio.
