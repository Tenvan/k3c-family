# B-684 · Die Ausschnitt-Bearbeitung zeigt wahlweise einen Rahmen und passt die Grafik in die Kachel ein

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GRJ
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31 autonom, Claude (Wunsch 🧑 im Chat)

## Ausgangslage

Im Asset-Studio zeigt die Bearbeitung des Ausschnitts einer Grafik heute nur den ausgeschnittenen Teil; die Grafik in der Kachel wird gescrollt statt eingepasst (`tools/k3c-studio/frontend/src`, Assets-Seite und Ausschnitt-Editor).

## Ziel

Der Ausschnitt lässt sich umschaltbar auf zwei Arten zeigen: als Rahmen über der ganzen Grafik oder wie bisher als ausgeschnittenes Bild. Die Grafik wird in der Kachel gestreckt oder geschrumpft (eingepasst), statt zu scrollen.

## Beteiligte und Zielgruppen

🧑 Ralf justiert Grafiken im Studio.

## Anforderungen

- Umschalter „Rahmen“ / „Ausschnitt“ in der Ausschnitt-Bearbeitung; Auswahl wird pro Viewer gemerkt (localStorage, in try/catch).
- Rahmen-Ansicht: ganze Grafik sichtbar, der Ausschnitt als Rahmen mit Maske außerhalb, bedienbar per Maus und Tastatur.
- Die Grafik passt sich der Kachelgröße an (Einpassen), auch bei Größenänderung des Splitters; kein Scrollen. Pixelgrafik bleibt scharf (kein Glätten).
- Radix nur über `@webkit`, Atomic Design, Datei ≤ 400 Zeilen.

## Nicht-Ziele

Zielmaß je Slot und Skalierung im Spiel (B-685).

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Oberflächen der Wails-Programme; Studio-Test-Port 5196.

## Beispiele

Grafik 128×128 in einer 96-px-Kachel → verkleinert dargestellt, Rahmen liegt auf dem Ausschnitt; Umschalten zeigt nur das Ausgeschnittene, ebenfalls eingepasst.

## Ausnahme- und Fehlerfälle

Grafik ohne Ausschnitt → Rahmen umfasst die ganze Grafik.

## Akzeptanzkriterien

- **AC-01** Vitest: Umschalter wechselt zwischen Rahmen- und Ausschnitt-Ansicht und merkt die Wahl.
- **AC-02** Vitest: Die Grafik wird eingepasst (Maßstab folgt der Kachelgröße), ohne Scrollbalken.
- **AC-03** Abnahme durch 🧑 im Studio.

## Offene Fragen

keine

## Notizen

- 2026-10-31 Claude: umgesetzt, Abnahme AC-03 offen. `assets/CutStage.tsx` (neu: Einpassen per `fitScale` und ResizeObserver, Rahmen mit Maske, Ziehen und Anfasser unten rechts, Pfeile verschieben, Shift+Pfeile ändern die Größe), Umschalter in `CutEditor.tsx` (Wahl gemerkt unter `k3c-studio.assets.cutview`, Standard Rahmen), Logik `fitScale`/`clampRect`/`nudgeRect` in `assets.ts`, Test `assets/CutEditor.test.tsx` (AC-01, AC-02). Kachelhöhe höchstens 360 px, Breite folgt dem Platz. Sichtprüfung im Browser-Pane mit Mock (Port 5196): Rahmen und Einpassen sichtbar; Ziehen mit der Maus dort nicht von Hand geprüft.
