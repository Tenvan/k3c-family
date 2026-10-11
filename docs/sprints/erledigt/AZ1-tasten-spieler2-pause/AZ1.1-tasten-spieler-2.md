# AZ1.1 · Hinweise, Glyphen und Skill-Leiste von Spieler 2 aus KEYBOARD_2

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** sprint/az1
- **Abhängig von:** –
- **Tickets:** B-371
- **Kriterien:** AC-01, AC-02

## Ziel

Im Feld von Spieler 2 an der Tastatur nennen Glyphen, Aktionshinweise, Skill-Leiste und Skill-Menü dessen Tasten (Enter, Pfeile, Strg rechts, Ziffernblock 0–5); Steuer- und Beitrittshinweis folgen dem zuletzt benutzten Tastatur-Spieler. Spieler 1 bleibt unverändert.

## Kontext

- **Layouts** (`src/input/keyboardLayouts.ts`, PLAT, nur lesen): `KEYBOARD_2` belegt `confirm` = keyCode 13 (Enter), `attack` = `Numpad0`, `skill1`–`skill4` = `Numpad1`–`Numpad4`, `skillMenu` = `Numpad5`, laufen = `ArrowLeft`/`ArrowRight`, sprinten = `ControlRight`; keine `pause`/`fullscreen`. Einzige Quelle der Tasten, keine zweite Liste in `src/scenes/`.
- **Gerät je Spieler:** `HudScene.update()` (`src/scenes/HudScene.ts`) reicht `device(slot)` an Aktionen-Overlay, Guide-Overlay und Skill-Leiste; `deviceOf(input)` kennt nur `'pad' | 'touch' | 'keyboard'` (`Device` aus `src/input/slotBindings.ts`, PLAT, nur lesen). Die beiden Tastatur-Spieler liegen in `GameScene.keyboards` (`[KeyboardInput, KeyboardInput]`, privat; Index 1 = Spieler 2).
- **Glyphen:** `glyphOf(action, device: string)` (`src/scenes/glyphs.ts`) liest bei `'keyboard'` `KEY_ACTIONS`; unbekanntes Gerät → Text-Rückfall. `glyphView.ts` zeichnet `key:*` als Tastenkappe mit `glyph.label`, ein neuer Schlüssel braucht kein Bild.
- **Skill-Leiste/-Menü:** `skillMenuView.ts` holt die Beschriftungen aus `slotBindings(device)` und den Hinweis aus `t('skill.hint.<device>')`.
- **Globale Hinweise:** `HudScene.showHints()` nutzt `t('hud.hint.<lastDevice>')` und `t('hud.join.<lastDevice>')`; `GameScene.trackLastDevice()` setzt `lastDevice` (Typ `'keyboard' | 'pad' | 'touch'`).
- **Vorschlag:** ein Szenen-Gerät `'keyboard2'` (Typ in `src/scenes/`, z. B. `HintDevice = Device | 'keyboard2'`), das `glyphOf` und die Slot-Beschriftungen aus `KEYBOARD_2` ableiten; Texte `hud.hint.keyboard2`, `hud.join.keyboard2`, `skill.hint.keyboard2` in de und en.
- **Fallstrick:** `GameScene.ts` hat 398 Zeilen (Grenze 400); dort höchstens die Zeile in `trackLastDevice` ändern und einen Getter auf 1 Zeile, sonst Logik in `HudScene.ts` oder ein neues Modul legen.
- Browser-Pane: `CLAUDE.md` › Im Browser-Pane testen (Tastatur-Ereignisse mit `code` + `keyCode`).

## Erlaubte Dateien

- `src/scenes/glyphs.ts`, `src/scenes/glyphs.test.ts`, `src/scenes/actionHints.ts`, `src/scenes/actionOverlay.ts`, `src/scenes/guideOverlay.ts`
- `src/scenes/skillMenuView.ts`, `src/scenes/HudScene.ts`, `src/scenes/GameScene.ts` (nur `lastDevice`/`trackLastDevice`)
- neues Modul unter `src/scenes/` mit Test, falls nötig
- `src/core/texts.de.ts`, `src/core/texts.en.ts` (nur neue Schlüssel `*.keyboard2`)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Belegung ändern (B-316); Änderungen in `src/input/`; Symbole je Controller-Familie (B-339); Beitritts-Hinweis für Spieler 2 sichtbar machen (B-391); Pause-Anzeige (AZ1.2).

## Schritte

1. Branch `sprint/az1` von `origin/develop` (Aktivierung ist der erste Commit), `Start-Commit` setzen, `Status: in Arbeit`.
2. Test zuerst (`glyphs.test.ts`): `glyphOf('confirm', 'keyboard2')` nennt Enter, `glyphOf('attack'|'skill1'…'skillMenu', 'keyboard2')` nennt Ziffernblock 0–5; Slot-Beschriftungen von Spieler 2 ebenso; `'keyboard'` unverändert.
3. Gerät `'keyboard2'` in `glyphs.ts` aus `KEYBOARD_2` ableiten (Beschriftung aus `KeySpec`: keyCode 13 → `t('hint.enter')`, `NumpadN` → `Num N`); Aktion ohne Taste im Layout → Text-Rückfall.
4. Skill-Leiste/-Menü und Hinweis-Texte auf `'keyboard2'` umstellen; `HudScene.deviceOf` erkennt Spieler 2 über `GameScene.keyboards[1]`; `trackLastDevice` setzt `'keyboard2'`, wenn zuletzt Spieler 2 tippte.
5. Browser-Pane (Freigabe 🧑 für diesen Lauf): zwei Tastatur-Spieler beitreten lassen, Spieler 2 an einen Bauplatz → Hinweis, Skill-Leiste und Skill-Menü im Feld von Spieler 2 lesen.
6. `task check` grün, Ergebnis eintragen, committen.

## Fertig, wenn

- [x] AC-01: Test grün: Glyph für `confirm` von Spieler 2 nennt Enter, seine Slot-Beschriftungen nennen Ziffernblock 0–5; Spieler 1 unverändert.
- [x] AC-02: Browser-Pane: Im Feld von Spieler 2 stehen weder „Leertaste“ noch E/Q/R/T/Z/K.
- [x] Nur ein Tastatur-Spieler → Layout 1 wie bisher.
- [x] `src/input/` unverändert; `task check` grün.

## Prüfen

```bash
task check
```

Browser-Pane laut Schritt 5, nur mit Freigabe 🧑 für diesen Lauf; ohne Freigabe AC-02 `blockiert` (Grund).

## Ergebnis

2026-10-10, Agent, Browser-Pane von 🧑 für diesen Lauf freigegeben (Vite des Worktrees auf 5183, Spielserver des Worktrees auf 8090, `game.html?dev=1`).

- **Umsetzung:** Szenen-Gerät `HintDevice = Device | 'keyboard2'` in `glyphs.ts`; `glyphOf(…, 'keyboard2')` und `slotLabels()` lesen `KEYBOARD_2` (Enter, `Num 0`–`Num 5`; Pause/Vollbild ohne Taste → Text-Rückfall). Skill-Leiste und -Menü nutzen `slotLabels`; `HudScene.deviceOf` erkennt Spieler 2 an `GameScene.keyboards[1]` (Feld nicht mehr privat), `trackLastDevice` setzt `keyboard2`. Neue Texte `hud.hint/join.keyboard2`, `skill.hint.keyboard2` (de, en). `GameScene.ts` bleibt bei 398 Zeilen, `src/input/` unverändert.
- **AC-01** geprüft: `glyphs.test.ts` (Glyph `confirm` = Enter, Slots = Ziffernblock 0–5, Layout 1 unverändert) grün.
- **AC-02** geprüft (Browser-Pane): zwei Tastatur-Spieler (Leertaste, Enter); Feld von Spieler 2 zeigt „Num 0 : Schlag“, „Num 1 – · Num 2 – · Num 3 – · Num 4 –“ und im Skill-Menü „◀ ▶ wählen · Enter bestätigen · Num 5 schließen“; Feld von Spieler 1 weiter E/Q/R/T/Z. Keine Taste von Spieler 1 im Feld von Spieler 2.
- Nur ein Tastatur-Spieler: `deviceOf` liefert für Tastatur 1 weiter `keyboard` (Layout 1).
- Beobachtung ohne Änderung: Der Beitritts-Hinweis folgt dem zuletzt benutzten Tastatur-Spieler und steht auch, wenn beide Tastatur-Spieler schon beigetreten sind (wie vorher mit „Leertaste“; gehört zu B-391).
- `task check` grün (k3c-dev, `checkout`).
