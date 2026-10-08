# PL2.2 · Monitor, Dungeon Master, Credits und Grafiken in der gewählten Sprache

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** offline
- **Branch:** pl2/2-monitor-dm-credits-grafiken
- **Abhängig von:** PL2.1
- **Tickets:** B-322
- **Kriterien:** AC-01

## Ziel

Monitoring-Seite (`monitor.html`), Dungeon-Master-Seite (`dm.html`) und Grafik-Seite (`grafiken.html`) holen ihre Texte über `t()`/`applyTexts()` aus `src/tools/texts.*.ts`; ihre Module stehen nicht mehr in `OFFEN`.

## Kontext

- PL2.1 hat `src/tools/texts.ts` (`t()`, `applyTexts()` für `data-t`, `data-t-aria`, `data-t-placeholder`), die Tabellen `src/tools/texts.de.ts`/`texts.en.ts` und `src/tools/textRule.test.ts` mit den Listen `OFFEN` und `DATEN` gebaut. Vorbild: `src/tools/leveltest.ts` mit `leveltest.html`.
- Dateien dieser Session: `monitor.ts`, `monitorChart.ts`, `monitorData.ts`, `monitorApi.ts`, `monitor.html`; `dm.ts`, `dmApi.ts`, `dm.html`; `grafiken.ts`, `grafiken.html`; `credits.ts` (hat keine deutschen Literale, nur prüfen).
- `grafikPacks.ts` ist Daten (Urheber, Lizenz, Recherche-Notizen) und steht in `DATEN`; übersetzt werden nur Beschriftungen, die `grafiken.ts` selbst schreibt.
- Schlüssel-Gruppen: `monitor.*`, `dm.*`, `grafik.*`. `src/tools/texts.de.ts` bleibt ≤ 400 Zeilen; wird es knapp, eine zweite Tabelle je Seitengruppe anlegen und in `texts.ts` zusammenführen.

## Erlaubte Dateien

- `src/tools/texts*.ts`, `src/tools/textRule.test.ts`
- `src/tools/monitor*.ts`, `src/tools/dm*.ts`, `src/tools/grafiken.ts`, `src/tools/credits.ts` und ihre Tests
- `monitor.html`, `dm.html`, `grafiken.html`
- Planungs-Dateien des Sprints

## Nicht-Ziele

Übersetzung der Grafik-Pack-Daten (`grafikPacks.ts`); Log-Meldungen; Seiten aus PL2.3.

## Schritte

1. Monitoring-Seite umstellen (Module und `monitor.html`), aus `OFFEN` streichen.
2. Dungeon-Master-Seite umstellen (Module und `dm.html`), aus `OFFEN` streichen.
3. Grafik-Seite umstellen (`grafiken.ts`, `grafiken.html`), aus `OFFEN` streichen.
4. `task check`.

## Fertig, wenn

- [ ] AC-01: `textRule.test.ts` grün; Monitor-, DM-, Grafik- und Credits-Module stehen nicht in `OFFEN`.
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
