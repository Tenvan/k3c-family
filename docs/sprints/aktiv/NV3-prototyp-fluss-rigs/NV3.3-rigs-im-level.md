# NV3.3 · Figuren des Levels als Rig zeichnen

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** nv3/3-rigs-im-level
- **Abhängig von:** NV3.1, NV3.2
- **Tickets:** B-789
- **Kriterien:** AC-03

## Ziel

Im Level des Prototyps laufen Monarchen, Reittiere, Truppen, Gegner und Bosse mit Teile-Baum als Rig; `requisite`-Figuren bleiben Sheets.

## Kontext

`native/k3c-ebiten` bindet `native/k3c-rig/rig` und den Painter ein (`replace` im `go.mod` oder gemeinsames Paket). Umschalter Rig/Sheet für die Messung. Ein Reiter mit Pferd kostet als Rig etwa 0,037 ms (B-778); Posen gleicher Figuren in gleicher Phase teilen, wenn es ohne Umbau geht.

## Erlaubte Dateien

- `native/k3c-ebiten/**`
- `native/k3c-rig/**`
- `native/Taskfile.yml`

## Nicht-Ziele

Spiel-Logik, neue Animationen, Waffen- und Rüstungstausch für Gegner.

## Schritte

1. Rig-Paket und Painter in `k3c-ebiten` einbinden.
2. Figuren des Levels als Rig anlegen, Schalter `--art rig|sheet`.
3. Zeichenreihenfolge mit Ufer und Fluss prüfen (Figuren hinter dem Fluss-Vordergrund).

## Fertig, wenn

- [ ] AC-03: Alle Figuren des Levels mit Teile-Baum werden als Rig gezeichnet, Tests grün, Screenshot.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
