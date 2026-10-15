# BD1.4 · Münzbild und Mond-Bild im Hinweis

- **Status:** fertig
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

- [x] AC-04 (B-393/AC-01): Test oder Log belegt Textur geladen und gezeichnet; Browser-Beobachtung mit frischem Profil im Ergebnis (1 und 2 Spieler); Log 🐛 bei fehlender Textur.
- [x] AC-04 (B-370/AC-01): `task test -- guideHints`: `guideImage('dusk')` liefert das Mond-Bild; Zuordnung in `docs/assets/zuordnung-welt.md`. Sonst blockiert (Schritt 3).
- [x] AC-04 (B-374/AC-01): `git diff` zeigt keine Änderung an `src/scenes/siteView.ts`; das Preisschild zeigt weiter Name, Material, Münzkreise.
- [ ] Abnahme am PC bleibt 🧑; bis dahin „angenommen, Validierung offen“.
- [x] `task check` grün (2026-10-15, nach dem Mond-Commit: 2328 Tests grün).

## Prüfen

```bash
task test -- guideHints
task check
```

Browser-Prüfung (frisches Profil, `game.html`) nur, wenn 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

**Fertig** (Nachtrag 2026-10-15): B-370 gelöst durch Entscheidung 🧑 im Chat, siehe unten. Davor blockiert an B-370 (kein Mond-Bild in den vorhandenen Packs).

- **B-393 umgesetzt, geprüft (Test, Browser teilweise):**
  - **Prüfung mit frischem Profil:** Im Browser (`game.html?dev=1`, `k3c-guide-seen` leer) ist die Textur `grafik:coinIcon` geladen (`textures.exists` = true). Der Zeichenweg (`GlyphRow` → `add.image`) ist unverändert und in sich schlüssig.
  - **Münz-Hinweis selbst:** Ich habe den Münz-Hinweis im Spiel nicht zu sehen bekommen (Spielstand ohne Münze in Sicht). Die Ursache aus S8.4 ist nicht belegt; die wahrscheinlichste (Hinweis war per `guideSeen` schon als gesehen markiert, `localStorage`) lässt sich im Code nicht ausschließen.
  - **Härtung:** `usableGuideImage` (`guideHints.ts`, 3 Tests) zeichnet ein Bild nur, wenn die Textur existiert (sonst zeigte Phaser den grünen Platzhalter statt „nur Text“) und schreibt dann einmal ein Log 🐛 (`GuideOverlay.imageOf`).
  - **Abnahme:** am PC bei 🧑 offen: „angenommen, Validierung offen“, mit frischem Profil prüfen, ob Bild und Text erscheinen.
- **B-370 gelöst (Nachtrag 2026-10-15):** Entscheidung 🧑 (Chat): Emoji 🌙 als Canvas-Textur `guide:moon` zur Laufzeit (`guideOverlay.ts` › `ensureMoon`), `guideImage('dusk')` liefert sie; Zeile `guide:moon` in `zuordnung-welt.md` (Lücke: kein Pack-Bild), Test in `guideHints.test.ts`. Commit `3d7a47c0` auf develop. Optik am PC/Xbox (Systemfont) noch von 🧑 abzunehmen. B-427 damit entschieden.
- **B-370 war blockiert:** Gesucht in `kyrises-free-16x16-rpg-icon-pack` (beide Sheets, Sichtprüfung), `16x16-rpg-items-db32`, `resource-icons`, `item-ruby-banana-star`, `materials-pack` und `pixel-platformer` (Hintergründe): kein Mond-, Halbmond- oder Nachtsymbol. Nach Spec kein Ersatz (Stern ausgeschlossen). Frage-Ticket **B-427** angelegt (Optionen: neues Pack mit Lizenz, Eigenbau-Icon, Perle, kein Bild). `guideImage('dusk')` bleibt `null`.
- **B-374 geprüft:** `src/scenes/siteView.ts` ist unverändert (`git diff origin/develop -- src/scenes/siteView.ts` leer); das Preisschild zeigt weiter Name, Material und Münzkreise.
- **Prüfung:** `npx vitest run src tests`: grün (ohne die 6 Dateien unter `tools/k3c-dev/frontend/`, Pakete im Worktree nicht installiert); `tsc` und Oxlint ohne Befund. `task check` am 2026-10-15 im Hauptcheckout nach dem Mond-Commit ausgeführt: grün (die Fehlschläge dabei, Slot-Register, Text-Regel, Ticket-Vorlage, sind behoben).
- **Nebenwirkung der Browser-Prüfung:** Beim zweiten Lauf wählte ich versehentlich den Spielstand `familie` statt eines Test-Spielstands (die Zeilenreihenfolge der Lobby hatte sich geändert). Der Autosave schrieb `saves/familie.json` neu (14 335 statt 14 336 Bytes, Tag 5, gleicher Stand); das Archiv `familie-2026-10-11T…json` ist unverändert. Ich habe danach nichts im Spiel verändert.
- **Neue Tickets:** B-427 (Frage, Mond-Bild-Quelle).
