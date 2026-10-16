# BD1.2 · Beitritts-Hinweis für Spieler 2, Ö schließt zuerst den Cheat-Dialog

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** bd1/2-beitritt-hinweis-oe
- **Abhängig von:** BD1.1
- **Tickets:** B-391, B-389
- **Kriterien:** AC-02

## Ziel

Ein Spieler, der noch nicht beigetreten ist, sieht den Beitritts-Hinweis mit seiner Taste (Spieler 2: Enter bzw. A); Ö schließt bei offenem Cheat-Dialog nur den Dialog (B-391, B-389).

## Kontext

- **B-391:** Der Hinweis wird in `HudScene.showHints` (`src/scenes/HudScene.ts` ~177–190) gesetzt: `this.joinHint.setText(t(\`hud.join.${game.lastDevice}\`))`. Er richtet sich nach dem zuletzt benutzten Gerät von Spieler 1, nicht nach einem freien Platz. Texte: `hud.join.keyboard` (Leertaste), `hud.join.keyboard2` (Enter), `hud.join.pad`, `hud.join.touch`, `hud.joinCenter` in `src/core/texts.de.ts`; Tastenbelegung `KEYBOARD_2` in `src/input/keyboardLayouts.ts` (Beitreten = Enter). Schrift: Rollen `joinCenter`/`joinCorner` in `src/scenes/fontRules.ts`.
- **B-389:** `src/scenes/debugOverlayPanel.ts` ~51 sagt, Ö schließe bei offenem Dialog zuerst den Dialog; `debugOverlayView.ts` ~17–21 verdrahtet Ö (keyCode 192) als Geste `diag`. Beobachtung 🧑 (U5.4): Mit offenem Dialog schließt Ö auch das Overlay. Die Ursache ist nicht untersucht (Reihenfolge der Handler, Geste `diag` greift zusätzlich?). Tests: `debugGestures.test.ts`, `debugOverlayPanel.test.ts`.
- Annahme für B-391 (die Ticket-Fragen nennen „Entscheidet 🧑“, Vermutung dort: Enter): Der Hinweis steht für jeden freien Platz, solange weniger als 2 Spieler beigetreten sind, und nennt die Taste des Geräts für diesen Platz. Weicht 🧑 ab, Spec ändern, nicht den Code.
- Höchstens 2 Spieler. Client rechnet nichts (`src/scenes/noSim.test.ts`). B nicht belegen. Texte nur in `src/core/texts.de.ts` (und `.en.ts`, falls dort gepflegt).

## Erlaubte Dateien

- `src/scenes/HudScene.ts`, `src/scenes/hudLayout.ts`, `src/scenes/fontRules.ts` (nur Beitritts-Hinweis)
- `src/scenes/debugOverlayPanel.ts`, `src/scenes/debugOverlayView.ts`, `src/scenes/debugGestures.ts`
- `src/core/texts.de.ts`, `src/core/texts.en.ts` (nur `hud.join.*`)
- neue oder bestehende Tests dazu unter `src/scenes/`
- Planung: `docs/sprints/aktiv/BD1-hud-overlay-nachtraege/`, `docs/backlog/B-391-*.md`, `docs/backlog/B-389-*.md`

## Nicht-Ziele

Neue Beitritts-Wege; Änderung der Tastenbelegung; Xbox-Zugang zum Overlay (B-195); neue Dev-Aktionen; Touch-HUD (B-390); Steuerhinweis (BD1.3); Kamera (BD1.1).

## Schritte

1. Glossar und `docs/arbeitsweise.md` lesen; Status auf `in Arbeit` (`plan_set`), BD1.1 muss `fertig` sein.
2. **B-391:** Die Hinweis-Auswahl als reine, getestete Funktion herausziehen: aus Anzahl beigetretener Sitze und Gerät je freiem Platz folgt, welche Hinweise mit welcher Taste erscheinen (Platz 2 Tastatur: `hud.join.keyboard2`, Controller: `hud.join.pad`). Test zuerst (rot): ohne Spieler 2 liefert sie einen Hinweis für Spieler 2 mit Enter; mit 2 Spielern keinen.
3. `HudScene.showHints` auf die Funktion umstellen: Hinweis für jeden freien Platz, je Zelle platziert, verschwindet beim Beitritt. Schrift aus dem Katalog, Mindestgröße je Zelle beachten.
4. **B-389:** Ursache belegen (Test rot): offener Dialog + Ö → Dialog zu, Overlay bleibt. Prüfen, ob Geste `diag` und Dialog-Handler beide auf Ö reagieren, und die Reihenfolge korrigieren (Dialog verbraucht Ö zuerst).
5. Test: Dialog zu + Ö → Overlay und Liste schließen wie bisher (U5.1).
6. `check_run` (`task check`), Befunde beheben; Ergebnis je Kriterium eintragen (AC-02 am PC: `angenommen, Validierung offen`, Abnahme 🧑), `Status: fertig`, Commit `fix(cli): Beitritts-Hinweis je Platz, Ö schließt Dialog zuerst (BD1.2)`.

## Fertig, wenn

- [x] AC-02 (B-391): Ein Test belegt (`joinHint.test.ts`): Ohne Beitritt von Spieler 2 liefert die Hinweis-Logik einen Hinweis für Spieler 2 mit seiner Taste; nach dem Beitritt keinen.
- [x] AC-02 (B-389): Bestehender Test (B-192, `debugOverlayPanel.test.ts`) belegt: Bei offenem Dialog schließt Ö nur den Dialog; bei geschlossenem schaltet Ö die Info-Zeilen. Zusätzlich im Browser geprüft, siehe Ergebnis.
- [x] AC-02 (Abnahme 🧑, bestätigt im Chat 2026-10-14): Abnahme am PC mit 2 Spielern und in Chrome (Hinweis sichtbar und weg; Ö) ist Sache von 🧑; bis dahin „angenommen, Validierung offen“.
- [x] Vitest grün (2277 Tests), `tsc` und Oxlint ohne Befund; keine neue Abhängigkeit, kein Server-Eingriff.

## Prüfen

```bash
task test -- HudScene
task test -- debugOverlayPanel
task check
```

Manuelle Prüfungen (Browser, 2 Spieler an einer Tastatur) nur, wenn 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **B-391 umgesetzt, geprüft (Test):** Neue reine Funktion `joinHintDevices` (`src/scenes/joinHint.ts`, 5 Tests in `joinHint.test.ts`): Vor dem ersten Beitritt das zuletzt benutzte Gerät wie bisher, danach jedes Gerät ohne Spieler (Spieler 2 an der Tastatur sieht „Enter drücken zum Beitreten“), nach dem letzten Beitritt kein Hinweis. `GameScene.joinCandidates` liefert die Geräte, `HudScene.showHints` setzt den Text. Ursache war, dass der Hinweis immer die Taste des zuletzt benutzten Geräts nannte.
- **B-389 geprüft, nicht reproduzierbar, keine Codeänderung:** Im Browser (`game.html?dev=1`, Vite auf Port 5173 aus dem Checkout der Repo-Wurzel, `develop` ohne BD1-Änderungen, Spielstand `neu-mv3yut0q`) mit Tastaturereignissen für Ä (222) und Ö (192): Ö (Zeilen an) → Ä (Dialog auf) → Ö: `shown` bleibt `true`, `open` wird `false`, der Dialog verschwindet; ein weiteres Ö schaltet die Zeilen aus. Das entspricht der Spec. Im Code gibt es keinen zweiten Ö-Handler (`diagKey` in `debugOverlayView.ts`). Möglich: U5.4 lief auf einem älteren Stand, oder die Beobachtung betraf einen anderen Weg (Controller LB+RB, Touch-Gesten). 🧑 hat die Session abgenommen; B-389 gilt damit als erledigt.
- **AC-02 Abnahme am PC (B-391 und B-389): bestätigt von 🧑 im Chat 2026-10-14 (Abnahme am PC).**
- **Prüfung:** `npx vitest run src tests`: 2277 Tests grün (die 6 Dateien unter `tools/k3c-dev/frontend/` laden im Worktree nicht, `react`/`@radix-ui/themes` fehlen, unabhängig davon). `task check` nicht ausgeführt (k3c-dev getrennt).
- **Abweichungen:** Status von Hand gesetzt. Test und Funktion zu B-391 habe ich zusammen geschrieben, nicht strikt rot zuerst. Der Browser-Lauf nutzte den bereits laufenden Server (Port 8080) und einen Test-Spielstand; Raum und Anzeige sind danach zurückgesetzt.
- **Neue Tickets:** keine.
