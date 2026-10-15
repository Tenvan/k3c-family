# BD1.5 · Review BD1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** bd1/5-review
- **Abhängig von:** BD1.4
- **Tickets:** B-392, B-391, B-389, B-399, B-398, B-414, B-393, B-370, B-374
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

BD1 ist abgenommen: alle Kriterien AC-01 bis AC-05 mit Nachweis, schwere Befunde behoben, der eine PR des Sprints ist offen und Auto-Merge eingeschaltet.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session: leicht, nur schwere Befunde (falsches Verhalten, Sicherheit, Determinismus/2 Spieler, Regeln aus `CLAUDE.md`, Kriterium ohne Nachweis). Nur der Diff `origin/develop...origin/sprint/bd1` wird gelesen. Domäne CLI. Läuft nie im selben Lauf wie eine Umsetzung.

Besonderheiten BD1:
- AC-01 und AC-02 (Kamera, Beitritt, Ö) sowie die Beobachtungen aus AC-03/AC-04 sind Abnahmen am PC; sie bleiben bei 🧑 und werden als `angenommen, Validierung offen` geführt, falls nicht bestätigt.
- B-374 hat keine Code-Änderung; der Diff darf `src/scenes/siteView.ts` nicht berühren.
- Hat BD1.4 mangels Mond-Bild blockiert, ist der Sprint nicht abnahmefähig: Review nicht beginnen.
- W6 berührt dieselben Dateien (`HudScene.ts`): Merge-Stand prüfen.

## Erlaubte Dateien

- Befunde beheben nur in den Dateien der Sessions BD1.1–BD1.4 (`src/scenes/`, `src/core/texts.*`, `docs/assets/zuordnung-welt.md`); außerhalb → Ticket
- Planung: Sprint-README BD1 (Abnahme, Status), `docs/sprints/README.md`, `docs/projekte/` (Sprint-Tabelle), Tickets B-392, B-391, B-389, B-399, B-398, B-414, B-393, B-370, B-374 (Status, Archiv)

## Nicht-Ziele

Stil, Benennung, Doku-Feinschliff und mögliche Vereinfachungen (kein Befund); neue Funktionen; manuelle Abnahme am Gerät anstelle von 🧑; einen Versions-Tag setzen (nur Vorschlag).

## Schritte

1. `check_run` (`task check`, `task check:go`) → grün.
2. `git fetch`; `git diff origin/develop...origin/sprint/bd1` lesen, nur den Diff.
3. Befunde nach der Liste in Kontext prüfen; schwere beheben (Domäne CLI), sonst Ticket.
4. Kriterien AC-01 bis AC-05 gegen die Session-Ergebnisse abgleichen; offene PC-Abnahmen als `angenommen, Validierung offen (Session)` führen.
5. Abnahme in die Sprint-README (≤ 5 Zeilen: Datum, Kriterien, behobene Befunde, neue Tickets), `Version: v… vorgeschlagen (Grund)`.
6. Erledigte Tickets (B-374: erledigt, entschieden) nach `backlog/archiv/` und in den Index-Abschnitt „Archiv“; Sprint-Ordner nach `sprints/erledigt/`, `Status: erledigt` (nur wenn keine Hardware-Session offen); Fahrplan anpassen.
7. Commit, `git merge origin/develop`, push; **den einen PR** des Sprints öffnen (Vorlage, eine Zeile je Session), danach `gh pr merge --auto --squash` (ist der PR schon `CLEAN`: `gh pr merge --squash`).

## Fertig, wenn

- [ ] AC-01 bis AC-05: je Kriterium Nachweis im Ergebnis (`geprüft`, `umgesetzt`, `angenommen, Validierung offen` oder `verschoben` mit Ticket).
- [ ] `task check` und `task check:go` grün; keine schweren Befunde offen.
- [ ] Abnahme in der Sprint-README steht (≤ 5 Zeilen), Versionsvorschlag eingetragen.
- [ ] Der eine PR des Sprints ist offen, Auto-Merge eingeschaltet; Ordner und Fahrplan stimmen.

## Prüfen

```bash
task check
task check:go
git fetch && git diff origin/develop...origin/sprint/bd1 --stat
```

## Ergebnis

Review 2026-10-15 im Hauptcheckout auf `develop` (kein Worktree). Sprint-Diff: `61b680ee` (PR #299, BD1.1–BD1.3 und Planung) plus `3d7a47c0`, `cb5f3119`, `8ff000ea` (BD1.4, Mond als Emoji-Textur); `origin/sprint/bd1` existiert nicht mehr, daher kein Diff `origin/develop...origin/sprint/bd1`.

- **Schritt 1 geprüft:** `task check` und `task check:go` über `check_run`: beide Exit 0.
- **Schritt 2–3 geprüft:** Diff von `src/scenes/` (GameScene, HudScene, `cameraFollow`, `controlsHint`, `goldFlash`, `bannerQueue`, `joinHint`, `guideHints`, `guideOverlay`, `hudLayout`, `worldRenderer`, `fontRules`) gelesen, dazu ein unabhängiger Reviewer-Agent. Kein `Math.random()`, keine Spiel-Logik im Client, B-Taste und View + Menu unberührt, `siteView.ts` unberührt (B-374), Kamera und Beitritts-Hinweis je Spieler getrennt.
- **Schwerer Befund behoben (B-399/AC-03):** Mit 2 Spielern an zwei Tastaturen (oder Controller und Tastatur) wechselt `lastDevice` ständig; `stepControlsHint` startete die 20 s dabei jedes Mal neu, der Steuerhinweis verschwand nie. Fix: Ein Gerätewechsel startet die Zeit nur noch mit einem Platz (`controlsHint.ts`), neuer Test für 2 Spieler. Außerdem das Log-Emoji des fehlenden Hinweis-Bilds von 🐛 auf 🤒 (Spec „Ausnahme- und Fehlerfälle“).
- **AC-01 umgesetzt, geprüft:** `cameraFollow.test.ts` (5 Tests); Abnahme am PC von 🧑 bestätigt (BD1.1, 2026-10-14).
- **AC-02 umgesetzt, geprüft:** `joinHint.test.ts` (5 Tests); B-389 nicht reproduzierbar, keine Änderung; Abnahme am PC von 🧑 bestätigt (BD1.2, 2026-10-14).
- **AC-03 umgesetzt, geprüft (Test); angenommen, Validierung offen (BD1.3):** `controlsHint`, `goldFlash`, `bannerQueue`, `hudLayout`, `goldOnlyInHud` getestet; Sichtprüfung am PC bei 🧑 (laut Memory ruhen HUD-Abnahmen bis B-337/AC-05).
- **AC-04 umgesetzt, geprüft (Test); angenommen, Validierung offen (BD1.4):** Münz-Bild nur mit geladener Textur, sonst Text und Log 🤒 (Ursache aus S8.4 nicht belegt: frisches Profil prüfen); Mond als Canvas-Textur aus dem Emoji 🌙 (B-370, Entscheidung 🧑), Optik/Systemfont am PC und Xbox offen; Preisschild ohne Taste (B-374, `siteView.ts` unverändert).
- **AC-05 geprüft:** `task check` grün, Texte aus `texts.de.ts` (Text-Regel im Check).
- **Hinweis (kein Befund):** Der Beitritts-Hinweis nennt nach dem ersten Beitritt auch Tastaturen, solange sie keinen Spieler steuern, auch auf der Xbox ohne Tastatur. Verhalten nach Spec, bei Bedarf Ticket nach der Xbox-Abnahme.
- **Abweichungen vom Plan:** Der eine PR des Sprints (#299) wurde schon vor BD1.4 und diesem Review gemergt; die Commits `3d7a47c0`, `cb5f3119`, `8ff000ea`, `3a8cc0d2` und dieser Review-Commit liegen nur lokal auf `develop`. Nichts gepusht, kein PR geöffnet, kein Tag (Rückfrage bei 🧑).
- **Neue Tickets:** keine. Offene Abweichung aus BD1.3 bleibt: Der Steuerhinweis ist ein Element für alle Zellen (nicht je Zelle).
