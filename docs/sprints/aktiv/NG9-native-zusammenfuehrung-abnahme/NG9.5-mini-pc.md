# NG9.5 · Sichtprüfung und Messung am Mini-PC

- **Status:** offen
- **Typ:** Review
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** –
- **Umgebung:** live
- **Branch:** ng9/5-mini-pc
- **Abhängig von:** NG9.2
- **Tickets:** B-786, B-780
- **Kriterien:** AC-05

## Ziel

Das native Spiel und der Rig-Monarch sind am Mini-PC geprüft und gemessen; Entscheidungen zu B-786 und B-780 sind notiert.

## Kontext

Gerät: Mini-PC

Spec: B-786 (ersetzt B-778/AC-06), Mini-PC-Teil von B-780/AC-01. Paket aus NG9.2 unter `.work/native-game/` mit `messen.cmd`. Wartet auf den Mini-PC (🧑 beschafft ihn, wenn das Spiel am PC fertig ist). Vergleichswerte Entwicklungs-PC: NG9.2 und NG3.4 unter `reports/`.

## Erlaubte Dateien

- Planung NG9, B-786, B-780, B-778 (über MCP)

## Nicht-Ziele

Code ändern; Befunde werden Tickets.

## Schritte

1. **Ralf:** Paket auf den Mini-PC bringen, Sichtprüfung zu zweit, Messung mit `messen.cmd`.
2. **Ralf:** Bericht zurück ins Repo (`reports/`), Befund in B-778 › Notizen und B-780 › Notizen.
3. **Claude:** Werte auswerten, Tickets für Abweichungen.

## Fertig, wenn

- [ ] AC-05: Messwerte und Entscheidung 🧑 in B-778 und B-780 › Notizen; B-786 erledigt.

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
