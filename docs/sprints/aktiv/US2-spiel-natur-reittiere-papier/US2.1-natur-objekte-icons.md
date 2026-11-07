# US2.1 · Natur, Objekte, Münzen und Icons als Papierschnitt

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** us2/1-natur-objekte-icons
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-01

## Ziel

Natur-Knoten, Objekte (Truhe, Portal, Ausgang, Stern, Schild), Münzen und Icons kommen aus `public/grafik/objekte/` und `public/grafik/icons/`.

## Kontext

`src/scenes/render/natureSprites.ts` (`NATURE_TEXTURES`, `nodeSprite`), `worldSprites.ts` (`WORLD_TEXTURES`, `OBJECTS`), `objectView.ts` (laden, Filter, Animationen), `mineSignView.ts`, `hints/guideHints.ts`. Papier-Sheets sind Streifen (`idle.png`, Frames nebeneinander), daher als Spritesheet laden. Maße stehen in `data/paper.json` › `objekte`.

## Erlaubte Dateien

- `src/scenes/render/`, `src/scenes/hints/guideHints.ts`
- `data/paper.json`, `tests/`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Gerüste, Plantagen-Pappel ohne Papier-Asset (D-Zeile).

## Schritte

1. Test (rot): Natur und Objekte ohne `gebaeude/*-32.png` und alte Packs, außer Lücken.
2. Zuordnung und Maße in `data/paper.json`, Test gegen Manifeste.
3. Laden als Spritesheet, Ursprung und Maßstab aus dem Rahmen, Animation bei mehreren Frames.
4. `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01: Test grün.
- [ ] `task check:diff` grün.

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
