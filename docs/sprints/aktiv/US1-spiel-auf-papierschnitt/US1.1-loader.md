# US1.1 · Spiel lädt Papierschnitt über die Gruppen-Indizes

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** us1/1-loader
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-01

## Ziel

Das Spiel lädt Figuren, Reittiere, Gebäude und Hintergründe aus `public/grafik/<gruppe>/index.paper.json`; kein Lade-Pfad zeigt mehr auf `atlas/` oder `grafik-db32/`.

## Kontext

Heute: `src/scenes/render/sprites.ts` lädt den Multiatlas `atlas/atlas.json`, `buildingSprites.ts` Pixel-Gebäude aus `grafik/gebaeude/*-32.png`. Papier-Ausgabe je Asset: `manifest.json` (je Animation `datei`, `frames`, `fps`, `frameW`, `frameH`, `rahmen` = [x, y, b, h] in Einheiten, Fußpunkt bei 0/0), Gebäude-Stufen `stand.s<n>.png` (D-15). Gruppen: figuren, truppen, gegner, bosse, reittiere, gebaeude-paper (MG6, D-MG6-01), hintergruende. Zuordnung alter Sheet-Namen zu Papier-Ids neu in `data/paper.json`. Schatten nach B-701 D-09.

## Erlaubte Dateien

- `src/scenes/render/`, `src/scenes/LoadScene.ts`, `src/scenes/render/*.test.ts`
- `data/paper.json` (neu)
- `tests/projectRules.test.ts`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-US1-xx)

## Nicht-Ziele

Alte Grafik und `gen:gebaeude` entfernen (CU1); Xbox-Skalierung; Natur und Objekte (kommen mit MG8).

## Schritte

1. Test (rot): Lade-Pfade der Render-Module ohne `atlas/` und `grafik-db32/`.
2. `paperSprites.ts`: Manifest → Spritesheets, Ursprung aus `rahmen`, Maßstab auf die bisherige Figurhöhe.
3. Figuren, Reittier-Monarch und Gebäude-Stufen umstellen; Schatten als abgedunkelte Sprite-Kopie.
4. Gezielte Tests, `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01: `tests/projectRules.test.ts` prüft, dass kein Lade-Pfad auf `atlas/` oder `grafik-db32/` zeigt; grün.
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
