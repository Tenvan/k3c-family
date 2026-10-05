# B-282 · Die Monitoring-Seite zeichnet Verläufe, Perzentile und die Fehler-Zeitleiste aus /api/metrics

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** MON2
- **Erstellt:** 2026-10-05
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, mit Sprint MON2

## Ausgangslage

B-281 (Sprint MON1) liefert Verläufe und Ereignisse über `GET /api/metrics?since=…` (Token). Eine Seite zum Ansehen gibt es nicht; heute bleiben nur `/api/status`, die TUI und die JSONL-Logs.

## Ziel

Am PC oder Handy ist auf einen Blick zu sehen, wie es dem Server geht. Latenzspitzen, langsame Ticks und Abstürze lassen sich zeitlich zueinander einordnen.

## Beteiligte und Zielgruppen

🧑 als Betreiber am Spieleabend und als Entwickler bei der Fehlersuche; 🧑 nimmt am Handy ab.

## Anforderungen

- Eigene Seite `monitor.html` mit `installPageChrome()`, Eintrag in `src/landing/pages.ts`, Logik unter `src/tools/`.
- Token-Eingabe einmal pro Gerät (nur in `localStorage`, nie in der URL).
- Pollt `/api/metrics?since=…` alle 2–5 s und hängt nur Deltas an; pausiert bei `document.hidden`.
- Ansichten: Übersicht (Ampel je Raum: Tick p99, schlechteste RTT, Fehler der letzten 5 min), Verlauf (Tick-Dauer, RTT je Gerät, Heap/CPU; Fenster 5 min / 1 h), Ereignisse (filterbar nach Raum und Art).
- Auswertung im gewählten Fenster: p50/p95/p99/Max für Tick und RTT, Zahl der Budget-Überschreitungen, Ausreißer markiert, Klick auf ein Ereignis springt zur Stelle im Verlauf.
- Diagramme selbst auf Canvas oder SVG gezeichnet, keine neue Chart-Abhängigkeit; die Seite kostet im Browser < 5 % CPU bei offenem 1-h-Fenster.
- Bedienbar mit Maus, Touch und Controller (B nicht belegt, View + Menu = zurück).

## Nicht-Ziele

- Keine Messung oder neuen Kennzahlen im Server (B-281).
- Keine Raum-Steuerung (DBG3/B-232).
- Keine Anzeige im Spiel am TV (Debug-Overlay B-181).

## Regeln und Einschränkungen

- Seitenregeln aus `CLAUDE.md` (Shell, `pages.ts`, `toggleFullscreen()`, `openPage()`/`goHome()`); `tests/projectRules.test.ts` grün.
- Entscheidung 001: die Seite rechnet nur Perzentile über gelieferte Punkte, keine Spiel-Logik.
- Domäne PLAT; braucht B-281 (MON1) fertig. Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

- Spieleabend: Handy zeigt 2 Räume grün, Tick p99 4 ms, RTT 12/15/40 ms.
- Xbox ruckelt: RTT-Spitze bei Gerät 2, Tick flach → Ursache Netz/Client.
- Absturz: Ereignis „💥 Raum ABCD“, Klick springt in den Verlauf.

## Ausnahme- und Fehlerfälle

- 404 (Diagnose aus) → Hinweis „Diagnose aus, `K3C_STATUS_TOKEN` setzen“.
- 401 → Token-Eingabe erneut.
- Server neu gestartet (neues `startedAt`) → alte Punkte verwerfen, Markierung „Neustart“ im Verlauf.
- Server nicht erreichbar → Ampel grau, Polling mit Backoff bis 30 s.

## Akzeptanzkriterien

- **AC-01** Vitest: Perzentile und Ausreißer-Erkennung über eine feste Punktreihe stimmen.
- **AC-02** Vitest: Delta-Anhängen verwirft Punkte bei neuem `startedAt` und begrenzt das Fenster.
- **AC-03** `monitor.html` in `pages.ts`, `installPageChrome()` aufgerufen, `task check` grün.
- **AC-04** Browser-Pane mit `task load` als Last: Screenshot zeigt Übersicht, Verlauf mit Perzentilen und Ereignis-Zeitleiste; 404 und 401 zeigen den Hinweis.
- **AC-05** 🧑 nimmt die Seite am Handy während eines Lasttests oder Spieleabends ab.

## Offene Fragen

- Geht die Seite später in der Dungeon-Master-Seite (DBG3) auf? 🧑 (blockiert nicht)

## Notizen

Aus B-281 herausgelöst, weil Server (SRV) und Seite (PLAT) verschiedene Domänen sind.
