# AZ1.3 · Review

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sprint/az1
- **Abhängig von:** AZ1.2
- **Tickets:** B-371, B-333
- **Kriterien:** alle

## Ziel

Sprint AZ1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, der PR des Sprints ist geöffnet; AC-03 und AC-04 stehen als `angenommen, Validierung offen (AZ1.4)`.

## Kontext

Leichter Review (nur schwere Befunde im Diff) durch einen unabhängigen Reviewer-Agenten mit frischem Kontext. Besonders prüfen:

- `src/input/` und `src/online/` unverändert; Tasten von Spieler 2 nur aus `KEYBOARD_2`, keine zweite Liste in `src/scenes/`.
- Mit einem Tastatur-Spieler zeigt alles Layout 1 wie vorher; Pad und Touch unverändert.
- Pause-Band verdeckt den Verbindungs-Hinweis nicht; `pauseAll`/`resumeAll` nur bei Zustandswechsel; nach Verlassen des Raums laufen Animationen wieder.
- `GameScene.ts` ≤ 400 Zeilen, Funktionen ≤ 60; Texte über `t()` in de und en; Schrift aus `FONTS`.

## Erlaubte Dateien

- `src/scenes/`, `src/core/texts.de.ts`, `src/core/texts.en.ts` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Umbau von Produktionscode; Browser- oder Geräteprüfungen (AZ1.4).

## Schritte

1. `Status: in Arbeit`, `task check` grün.
2. `git diff origin/develop...sprint/az1` lesen, Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium aus den Ergebnissen von AZ1.1 und AZ1.2 prüfen; AC-03 und AC-04 als `angenommen, Validierung offen (AZ1.4)` führen.
4. Abnahme (höchstens fünf Zeilen) mit Versionsvorschlag in die Sprint-README.
5. AZ1.4 ist offen: Sprint bleibt aktiv. Committen, `git merge origin/develop`, pushen, PR des Sprints öffnen (`sprint/az1` → `develop`).

## Fertig, wenn

- [x] AC-01 und AC-02 haben einen Nachweis im Ergebnis von AZ1.1; AC-03 und AC-04 stehen als `angenommen, Validierung offen (AZ1.4)`.
- [x] Schwere Befunde behoben (einer, siehe Ergebnis).
- [x] `task check` grün; PR des Sprints geöffnet.

## Prüfen

```bash
task check
```

## Ergebnis

- **Review:** unabhängiger Reviewer-Agent (frischer Kontext) las `git diff origin/develop...sprint/az1`; AZ1.1 ist bereits in `develop` (#253), der Diff enthält AZ1.2. `task check` und `task check:go` liefen danach selbst: grün.
- **Befund (schwer, behoben):** `GameScene.leaveRoom()` stoppt das HUD; stand der Raum in der Dev-Pause, blieb `anims.pauseAll()` global wirksam und im nächsten Raum standen alle Animationen still. Fix: Shutdown-Handler in `HudScene.create()` ruft `resumeAll()`.
- **AC-01, AC-02:** Nachweis in AZ1.1 (`glyphs.test.ts`, Browser-Pane). **AC-03, AC-04:** angenommen, Validierung offen (AZ1.4); AC-03 hat den Test der Anzeige-Funktion, AC-04 ist umgesetzt, aber nicht beobachtet.
- **Prüfung:** `task check` und `task check:go` grün.
- **Abweichungen:** keine; kein Browser-Pane-Lauf (nicht freigegeben).
- **Sprint:** bleibt `aktiv` (AZ1.4 offen). **Version:** v0.18.0 vorgeschlagen, nicht gesetzt.
