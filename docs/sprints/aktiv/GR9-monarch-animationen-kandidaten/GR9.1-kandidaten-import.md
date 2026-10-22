# GR9.1 · Kandidaten-Import hält task check grün

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** gr9/1-kandidaten-import
- **Abhängig von:** –
- **Tickets:** B-420
- **Kriterien:** AC-01

## Ziel

Dateien unter `public/grafik/kandidaten/` mit Credit-Zeile `kandidaten/<datei>` lassen `task check` grün (B-420).

## Kontext

`src/tools/grafikPacks.test.ts` verlangt, dass jede Datei unter `public/grafik/` zu einem Pack aus `GRAFIK_PACKS` gehört. Entscheidung (autonom, Claude, 2026-10-22): `kandidaten/` bleibt kein Pack; der Test nimmt den Ordner aus (wie `gebaeude/`), `credits.test.ts` prüft die Credit-Zeilen der Importe und `lizenzen.html` rendert sie bereits (`src/tools/credits.ts`).

## Erlaubte Dateien

- `src/tools/grafikPacks.test.ts`, `src/tools/credits.test.ts`, `src/tools/credits.ts` (nur falls nötig)

## Nicht-Ziele

Importe selbst, Zuordnung der Kandidaten zu Objekten.

## Schritte

1. Filter `kandidaten/` in `files` des Tests; Kommentar mit B-420.
2. Vitest-Fall: Credit-Zeile `kandidaten/coin.png` erscheint in `renderCredits`/lizenzen mit Urheber, Lizenz, Quelle (falls `credits.test.ts` das nicht schon prüft).
3. Gezielter Test, dann `task check:diff`.

## Fertig, wenn

- [ ] AC-01: Test belegt, dass eine Datei in `public/grafik/kandidaten/` mit Credit-Zeile den Pack-Test nicht rot macht und in der Lizenzliste erscheint.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
