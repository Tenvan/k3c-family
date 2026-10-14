# BD1.3 · Steuerhinweis blendet aus, Gold im HUD, Welt-Meldungen im globalen Band

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** bd1/3-hud-hinweis-gold-meldungen
- **Abhängig von:** BD1.2
- **Tickets:** B-399, B-398, B-414
- **Kriterien:** AC-03

## Ziel

Der Steuerhinweis blendet 20 s nach dem Beitritt aus und ist über „Hinweise“ wieder einschaltbar, das Gold steht nur im HUD und wird beim Bezahlen kurz hervorgehoben, Welt-Meldungen erscheinen im globalen Band oben (B-399, B-398, B-414).

## Kontext

- **Alle drei Teile berühren `HudScene.ts`** (und `hudLayout.ts`); Reihenfolge deshalb B-399 → B-398 → B-414, je ein kleiner Schritt mit Test. Richtwert: ≤ ~400 geänderte Code-Zeilen; wird es mehr, B-414 als Ticket-Rest abspalten (Status `blockiert` nicht nötig, Ergebnis nennt `verschoben`).
- **B-399:** Steuerhinweis = Element `controls` (`HudScene.controlsHint`, `showHints` ~177–190, Text `hud.hint.<Gerät>`, Rolle `controlsHint` 24 px, `hudLayout.ts` ~140). Einschalter: Option `guide` → `settings.guideHints` (`src/core/settings.ts`, `optionsLogic.ts`, `OptionsScene.ts`; „wieder an“ setzt `guideSeen.reset()`).
- **B-398:** Beutel-Text über dem Reiter: `purse` in `src/scenes/worldRenderer.ts` (`updatePlayer`). HUD-Spielerzeile: `HudScene.showCells`, Texte `hud.playerFull`/`hud.playerShort`. HP-Balken bleiben (B-173).
- **B-414:** Mittel-Banner `HudScene.showBanner` (~209–220), `bannerQueue`, `BANNER_SECONDS`, Rolle `banner` 56 px; Leiste `BAR_HEIGHT` 134 px in `src/scenes/layout.ts`/`hudLayout.ts`. Entschieden: Warteschlange bleibt, langer Text umbrechen sonst „…“, gleiche Meldung nur einmal. Hinweis: Anzeige-Sprint W6 berührt dieselben Dateien; vor dem Start `git log origin/develop -- src/scenes/HudScene.ts` prüfen und mergen.
- Texte nur in `src/core/texts.de.ts`; Schriften als Rolle in `fontRules.ts` (Mindestgrößen, `fontRules.test.ts`); Client rechnet nichts. Höchstens 2 Spieler.

## Erlaubte Dateien

- `src/scenes/HudScene.ts`, `src/scenes/hudLayout.ts`, `src/scenes/layout.ts`, `src/scenes/fontRules.ts`
- `src/scenes/worldRenderer.ts` (nur Entfernen des Beutel-Textes `purse`)
- `src/scenes/optionsLogic.ts`, `src/scenes/OptionsScene.ts` (nur falls der Einschalter dort anzupassen ist)
- `src/core/texts.de.ts`, `src/core/texts.en.ts` (nur HUD-Texte)
- neue oder bestehende Tests unter `src/scenes/` (`hudLayout.test.ts`, `fontRules.test.ts`, …)
- Planung: Sprint-Ordner BD1, `docs/backlog/B-399-*.md`, `B-398-*.md`, `B-414-*.md`

## Nicht-Ziele

Aktionen-Overlay (B-319) und geführte Hinweise (B-148); Münz-Animationen (GR5); Änderung der Goldwerte; neue Meldungsarten oder Server-Ereignisse; Änderung der Meldungstexte; Änderung der Leistenzeilen (B-413); Server und Protokoll; 3–4 Spieler.

## Schritte

1. Glossar und `docs/arbeitsweise.md` lesen; Status `in Arbeit`; `git merge origin/develop`, Stand von `HudScene.ts` prüfen (W6).
2. **B-399:** Reine Funktion „Steuerhinweis sichtbar?“ aus Beitrittszeit und Gerät; Test zuerst: 20 s sichtbar, danach nicht; Gerätewechsel startet neu; Raumwechsel desselben Spielers keine neue Anzeige; `guideHints` einschalten zeigt ihn wieder. In `showHints` einsetzen; `hudLayout` gibt den Platz frei.
3. **B-398:** `purse` in `worldRenderer.ts` entfernen; Spielerzeile zeigt das Gold weiter; kurze Hervorhebung der Zahl bei Änderung (Zustandsfunktion testbar, kein Zufall, keine Server-Daten nötig). Test: Ansicht erzeugt keinen Goldtext über dem Reiter, Zeile nennt das Gold.
4. **B-414:** Welt-Meldungen vom Mittel-Banner ins Band oben: Rolle in `FONTS` (Pflicht-Info, ≥ 28 px), Lage innerhalb `BAR_HEIGHT` ohne Überlagerung der drei Leistenzeilen; Warteschlange wie bisher; Umbruch sonst „…“; dieselbe Meldung nur einmal. Tests in `hudLayout.test.ts` (1 und 2 Spieler) und `fontRules.test.ts`.
5. `check_run` (`task check`); Datei ≤ 400, Funktion ≤ 60 Zeilen beachten (HudScene prüfen, ggf. Teile in eigene Datei).
6. Ergebnis je Kriterium; Beobachtungen am PC bleiben 🧑; `Status: fertig`; Commit `feat(cli): Steuerhinweis, Gold im HUD, Meldungen im Band (BD1.3)`.

## Fertig, wenn

- [ ] AC-03 (B-399/AC-01): Test belegt 20 s Sichtbarkeit, danach weg, Gerätewechsel startet neu, „Hinweise“ schaltet ihn wieder ein.
- [ ] AC-03 (B-398/AC-01): Test belegt: kein Goldtext über dem Reiter; Spielerzeile nennt das Gold; Hervorhebung beim Bezahlen.
- [ ] AC-03 (B-414/AC-01, AC-02): `hudLayout.test.ts` belegt die Lage im Band für 1 und 2 Spieler, `fontRules.test.ts` die Mindestgröße.
- [ ] AC-03 (B-399/AC-02, B-398/AC-02, B-414/AC-03): Abnahme am PC bzw. Gerät ist Sache von 🧑; bis dahin „angenommen, Validierung offen“.
- [ ] `task check` grün.

## Prüfen

```bash
task test -- hudLayout
task test -- fontRules
task check
```

Manuelle Prüfungen (2 Spieler am PC, Meldungen im Band) nur, wenn 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
