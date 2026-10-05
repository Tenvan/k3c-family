# MON2.2 · Seite monitor.html mit Übersicht, Verlauf und Ereignissen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** mon2/2-seite
- **Abhängig von:** MON2.1
- **Tickets:** B-282
- **Kriterien:** AC-03, AC-04

## Ziel

`monitor.html` mit Kachel „Monitor“ auf der Landingpage zeigt Ampel je Raum, Verläufe mit Perzentilen und die
Fehler-Zeitleiste aus `/api/metrics`.

## Kontext

- Logik aus MON2.1: `src/tools/monitorData.ts`, `src/tools/monitorApi.ts`.
- Seitenregeln aus `CLAUDE.md`: `installPageChrome()`, Eintrag in `src/landing/pages.ts`, oben ca. 70 px frei, B nicht
  belegen, View + Menu reserviert. `tests/projectRules.test.ts` prüft Eintrag und Aufruf.
- Controller-Fokus wie `src/tools/testing.ts` (`nextFocus` aus `testTiles.ts`): Steuerkreuz/Stick wandert, A klickt.
- Token nur in `localStorage` (nie in der URL). Diagramme selbst auf Canvas, keine neue Abhängigkeit; neu gezeichnet nur
  nach einer Antwort oder Bedienung (CPU), Polling pausiert bei `document.hidden`.
- Fehlerfälle (B-282): 404 → „Diagnose aus, `K3C_STATUS_TOKEN` setzen“; 401 → Token-Eingabe; nicht erreichbar → Ampel
  grau, Backoff bis 30 s; Neustart → Markierung im Verlauf.

## Erlaubte Dateien

- `monitor.html`, `src/tools/monitor.ts`, `src/tools/monitorChart.ts`
- `src/tools/monitorData.ts`, `src/tools/monitorApi.ts` und ihre Tests (Ergänzungen)
- `src/landing/pages.ts`, `src/tools/codemap.md`
- Planungsdateien

## Nicht-Ziele

Neue Kennzahlen im Server, Raum-Steuerung (DBG3), Anzeige im Spiel. Abnahme am Handy (MON2.4).

## Schritte

1. `monitor.html` (einspaltig, responsiv) und Eintrag in `pages.ts`.
2. Ansichten Übersicht (Ampel, Tick p99, schlechteste RTT, Fehler 5 min), Verlauf (Tick, RTT je Gerät, Heap/CPU; Fenster
   5 min / 1 h; p50/p95/p99/Max, Budget-Überschreitungen, Ausreißer markiert, Neustart- und Ereignis-Marken),
   Ereignisse (Filter Raum und Art; Klick springt zur Stelle im Verlauf).
3. Token-Eingabe und Hinweise für 404, 401, offline.
4. Bedienung mit Maus, Touch und Controller.
5. Manuell, nur mit Freigabe 🧑 für den Lauf: Browser-Pane mit `task load` als Last (AC-04).

## Fertig, wenn

- [ ] AC-03: `monitor.html` in `pages.ts`, `installPageChrome()` aufgerufen, `task check` grün.
- [ ] AC-04: Browser-Pane-Screenshot unter Last mit Übersicht, Verlauf mit Perzentilen und Ereignis-Zeitleiste; 404 und 401
  zeigen den Hinweis (manuell, nur mit Freigabe).

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser-Pane mit `task load`, AC-04) nur, wenn 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
