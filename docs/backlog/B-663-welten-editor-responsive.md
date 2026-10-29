# B-663 · Der Welten Editor bricht nach der Breite seiner Seite um und zeigt keine waagrechte Scrollleiste

- **Domäne:** DEV
- **Typ:** Problem
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

Der Welten Editor bricht per `@media (max-width: 900px)` nach der **Fensterbreite** um (`tools/k3c-studio/frontend/src/styles/world.css`, `.wd-columns`; ebenso `.wd-monarch`). Die Seite teilt sich das Fenster aber mit dem Chat (ca. 380 px). Gemessen am 2026-10-29 (Studio mit `?mock=1`, Chat offen): Bei 1024 px Fenster bleiben der Seite 609 px, das Raster `.wd-columns` braucht `minmax(18rem, 1fr) minmax(24rem, 1.4fr)` plus 16 px Lücke = 688 px. Ergebnis: waagrechte Scrollleiste auf `.wd-page`, `.rt-TabsContent` und `.wk-page`.

## Ziel

Bei jeder Fensterbreite ab 800 px, mit und ohne Chat, hat der Welten Editor keine waagrechte Scrollleiste. Die Spalten stapeln sich nach der Breite der Seite, nicht nach der des Fensters.

## Beteiligte und Zielgruppen

- 🧑 Ralf bedient das Studio am PC, der Chat ist meist offen.
- Sessions der Domäne DEV setzen um.

## Anforderungen

- Umbruch per Container Query auf dem Seitencontainer (`container-type: inline-size`), nicht per Viewport-Media-Query.
- Spalten mit `minmax(0, …)`; was nicht passt, bricht um (`flex-wrap`) oder wird gekürzt (Ellipsis). Keine feste Mindestbreite über der Seitenbreite.
- Ab ca. 1100 px Seitenbreite zwei Spalten, darunter eine; der Wert steht an einer Stelle in `world.css`.
- Die Live-Vorschau behält die volle Spielhöhe (`stagePx`), wird aber nie breiter als die Seite.
- Gilt auch für die anderen Studio-Seiten, die `world.css` nutzen (Monarch `.wd-monarch`, Gegner).

## Nicht-Ziele

- Neues Layout mit zwei Modi und Inspektor (B-664, B-665).
- Bedienung des Studios per Touch oder am Handy.

## Regeln und Einschränkungen

- Domäne DEV, nur `tools/k3c-studio/frontend/` (CSS, bei Bedarf eine Wrapper-Klasse).
- Oberflächen der Wails-Programme: Radix nur über `@webkit`, Atomic Design (`tests/atomicDesign.test.ts`).
- Keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- jsdom kennt kein Layout: Prüfung per Test auf die CSS-Regeln plus Messung im Browser-Pane.
- Browser-Tests des Studios nur auf dem Test-Port 5196 (launch.json `k3c-studio-ui`), nie 5192 oder 8091.

## Beispiele

- Fenster 1024 × 768, Chat offen → Seite 609 px breit, Weltstruktur und Planung untereinander, keine waagrechte Scrollleiste.
- Fenster 1920 × 1080, Chat zu → zwei Spalten nebeneinander.

## Ausnahme- und Fehlerfälle

- Chat ein- oder ausblenden → der Umbruch passt sich ohne Neuladen an.
- Seite schmaler als 500 px → Bedienleisten brechen um, nichts wird abgeschnitten.

## Akzeptanzkriterien

- **AC-01** In `tools/k3c-studio/frontend/src/styles/world.css` gibt es für `.wd-columns` und `.wd-monarch` keine `@media (max-width: …)`-Regel mehr; der Umbruch steht in `@container`-Regeln. Ein Vitest im Studio-Frontend liest die CSS-Datei und prüft das.
- **AC-02** Browser-Pane, Studio auf Port 5196 mit `?mock=1`, Fenster 1024 × 768, Chat offen: Auf dem Welten Editor gilt `scrollWidth == clientWidth` für `.wd-page` und `.rt-TabsContent`.
- **AC-03** Dieselbe Messung bei 1440 × 900 und 1920 × 1080, jeweils mit und ohne Chat: keine waagrechte Scrollleiste; ab 1100 px Seitenbreite stehen die Spalten nebeneinander.
- **AC-04** Chat ein- und ausblenden ändert den Umbruch ohne Neuladen (Browser-Pane).

## Offene Fragen

- Grenze für zwei Spalten bei 1100 px Seitenbreite? Default: 1100 px. Entscheidet 🧑.

## Notizen

- Messung 2026-10-29 im Browser-Pane (Mock-Daten, Chat offen).
- Mock: https://claude.ai/artifact/8EESxobV9HwKvqudPmDgbi (Tafeln „Schmale Breite“ und „Ist-Zustand“).
- Folge-Tickets B-664 bis B-667 bauen darauf auf; dieses Ticket behebt die Scrollleiste schon im heutigen Layout.
