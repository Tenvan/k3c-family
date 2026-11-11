# NG3.3 · Bürger, Truppen, Gegner und Bosse

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** figuren
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng3/3-figuren-alle
- **Abhängig von:** NG3.2
- **Tickets:** B-796
- **Kriterien:** AC-03

## Ziel

Bürger, Truppen, Gegner und Bosse erscheinen mit ihren Zuständen; Figuren mit Teile-Baum als Rig, `requisite` als Sheet.

## Kontext

Spec: B-796. Figurentypen in `data/` (Truppen, Gegner, Bosse) und `engine/sim/unit_*.go`, `enemies*.go`, `boss*.go`. Vorbild: `src/scenes/render/citizenView.ts`, `worldSprites.ts`. Rig-Typen: 6 Truppen, 7 Gegner, 5 Bosse (`figur`), 11 Reittiere, 2 Gegner (`vierbeiner`); 8 Gegner und 1 Boss `requisite` (Sheet). Vertrag und Importregel wie NG3.2.

## Erlaubte Dateien

- `native/k3c-game/actors/**`
- Planung NG3, B-796 (über MCP)

## Nicht-Ziele

Neue Figuren; Effekte über Figuren (NG6).

## Schritte

1. Test zuerst: jeder Figurentyp aus `data/` hat Rig oder Sheet; Zustand → Animation je Gruppe.
2. Bürger und Truppen je Beruf.
3. Gegner und Bosse, `requisite` über Sheets aus `art`.

## Fertig, wenn

- [ ] AC-03: Test über alle Figurentypen grün; Screenshot einer Nacht mit Gegnern im Ergebnis.
- [ ] `task check:diff` grün.

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
