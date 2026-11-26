# WK2.2 · Höchstens fünf markierte Steine (B-1252)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/wk2
- **Abhängig von:** WK2.1
- **Tickets:** B-1252
- **Kriterien:** AC-03

## Ziel

Höchstens fünf Steine sind gleichzeitig markiert.

## Kontext

B-1252: Steine bilden keine Ketten; Grenze fünf, wird nach dem Abbau wieder frei; Erzadern zählen als Markierung. Default: die Grenze gilt je Stufe für beide Spieler gemeinsam; eine sechste Markierung wird abgelehnt. Wert nach `data/`.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-game/**`
- `data/**`
- `docs/rules/**`

## Nicht-Ziele

Markieren von Bäumen ändern.

## Schritte

1. Test mit zwei Spielern.
2. Grenze in der Sim, Anzeige nur falls nötig.

## Fertig, wenn

- [ ] AC-03: Test grün.

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
