# PL1.1 · „Neues Spiel“ nachweisen, Ursache Overlay auf der Xbox

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** pl1/1-neues-spiel-overlay
- **Abhängig von:** –
- **Tickets:** B-195
- **Kriterien:** AC-01, AC-03

## Ziel

Der Nachweis für „Neues Spiel startet immer neu“ (B-292/AC-01, AC-03) liegt vor, und die Ursache, warum das Debug-Overlay auf der Xbox nicht aufgeht, steht mit Code-Pfad und Beobachtung in B-195; liegt die Ursache in der Eingabe (PLAT), ist sie behoben.

## Kontext

**2026-10-07:** B-292 („Neues Spiel“) ist nach LP1 gewechselt (Beschluss 🧑); alle B-292-Schritte und die PL1/AC-01-Punkte dieser Session entfallen.

- **B-292 ist im Code schon umgesetzt** (Commit `f07d6fae`, PR #153): `src/landing/pages.ts` › `newGameHref(now)` liefert `game.html?fresh=1&save=neu-<Zeit base36>`, die Kachel „Neues Spiel“ hat `href: () => newGameHref()` und die Beschreibung „… frischer Spielstand mit eigenem Namen … alte Spielstände bleiben“. `src/landing/pages.test.ts` prüft `fresh=1` und den Namen. Diese Session prüft nur nach (Format `^[a-z0-9-]{1,32}$`, zwei Zeitpunkte = zwei Namen, Beschreibung ohne „Seed k3c“ und ohne „alter Spielstand wird gesichert“) und ergänzt den Test, falls ein Teil fehlt. B-292/AC-02 (Browser mit vorhandenem Stand `familie`) prüft 🧑 in PL1.5.
- **B-195, Stand im Code:** Das Ticket nennt noch LS (Taste 10). Heute öffnet `src/scenes/debugOverlayView.ts` die Diagnose mit Ö, RB 3 s halten oder Doppeltap mit einem Finger, den Cheat-Dialog mit Ä, LB + RB 3 s halten oder Doppeltap mit zwei Fingern (B-093, B-231; `src/scenes/debugGestures.ts`: `HOLD_MS = 3000`, `holdStep`). Erzeugt wird das Overlay nur, wenn `debugEnabled(location.search)` gilt (`src/scenes/debugOverlay.ts`: `?dev=0` schaltet ab). Die Pad-Indizes stehen in `src/input/slotBindings.ts` › `PAD` (LB 4, RB 5).
- **Rückfrage aus B-195 bleibt offen, blockiert nicht** (Beschluss 🧑 2026-10-06): Auf welcher Seite (`game.html`, `testing.html`) und ob mit `?dev=0` versucht wurde, ist nicht geklärt. Deshalb alle vier Fälle im Browser-Pane nachstellen.
- **Nachstellen im Browser-Pane** (`CLAUDE.md` › Im Browser-Pane testen): Die Seite liegt im iframe der Shell (`document.getElementById('frame').contentWindow`); Controller mocken mit `navigator.getGamepads = () => [pad, null, null, null]`, `mapping: 'standard'`, 17 `buttons`, 4 `axes`, nach jeder Änderung `pad.timestamp = performance.now() + 100000`; bei verdecktem Pane Frames mit `game.loop.step(t)` takten.
- **Domäne:** Eingabe (`src/input/`) ist PLAT, das Overlay selbst (`src/scenes/debugOverlay*.ts`, `debugGestures.ts`) ist CLI. Liegt die Ursache in `src/scenes/`, wird sie nicht hier behoben, sondern als Ticket (Domäne CLI) angelegt; AC-03 ist dann für den Code-Teil mit Ticket verschoben.
- Regeln: B nicht belegen, View + Menu reserviert, keine Spieltaste für das Overlay; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `src/landing/pages.ts`, `src/landing/pages.test.ts`
- `src/input/` (nur falls die Ursache in der Eingabe liegt, mit Test daneben)
- `docs/backlog/B-195-debug-overlay-xbox.md` (Ursache unter „Notizen“), `docs/backlog/B-292-neues-spiel-eindeutiger-name.md` (nur Status)
- `docs/sprints/geplant/PL1-start-tastatur-koop-texte/`, `docs/sprints/aktiv/PL1-start-tastatur-koop-texte/`, `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Server oder Protokoll ändern (B-292); Namensdialog in der Lobby (B-105); Änderungen an `src/scenes/`; neue Overlay-Gesten; Abnahme an der Xbox (PL1.5).

## Schritte

1. Branch anlegen und pushen, `Status: in Arbeit`; `Start-Commit` in der Sprint-README setzen, falls noch `–`. `docs/arbeitsweise.md` und `docs/glossar.md` lesen.
2. B-292: `task test -- pages` ausführen; fehlt im Test das Namensformat `^[a-z0-9-]{1,32}$`, der Vergleich zweier Zeitpunkte oder die Prüfung der Beschreibung (kein „Seed k3c“, kein „gesichert“), ergänzen.
3. B-195: Dienste über k3c-dev starten (`svc_start`), im Browser-Pane `game.html` und `testing.html` je mit und ohne `?dev=0` öffnen, RB 3 s und LB + RB 3 s mit gemocktem Pad halten, Ö und Ä drücken; je Fall notieren, ob das Overlay erzeugt wird und aufgeht.
4. Ursache eingrenzen (z. B. Overlay nicht erzeugt, Pad-Zustand kommt nicht an, Halten wird unterbrochen) und mit Code-Pfad und Beobachtung unter „Notizen“ in B-195 schreiben. Lässt sie sich im Browser nicht nachstellen, das als Beobachtung eintragen und die Prüfschritte für 🧑 an der Xbox in PL1.5 konkretisieren.
5. Liegt die Ursache in `src/input/`: Test zuerst, dann beheben. Liegt sie in `src/scenes/`: Ticket (Domäne CLI) anlegen, im Ergebnis nennen.
6. `task check` ausführen, Ergebnis schreiben, `Status: fertig`, Tabelle der Sprint-README anpassen.

## Fertig, wenn

- [ ] AC-01: `task test -- pages` grün; der Test prüft `fresh=1`, das Namensformat, verschiedene Namen zu verschiedenen Zeiten und die Kachelbeschreibung (B-292/AC-01, AC-03).
- [ ] AC-03: B-195 nennt unter „Notizen“ die Ursache mit Code-Pfad und Beobachtung aus den vier nachgestellten Fällen (B-195/AC-01); eine Ursache in `src/input/` ist mit Test behoben, eine in `src/scenes/` steht als Ticket.
- [ ] `task check` grün; keine Datei über 400 Zeilen, keine Funktion über 60.

## Prüfen

```bash
task test -- pages
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
