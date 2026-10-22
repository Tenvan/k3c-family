# BD4.1 · HUD-Leiste oben und Kompass je Feld, Layout-Tests

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sprint/bd4
- **Abhängig von:** –
- **Tickets:** B-413
- **Kriterien:** AC-01, AC-02, AC-04

## Ziel

Die Kriterien AC-01, AC-02 und AC-04 der HUD-Leiste sind durch Layout-Tests belegt (die Leiste selbst kam mit #277 auf develop).

## Kontext

Leiste, Spielerkarten und Kompass oben im Feld stehen seit B-413/#277 in `src/scenes/hud/hudLayout.ts` (`BAR`, `hudItems`), Zellen in `src/scenes/layout.ts` (`BAR_HEIGHT` 134, `WORLD_TOP` 420). `hudLayout.test.ts` prüft Überlagerung, Home-Button, Touch-Flächen, Debug-Bereich; es fehlen gezielte Tests für AC-01 (alle gemeinsamen Anzeigen nur in der Leiste, 1 bis 4 Spieler) und AC-02 (Kompass je Feld oben, oberstes Feld unter der Leiste). Der POC-Branch `poc/hud-leiste-fluss` existiert nicht mehr; sein Stand steckt in develop.

## Erlaubte Dateien

- `src/scenes/hud/hudLayout.test.ts`
- `src/scenes/hud/hudLayout.ts` (nur falls ein Test einen echten Fehler zeigt)
- `src/scenes/hud/codemap.md`

## Nicht-Ziele

Neue Grafiken; Fluss (BD4.2); Austreten (BD4.3).

## Schritte

1. Test zuerst: AC-01 für 1, 1 mit Partner, 2, 3, 4 Spieler: `shared`, `clock`, `fight`, `citizens`, `info`, `places` liegen in `BAR` und schneiden kein Spielerfeld.
2. AC-02: `radar:<i>` liegt in seiner Zelle, nahe deren Oberkante; Zellen beginnen bei `BAR_HEIGHT`, kein Radar überlappt die Leiste.
3. AC-04 steht bereits (Home-Button, Touch 16:9 und 4:3); bestätigen.
4. Weicht ein Test ab, `hudLayout.ts` korrigieren.

## Fertig, wenn

- [ ] AC-01: neuer Test grün.
- [ ] AC-02: neuer Test grün.
- [ ] AC-04: bestehender Test grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
