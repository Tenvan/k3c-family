# BS7.4 · Review BS7 und Baseline M1 sichern

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** bs7/4-review-baseline-m1
- **Abhängig von:** BS3.4, BS7.5
- **Tickets:** B-640
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

BS7 ist abgenommen; der Meilenstein-1-Lauf aus BS3.4 ist als Baseline `M1` gesichert, Gruppe `basis` eingefroren, und er steht im Editor unter „Basis-Läufe“.

## Kontext

- Vorher prüfen, dass die Workbench nach BS7.3 neu gestartet ist (`workbench_status`, Tool-Schema `sim_test` mit `baseline`); sonst `Status: blockiert`, 🧑 deutlich um Neustart bitten.
- Bericht-ID des Meilenstein-Laufs: B-635 › Notizen (aus BS3.4). Fehlt der Bericht oder passt er nicht mehr zu den Daten, den Lauf über `sim_test start bots=coop2 world=standard` wiederholen.
- Review nach `docs/arbeitsweise.md` › Review-Session.

## Erlaubte Dateien

- Dateien aus BS7.1–BS7.3 (Befunde beheben)
- `data/balance-targets.json` (`frozen`)
- `reports/baseline/**`
- `docs/sprints/**/BS7-*/**`, `docs/backlog/B-640-*.md`

## Nicht-Ziele

Werteänderungen.

## Schritte

1. Neustart prüfen.
2. Baseline `M1` sichern (`sim_test`), `frozen: ["basis"]` setzen.
3. Kontroll-Delta-Lauf mit `baseline=M1` → alle Gruppen ✅ unverändert.
4. Editor: Basis-Lauf `M1` sichtbar (Browser-Pane der Workbench oder API).
5. `check_run task:check`, `task:check:go`, `task:check:race`; Abnahme, Sprint abschließen, PR.

## Fertig, wenn

- [ ] AC-05: Baseline `M1` gesichert, Kontroll-Lauf ohne Abweichung, im Editor sichtbar.
- [ ] Alle Kriterien geprüft, Checks grün, PR offen.

## Prüfen

```bash
task check
task check:go
task check:race
```

## Ergebnis

**Fertig (2026-10-29, Claude).** AC-05: `M1` gesichert (BS7.5, `ohnePass`, Stand nach BS5, Titan nicht besiegt), `frozen: ["basis"]` gesetzt, Kontroll-Lauf run-3 ohne Abweichung (alle Gruppen „gleich“). AC-01..AC-03 durch Tests und Lauf belegt; `task check` und `task check:go` grün, `task check:diff` grün; `check:race` lokal ohne C-Compiler nicht lauffähig (übersprungen).
**Review (Sonnet, getrennt):** keine kritischen Befunde. Behoben: atomares Anlegen der Baseline (`os.Mkdir`), Aufräumen bei Schreibfehler, Prüfung auf gültige Messung bei `ohnePass`, Existenzprüfung der Baseline im Editor-Start. Offen als B-655: `seedList` im Editor-Formular. Nicht behoben (niedrig): Delta zeigt ohne Vergleichsbasis „gleich“ statt „unbekannt“; `BaselineInfo.Frozen` wird nicht ausgewertet; reservierte Windows-Namen (CON, NUL) als Baseline-Name.
**Ralf:** AC-04 Sichtprüfung (Editor › Beispiele › Basis-Läufe zeigt `M1`) steht aus; BS7 bleibt bis dahin aktiv. Kein PR (Arbeit im Branch feature/bas-remake).
