# B-895 · Die Oberfläche der Workbench wächst nicht unbegrenzt im Speicher und bleibt nach Stunden mit parallelen Prüfläufen bedienbar

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** WZ60
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 („sollte zuerst gefixt werden“)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

2026-11-14, 16:12: Die Workbench-EXE läuft (Backend antwortet, MCP-Aufrufe ok), das Fenster zeigt aber nichts mehr. `workbench_resources`: Renderer `msedgewebview2.exe` 1,29 GiB, Trend Kinder-RSS +1,16 GiB/h über 40 min; Backend nur 197 MiB RSS. Gleichzeitig liefen zwei Agenten mit 11 `check_run`-Läufen (19,5 GiB Allokation, Prüflauf-Text 313 KiB) und vielen Planungs-Aufrufen. Vermutet (ungeprüft): Die Oberfläche hält Live-Daten ohne Grenze (MCP-Seite: Live-Statistik und check_run-Auswertung aus B-862, Konsole/Logs, Prüflauf-Ausgaben) oder Abos werden nicht abgemeldet.

## Ziel

Die Oberfläche bleibt nach mehreren Stunden mit parallelen Prüfläufen unter einer festen Speichergrenze und bedienbar.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Ursache mit Heap-Snapshot des WebView (DevTools im `-dev`-Modus unter Vite 5191) eingrenzen.
- Alle Live-Puffer im Frontend begrenzen (Ringpuffer, z. B. letzte N Aufrufe/Zeilen/Läufe), Abos beim Seitenwechsel abmelden.
- `workbench_resources` meldet den Renderer-RSS getrennt und warnt (🤒) ab einer Grenze.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Oberflächen nach Atomic Design über `@webkit`.

- **Bereich:** `tools/k3c-workbench/frontend/src/**`, `tools/k3c-workbench/internal/mcpsrv/resources.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ein Test simuliert 10 000 Live-Ereignisse und prüft, dass die Puffer begrenzt bleiben.
- **AC-02** Nach 1 h mit zwei parallelen check_run-Schleifen bleibt der Renderer unter 500 MiB (Messung, Nachweis im Ergebnis).
- **AC-03** Überschreitet der Renderer die Grenze, erscheint ein 🤒-Eintrag im Log.

## Offene Fragen

- Grenze 500 MiB? Default: ja.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
