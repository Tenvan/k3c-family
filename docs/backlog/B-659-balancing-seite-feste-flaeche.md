# B-659 · Die Balancing-Seite füllt die Fläche, nur Liste und Detail scrollen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ32
- **Projekt:** WZG
- **Erstellt:** 2026-10-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-29 🧑 Chat (Revision 1, offene Fragen mit Default von Claude)

## Ausgangslage

Die Balancing-Seite (`tools/k3c-workbench/frontend/src/balancing/BalancingPage.tsx`, `styles/balancing.css`) ist eine lange Seite, die als Ganzes scrollt. Gemessen am 2026-10-29 bei 1.000 px Fensterbreite mit offenem Bericht: Seite 7.059 px hoch, sichtbar 900 px. Links steht das Formular (1.086 px hoch), darunter erst die Läufe (ab 1.179 px). Rechts steht die Berichtsliste (1.100 px), darunter der Bericht (ab 1.193 px), die Diagramme ab 6.046 px. Nach „Lauf starten“ bleibt der Fortschritt außer Sicht; wer einen Bericht liest, sieht die Liste nicht mehr.

## Ziel

Die Seite hat den Aufbau der anderen Workbench-Seiten: feste Köpfe, Liste links, Detail rechts, nur diese beiden Bereiche scrollen. Laufende Läufe und der offene Bericht sind ohne Scrollen der ganzen Seite sichtbar.

## Beteiligte und Zielgruppen

Ralf beim Balancing am PC (Wails-Fenster, halbes Fenster, Web-Client nach B-614); Agent DEV.

## Anforderungen

- Die Seite füllt die Inhaltsfläche; die Seite selbst scrollt nie.
- Werkzeugleiste oben fest: „Neuer Lauf“, „Konfigurationsdateien“, Zustand der Warteschlange (Anzahl läuft / wartet).
- Darunter zwei Bereiche: Liste (links, etwa 380 px) und Detail (rechts, Rest); jeder scrollt für sich, Kopf und Reiter des Details bleiben stehen.
- Unter 1.100 px stehen Liste (oben, etwa 40 % der Höhe) und Detail (darunter) untereinander, beide scrollen für sich.
- Unter 760 px zeigt die Seite entweder die Liste oder das Detail: Ein Klick in der Liste öffnet das Detail, der Knopf „Liste“ führt zurück, die Auswahl bleibt.
- Die Breite der Liste ist (wie bei der Planungsseite, B-400) per Splitter änderbar und gemerkt (`localStorage` über den Helfer der Workbench).

## Nicht-Ziele

Inhalt der Liste und Reiter des Details (B-660), der Lauf-Editor (B-661), Änderungen der Go-Seite (B-662). Der schnelle Fix des Umbruchs ist B-658.

## Regeln und Einschränkungen

- `docs/standards/workbench-seiten.md` › Gemeinsame Regeln (Höhe, Zwei Spalten, Gemerkter Zustand).
- Atomic Design über `@webkit`; ein Seitenrahmen mit Liste und Detail gehört nach `tools/webkit/templates/`, falls Planung oder Tasks ihn schon so haben, sonst in die Seite.
- Komplexitäts-Budget: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

- 1.440 px: Liste links mit laufenden Läufen oben, Detail rechts mit dem gewählten Bericht; Diagramme im eigenen Reiter erreichbar, ohne die Seite zu scrollen.
- 960 px (halbes Fenster): Liste oben, Detail darunter, beide mit eigenem Scrollbalken.
- 600 px: Liste; Klick auf einen Bericht → Detail mit Knopf „Liste“.

## Ausnahme- und Fehlerfälle

- Kein Bericht gewählt → Detail zeigt einen Hinweis und „Neuer Lauf“.
- Sehr niedriges Fenster (unter 500 px Höhe) → Liste und Detail behalten je mindestens 160 px, die Seite darf dann scrollen.

## Akzeptanzkriterien

- **AC-01** Vitest: Die Seite rendert Werkzeugleiste, Liste und Detail als getrennte Bereiche; Liste und Detail haben eigene Scroll-Container (`overflow: auto`), die Seite selbst nicht.
- **AC-02** Vitest: Unter 760 px (Container-Breite gemockt) wechselt ein Klick in der Liste auf das Detail und „Liste“ zurück; die Auswahl bleibt.
- **AC-03** Beobachtung im Browser-Pane mit `?mock=1` bei 1.440, 960 und 600 px: Aufbau wie im Mock, kein waagrechter Überlauf, `document.documentElement.scrollHeight` = `innerHeight`.

## Offene Fragen

- Splitter für die Listenbreite: gleich wie B-400 (Planungsseite) oder feste Breite? Default (Claude): Splitter, falls B-400 ihn als Baustein liefert, sonst feste 380 px. Entscheidet 🧑.

## Notizen

- Mock: Design-Canvas „Balancing-Seite Mock“ (2026-10-29), Boards „Vorschlag: Läufe & Berichte“ und „Vorschlag: schmale Fenster“.
- Baut auf B-658 auf (Container Query).
