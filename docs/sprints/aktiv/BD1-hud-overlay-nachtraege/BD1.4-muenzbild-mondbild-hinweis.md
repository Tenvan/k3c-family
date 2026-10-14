# BD1.4 · Münzbild und Mond-Bild im Hinweis

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** bd1/4-muenzbild-mondbild-hinweis
- **Abhängig von:** BD1.3
- **Tickets:** B-393, B-370, B-374
- **Kriterien:** AC-04

## Ziel

Der Münz-Hinweis zeigt das Münzbild vor dem Text, die Nacht-Warnung ein Mond-Bild aus den vorhandenen Packs (B-393, B-370). Das Preisschild bleibt unverändert; B-374 ist nur die festgehaltene Entscheidung (keine Taste im Preisschild).

## Kontext

- **B-393:** `guideImage('coin')` → `grafik:coinIcon` (`src/scenes/guideHints.ts`), Textur in `src/scenes/worldSprites.ts`, Zeichnen in `src/scenes/guideOverlay.ts`/`glyphView.ts` (`RowImage`). Abnahme S8.4: Bild war nicht zu sehen; unklar, ob der Text selbst erschien (möglich: `guideSeen` schon gesetzt, `src/core/guideSeen.ts`, oder Textur nicht geladen). Fehlende Textur bleibt bisher still.
- **B-370:** `guideImage('dusk')` hat kein Bild. S8.2 (2026-10-09) fand in `kyrises-free-16x16-rpg-icon-pack`, `16x16-rpg-items-db32`, `resource-icons`, `item-ruby-banana-star` keinen Mond. Entscheidung 🧑 (2026-10-14): Mond-Icon aus vorhandenen Packs, der Stern (`part-star.png`, für Skillpunkte vergeben) ist kein Ersatz. Zuordnung gehört in `docs/assets/zuordnung-welt.md`; neue Packs nur mit Lizenz-Eintrag in `docs/assets/zuordnung.md` und Zustimmung 🧑.
- **B-374:** Entschieden 2026-10-14 (🧑): Preis und Münzkreise genügen. `drawPrice` (`src/scenes/siteView.ts`) bleibt unverändert; **keine Code-Arbeit**, nur der Nachweis.
- Pixel-Art ganzzahlig skaliert; Client rechnet nichts; Texte in `texts.de.ts`.

## Erlaubte Dateien

- `src/scenes/guideHints.ts`, `src/scenes/guideOverlay.ts`, `src/scenes/glyphView.ts`, `src/scenes/worldSprites.ts`
- `public/` nur für ein Mond-Bild aus einem **bereits vorhandenen** Pack (kopieren/Zuordnung, keine neuen Packs)
- `docs/assets/zuordnung-welt.md` (Zeile für das Mond-Bild)
- `src/core/clientLog.ts` nur für das Log 🐛 bei fehlender Textur
- Tests dazu (`src/scenes/guideHints.test.ts`)
- Planung: Sprint-Ordner BD1, `docs/backlog/B-393-*.md`, `B-370-*.md`, `B-374-*.md`

## Nicht-Ziele

Jede Änderung am Preisschild (`drawPrice`, `siteView.ts`, Glyph oder Taste); den Aktionen-Hinweis am Preisschild zurückholen (B-319/AC-02); selbst gemalte Grafik; neue Packs; Bilder für Bauplatz und Bauer; den Stern als Ersatz-Bild.

## Schritte

1. Glossar und `docs/arbeitsweise.md` lesen; Status `in Arbeit`.
2. **B-393 Ursache:** mit frischem Profil (kein `guideSeen`, Browser-Pane `game.html`, neuer Spielstand, erste Münze) prüfen, ob Hinweistext und Bild erscheinen; Textur, `RowImage` und Draw-Aufruf per Test oder Log belegen. Korrigieren; bei fehlender Textur Log 🐛 (Emoji vorn) und Text weiter ohne Bild.
3. **B-370:** In den vier vorhandenen Packs gezielt nach Mond/Nacht-Symbol suchen. Gefunden: Bild nach `public/` einbinden bzw. Textur anlegen, `guideImage('dusk')` liefert es, Zeile in `docs/assets/zuordnung-welt.md`, Test in `guideHints.test.ts`. **Nicht gefunden:** kein Ersatz; `Status: blockiert`, Frage-Ticket an 🧑 (neues Pack mit Lizenz oder Verzicht), Stand pushen, Entwurfs-PR, aufhören.
4. **B-374:** nichts ändern; im Ergebnis festhalten, dass `drawPrice` unverändert ist (Diff zeigt keine Änderung an `siteView.ts`).
5. `check_run` (`task check`).
6. Ergebnis je Kriterium; `Status: fertig`; Commit `fix(cli): Münzbild und Mond-Bild im Hinweis (BD1.4)`.

## Fertig, wenn

- [ ] AC-04 (B-393/AC-01): Test oder Log belegt Textur geladen und gezeichnet; Browser-Beobachtung mit frischem Profil im Ergebnis (1 und 2 Spieler); Log 🐛 bei fehlender Textur.
- [ ] AC-04 (B-370/AC-01): `task test -- guideHints`: `guideImage('dusk')` liefert das Mond-Bild; Zuordnung in `docs/assets/zuordnung-welt.md`. Sonst blockiert (Schritt 3).
- [ ] AC-04 (B-374/AC-01): `git diff` zeigt keine Änderung an `src/scenes/siteView.ts`; das Preisschild zeigt weiter Name, Material, Münzkreise.
- [ ] Abnahme am PC bleibt 🧑; bis dahin „angenommen, Validierung offen“.
- [ ] `task check` grün.

## Prüfen

```bash
task test -- guideHints
task check
```

Browser-Prüfung (frisches Profil, `game.html`) nur, wenn 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
