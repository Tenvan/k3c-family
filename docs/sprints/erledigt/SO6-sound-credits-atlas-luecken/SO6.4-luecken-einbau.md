# SO6.4 · Slots, Credits und Einbau der Sounds für die Lücken

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** so6/4-luecken-einbau
- **Abhängig von:** SO6.2
- **Tickets:** B-492
- **Kriterien:** AC-02, AC-03

## Ziel

Münze geben, Bau-Fortschritt, Wiederbeleben durch Mitspieler, Boss-Auftritt und Hub-Ausbau haben je einen Slot mit Sound und klingen im Spiel; Skill je Klasse, UI-Klicks und Reiten stehen mit Begründung im Katalog.

## Kontext

B-492, Default B-558: Sounds aus schon vorhandenen Kenney-Dateien (CC0, Credits liegen vor). Auslöser: `coinGive`, `buildProgress`, `revived`, `bossSpawned` (global) aus den Ereignissen; Hub-Ausbau als Anstieg von `hubLevel` zwischen zwei Snapshots (reiner Vergleich, kein Spielzustand). Nach den neuen Slots `task gen:soundatlas`.

## Erlaubte Dateien

- `data/assets.json` (nur neue Effekt-Slots), `docs/assets/sounds.md`, `public/audio/atlas.*`
- `src/audio/` (Mapping, Tests), `src/scenes/GameScene.ts` (nur Hub-Stufe an den Audio-Kern melden)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Reiten-Schleife (Kern-Erweiterung, Ticket bei Wunsch), UI-Klänge, Skill je Klasse, neue Fremd-Sounds.

## Schritte

1. Fünf Slots in `data/assets.json`.
2. `cueFor` um `coinGive`, `buildProgress`, `revived`, `bossSpawned` erweitern; Hub-Ausbau als reine Funktion (vorher/nachher) mit Test.
3. Katalog nachziehen (Zeilen mit Slot; Lücken mit Begründung und Ticket).
4. Atlas neu erzeugen, `task check:diff`.

## Fertig, wenn

- [ ] AC-03 (B-492/AC-01): Jede Lücken-Zeile hat Slot und Sound oder Begründung mit Ticket (Katalog-Test).
- [ ] AC-03 (B-492/AC-02): Jede neue Datei hat Credit (keine neue Datei; Test).
- [ ] AC-02: Atlas enthält die neuen Slots.

## Prüfen

```bash
task check:diff
```

## Ergebnis

2026-10-22, autonom (Claude). Auswahl ist der Default aus B-558 (SO6.3 bleibt bei 🧑).

- **AC-03 (B-492/AC-01) umgesetzt, geprüft:** fünf neue Slots in `data/assets.json` (`sfx:coinGive`, `sfx:buildProgress`, `sfx:revived`, `sfx:boss`, `sfx:hub`) mit vorhandenen Kenney-Dateien (CC0); `cueFor` als Tabelle (statt `switch`, Komplexitäts-Grenze) mit `coinGive`, `buildProgress`, `revived`, `bossSpawned` (global); Hub-Ausbau: reine Funktion `hubRaised` + `AudioCore.onHubLevel` aus `world.hubLevel` (je Frame, neues Spiel vergisst den Stand). Katalog: Skill je Klasse, UI-Klicks als Lücke mit Begründung (B-558), Reiten „Einbau offen“ (B-558). Tests: `events.test.ts`, `audioCore.test.ts`, `soundKatalog.test.ts`.
- **AC-03 (B-492/AC-02):** keine neue Datei; Credits liegen vor (Test grün).
- **AC-02:** Atlas neu erzeugt, 16 Effekte (Test „Atlas enthält jeden aktiven Effekt-Slot“).
- `GameEvent` kennt jetzt `bossSpawned` (wie `docs/protocol.md` und `engine/sim/boss.go`, keine Protokolländerung). `GameScene` +2 Zeilen (Hub-Stufe an den Audio-Kern).
- `task check:diff` grün. Hören am TV: SO6.6.
