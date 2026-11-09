# RX1.1 · Render-Werte ins Client-Log

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** rx1/1-render-log
- **Abhängig von:** –
- **Tickets:** B-762
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Das Render-Verhalten jedes Clients steht im Server-Log: Startzeile, Ladedaten, Kontext-Ereignisse und Leistungsverlauf (B-762).

## Kontext

`src/core/renderInfo.ts` (B-761, in `develop`) liefert Renderer, GPU, max. Texturgröße, Texturzahl und -speicher für die Technik-Anzeige. `src/core/clientLog.ts` sendet an `POST /api/clientlog` (Rate-Limit). `src/main.ts` startet mit `Phaser.AUTO`. Die Werte aus `renderInfo.ts` wiederverwenden, keine zweite Messung. Spec: B-762.

## Erlaubte Dateien

- `src/core/renderInfo.ts`, `src/core/renderLog.ts` (neu), `src/core/clientLog.ts`, `src/main.ts`
- zugehörige `*.test.ts`

## Nicht-Ziele

Keine neue Anzeige, keine Leistungsoptimierung, kein Kontextschutz (RX1.2), kein serverseitiger Code.

## Schritte

1. Startzeile nach dem Laden (alle Felder aus B-762 › Anforderungen, `device` in jeder Zeile).
2. Ladezeilen je Gruppe/Stufe (Anzahl, Größe, Zeit) und Fehlerzeile mit Datei und Status.
3. Hook-Punkte für `webglcontextlost`/`-restored`/Canvas-Fallback mit Zustand (Texturzahl, -speicher, FPS, Zeit seit Start); RX1.2 nutzt sie.
4. Leistungsverlauf alle 30 s, FPS-Warnung < 20 höchstens je Minute.
5. Schalter „Technik-Anzeige“ (B-761, `?gfx=1`) steht standardmäßig auf **an** (Wunsch von 🧑; ändert B-761/AC-04 „Standard aus“); Menüeintrag und `?gfx=0` schalten ab. `optionsLogic.test.ts` und `settings.test.ts` anpassen.
6. Tests (Fake-Timer) für alle Zeilen, Emoji-Präfixe prüfen.

## Fertig, wenn

- [ ] AC-01: Test: Startzeile enthält alle Felder für WebGL und Canvas.
- [ ] AC-02: Test: simulierter Kontextverlust und -wiederherstellung schreiben je eine Zeile mit Zustand.
- [ ] AC-03: Test: Verlauf höchstens alle 30 s, Warnung höchstens je Minute.
- [ ] AC-04: Test: fehlgeschlagenes Laden erzeugt Fehlerzeile mit Datei und Status.
- [ ] Menge bleibt im Rate-Limit von `/api/clientlog`.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
