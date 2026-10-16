# GEB6.3 · Adern in vier Materialstufen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb6/3-adern
- **Abhängig von:** GEB6.2
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-04

## Ziel

`ader.json` (2 × 2 Kacheln, Stufen 1–4 = Stein, Kupfer, Eisen, Kristall) liegt mit PNGs in `public/grafik/gebaeude/` und erscheint mit der Stufenreihe in der Vorschau.

## Kontext

- **Spec:** GEB6 AC-01, AC-04, › Anforderungen (Adern), Offene Fragen 3. Regeln: `docs/standards/gebaeude-grafik.md` (§ 1, § 3, § 5, § 6).
- **Fachlich:** Eine Ader ist eine **unendliche Quelle** für Stein, Kupfer, Eisen oder Kristall (Glossar „Ader“, `data/economy.json › veins`); höchstens 2 Bauern bauen gleichzeitig ab. Jedes Biom hat eine Art (`data/biomes/*.json › veins`): Höhle Stein, Mine Kupfer, Eisenstollen Eisen, Kristallhöhle Kristall.
- **Heute:** `vein:*` zeigen je eine Zeile des Erz-Blatts (80 × 16 px, 5 Spalten); im Spiel werden sie vermutlich gar nicht abgefragt (GEB6.5 prüft das).
- **Entwurf:** `ader.json` ist **eine** Datei, 2 × 2 Kacheln (64 × 64 px): Felsbuckel mit einer deutlichen Erzader als Streifen. Die Grundform ist Stufe 1 = **Stein** (Stein `dim-gray`, Licht `smokey-ash`, Ader in einer hellen Steinfarbe); `stufen` 2 = **Kupfer** (Ader `tahiti-gold` mit Patina `verdigris`), 3 = **Eisen** (Ader `heather`/`light-steel`), 4 = **Kristall** (Ader `royal-blue`/`cornflower`, Glanz `white`, Zusätze: Kristallspitzen; Fuge `viking` leuchtend). Materialien nach § 3 (Mauer-/Turmstufen 3–5 dienen als Farbvorbild, damit Mauer und Ader zusammenpassen). `stufeBauen` tauscht jede Farbe der Grundform global: Die Streifenfarbe darf in der Grundform **nur im Streifen** vorkommen (nicht zugleich als Licht oder Fuge der Felsmasse), sonst färbt der Tausch den Fels mit. Der Farbtausch betrifft **eine** Streifenfarbe plus Zusätze; die Felsmasse bleibt in allen Stufen Stein.
- **Stufe = Material**, nicht Biom: Die Adern brauchen keinen Biom-Look; `aenderung` nennt das Material.
- Vorbild: `mauer.json` (Farbtausch-Tabelle, Stufen 1–5), `turm.json`.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `ader.json` und PNGs; `index.json`)
- `tools/gebaeude/` (nur nötige Erweiterungen, mit Test)
- `src/tools/assetvorschau*.ts` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Darstellung im Spiel (GEB6.5), keine zusätzlichen Stufen jenseits der vier Materialien, keine Biom-Looks für Adern, keine Animation (Glitzern), keine Änderung an vorhandenen JSONs und PNGs.

## Schritte

1. `sprint/geb6` auschecken, `Status: in Arbeit`, committen, pushen; md5 aller vorhandenen PNGs in `public/grafik/gebaeude/` sichern.
2. Vorlage zuerst (§ 5), dann `ader.json` (2 × 2): Grundform Stufe 1 Stein, `stufen` 2 Kupfer, 3 Eisen, 4 Kristall (Farbtausch der Ader-Streifenfarbe, Zusätze für Patina und Kristallspitzen, `aenderung` je Stufe).
3. `task gebaeude`; Vergleichsbild Mauer-Stufen neben Ader-Stufen sichten (gleiche Materialfarben), nachbessern; md5 der vorhandenen PNGs gleich; zweiter Lauf ohne Änderung.
4. Regeldatei § 1 (Maße Ader 2 × 2), § 4 (Bausteine), § 6 (Stufe = Material bei Adern); Vorschau zeigt die Stufenreihe.
5. Code-Review (getrennter Agent), Ergebnis ausfüllen, `Status: fertig`, ein Commit `feat(dev): Adern in vier Stufen (GEB6.3)`, `git fetch`, `git merge origin/develop`, push.

## Fertig, wenn

- [ ] AC-01/AC-04: `ader-1-32.png` … `ader-4-32.png` (je 64 × 64 px) plus 16/64/128, `ader.json` mit `vorlage` und vier Stufen (Stein, Kupfer, Eisen, Kristall); Materialfarben wie die Mauer-Stufen 2, 3, 4, 5.
- [ ] AC-04: md5 aller vorher vorhandenen PNGs gleich; ein zweiter `task gebaeude`-Lauf ändert nichts.
- [ ] AC-04: Regeldatei § 1, § 4 und § 6 nachgetragen; Vorschau zeigt die Stufenreihe.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Mit laufender Workbench stattdessen `check_run` (mit `checkout`). Keine manuellen Prüfungen; die Sichtprüfung gehört GEB6.6.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
