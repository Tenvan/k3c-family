# GR9.3 · Monarch-Animationen im Client abspielen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** gr9/3-monarch-einbau
- **Abhängig von:** GR9.2
- **Tickets:** B-505
- **Kriterien:** AC-02

## Ziel

Der Client spielt `give`, `hit`, `death` und `respawn` für den Monarchen (zu Fuß und beritten); fehlende Anim fällt auf `idle` zurück.

## Kontext

- `src/scenes/render/sprites.ts`: `AnimName`, `ANIMS`, `createSheetAnims` (repeat nur bei `attack` = 0), `playAnim` (Rückfall idle, laufender attack wird nicht unterbrochen). `tools/atlas/atlas.go` Zeile 17 `anims` muss dieselbe Liste kennen (Atlas packt die neuen PNG).
- `src/scenes/render/worldRenderer.ts` `updatePlayer` (einzige Stelle der Monarch-Animwahl; `respawnIn>0` = tot, bisher Deckkraft 0,25), `swing(id)` als Vorbild für Einmal-Animationen; `stageView.ts` Zeile ~48 ruft `swing`.
- Reiter: `mountView.ts` `styleRider` spielt immer `idle`; Einmal-Animationen müssen auch den Reiter-Sprite erreichen.
- Events (`src/model/types.ts`): `coinGive{player}` → give, `hit{target:'player',id}` → hit, `playerDown{player}` bzw. `respawnIn>0` → death (letzter Frame hält), `respawnIn` 1→0 bzw. `revived{player}` → respawn. Kein neues Protokollfeld, kein Server-Eingriff.
- Zwei Spieler: Animationen je Spieler-ID, unabhängig. Keine Logik im Client (nur zeichnen).

## Erlaubte Dateien

- `src/scenes/render/**`, `src/scenes/effects/**` (nur Einhängen), `tools/atlas/atlas.go` und `atlas_test.go`, Tests daneben
- `src/tools/spriteReference.ts` (Referenzseite, falls sie die Anim-Liste kennt)

## Nicht-Ziele

Sheets (GR9.2), Protokoll, Sim, Skill-Darstellung (SK2).

## Schritte

1. Anim-Liste in Client und Atlas erweitern (Wiederholung: nur idle/run Schleife, die anderen einmal; death hält letzten Frame).
2. Zustandslogik in `updatePlayer`/Eventhaken; Alpha 0,25 beim Tod nur, wenn keine death-Anim vorhanden.
3. Reiter-Pfad.
4. Unit-Tests (reine Auswahl-Funktion: Zustand/Event → Animname), `task gen:atlas`.

## Fertig, wenn

- [ ] AC-02 (Teil Einbau): Test belegt die Auswahl Event/Zustand → Animation und den Rückfall; `task check:diff` grün; TV-Abnahme in GR9.5.

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
