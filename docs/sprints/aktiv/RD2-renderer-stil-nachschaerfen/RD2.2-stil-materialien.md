# RD2.2 · Stil: neue Materialien und Umstellung der Assets

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** stil
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** rd2/2-stil-materialien
- **Abhängig von:** RD2.1
- **Tickets:** B-712
- **Kriterien:** AC-01, AC-04

## Ziel

Die Stil-Datei kennt `schwarz`, `eisen`, `leinen`, `kupfer`, `kristall` und je Biom einen Ton; die Assets mit Ersatz-Materialien nutzen sie.

## Kontext

Ticket B-712. Die Ersatz-Abbildungen stehen als D-Zeilen im Ergebnis der erledigten Sprints unter `docs/sprints/erledigt/` (MG2 D-MG2-01, MG3 D-MG3-04, MG4 D-MG4-05, MG5 D-MG5-02, MG7 D-MG7-04, MG8 D-MG8-02; `grep -rn "D-MG2-01" docs/sprints/erledigt`). Materialien sind Tripel `licht`/`basis`/`schatten` in `public/grafik/stil.paper.json`; Biom-Töne sind `wald`, `hoehle`, `mine`, `eisenstollen`, `kristall`. Farbwerte aus dem Prototyp und der Spielpalette ableiten (Metall kühl grau, Kupfer orange-braun, Leinen hell-beige, Schwarz nicht reines #000, Kristall türkis). Nach dem Umstellen `task gen:paper`, Golden neu.

## Erlaubte Dateien

- `public/grafik/stil.paper.json`, `tools/paper/schema/stil.schema.json` (falls nötig)
- `public/grafik/**/*.paper.json` (nur Materialzuordnung der Teile, keine Formen)
- erzeugte PNGs und Manifeste, `tools/paper/golden/**`, `tools/paper/fixtures/**`

## Nicht-Ziele

Formänderungen an Assets; neue Assets; Schatten (RD2.1).

## Schritte

1. Liste der Ersatz-Abbildungen aus den D-Zeilen erstellen (Asset, Teil, Ersatz, Ziel-Material).
2. Test: Stil enthält die fünf Materialien und je Biom einen Ton.
3. Materialien in der Stil-Datei anlegen.
4. Specs umstellen, `task gen:paper`, Golden neu.
5. Liste der umgestellten Teile im Ergebnis.

## Fertig, wenn

- [ ] AC-01: Material- und Biom-Werte stehen in der Stil-Datei, Schema-Test grün (B-712/AC-02).
- [ ] AC-01: Kein Asset nutzt mehr die Ersatz-Materialien aus den sechs D-Zeilen; Liste im Ergebnis (B-712/AC-03).
- [ ] AC-04: Golden aller Gruppen neu, `bun test` in `tools/paper` grün (B-712/AC-04).

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
