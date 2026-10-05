# DBG3.2 · Seite /dm mit Raumliste, Diagnose und Dev-Aktionen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** dbg3/2-seite-dm
- **Abhängig von:** DBG3.1
- **Tickets:** B-232
- **Kriterien:** AC-01, AC-02, AC-03, AC-05

## Ziel

`dm.html` zeigt Raumliste, Live-Diagnose des gewählten Raums und Knöpfe für alle Dev-Aktionen; am Handy (375 px) bedienbar.

## Kontext

API aus DBG3.1 (`/api/dev`, siehe `docs/protocol.md`). Die Seite läuft außerhalb der Shell (Aufruf `/dm`), ist daher
nicht in `src/landing/pages.ts` und ruft kein `installPageChrome()`. Im Dev-Server leitet Vite `/api` an den Go-Server weiter.

## Erlaubte Dateien

- `dm.html`, `src/tools/dm.ts`, `src/tools/dmApi.ts`, `src/tools/dmApi.test.ts`
- `tests/projectRules.test.ts` (Ausnahme für `dm.html`)
- Planungsdateien

## Nicht-Ziele

Spielen auf der Seite, Passwortschutz, Abnahme am Gerät (DBG3.4).

## Schritte

1. API-Aufrufe als kleine, getestete Funktionen (`dmApi.ts`).
2. Seite: Raumliste, Auswahl, Diagnose im Sekundentakt, Aktionsknöpfe; ohne Dev-Mode ausgegraut; Raum weg → zurück zur Liste.
3. Responsive: eine Spalte, keine festen Breiten.

## Fertig, wenn

- [ ] AC-01: Seite gebaut, Layout ohne feste Breiten > 375 px.
- [ ] AC-02: Diagnose wird im Sekundentakt neu geholt.
- [ ] AC-03, AC-05: Knöpfe schicken die Aktionen (Test der Anfragen).
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

## Ergebnis

–
