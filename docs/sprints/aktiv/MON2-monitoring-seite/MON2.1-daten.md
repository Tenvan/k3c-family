# MON2.1 · Daten-Schicht: Delta, Neustart, Perzentile, Ausreißer

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
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

- [ ] AC-01: `monitorData.test.ts` prüft Perzentile und Ausreißer über eine feste Reihe.
- [ ] AC-02: `monitorData.test.ts` prüft Delta-Anhängen, Verwerfen bei neuem `startedAt` und das 1-h-Fenster.
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

## Ergebnis

–
