# SK2.3 · Darstellung der Skills im Spiel

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** sk2/3-skill-darstellung
- **Abhängig von:** – (Default aus Offene Fragen)
- **Tickets:** B-503
- **Kriterien:** AC-03

## Ziel

Wirkt ein Monarch einen aktiven Skill oder lernt er einen passiven, erscheint an ihm ein Ring in der Farbe der Linie.

## Kontext

Effekte: `src/scenes/effects/effects.ts` (`EffectKind`, `EFFECT_CONFIG`, `effectFor`), gezeichnet von `StageEffects` (`effectsView.ts`), ausgelöst in `GameScene.ts` (`spawnEffects` je Frame). Zustand je Spieler: `slots`, `cooldowns`, `skills` (`src/model/types.ts`). Default: keine neuen Ereignisse; Wirken = Abklingzeit eines Slots steigt zwischen zwei Frames (Sim setzt sie beim Wirken auf den vollen Wert), Lernen passiv = neuer Eintrag in `skills`. Reiner Vergleich wie `hubRaised` (SO6.4). Linien aus `MONARCH` (`src/model/data`).

## Erlaubte Dateien

- `src/scenes/effects/` (neue Datei `skillCasts.ts` + Test, `effects.ts`, `effectsView.ts`)
- `src/scenes/GameScene.ts` (nur Aufruf und Zurücksetzen)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Effekt je Skill, Sounds, Studio-Seite (Ticket), Animationen.

## Schritte

1. Effekt-Art `skill` mit Farbe je Linie (`LINE_COLORS`), `Effect.color` optional, `StageEffects` nutzt sie.
2. `SkillCasts.diff(players)`: je Spieler-Index Abklingzeiten und Skills merken, Effekte liefern; Test mit 2 Spielern.
3. `GameScene` ruft es je Frame, `init` setzt zurück.
4. Im Browser-Pane prüfen, sonst „nicht am Gerät geprüft“. `task check:diff`.

## Fertig, wenn

- [ ] AC-03 (B-503/AC-01): Vitest grün (Anstieg → Ring der Linie, passiver Skill gelernt → Ring, jede Linie mit Skills hat eine Farbe).
- [ ] AC-03 (B-503/AC-02): TV-Abnahme in SK2.5.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
