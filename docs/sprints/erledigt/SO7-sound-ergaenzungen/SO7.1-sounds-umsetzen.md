# SO7.1 · Niederlage-Stück, Hufschlag, UI-Klicks; Skill-Sound je Klasse als Lücke

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** so7/1-sounds-umsetzen
- **Abhängig von:** –
- **Tickets:** B-573, B-574, B-575, B-576
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

Niederlage-Stück, Hufschlag und UI-Klicks sind eingebunden; B-576 ist als Lücke geführt.

## Kontext

B-573 bis B-576; Katalog `docs/assets/sounds.md`, Credits `public/audio/README.md`.

## Erlaubte Dateien

- `src/audio`, `public/audio`, `data/assets.json`, `docs/assets/sounds.md`
- `src/scenes/GameScene.ts`, `LobbyScene.ts`, `OptionsScene.ts` (je wenige Zeilen)

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Musikzustand defeat. 2. Hufschlag-Schleife. 3. UI-Klicks. 4. Katalog und Credits.

## Fertig, wenn

- [x] AC-01 bis AC-03 und AC-05; AC-04 verschoben (B-576).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 umgesetzt: Zustand `defeat`, Slot `music:niederlage` (selbst erzeugt, CC0).
AC-02 umgesetzt: `rideLoops`, Schleife `horse-gallop-loop-01.ogg`, eine je Reiter.
AC-03 umgesetzt: `sfx:uiSelect`, `sfx:uiConfirm` in Lobby und Optionen.
AC-04 verschoben (B-576): Der Snapshot kennt weder Klasse noch ein Skill-Ereignis; braucht Protokoll- und Engine-Arbeit.
AC-05 umgesetzt: Credit-Zeilen in `public/audio/README.md`.
Geprüft: `vitest run src/audio tests/projectRules` (152), volle Vitest-Suite (2600), `tsc`, oxlint. Hörprobe am TV offen (nur synthetisch erzeugt, Schleifennaht ungeprüft).
