# MON2.1 · Daten-Schicht: Delta, Neustart, Perzentile, Ausreißer

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** mon2/1-daten
- **Abhängig von:** B-281
- **Tickets:** B-282
- **Kriterien:** AC-01, AC-02

## Ziel

Reine, getestete Logik der Monitoring-Seite: `/api/metrics` abfragen, Deltas an einen Puffer hängen (Neustart, 1-h-Fenster),
Perzentile, Ausreißer und Ampel je Raum ausrechnen. Noch ohne Seite.

## Kontext

- API aus MON1 (B-281): `docs/protocol.md` › „Diagnose: Verläufe über /api/metrics (B-281)“ (bis zum Merge von MON1 auf
  `origin/sprint/mon1`). Felder: `startedAt`, `now`, `server[]`, `rooms{code: []}`, `devices{kürzel: {room, points[]}}`,
  `events[]`; Zeiten in Unix-ms. Antwort enthält nur Punkte nach `since`; ein neuer `startedAt` = Neustart.
  Schutz: `Authorization: Bearer <K3C_STATUS_TOKEN>`; 404 = Diagnose aus, 401 = Token fehlt/falsch.
- Muster: `src/tools/dmApi.ts` (injizierbares `FetchLike`, nie Exception) und `src/tools/levelApi.ts`.
- Entscheidung 001: Die Seite rechnet nur Statistik über gelieferte Punkte, keine Spiel-Logik.
- Perzentil nach Nearest-Rank; Ausreißer über den Tukey-Zaun (Q3 + 1,5 · IQR).
- Ampel je Raum (Schwellen angenommen, Glossar „Ampel“): rot bei Absturz in den letzten 5 min, Tick p99 > 33,3 ms (ein Takt),
  RTT > 250 ms oder kein Pong; gelb bei anderem Diagnose-Ereignis in 5 min, Tick p99 > 16,7 ms oder RTT > 100 ms;
  grau ohne Tick-Punkt in 5 min; sonst grün.
- Backoff: Abfrage alle 3 s, bei Fehlern verdoppelt bis 30 s.

## Erlaubte Dateien

- `src/tools/monitorData.ts`, `src/tools/monitorData.test.ts`
- `src/tools/monitorApi.ts`, `src/tools/monitorApi.test.ts`
- Planungsdateien

## Nicht-Ziele

Seite, Zeichnen, Token-Speicher (MON2.2). Keine Änderung am Server (`engine/`); fehlt dort etwas → SRV-Ticket.

## Schritte

1. Typen der Antwort und `applyDelta` (Anhängen, Neustart verwirft Puffer und merkt die Neustart-Zeit, Fenster 1 h).
2. `percentile`, `stats` (p50/p95/p99/Max), `outlierLimit`, Fenster-Auswahl, `roomSummaries` mit Ampel.
3. `fetchMetrics` (Ergebnis `ok`/`off`/`unauthorized`/`offline`) und `nextDelay` (Backoff).
4. Vitest mit fester Punktreihe.

## Fertig, wenn

- [x] AC-01: `monitorData.test.ts` prüft Perzentile und Ausreißer über eine feste Reihe.
- [x] AC-02: `monitorData.test.ts` prüft Delta-Anhängen, Verwerfen bei neuem `startedAt` und das 1-h-Fenster.
- [x] `task check` grün.

## Prüfen

```bash
task check
```

## Ergebnis

- AC-01 geprüft: `src/tools/monitorData.test.ts` › „Perzentile und Ausreißer“ – Reihe 1..20 ms plus 90 ms ergibt
  p50 11, p95 20, p99 90, Max 90 (Nearest-Rank); Tukey-Zaun 31 ms markiert nur die 90.
- AC-02 geprüft: `src/tools/monitorData.test.ts` › „Delta-Puffer“ – Anhängen, neuer `startedAt` verwirft alte Reihen und
  merkt den Neustart, Punkte älter als 1 h vor `now` und leere Reihen fallen weg.
- Zusätzlich getestet: Fenster-Auswahl mit Budget-Überschreitungen, Ampel je Raum (`roomSummaries`), `fetchMetrics`
  (Bearer, 404/401/offline mit gemocktem fetch), Backoff 3 s bis 30 s.
- `task check` grün (2026-10-05, Agent). Gebaut gegen die API-Beschreibung auf `origin/sprint/mon1` (MON1 noch nicht in
  `develop`).
