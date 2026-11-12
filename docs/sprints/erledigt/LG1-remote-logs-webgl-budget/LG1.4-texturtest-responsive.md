# LG1.4 · Textur-Testseite passt auf einen Screen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** lg1/4-texturtest-responsive
- **Abhängig von:** –
- **Tickets:** B-773
- **Kriterien:** AC-04

## Ziel

Die Textur-Testseite `texturetest.html` passt ohne vertikales Scrollen auf einen Screen (B-773).

## Kontext

Die Seite ist die Kachel „Texturen“ unter Seiten > Performance (`src/tools/devTiles.ts`) und zeigt die Diagnose aus B-769 (7 Kontext-Versuche, Renderer, maxTex). Rahmen: `installPageChrome()` (`src/core/shell.ts`), oben ca. 70 px frei. Ob wirklich diese Seite gemeint ist, ist eine offene Frage in B-773; `stress.html` und `monitor.html` werden mitgeprüft. Spec: B-773.

## Erlaubte Dateien

- `texturetest.html` und deren Script und Stile unter `src/tools/` bzw. `src/stress/`, zugehörige `*.test.ts`
- `stress.html`/`monitor.html` nur bei bestätigtem Befund

## Nicht-Ziele

Keine neue Messung, keine Änderung der Diagnose-Logik, kein Umbau des Panel-Rahmens.

## Schritte

1. Ist-Stand bei 1920×1080, 1280×720, 800×600 messen (`scrollHeight` gegen `clientHeight`), Schwesterseiten prüfen.
2. Layout auf Viewport-Höhe und Raster umstellen, Inhalt kompakter, nichts abschneiden.
3. Test der drei Auflösungen, `projectRules` grün.

## Fertig, wenn

- [ ] AC-04: `B-773/AC-01` und `AC-02` grün: kein Seitenscroll bei den drei Auflösungen, `installPageChrome()` unverändert.

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

- AC-04 umgesetzt: `texturetest.html` als Viewport-Raster (3 Spalten, `100dvh`, nur Textfelder scrollen intern, unter 640×480 Seitenscroll erlaubt). `installPageChrome()` und 70 px oben unverändert.
- AC-04 geprüft: Edge headless, `scrollHeight == clientHeight` bei 1920×1080, 1280×720, 800×600; `tests/textureLayout.test.ts` (Vertrag im Quelltext, kein Layout-Motor in Vitest); `check:diff` grün.
- Schwesterseiten: `stress.html` (festes Vollbild-Menü mit eigenem Scroll) und `monitor.html` (Dashboard) nur gesichtet, kein Umbau, da nicht die gemeinte Seite.
- Manuelle Abnahme B-773/AC-03 (Xbox) bleibt bei 🧑 (LG1.6). Hinweis: im frischen Worktree fehlten die nativen `.node`-Dateien (rolldown, oxlint), aus dem Hauptrepo kopiert.
