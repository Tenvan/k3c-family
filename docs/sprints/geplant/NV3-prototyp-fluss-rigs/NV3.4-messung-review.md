# NV3.4 · Messung am Entwicklungs-PC und Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** live
- **Branch:** nv3/4-messung-review
- **Abhängig von:** NV3.3
- **Tickets:** B-789
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Messbericht vom Entwicklungs-PC und Review des Sprints.

## Kontext

`native/measure.ps1`, `task native:ebiten:measure`. Je Fassung (Rig, Sheet) ein Lauf, keine Wiederholungen ohne Anlass. Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Batching, Allokationen im Frame, Texturbudget).

## Erlaubte Dateien

- `native/**`
- Planung B-789, NV3 (über MCP)

## Nicht-Ziele

Messung am Mini-PC (B-786); Simulations- und Balancing-Läufe (gesperrt).

## Schritte

1. Messlauf Rig und Sheet im selben Level, Bericht nach `reports/`.
2. Review des Sprint-Diffs, schwere Befunde beheben.
3. Werte in B-789 › Notizen, Abnahme in der Sprint-README, PR.

## Fertig, wenn

- [ ] AC-04: Messbericht mit Bildrate, Grafikspeicher, Rechenzeit, Rig gegen Sheet.
- [ ] AC-01 bis AC-03 aus den Ergebnissen von NV3.1 bis NV3.3 bestätigt.

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
