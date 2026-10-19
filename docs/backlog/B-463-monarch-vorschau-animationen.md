# B-463 · Eine Monarch-Seite im Asset-Studio zeigt Reiten, Schlagen, Skills und weitere Vorschauen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ10
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-17, Ralf im Chat (Revision 1)

## Ausgangslage

Der Monarch nutzt je Spieler-Index einen Eintrag aus `data/sprites.json` › `players` (Index 0–3: `medieval-king`, `medieval-king-2`, beide ein zweites Mal mit `tint` `#9ecbff` bzw. `#ffb3c8`); beide Sheets haben nur `idle`, `run` und `attack`. Im Spiel reitet er immer auf dem Standard-Reittier `MONARCH.mount.sprite` (`horse`, `data/monarch.json` › `mount`); `data/sprites.json` › `mounts` kennt 13 Reittiere mit Sattelpunkt je Frame, dazu `riderWaist` 0,6. Der Figuren-Atlas (`task atlas`, `public/atlas/`, nicht eingecheckt) enthält nur Sheets mit Rolle; das Standard-Reittier lädt `preloadSprites` als Einzel-PNG, die übrigen Reittiere lädt das Spiel nicht.

Sprinten zeigt das Spiel beritten nur über schnelleres Abspielen (`timeScale` aus `src/scenes/render/mountPose.ts`, `sprintMultiplier` 1,8). Schlag, Treffer, Münzen und Niederlage zeigt es über Effekte (`swing`, `hit`, `coinGive`, `coin`, `down` in `src/scenes/effects/effects.ts`), die Wartezeit bis zum Respawn über Transparenz (`src/scenes/render/worldRenderer.ts`). Skills haben keine eigene Darstellung, nur `noTarget` bei `castFailed`. `data/monarch.json` › `skills` hat 24 Skills (Tank, Zauberer, Heiler); das Preset `thief` nennt `backstab` und `smokeBomb`, die dort fehlen.

`figuren.html` und `aufstellung.html` (`src/tools/figuren.ts`, `aufstellung.ts`, Logik in `spriteReference.ts`) zeichnen Figuren ohne Phaser auf Canvas im festen Wechsel; die Animationen des Spiels (`createSpriteAnims`, `playAnim` in `src/scenes/render/sprites.ts`) sieht man nur im Spiel. Das Asset-Studio mit den Aliasen `@game` (→ `src/`) und `@data` (→ `data/`) entsteht in WZ6 (B-459).

## Ziel

Eine Monarch-Seite im Asset-Studio (B-459) spielt den Monarchen mit den Animationen des Spiels ab: stehen, laufen und sprinten (der Monarch ist immer beritten, Laufen oder Sprinten zu Fuß gibt es nicht), reiten auf jedem Reittier, schlagen, Skills, Münzen werfen und geben, getroffen, Tod und Respawn, alle Spielerfarben nebeneinander. Alles erscheint im Spielmaßstab und mit Grafik-Auswahl aus dem Slot-Register. Was das Spiel nicht darstellt, ist als Lücke sichtbar und damit Grundlage für Grafik- und Client-Tickets. `figuren.html` und `aufstellung.html` fallen weg, ihr Inhalt liegt im Asset-Studio.

## Beteiligte und Zielgruppen

🧑 prüft Monarch, Reittiere und Lücken im Asset-Studio am PC und entscheidet über neue Grafik (Projekt GRA) oder Darstellung im Spiel (CLI). Agenten setzen in DEV um, ein getrennter Reviewer prüft. Spieler sind nicht betroffen: Die Spieler-Seiten (`src/landing/pages.ts`: Weiterspielen, Neues Spiel, Online, Lizenzen, Entwicklerseite) verweisen nicht auf `figuren.html` oder `aufstellung.html`.

## Anforderungen

- **Spiel-Animationen:** Abspielen mit Phaser und denselben Definitionen wie im Spiel: `preloadSprites`, `createSpriteAnims`, `makeSprite`, `playAnim`, `face` aus `src/scenes/render/sprites.ts`, Sitzpose aus `mountPose` (`src/scenes/render/mountPose.ts`), Effekte aus `effectFor`/`EFFECT_CONFIG` (`src/scenes/effects/effects.ts`), Münz-Grafik aus `src/scenes/render/objectView.ts`. Import über `@game`/`@data` (WZ6), keine Kopie von Werten.
- **Sheets außerhalb des Atlas** (Reittiere außer `horse`, Slot-Alternativen) lädt die Seite als Einzel-PNG `sprites/<sheet>/<anim>.png` nach dem Muster von `preloadSprites`; Bildzahl und Bildrate aus `data/sprites.json`, Wiederholung wie im Spiel (`attack` einmal, sonst Schleife).
- **Phaser und Bühne:** Die Phaser-Abhängigkeit im Studio-Frontend und die Bühne (Phaser-Instanz mit Szene) führt B-460 mit WZ7.3 ein. B-463 ist nicht Eigentümer der Abhängigkeit und trägt kein `phaser` in die `package.json` des Studios ein; WZ10.1 nutzt die Bühne aus WZ7.3 wieder.
- **Gemeinsame Abspiel-Bausteine** für Monarch- und Gegner-Seite (B-464) auf dieser Bühne: Steuerung (Schleife, Einzelbild, Tempo), Maßstab, Reittier-Lader, Lücken-Markierung. Eine Phaser-Instanz je Seite, nicht je Karte.
- **Monarch immer beritten:** Der Monarch reitet immer (Glossar › Monarch). Laufen und Sprinten gibt es nur beritten; die Seite bietet kein Laufen und kein Sprinten zu Fuß an und markiert dafür keine Lücke. Die Vorschau der 12 weiteren Reittiere ist Darstellung, keine Spielregel (Glossar › Reittier: im Spiel das Standard-Pferd `horse`).
- **Vorschauen:** stehen; laufen und sprinten, jeweils beritten; reiten je Reittier aus `mounts`; schlagen; jeder Skill; Münzen werfen und geben, Münze aufheben; getroffen; Tod; Respawn; Spielerfarben 0–3 nebeneinander.
- **Spielerfarbe und Kennfarbe:** Die Spielerfarbe ist die Variante des Monarchen je Spieler-Index (Sheet und `tint` aus `players`); die Kennfarbe ist die Farbe des Spielers aus `PLAYER_COLORS` und steht an der Figur.
- **Lücken:** Fehlt im Spiel eine Animation oder Darstellung, zeigt die Karte den Ersatz des Spiels (z. B. `idle`) und die Markierung „Lücke“ mit Grund. Die Seite baut keine Ersatz-Animation.
- **Grafik-Auswahl:** Zuordnungen der Slots `monarch:1`–`monarch:4` und `mount:*` aus `data/assets.json` als Vorschau wählbar, ohne das Register zu ändern.
- **Qualität:** reine Logik (Vorschau-Liste, Lücken, Reittiere, Skills) als Funktionen mit Vitest-Test im Studio-Frontend; keine Zufallswerte, gleiche Auswahl ergibt dieselbe Darstellung; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Nicht-Ziele

- Neue Animationen, Sheets oder Skill-Effekte: Grafik in Projekt GRA, Darstellung im Spiel in CLI (eigene Tickets aus den Lücken).
- Der Monarch zu Fuß: Laufen, Sprinten oder Stehen ohne Reittier (der Monarch ist immer beritten, Entscheidung von 🧑 vom 2026-10-17).
- Phaser oder die Bühne einführen (B-460, WZ7.3).
- Werte von Angriff, Skills, Tempo und Respawn (REG); Änderungen an `data/sprites.json`, `data/monarch.json`, `data/assets.json`.
- Änderungen an `src/scenes/`, `src/core/`, `src/model/` (CLI); reicht eine Spiel-Funktion nicht, entsteht ein CLI-Ticket.
- Gegner und Bosse (B-464); Truppen (bleiben in der Figuren-Szene der Grafik-Seite, WZ6.3).
- `spriteReference.ts` als Ganzes: `renderReference` nutzt die Grafik-Seite (WZ6.3), `installPadScroll` nutzen `grafiken.ts` und die Spieler-Seite `lizenzen.html`; beides bleibt. `grafiken.html` und `soundtest.html` bleiben (B-459).
- Prüfung am TV oder an der Xbox (das Asset-Studio läuft am PC).

## Regeln und Einschränkungen

- **Domäne DEV:** `tools/k3c-studio/` (Studio-Frontend, DEV nach WZ6.1) und `src/tools/`. Spiel-Code aus `src/scenes/`, `src/core/` und `src/model/` liest das Studio nur über `@game`.
- **Grenzfall DEV** (wie WZ6.6 und B-299): Beim Entfernen von `figuren.html` und `aufstellung.html` ändert DEV auch die Wurzel-Seiten, `docs/game-design.md` (nur Z. ~187–188), `public/sprites/CREDITS.md` (nur Z. ~56) und `src/tools/codemap.md`. Tickets (`docs/backlog/`, auch `archiv/`) und erledigte Sprints bleiben als Historie unverändert.
- **Seiten-Regeln** aus `CLAUDE.md` gelten für Seiten im Spiel; das Asset-Studio ist ein Wails-Fenster ohne Shell, `installPageChrome()` entfällt dort. Nach dem Entfernen bleiben `devTiles.ts` und `tests/projectRules.test.ts` stimmig (jede Wurzel-Seite hat eine Kachel).
- **Firewall:** EXE mit festem Pfad, Proben nur auf Loopback (B-459). Fensterprüfungen macht ein Agent nur mit Freigabe von 🧑 für diesen Lauf.
- **Komplexitäts-Budget:** Datei ≤ 400, Funktion ≤ 60 Zeilen, Session ≤ ~400 geänderte Code-Zeilen.
- **Begriffe** nach `docs/glossar.md` (Monarch, Reittier, Schlag, Skill, Preset, Münze, Respawn, Grabstein, Asset-Slot, Präferenz). Neue Begriffe (Monarch-Seite, Abspiel-Baustein, Lücke, Spielerfarbe, Kennfarbe) kommen vor der Freigabe ins Glossar; „Kennfarbe“ und „Spielerfarbe“ gelten in allen Tickets und Sprints von WZG gleich (siehe Anforderungen).
- **Abhängigkeiten:** WZ6 erledigt (Asset-Studio, Aliase, Grafik-Seite mit Figuren-Szene und Katalog); WZ7.3 erledigt (Phaser und Bühne im Studio-Frontend, B-460).

## Beispiele

- Monarch-Seite › Reiten › Einhorn, Spielerfarbe 2 → `medieval-king` mit `tint` `#9ecbff` sitzt mit der Hüfte (`riderWaist` 0,6) auf dem Sattelpunkt des Einhorns, Frame für Frame wie `mountPose` im Spiel.
- Bewegung › Sprinten (Standard-Reittier `horse`) → das Pferd spielt `run` mit dem `timeScale` aus `mountPose` bei 5 × 1,8 = 9 Units/s; Laufen (5 Units/s) läuft sichtbar langsamer.
- Bewegung → die Auswahl bietet nur Laufen und Sprinten beritten; es gibt keinen Eintrag „zu Fuß“ und keine Lücke dafür.
- Schlagen, Tempo 0,5× → Reiter spielt `attack` einmal (6 Bilder, 14 fps) halb so schnell, dazu Effekt `swing`.
- Skills › Feuerball → Eintrag „Zauberer, Stufe 1, aktiv“ mit „Lücke: keine Darstellung im Spiel“; Skills › ohne Ziel → Effekt `noTarget` mit dem Text des Spiels.
- Spielerfarben → vier Monarchen (Index 0–3) nebeneinander im selben Takt, je mit Kennfarbe aus `PLAYER_COLORS`.
- Grafik-Auswahl › Slot `monarch:2`, zweite Zuordnung → Vorschau wechselt das Sheet; `data/assets.json` bleibt unverändert.

## Ausnahme- und Fehlerfälle

- Sheet ohne die gewählte Animation (Reittier ohne `attack`, Slot-Sheet ohne `run`) → wie `playAnim` im Spiel Rückfall auf `idle`, Karte markiert die Lücke.
- Reittier ohne Sattelpunkt für die Animation oder Bild fehlt in `public/sprites/` → `mountPose` liefert `null`; die Karte zeigt den Rückfall des Spiels (`createRider` → `makeSprite`, Monarch-Figur ohne Sitzpose) und die Lücke „Reittier unbekannt“ bzw. „Bild fehlt“. Das ist ein Fehlerfall der Daten, keine wählbare Vorschau „zu Fuß“.
- Preset nennt einen Skill, der in `skills` fehlt (heute `backstab`, `smokeBomb` bei `thief`) → Eintrag mit Lücke „Skill fehlt in data/monarch.json“, kein Absturz.
- Figuren-Atlas fehlt (`public/atlas/atlas.json` nicht gebaut) → die Seite meldet „Figuren-Atlas fehlt (`task atlas`)“ statt einer leeren Bühne.
- Slot ohne Zuordnung oder mit einem Sheet ohne Eintrag in `sprites.json` › `sheets` → Lücke, die Vorschau zeigt die Spiel-Figur.
- Spiel-Code ruft `clientLog` ohne Spielserver auf → Fehler werden wie in WZ6.3 geschluckt, die Seite bleibt bedienbar.
- Bühne aus WZ7.3 nicht vorhanden (Sprint WZ7 nicht erledigt) → WZ10.1 startet nicht, die Session setzt `Status: blockiert`.

## Akzeptanzkriterien

- **AC-01** Die Monarch-Seite des Asset-Studios spielt jede Animation der Monarch-Sheets aus `data/sprites.json` › `players` (`medieval-king`, `medieval-king-2`: `idle`, `run`, `attack`) einzeln ab, immer beritten (der Monarch ist immer beritten): `idle` und `attack` mit dem Monarchen auf dem Sattel des gewählten Reittiers (Standard `MONARCH.mount.sprite`, `horse`), `run` beritten, also das Reittier spielt `run` und der Reiter sitzt mit `run` auf; ein Abspielen von `run` zu Fuß gibt es nicht. Dazu Schleife an/aus, Einzelbild vor/zurück und Tempo 0,5×, 1× und 2×.
- **AC-02** Die Seite nutzt `preloadSprites`, `createSpriteAnims`, `makeSprite` und `playAnim` aus `src/scenes/render/sprites.ts` über `@game`; Bildrate, Bildzahl, Ursprung und Maßstab stammen nur aus `data/sprites.json` bzw. diesen Exporten, im Studio-Code stehen dafür keine Zahlen (Review).
- **AC-03** Reiten zeigt den Monarchen auf jedem der 13 Reittiere aus `data/sprites.json` › `mounts`; ein Vitest-Test vergleicht für jedes Reittier die Pose der Seite mit `mountPose` für Frame 0 von `idle` und `run`.
- **AC-04** Laufen und Sprinten sind nur beritten wählbar (der Monarch ist immer beritten): Das Reittier spielt `run` mit dem `timeScale` aus `mountPose` bei `base.speed` (Laufen) bzw. `base.speed × sprintMultiplier` (Sprinten). Die Seite bietet kein Laufen und kein Sprinten zu Fuß an und markiert dafür keine Lücke; ein Vitest-Test prüft die Vorschau-Liste.
- **AC-05** Schlagen spielt `attack` des Monarchen einmal und den Effekt `swing` aus `EFFECT_CONFIG`, beritten auf dem gewählten Reittier.
- **AC-06** Die Seite listet jeden Skill aus `data/monarch.json` › `skills` (heute 24) mit Linie, Stufe und Art; jeder Skill ohne Darstellung im Spiel trägt die Markierung „Lücke“.
- **AC-07** Jeder Skill aus `presets.*.activeSkills`, der in `skills` fehlt, erscheint mit der Lücke „Skill fehlt in data/monarch.json“; ein Vitest-Test prüft das für `backstab` und `smokeBomb`.
- **AC-08** Münze geben zeigt den Effekt `coinGive`, Münze aufheben den Effekt `coin` mit der Münz-Grafik aus `src/scenes/render/objectView.ts`; die fehlende Wurf-Animation des Monarchen ist als Lücke markiert.
- **AC-09** Getroffen zeigt den Effekt `hit`, Tod den Effekt `down` und die Transparenz des Spiels bei `respawnIn > 0` (`src/scenes/render/worldRenderer.ts`), Respawn den Wechsel zurück auf volle Deckkraft; jede dafür fehlende Animation des Sheets ist als Lücke markiert.
- **AC-10** Alle 4 Einträge aus `players` (Index 0–3, Sheet und `tint` als Spielerfarbe) laufen nebeneinander synchron in der gewählten Vorschau, jeder mit seiner Kennfarbe aus `PLAYER_COLORS` (`src/core/constants.ts`).
- **AC-11** Die Figuren erscheinen im Spielmaßstab (`UNIT_PX` = 32 px je Unit, `scale` von Sheet und Figur) mit einer Unit-Skala an der Bodenlinie; ein Vitest-Test prüft, dass die Höhe des Monarchen auf der Seite gleich `spriteTop` des Spiels ist.
- **AC-12** Für die Slots `monarch:1`, `monarch:2`, `monarch:3`, `monarch:4` und `mount:*` aus `data/assets.json` ist jede Zuordnung als Vorschau wählbar, Präferenz 1 ist voreingestellt; die Auswahl ändert `data/assets.json` nicht.
- **AC-13** `figuren.html`, `aufstellung.html`, `src/tools/figuren.ts`, `src/tools/aufstellung.ts`, ihre Kacheln, die Texte `tile.figuren.*`, `tile.aufstellung.*`, `sel.figuren`, `sel.aufstellung` und die Verweise außerhalb der Historie sind entfernt; `git grep -n -E "figuren\.html|aufstellung\.html" -- . ":!docs/backlog" ":!docs/sprints"` findet nichts, `task check` und `task build` sind grün.
- **AC-14** Vor dem Entfernen hat jede Karte von `figuren.html` und `aufstellung.html` einen Ort im Asset-Studio (Monarch-Seite, Gegner-Seite, Figuren-Szene oder Katalog der Grafik-Seite); das Ergebnis der Session nennt die Zuordnung je Gruppe.
- **AC-15** Die reine Logik der Monarch-Seite (Vorschauen, Lücken, Reittiere, Skills) hat Vitest-Tests im Studio-Frontend, die in `task check:all` und in der CI laufen.
- **AC-16** Die Seite nutzt die Phaser-Bühne aus B-460/WZ7.3 und führt keine eigene Phaser-Abhängigkeit ein. Nachweis: `git diff` auf `tools/k3c-studio/frontend/package.json` enthält keine Änderung an `phaser`, Review.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat):

1. **Zustände:** Reiten (je Reittier), Schlagen, Skills, dazu Münzen werfen/geben, Getroffen/Tod/Respawn, Laufen/Sprinten (nur beritten), alle Spielerfarben nebeneinander. Fehlende Animationen (das Sheet hat nur `idle`/`run`/`attack`) werden als Lücke markiert, nicht gebaut.
2. **Technik:** Abspielen mit Phaser und denselben Animationsdefinitionen wie das Spiel über die Aliase `@game`/`@data` aus WZ6; gemeinsame Abspiel-Bausteine für Monarch- und Gegner-Seite.
3. **Ersatz:** `figuren.html` und `aufstellung.html` werden entfernt (Seiten, Kacheln, Texte, Verweise), Inhalte ins Asset-Studio; Grenzfall DEV wie WZ6.6/B-299.
4. **Truppen und ungenutzte Sheets** (2026-10-17, Chat): Truppen in der Figuren-Szene der Grafik-Seite (WZ6.3), ungenutzte Sheets nur im Katalog der Grafik-Seite (statisch, nicht animiert).
5. **Tempo-Stufen** (2026-10-17, Chat): 0,5×, 1×, 2× (AC-01).
6. **Monarch immer beritten:** Laufen und Sprinten gibt es nur beritten, keine Lücke „zu Fuß“ (AC-04, Anforderungen, Beispiele angepasst; deckt sich mit Glossar › Monarch).
7. **Phaser und Bühne:** B-460 führt beides mit WZ7.3 ein, B-463 ist nicht Eigentümer der Abhängigkeit; WZ10.1 nutzt die Bühne wieder (AC-16).

Offen: nichts.

## Notizen

**Fundstellen zum Entfernen** (Stand 2026-10-17, ohne Historie):

- `figuren.html`, `aufstellung.html`, `src/tools/figuren.ts`, `src/tools/aufstellung.ts`
- `src/tools/devTiles.ts` (Kacheln `aufstellung`, `figuren`), `src/tools/devTiles.test.ts` (`TOOLS`)
- `src/tools/texts.de.ts`, `texts.en.ts`: `tile.aufstellung.*`, `tile.figuren.*`, `sel.figuren`, `sel.aufstellung`, Kommentar „Reference pages“
- `src/tools/spriteReference.ts`: Exporte `allFigures`, `lineup` (nur von den beiden Seiten genutzt), Kopfkommentar; `src/tools/selection.ts`: Kopfkommentar
- `src/tools/codemap.md` (Z. ~9, 11, 19, 27), `docs/game-design.md` (Z. ~187–188), `public/sprites/CREDITS.md` (Z. ~56)
- `.codemap/state.json` erzeugt das Codemap-Werkzeug, nicht von Hand ändern.

**Hinweise:** HW1.4 (Abnahme Entwicklerseite am PC, B-335) nennt die beiden Kacheln nicht einzeln; nach WZ11 fehlen sie dort, B-335/AC-02 bleibt erfüllt. Keine Spieler-Seite und keine offene Abnahme verweist auf `figuren.html` oder `aufstellung.html`. Das Spiel lädt nur das Standard-Reittier `horse`; die übrigen 12 Reittiere sind erst auf der Monarch-Seite mit dem Spiel-Code zu sehen.

**Korrektur 2026-10-17 (nach Entscheidungen von 🧑, Spec weiter Entwurf):** Monarch immer beritten (kein Laufen und Sprinten zu Fuß, AC-04); Phaser und Bühne gehören B-460/WZ7.3 (neues AC-16, Text zur Abhängigkeit korrigiert); „Kennfarbe“ und „Spielerfarbe“ sind getrennt definiert und im Glossar einzutragen. Boss-Effekte betreffen B-464 und B-469, nicht diese Seite.

**Sprint-Teilung 2026-10-17 (Entscheidung von 🧑):** B-463 gehört zu WZ10 (Abspiel-Bausteine und Monarch-Seite, WZ10.1–WZ10.3). Die Kriterien AC-13 (Entfernen von `figuren.html`/`aufstellung.html`) und AC-14 (Ort im Studio vor dem Entfernen) laufen in WZ11 (dort AC-18 und AC-19, Session WZ11.3), weil sie auch die Gegner-Seite (B-464) belegen muss.
