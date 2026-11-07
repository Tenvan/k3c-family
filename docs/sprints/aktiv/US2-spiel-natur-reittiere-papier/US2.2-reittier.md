# US2.2 · Berittener Monarch nach Reittier aus data/monarch.json

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** us2/2-reittier
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-02

## Ziel

Der berittene Monarch zeigt das Reittier aus `data/monarch.json` (`mount.sprite` → `reittiere/<id>` über `data/paper.json`), wenn das Monarch-Asset Gangarten dafür hat; sonst das Pferd.

## Kontext

`src/scenes/render/mountView.ts`, `mountGait.ts`, `paperAssets.ts`. Der Papier-Renderer setzt den Reiter per `animationen.<name>.reittier` auf jedes Reittier; das Monarch-Asset hat heute nur `pferd` (MG1 aktiv, Spec dort). Der Spielstand kennt kein Reittier je Spieler, nur den Default.

## Erlaubte Dateien

- `src/scenes/render/`
- `data/paper.json`, `tests/`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Monarch-Spec ändern; Reittier je Spieler im Protokoll.

## Schritte

1. Test (rot): Reittier-Schlüssel → Papier-Id und Animationsname, Rückfall Pferd.
2. Umsetzen in `mountView.ts`.
3. `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-02: Test grün.
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
