# WZ11 · DEV · Gegner- und Boss-Vorschau im Asset-Studio, Figuren-Seiten entfernen

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-464
- **Start-Commit:** 2ab60241
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-17, Ralf im Chat (Revision 1)

## Ausgangslage

`B-464 › Ausgangslage`: 17 Gegnertypen (`data/enemies.json`, Grafik aus `data/sprites.json` › `enemies`, sechs Platzhalter nach B-326) und 6 Bosse (5 Minibosse je Tiefe 0–4, Endboss `crystalHeartWarden`) sind nur statisch im Wechsel in `figuren.html`/`aufstellung.html` zu sehen; Laufen, Angreifen, Treffer und Sterben nur im Spiel. Boss-IDs haben keinen Eintrag in `sprites.json`, das Spiel zeichnet sie mit dem Rückfall `goblin`. Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner zeichnet das Spiel erst nach K5.1 (B-132); WZ11 zeigt sie als Lücke, die Anbindung ist B-469.

Herkunft: WZ11 entstand am 2026-10-17 durch die Teilung des früheren WZ10 (Entscheidung von 🧑). Die Abspiel-Bausteine und die Monarch-Seite liefert WZ10 (B-463); WZ11 baut darauf die Gegner-Seite und entfernt danach die Referenzseiten (`B-463 › Offene Fragen` 3, Kriterien B-463/AC-13 und B-463/AC-14). Voraussetzung: Asset-Studio mit Aliasen `@game`/`@data` aus WZ6, Phaser und Bühne aus WZ7.3 (B-460), Abspiel-Bausteine aus WZ10.1.

## Ziel

Nach WZ11 zeigt das Asset-Studio alle Gegnertypen nach Biom und alle Bosse je Tiefe mit den Animationen und Effekten des Spiels, markiert jede fehlende Darstellung als Lücke (auch Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner bis B-469), und `figuren.html`/`aufstellung.html` sind entfernt, weil ihr Inhalt im Studio liegt.

Am Ende sichtbar: Gegner-Seite im Fenster „Asset-Studio“ neben Grafik, Sound und Monarch; Entwicklerseite ohne die Kacheln „Unsere Aufstellung“ und „Alle Figuren“; `task check:all` und CI grün; von 🧑 am PC abgenommen.

## Beteiligte und Zielgruppen

`B-464 › Beteiligte und Zielgruppen`. Agenten setzen die Sessions autonom um (DEV), ein getrennter Reviewer prüft in der Review-Session, 🧑 nimmt am PC ab und entscheidet über Folge-Tickets aus den Lücken (GRA, B-329; CLI, K5; B-469). Spieler sind nicht betroffen: Keine Spieler-Seite verweist auf `figuren.html` oder `aufstellung.html`.

## Anforderungen

`B-464 › Anforderungen`, dazu `B-463 › Anforderungen` (Abspiel-Bausteine, Fundstellen zum Entfernen in `B-463 › Notizen`). Sprint-eigen:

- Die Gegner-Seite importiert die Abspiel-Bausteine aus WZ10.1 (Steuerung, Maßstab, Reittier-Lader für den Monarch-Vergleich, Lücken-Markierung) und die Bühne aus WZ7.3; es entsteht weder ein zweiter Abspieler noch eine zweite Bühne noch eine eigene Phaser-Abhängigkeit. Reicht ein Baustein nicht, wird er selbst angepasst, nicht kopiert.
- Gegner und Boss erscheinen als eigene Seite in der Navigation des Asset-Studios neben Grafik, Sound und Monarch (WZ6.2).
- Je Boss erscheinen Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner nur dort, wo der Effekt zutrifft, als Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“: Warnkreis nur bei Flächenschlag (`trollKing`, `crystalHeartWarden`), Phasen-Text nur beim Endboss mit Phasen (`crystalHeartWarden`), Boss-Leiste und Event-Banner bei jedem Boss (`goblinLeader`: kein Warnkreis, kein Phasen-Text). Die Anbindung ist B-469, nicht Teil von WZ11.
- Das Entfernen von `figuren.html` und `aufstellung.html` (B-463/AC-13) läuft erst, nachdem jede Karte der alten Seiten einen Ort im Studio hat (B-463/AC-14): Monarch-Seite (WZ10), Gegner-Seite (WZ11.1, WZ11.2), Figuren-Szene oder Katalog der Grafik-Seite (WZ6.3). `spriteReference.ts` bleibt; nur die Exporte `allFigures` und `lineup` entfallen.
- Die Lücken der Gegner-Seite und der Bosse sind im Ergebnis der Review-Session als Liste zusammengefasst, damit 🧑 Folge-Tickets entscheiden kann.

## Nicht-Ziele

`B-464 › Nicht-Ziele`. Dazu:

- Monarch-Seite und Abspiel-Bausteine bauen: WZ10 (B-463). WZ11 nutzt sie nur.
- Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner zeigen (statt als Lücke markieren), auch Beispielzustände für Boss, Phase und Warnkreis: B-469, abhängig von K5.1 (B-132) und WZ11.
- Zeichnen dieser Anzeigen im Spiel (K5.1, CLI); Änderungen an `src/scenes/`, `src/core/`, `src/model/`, `src/online/` (stattdessen CLI-Ticket).
- Neue Gegner, Bosse oder Grafiken (GRA, B-329); Änderungen an `data/enemies.json`, `data/bosses.json`, `data/biomes/*.json`, `data/sprites.json`, `data/assets.json`.
- Boss-Kampfablauf als Simulation.
- Phaser oder die Bühne einführen (WZ7.3).
- `spriteReference.ts` ganz entfernen: `renderReference` (Grafik-Seite) und `installPadScroll` (`grafiken.ts`, `lizenzen.html`) bleiben; `grafiken.html` und `soundtest.html` bleiben.
- Truppen und ungenutzte Sheets in neuen Seiten zeigen; ihr Ort wird in `Offene Fragen` entschieden (Vorschlag: Figuren-Szene bzw. Katalog der Grafik-Seite).
- Folge-Tickets aus den Lücken umsetzen; Glossar-Pflege während der Sessions (Begriffe kommen vor der Freigabe ins Glossar).

## Regeln und Einschränkungen

`B-464 › Regeln und Einschränkungen`, dazu `B-463 › Regeln und Einschränkungen`. Für den ganzen Sprint:

- **Domäne DEV** in allen Sessions (`tools/k3c-studio/`, `src/tools/`); Spiel-Code nur lesend über `@game`. Das Entfernen der Referenzseiten (WZ11.3) ist Grenzfall DEV wie WZ6.6/B-299: DEV ändert dabei auch die Wurzel-Seiten, `docs/game-design.md` (nur Z. ~187–188), `public/sprites/CREDITS.md` (nur Z. ~56) und `src/tools/codemap.md`; Tickets und erledigte Sprints bleiben als Historie unverändert.
- **Abhängigkeiten:** WZ6 erledigt; WZ7.3 erledigt (Phaser und Bühne, B-460); WZ10 erledigt, mindestens WZ10.1 (Abspiel-Bausteine) und die Monarch-Seite (für den Monarch-Vergleich und B-463/AC-14). WZ11.1 und WZ11.2 vor WZ11.3; WZ11.3 erst, wenn Monarch-Seite und Gegner-Seite die Inhalte belegen (AC-19). K5.1 ist keine Abhängigkeit: Fehlender Zeichencode wird zur Lücke (AC-12); die Anbindung macht B-469.
- **Session-Größe:** je Session eine Domäne, ein Commit, ≤ ~400 geänderte Code-Zeilen; Code-Review vor jedem Commit.
- **Sprintgröße:** 5 Sessions (3 Umsetzung, Review, Abnahme) liegen im Richtwert 3–6.
- **Client rechnet nichts** (`src/scenes/noSim.test.ts`); Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Prüfung:** Vitest im Studio-Frontend über `task check:all`; Fensterprüfungen nur mit Freigabe von 🧑 für den Lauf; die Review-Session läuft nie im selben Lauf wie eine Umsetzung.
- **Begriffe** nach `docs/glossar.md` (Biom, Tiefe, Miniboss, Endboss); neue Begriffe vor der Freigabe eintragen.

## Beispiele

`B-464 › Beispiele`, dazu `B-463 › Beispiele`. Sprint-eigen:

- Gruppe Wald → `greed`, `goblin`, `goblinArcher` (Portal), danach `wolf` (Nacht); `alphaWolf` steht in der Gruppe „ohne Biom“.
- `mineGhost` › Laufen → Sheet `ghost` hat kein `run`, die Vorschau zeigt `idle` und „Lücke: run fehlt“.
- Bosse › Tiefe 0 → Goblin-Anführer: Lücken nur für Boss-Leiste und Event-Banner, keine Warnkreis-Lücke, keine Phasen. Bosse › Tiefe 4 → Kristallherz-Wächter: Lücken für Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner, Grafik mit Rückfall `goblin` und Lücke „keine eigene Boss-Grafik“.
- Monarch-Vergleich an → Monarch 0 beritten auf `horse` neben dem Höhlentroll auf derselben Bodenlinie.
- Entwicklerseite (`dev.html`) nach WZ11.3 → Abschnitt Entwicklung ohne „Unsere Aufstellung“ und „Alle Figuren“; `lizenzen.html` und `grafiken.html` funktionieren unverändert; `git grep -n -E "figuren\.html|aufstellung\.html" -- . ":!docs/backlog" ":!docs/sprints"` findet nichts.

## Ausnahme- und Fehlerfälle

`B-464 › Ausnahme- und Fehlerfälle`, dazu `B-463 › Ausnahme- und Fehlerfälle`. Sprint-eigen:

- Abspiel-Bausteine aus WZ10.1 fehlen (WZ10 nicht erledigt) → WZ11.1 startet nicht, die Session setzt `Status: blockiert`.
- Boss- oder Gegner-ID ohne Eintrag in `sprites.json` › `enemies` → Rückfall `goblin` wie im Spiel, Lücke „keine eigene Grafik“ bzw. „keine eigene Boss-Grafik“.
- Biom nennt einen unbekannten Gegner → Karte mit Lücke „Gegner fehlt in data/enemies.json“, kein Absturz.
- Endboss-Phase ohne `ability` → die Phasen-Karte nennt die Fähigkeit der vorigen Phase.
- Importiert nach WZ11.3 noch Code `allFigures` oder `lineup` (z. B. das Studio), bleiben die Exporte und die Session nennt den Grund im Ergebnis.
- Findet WZ11.3 eine Karte der alten Seiten ohne Ort im Studio, setzt die Session `Status: blockiert` mit Frage-Ticket statt zu entfernen (AC-19).

## Akzeptanzkriterien

AC-01 bis AC-16 entsprechen B-464/AC-01 bis B-464/AC-16 mit gleicher Nummer. AC-17 ist die Abnahme, AC-18 und AC-19 sind die Kriterien des Entfernens aus B-463 (Ticket B-463 gehört zu WZ10, diese Kriterien laufen hier).

- **AC-01** `B-464/AC-01` alle 17 Gegnertypen gelistet (Test).
- **AC-02** `B-464/AC-02` Gruppen je Biom nach Tiefe, `portal` vor `night`, Gruppe „ohne Biom“ (Test).
- **AC-03** `B-464/AC-03` Gegner-Animationen einzeln mit `tint`, `scale`, `alpha`, `y`.
- **AC-04** `B-464/AC-04` fehlende Animation → `idle` und Lücke (Test).
- **AC-05** `B-464/AC-05` Getroffen `hit`, Sterben `kill`, eigene Animationen als Lücke.
- **AC-06** `B-464/AC-06` Merkmale an der Karte, ohne Darstellung als Lücke.
- **AC-07** `B-464/AC-07` Boss-Gruppe je Tiefe mit 6 Bossen (Test).
- **AC-08** `B-464/AC-08` Boss-Grafik wie im Spiel, Lücke „keine eigene Boss-Grafik“.
- **AC-09** `B-464/AC-09` verschoben nach `B-469/AC-01` (Warnkreis); in WZ11 nicht zu erfüllen, die Lücke deckt AC-12 ab.
- **AC-10** `B-464/AC-10` verschoben nach `B-469/AC-02` und `B-469/AC-03` (Boss-Leiste, Endboss-Phasen); in WZ11 nicht zu erfüllen, die Lücke deckt AC-12 ab.
- **AC-11** `B-464/AC-11` verschoben nach `B-469/AC-04` (Event-Banner für `bossSpawned`, `bossPhase`, `bossDefeated`); in WZ11 nicht zu erfüllen, die Lücke deckt AC-12 ab.
- **AC-12** `B-464/AC-12` je Boss die zutreffenden Effekte (Warnkreis nur bei Flächenschlag, Phasen-Text nur beim Endboss mit Phasen, Boss-Leiste und Event-Banner bei jedem Boss, zusammen 15 Einträge) als Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“, kein Ersatz im Studio (Test, Review).
- **AC-13** `B-464/AC-13` Spielmaßstab, Monarch-Vergleich zuschaltbar.
- **AC-14** `B-464/AC-14` Gegner-Seite nutzt die Abspiel-Bausteine aus WZ10.1 und die Bühne aus WZ7.3; Bühne, Steuerung und Lücken-Markierung gibt es im Studio je nur einmal (Review).
- **AC-15** `B-464/AC-15` Grafik-Auswahl je Gegner-ID und `boss`, Register unverändert.
- **AC-16** `B-464/AC-16` Vitest der Gegner-Logik in `task check:all` und CI.
- **AC-17** 🧑 hat die Gegner-Seite mit den Bossen im Asset-Studio am PC abgenommen, die Lücken-Liste gesehen und bestätigt, dass `figuren.html`/`aufstellung.html` nichts Fehlendes hinterlassen (Abnahme-Session).
- **AC-18** `B-463/AC-13` `figuren.html`, `aufstellung.html`, `src/tools/figuren.ts`, `src/tools/aufstellung.ts` samt Kacheln, Texten (`tile.figuren.*`, `tile.aufstellung.*`, `sel.figuren`, `sel.aufstellung`) und Verweisen außerhalb der Historie entfernt; `spriteReference.ts` bleibt, nur `allFigures` und `lineup` entfallen; `git grep -n -E "figuren\.html|aufstellung\.html" -- . ":!docs/backlog" ":!docs/sprints"` findet nichts, `task check` und `task build` grün.
- **AC-19** `B-463/AC-14` vor dem Entfernen hat jede Karte der alten Seiten einen Ort im Studio (Monarch-Seite, Gegner-Seite, Figuren-Szene oder Katalog der Grafik-Seite); das Ergebnis der Session nennt die Zuordnung je Gruppe.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat): `B-464 › Offene Fragen` 1–5 (Bosse als Text und Lücke, Gruppierung nach Biom, Technik und Ersatz der Referenzseiten, Boss-Effekte als Lücke mit Anbindung in B-469, Bühne aus WZ7.3), `B-463 › Offene Fragen` 3 (`figuren.html` und `aufstellung.html` entfallen) sowie die Teilung von WZ10 in WZ10 (Monarch) und WZ11 (Gegner, Bosse, Entfernen).

**Ort für Truppen und ungenutzte Sheets** (`B-463 › Offene Fragen` 4): entschieden von 🧑 am 2026-10-17 (Chat): Truppen in der Figuren-Szene der Grafik-Seite (WZ6.3), ungenutzte Sheets nur im Katalog der Grafik-Seite (statisch). Gilt für AC-19 und WZ11.3.

**Tempo-Stufen** (`B-463 › Offene Fragen` 5): entschieden mit WZ10 (0,5×, 1×, 2×).

Vor der Freigabe offen: nichts.

**Neue Begriffe im Glossar** (eingetragen mit PR 349): Gegner-Seite, Warnkreis, Boss-Leiste, Event-Banner; Lücke und Abspiel-Baustein kommen aus WZ10.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ11.1 | `WZ11.1-gegner-seite.md` | Umsetzung | autonom | fertig |
| WZ11.2 | `WZ11.2-bosse.md` | Umsetzung | autonom | fertig |
| WZ11.3 | `WZ11.3-figuren-seiten-entfernen.md` | Umsetzung | autonom | fertig |
| WZ11.4 | `WZ11.4-review.md` | Review | autonom | fertig |
| WZ11.5 | `WZ11.5-abnahme.md` | Review | Mensch | offen |

- WZ11.1 (DEV) Gegner-Seite: Gegnertypen nach Biom, Animationen mit `tint`/`scale`/`alpha`/`y`, Rückfall `idle`, Getroffen/Sterben, Merkmale, Monarch-Vergleich, Grafik-Auswahl je Gegner-ID (AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-13, AC-14, AC-15, AC-16)
- WZ11.2 (DEV) Bosse je Tiefe mit Name, Art, Fähigkeit, Phasen, Boss-Grafik wie im Spiel und den Lücken je Boss für die zutreffenden Effekte (Warnkreis, Boss-Leiste, Phasen-Text, Event-Banner); Anbindung später in B-469 (AC-07, AC-08, AC-12, AC-15, AC-16; AC-09 bis AC-11 nach B-469 verschoben)
- WZ11.3 (DEV) `figuren.html` und `aufstellung.html` samt Skripten, Kacheln, Texten und Verweisen entfernen; Grenzfall DEV wie WZ6.6/B-299; `spriteReference.ts` bleibt, nur `allFigures`/`lineup` entfallen; vorher Zuordnung je Gruppe festhalten (AC-18, AC-19)
- WZ11.4 (DEV) Review, autonom, mit Lücken-Liste (Gegner, Bosse) für 🧑 (alle; AC-09 bis AC-11 sind nach B-469 verschoben, AC-17 folgt in WZ11.5)
- WZ11.5 (DEV) Abnahme am PC, Agent: Mensch (AC-17)

Bei `Reife: Entwurf` genügen Stichpunkte; vor dem Aktivieren wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-18, Review WZ11.4 (autonom, getrennter Reviewer): AC-01 bis AC-08, AC-12 bis AC-16, AC-18, AC-19 geprüft; AC-09 bis AC-11 verschoben (B-469); AC-17 offen (WZ11.5).
Behoben: AC-06 (Zusatzfelder `swarmSize`, `aoe`, `phases`, `kiteDistance` an der Karte, mit Lücke). Keine neuen Tickets.
Lücken-Liste und Abnahme-Liste: `WZ11.5-abnahme.md` › Kontext.
Version: v– vorgeschlagen (keine, reines Entwickler-Werkzeug).
