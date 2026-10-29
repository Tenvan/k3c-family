# B-658 · Die Balancing-Seite stellt Konfiguration und Berichte unter 1.100 px untereinander

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ32
- **Projekt:** WZG
- **Erstellt:** 2026-10-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-29 🧑 Chat (Revision 1, offene Fragen mit Default von Claude)

## Ausgangslage

`tools/k3c-workbench/frontend/src/styles/balancing.css` setzt `container-type: inline-size` auf `.bal-page` und will mit `@container (max-width: 1000px) { .bal-page { grid-template-columns: 1fr; } }` umbrechen. Eine Container Query fragt aber immer einen **Vorfahren** ab, nie das Element selbst, und kein Vorfahr von `.bal-page` ist ein Container. Die Regel greift also nie.

Gemessen am 2026-10-29 in der laufenden Workbench (Port 5191): Bei 947 px Seitenbreite stehen weiter zwei Spalten (373 + 560 px), bei 754 px Fensterbreite zwei Spalten mit 300 + 420 px und waagrechtem Überlauf (`scrollWidth` 754 bei 375 px Gerätebreite).

## Ziel

Die Balancing-Seite bricht in schmalen Fenstern auf eine Spalte um und läuft nie waagrecht über, wie es die gemeinsame Spec der Workbench-Seiten verlangt.

## Beteiligte und Zielgruppen

Ralf beim Balancing in einem halben Fenster oder im Browser (Web-Client, B-614); Agent DEV.

## Anforderungen

- Unter **1.100 px** Breite der Inhaltsfläche stehen Konfiguration und Berichte untereinander (Grenze aus `docs/standards/workbench-seiten.md` › Gemeinsame Regeln › Zwei Spalten, heute 1.000 px).
- Die Container Query fragt einen Vorfahren ab (eigener Wrapper mit `container-type` oder `container-type` am Eltern-Element), nicht `.bal-page` selbst.
- Keine Spalte erzwingt eine Mindestbreite, die über die Fensterbreite hinausgeht (`minmax(300px, …)` / `minmax(420px, …)` greifen nur im Zwei-Spalten-Raster).
- Andere Seiten der Workbench mit demselben Muster werden mitgeprüft und, falls betroffen, im selben Zug repariert.

## Nicht-Ziele

Neuer Seitenrahmen mit festen Köpfen und eigenen Scroll-Bereichen (B-659), neue Liste und Detail-Reiter (B-660), Lauf-Editor (B-661).

## Regeln und Einschränkungen

- Gemeinsame Spec der Workbench-Seiten (Grenze 1.100 px, Container Query, wo die Seite eine eigene Fläche hat).
- Atomic Design der Wails-Frontends (`tests/atomicDesign.test.ts`); nur CSS und ggf. ein Wrapper-Element in `BalancingPage.tsx`.
- Komplexitäts-Budget: Datei ≤ 400 Zeilen.

## Beispiele

- Fenster 1.400 px → zwei Spalten wie heute.
- Fenster 1.000 px (halber Bildschirm) → eine Spalte: Konfiguration oben, Berichte darunter, kein waagrechter Scrollbalken.
- Fenster 600 px → eine Spalte, Eingabefelder füllen die Breite.

## Ausnahme- und Fehlerfälle

Browser ohne Container Queries: nicht relevant, Wails (WebView2) und aktuelle Browser können sie.

## Akzeptanzkriterien

- **AC-01** Vitest liest `styles/balancing.css` und prüft: Jede `@container`-Regel zielt auf ein Element **innerhalb** des Elements, das `container-type` setzt (nicht auf denselben Selektor); die Grenze ist 1.100 px.
- **AC-02** Beobachtung im Browser-Pane mit `?mock=1`: bei 1.000 px Fensterbreite eine Spalte, bei 600 px `document.documentElement.scrollWidth` = `innerWidth` (kein Überlauf).

## Offene Fragen

keine

## Notizen

- Befund und Mock: Design-Canvas „Balancing-Seite Mock“ (2026-10-29), Board „Befund: Ist-Zustand“.
- Schneller Fix möglich, unabhängig vom Umbau der Seite (B-659).
