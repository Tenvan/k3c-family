# BD1.2 · Beitritts-Hinweis für Spieler 2, Ö schließt zuerst den Cheat-Dialog

- **Status:** offen
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

- [ ] AC-02 (B-391): Ein Test belegt: Ohne Beitritt von Spieler 2 liefert die Hinweis-Logik einen Hinweis für Spieler 2 mit seiner Taste; nach dem Beitritt keinen.
- [ ] AC-02 (B-389): Ein Test belegt: Bei offenem Dialog schließt Ö nur den Dialog; bei geschlossenem Dialog schließt Ö Overlay und Liste.
- [ ] AC-02: Abnahme am PC mit 2 Spielern und in Chrome (Hinweis sichtbar und weg; Ö) ist Sache von 🧑; bis dahin „angenommen, Validierung offen“.
- [ ] `task check` grün; keine neue Abhängigkeit, kein Server-Eingriff.

## Prüfen

```bash
task test -- HudScene
task test -- debugOverlayPanel
task check
```

Manuelle Prüfungen (Browser, 2 Spieler an einer Tastatur) nur, wenn 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
