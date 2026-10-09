# PL1.3 · Texte von Touch-Overlay und Shell zentral

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** pl1/3-texte-touch-shell
- **Abhängig von:** –
- **Tickets:** B-215
- **Kriterien:** AC-04

## Ziel

`src/input/touchInput.ts` und `src/core/shell.ts` holen ihre Texte über `t()` aus `src/core/texts.de.ts` und `src/core/texts.en.ts`; ein Test verhindert neue deutsche Literale in beiden Dateien.

## Kontext

- **Texte heute:** S5.3 (B-172) hat die Spieltexte nach `src/core/texts.de.ts` (`de`, Typ `TextKey`) und `src/core/texts.en.ts` (`en`) verlegt; `src/core/texts.ts` liefert `t(key, params?)`, die Sprache kommt aus `loadSettings().language` (`src/core/settings.ts`), Rückfall Deutsch. Tests: `src/core/texts.test.ts`.
- **Noch im Code:** `src/input/touchInput.ts` (`aria-label` der Touch-Tasten, z. B. `SKILL_ARIA` mit „Skill-Menü“, dazu „Vollbild“, „Sprinten“, „Münzen geben / beitreten“) und `src/core/shell.ts` (Home-Button, `button.title = 'Zurück zur Startseite (View + Menu, Pos1)'`, Zeile ~132).
- **Regel-Test als Vorbild:** `src/scenes/textRule.test.ts` liest Quellen per `import.meta.glob(..., { query: '?raw' })` und meldet Literale mit Umlaut, ß oder zwei Wörtern (`GERMAN`); ausgenommen Kommentare, CSS-Werte, `clientLog(`/`console.`. Er liegt in `src/scenes/` (CLI): nicht ändern, sondern einen eigenen Test in `src/core/` mit derselben Regel für die zwei Dateien anlegen.
- **Abhängigkeit:** `src/core` importiert nichts aus `src/scenes` (B-215). Die Shell läuft auch auf der Landingpage ohne Spiel; `t()` liest die Sprache dort aus `loadSettings()`, ohne Speicher gilt Deutsch.
- **Werkzeug-Seiten (`src/tools/`):** Beschluss 🧑 2026-10-06: Sie werden auch übersetzt, aber nicht in PL1 (sonst mehr als 4 Sessions); eigenes Ticket, nicht Teil dieser Session.
- Beispiel: Sprache English → Home-Button „Back to start (View + Menu, Home)“, Touch-Taste „Coins / join“.
- Regeln: `installPageChrome()` bleibt der Einstieg der Seiten; keine neue Abhängigkeit; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `src/input/touchInput.ts`, `src/core/shell.ts`, `src/core/texts.de.ts`, `src/core/texts.en.ts`, `src/core/texts.test.ts`
- neu: `src/core/platformTextRule.test.ts` (Regel-Test für die zwei Dateien)
- `docs/sprints/geplant/PL1-start-tastatur-koop-texte/`, `docs/sprints/aktiv/PL1-start-tastatur-koop-texte/`, `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Werkzeug-Seiten `src/tools/` (eigenes Ticket); weitere Sprachen; Texte in `src/scenes` und `src/online`; Änderungen an `src/scenes/textRule.test.ts`; Abnahme am Gerät (PL1.5).

## Schritte

1. Branch anlegen und pushen, `Status: in Arbeit`. `docs/arbeitsweise.md` lesen.
2. Regel-Test `src/core/platformTextRule.test.ts` für `src/input/touchInput.ts` und `src/core/shell.ts` schreiben; er schlägt zuerst fehl.
3. Schlüssel in `texts.de.ts` und `texts.en.ts` anlegen, die Literale in beiden Dateien durch `t()` ersetzen; der englische Text folgt dem Beispiel oben.
4. In `texts.test.ts` belegen: Englisch liefert die englischen Texte der neuen Schlüssel, eine unbekannte Sprache fällt auf Deutsch zurück.
5. `task check` ausführen, Ergebnis schreiben, `Status: fertig`, Tabelle der Sprint-README anpassen.

## Fertig, wenn

- [x] AC-04: `src/core/platformTextRule.test.ts` grün; `touchInput.ts` und `shell.ts` enthalten kein deutsches Text-Literal, die Texte stehen in `texts.de.ts` und `texts.en.ts` (B-215/AC-01).
- [x] AC-04: `texts.test.ts` belegt Englisch und den Rückfall auf Deutsch für die neuen Schlüssel (Vorbereitung für B-215/AC-02, Abnahme in PL1.5).
- [x] `task check` grün; keine Datei über 400 Zeilen, keine Funktion über 60.

## Prüfen

```bash
task test -- text
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-04 geprüft (B-215/AC-01): `src/core/platformTextRule.test.ts` – erst rot („Zurück zur Startseite …“, „Skill-Menü“, „Münzen geben / beitreten“), dann grün. Neue Schlüssel `shell.home`, `shell.homeTitle`, `touch.fullscreen`, `touch.sprint`, `touch.attack`, `touch.confirm`, `touch.skill1`–`touch.skill4`, `touch.skillMenu` in `texts.de.ts`/`texts.en.ts`; `touchInput.ts` und `shell.ts` nutzen `t()` (auch für die Ein-Wort-Labels, die die Regel nicht fängt).
- AC-04 geprüft (Vorbereitung B-215/AC-02): `texts.test.ts` › „Shell und Touch-Overlay (B-215)“: Englisch „Back to start (View + Menu, Home)“, „Coins / join“, „Skill menu“; Sprache `fr` → Deutsch. Browser-Pane mit Sprache English und `?touch=1`: Home-Titel englisch, Touch-Tasten „Fullscreen, Sprint, Skill 1–4, Skill menu, Strike, Coins / join“; Sprache danach zurück auf Deutsch.
- `task check` grün (`check_run task:check`).
- Abweichung: Die Regel weicht von `src/scenes/textRule.test.ts` an zwei Stellen ab, sonst schlüge sie auf Code-Werte an: Zeilen mit `font` gelten als CSS („Segoe UI“), und in HTML-Markup (`<…>`) zählen nur Umlaute und ß (Tag- und Attributnamen wären sonst „zwei Wörter“). Selbsttest belegt beides. Browser mit Freigabe 🧑 für BED (2026-10-09).
- Neue Tickets: keine.
