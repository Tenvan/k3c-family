# RD2.2 · Stil: neue Materialien und Umstellung der Assets

- **Status:** fertig
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

2026-11-07, Claude.

- AC-01 umgesetzt, geprüft (neuer Test `tools/paper/golden/materialien.test.ts`, 27 Fälle grün; `check:diff` grün): `schwarz`, `eisen`, `leinen`, `kupfer`, `kristall` und je Biom ein Material `biom-wald`, `biom-hoehle`, `biom-mine`, `biom-eisenstollen`, `biom-kristall` (Tripel licht/basis/schatten) in `stil.paper.json`. Das Biom-Material ist ein eigenes Material statt eines Tons, weil `kristall` sonst doppelt vorkäme.
- AC-01 (B-712/AC-03) umgestellt, 499 Material-Zeilen in 46 Specs: rappe, albtraum, hoellenhund `maehne` → `schwarz`; Truppen (bauer, krieger, elite-krieger, bogenschuetze, elite-bogenschuetze) `stein` → `eisen`; Bauer-Kittel und Ärmel `haar` → `leinen`; icon-kupfer, ader-kupfer, erzbrocken `fell-fuchs` → `kupfer`; icon-eisen und ader-eisen `stein`(Schatten)/Rost → `eisen`/`kupfer`; icon-kristall, ader-kristall `koenigsblau` → `kristall`; alle 30 Hintergründe `smaragd`/`stein`/`koenigsblau` → `biom-<biom>` (Verhältnis zum Bild: sichtbar geprüft, Montage `.omc/artifacts/rd2-1/rd22.png`).
- AC-04 umgesetzt: alle neun Gruppen neu gerendert, `fixtures.golden.json` neu (Stil-Hash ändert sich).
- Verschoben: Stroh (Strohhut), Lava, Geisterweiß, `fell-grau` und Eisen/Kristall bei Gegnern und Bossen sind nicht in der Materialliste von B-712 → B-732. Der Optik-Eindruck der neuen Farbwerte ist Ralfs Sichtprüfung.
- Abweichung: Hintergrund-Töne sind Materialien `biom-*`, nicht Töne einer Tabelle.
