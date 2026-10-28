# BS7.5 · Simulationen BS7: Baseline M1 sichern und Delta-Lauf prüfen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** bs7/5-simulationen
- **Abhängig von:** BS7.3, BS3.6
- **Tickets:** B-640
- **Kriterien:** AC-02, AC-03, AC-05

## Ziel

Die Simulationen von BS7 laufen hier, nicht in den Umsetzungs-Sessions: ein echter `sim_test`-Lauf prüft Sichern und Vergleichen einer benannten Baseline samt Delta-Bericht, und die Baseline `M1` aus dem Meilenstein-Lauf wird gesichert (AC-05).

## Kontext

Regel von 🧑 (2026-10-28): Umsetzungs-Sessions fahren keine Simulationen; BS7.1–BS7.3 haben `baseline`, `saveBaseline`, den Delta-Bericht und die Basis-Läufe im Editor nur mit Fake-Prozessen getestet. Voraussetzung: Workbench nach BS7.2/BS7.3 neu gestartet (`workbench_status`), und der Meilenstein-Lauf aus BS3.6 liegt vor (`coop2`, Welt standard, Seed `k3c-dev`, Endboss Insel 1 besiegt). Läufe nur über `sim_test`, lange Läufe blockierend (`wait`).

## Erlaubte Dateien

- `docs/backlog/B-640-*.md` (Notizen über Planungs-Tools)
- `reports/baseline/**` (nur über `sim_test`, nicht von Hand)
- Befunde als Ticket, keine Code-Änderung

## Nicht-Ziele

Code- und Mechanikänderungen; Werte-Justierung (BS3.6); Baseline löschen.

## Schritte

1. `workbench_status`: Neustart nach BS7.3 prüfen, sonst `Status: blockiert` und 🧑 um Neustart bitten.
2. Bericht des grünen Meilenstein-Laufs aus BS3.6 nehmen; fehlt er, `Status: blockiert` (wartet auf BS3.6).
3. `sim_test {action: baseline, id: <Lauf>, saveBaseline: M1}`; `sim_test {action: baseline}` listet `M1`.
4. Delta-Lauf: `sim_test {action: start, bots: coop2, world: standard, days: 40, baseline: M1, wait: true}`; der Bericht zeigt „Delta je Werte-Gruppe“ (basis, holz, schiff), eingefrorene Gruppe `basis` schweigt, solange nichts kippt.
5. Gruppe `basis` in `data/balance-targets.json` › `frozen` einzutragen ist Sache von 🧑 oder BS7.4; hier nur als Vorschlag im Ergebnis nennen.
6. Ergebnis: Bericht-IDs, Befunde als Ticket.

## Fertig, wenn

- [ ] AC-02/AC-03 (echter Lauf): `saveBaseline` und `baseline` funktionieren mit dem Offline-Runner, der Bericht enthält den Delta-Abschnitt.
- [ ] AC-05: Baseline `M1` gesichert (`reports/baseline/M1/`), Bericht-ID und Endboss-Tag in B-640 › Notizen.
- [ ] AC-04 (Sichtprüfung): „Beispiele › Basis-Läufe“ zeigt `M1` (🧑 nimmt am Gerät ab).

## Prüfen

```text
sim_test {action: baseline}
sim_test {action: start, bots: coop2, world: standard, days: 40, baseline: M1, wait: true}
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
