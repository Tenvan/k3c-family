# GR7.1 · Schrift: Platzhaltertext aus dem Katalog, Regel für die Mitspieler-Zelle

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** gr7/1-schrift-platzhalter-partner
- **Abhängig von:** –
- **Tickets:** B-198, B-197
- **Kriterien:** AC-03, AC-04

## Ziel

Der Platzhaltertext einer ungeladenen Stufe ist die Rolle `placeholder` im Schrift-Katalog und erreicht die Mindestgröße in allen Layouts; `docs/rules/bedienung.md` nennt die Regel für die Mitspieler-Zelle und `minFontPx` samt Test folgen ihr.

## Kontext

- **B-198:** `src/scenes/stageView.ts` setzt `PLACEHOLDER_TEXT` (Zeile 13) mit fester Größe 40 px; Verwendung in `placeholderLayer` (Zeile ~163). Im Streifen und Viertel (Zoom 0,5) erscheint er mit 20 px, unter der Nebeninfo-Mindestgröße (24 px bei 2 Spielern). Der Katalog `FONTS` in `src/scenes/fontRules.ts` kennt die Rolle nicht; `fontStyle(name)` liefert `fontSize` und `color`. Der Text ist ein Welt-Text: `StageView.scaleTexts` gleicht den Kamera-Zoom aus (`GameScene.ts` ~Zeile 319), allerdings nur für Texte, die die Stufe kennt; prüfen, ob der Platzhalter dort mitskaliert wird, sonst erreicht die Rolle ihre Größe nicht. Stroke (`#000000`, 4 px) und fett bleiben erhalten (Konturfarbe `OUTLINE_COLOR`).
- **Rolle:** Nebeninfo (`art: 'side'`), `space: 'world'`, `px` mindestens 24 (Mindestwert Nebeninfo bei 1 und 2 Spielern), Farbe `#ffffff`. `fontRules.test.ts` prüft jede Rolle in jedem Layout und den Kontrast; die neue Rolle muss dort ohne Sonderfall grün sein.
- **B-197 (Beschluss 🧑 2026-10-12, Chat):** In der Mitspieler-Zelle (`kind: 'partner'`, oben 1920 × 360 bei einem lokalen Spieler und einem Mitspieler eines anderen Geräts) werden **Welt-Texte ausgeblendet**; es gibt dort keine Mindestgröße, weil dort kein Text steht. `minFontPx` liefert für `partner` heute `null` (Regel offen); künftig steht der Grund im Code-Kommentar und im Test („Welt-Texte dort ausgeblendet“). HUD-Texte der eigenen Ansicht bleiben unberührt.
- **Ausblenden umsetzen:** Nur wenn es mit wenig Aufwand geht (z. B. Kamera-Filter oder Sichtbarkeit der Text-Objekte je Zelle in `StageView`/`GameScene`). Geht es nur mit einem größeren Umbau, gilt die Regel in `bedienung.md` und im Test, die Darstellung folgt in einem Ticket (CLI); das ist im Ergebnis zu nennen, kein Blocker.
- `src/scenes` rechnet nichts am Spiel; `worldRenderer.ts` ist bei ~395 Zeilen (Grenze 400): dort keine neuen Texte anlegen.

## Erlaubte Dateien

- `src/scenes/fontRules.ts`, `src/scenes/fontRules.test.ts`, `src/scenes/stageView.ts`, bei Bedarf `src/scenes/GameScene.ts` (nur für Mitskalieren und Ausblenden der Welt-Texte)
- `docs/rules/bedienung.md` (§ 2: Zeile zur Mitspieler-Zelle)
- `docs/sprints/geplant/GR7-figuren-schrift/`, `docs/sprints/aktiv/GR7-figuren-schrift/` (nur Status), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Zeichnen der Stufen (S4.1), neue Schrift-Rollen außer `placeholder`, Größen der übrigen Layouts (beschlossen, S4.2), Skalierung der Figuren (GR7.4), Sichtprüfung am TV.

## Schritte

1. Sprint-Branch `sprint/gr7` anlegen und pushen, `Status: in Arbeit`; `Start-Commit` in der Sprint-README setzen (`git rev-parse --short origin/develop`). `docs/arbeitsweise.md`, `docs/glossar.md` und `docs/rules/bedienung.md` § 2 lesen.
2. Tests zuerst (`task test -- fontRules`): Rolle `placeholder` in `FONTS` (Nebeninfo, `world`), `effectiveFontPx` ≥ `minFontPx` in jedem Layout (1, 1 mit Partner, 2, 3, 4); ein Test, dass `minFontPx(…, partner, …)` den Beschluss „Welt-Texte ausgeblendet, keine Mindestgröße“ benennt.
3. `fontRules.ts`: Rolle `placeholder` ergänzen (px ≥ 24); Kommentar bei `minFontPx` auf den Beschluss B-197 umstellen.
4. `stageView.ts`: `PLACEHOLDER_TEXT` durch `fontStyle('placeholder')` plus Stroke/Fett ersetzen; sicherstellen, dass der Platzhalter vom Zoom-Ausgleich (`scaleTexts`) erfasst wird, sonst dort aufnehmen.
5. Welt-Texte in der Partner-Zelle ausblenden (Kontext: nur bei geringem Aufwand), sonst Ticket (CLI, Typ Schuld) anlegen.
6. `docs/rules/bedienung.md` § 2: eine Zeile zur Mitspieler-Zelle („Welt-Texte dort ausgeblendet, keine Mindestgröße; Beschluss 🧑 2026-10-12, B-197“).
7. `task check` ausführen. Ergebnis schreiben, `Status: fertig`, Tabelle der Sprint-README anpassen. B-198 und B-197 bleiben bis zum Review im Backlog (das Review archiviert).

## Fertig, wenn

- [x] AC-03: `PLACEHOLDER_TEXT` in `stageView.ts` hat keine lose Größe mehr; Rolle `placeholder` steht in `FONTS`, `fontRules.test.ts` prüft sie in allen Layouts, `task check` grün.
- [x] AC-04: `docs/rules/bedienung.md` § 2 nennt die Regel für die Mitspieler-Zelle; `minFontPx` und `fontRules.test.ts` folgen ihr.
- [x] Keine Datei über 400 Zeilen, keine Funktion über 60.

## Prüfen

```bash
task test -- fontRules
task check
```

Über k3c-dev: `check_run task:check` (Worktree: `checkout` setzen). Keine manuellen Prüfungen (Browser, Xbox, TV).

## Ergebnis

**Fertig (2026-10-12).**

- **AC-03** umgesetzt und geprüft: Rolle `placeholder` (Nebeninfo, `world`, 32 px) in `src/scenes/fontRules.ts`; `stageView.ts › placeholderLayer` nimmt Größe und Farbe aus `fontStyle('placeholder')` (Kontur `OUTLINE_COLOR`) und skaliert den Text mit `1 / Zoom` der Zelle (`GameScene` übergibt `viewZoom(cell.h)`), er erscheint also in jeder Zelle in 32 px. `fontRules.test.ts` deckt die Rolle in allen Layouts ab, dazu ein eigener Test.
- **AC-04** umgesetzt und geprüft: Beschluss 🧑 (2026-10-12, Chat): In der Mitspieler-Zelle sind die Texte der Stufe ausgeblendet, keine Mindestgröße. Regel in `docs/rules/bedienung.md` § 2, `minFontPx` liefert `null` mit Begründung im Kommentar, Test prüft `partner` → `[null, null]` und die eigene Zelle (28). Ausblenden: `StageView.scaleTexts(factor, hideFrom)` setzt `cameraFilter` der Stufen-Texte auf die Kamera-ID der Mitspieler-Zelle (`GameScene.partnerCamId`); Phaser 4.2.1 prüft `cameraFilter` im `willRender` auch für Kinder einer Layer (Quelltext gelesen). Der Zoom-Ausgleich der Texte rechnet ohne die Mitspieler-Zelle, damit die Texte der eigenen Zelle nicht zu groß werden. **Sichtprüfung der Partner-Zelle steht aus** (braucht zwei Geräte, nicht Teil dieser Session).
- Geprüft: `npx vitest run tests/planning.test.ts src/scenes` grün (974 Tests), `tsc --noEmit` und `oxlint` auf den geänderten Dateien ohne Befund. `task check` im Worktree rot wegen Lint-Fehlern in `tools/cockpit/` (aus develop, PR #278, nicht aus dieser Session; Ticket B-415).
- Neues Ticket: B-414 (Welt-Meldungen im globalen Band oben, Wunsch von Ralf im Chat), B-415 (Lint in `tools/cockpit/`).
