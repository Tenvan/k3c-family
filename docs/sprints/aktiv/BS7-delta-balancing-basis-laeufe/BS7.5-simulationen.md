# BS7.5 · Simulationen BS7: Baseline M1 sichern und Delta-Lauf prüfen

- **Status:** blockiert
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

**Weiter blockiert (2026-10-29, Claude): Die Workbench ist aktuell (Dev-Modus), aber `saveBaseline` lehnt jeden Lauf ab, der nicht `Pass` ist.**
- `sim_test` run-1 (coop2, standard, 40 Tage, `k3c-dev`, 4m22s, Bericht `reports/simtest-run-1/balance-20261029-090228.html`) und `task balance:milestone` auf HEAD liefern dasselbe (Stufe 4, Tag 24, Lava-Golem nicht besiegt, Burg gefallen). Die EXE-Hypothese vom 2026-10-28 ist damit erledigt.
- Bisect mit dem Go-Test (Seed `k3c-dev`, nur Messung): `ecf262ec` Titan Tag 26; `b70d6535` (BS12.6) Stufe 5 Tag 23, Titan nein; `ee94ae7e` dito; `5d51f15b` (BS4) Stufe 3 Tag 16, Ratten-Königin nein; HEAD (BS5) Stufe 4 Tag 24. BS12.6 kostet den Titan bei diesem Seed (4 Seeds: Titan 2/4, siehe BS12.6), BS4 kostet weitere Stufen, weil die Bots noch nicht roden (BS8).
- `Pass` verlangt erreichte Ziele und Korridore (12 von 16 verletzt); der Endboss fällt 0/4 (B-651). Eine Baseline `M1` ist so nicht ohne Code-Änderung am Pass-Kriterium oder ohne gelösten B-651 zu sichern. Code-Änderungen sind in dieser Session ausgeschlossen.
**Vorschlag (🧑 entscheidet):** (a) `saveBaseline` bekommt einen Schalter `force`/`ohnePass` für offene Meilensteine (kleine Workbench-Änderung, eigene Session in BS7), dann `M1` aus run-1 sichern, oder (b) B-651 zuerst lösen. Für BS8/BS9 reicht (a): Delta gegen den Stand vor den Bot-Änderungen.
- Kein Code geändert.
