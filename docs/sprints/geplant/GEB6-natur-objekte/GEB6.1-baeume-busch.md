# GEB6.1 · Bäume: je Art drei Formen mit Biom-Looks, Busch

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb6/1-baeume-busch
- **Abhängig von:** GEB5.3 (fertig, Domänen-Sperre DEV)
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-04

## Ziel

Sechs weitere Baum-Formen (`tanne-2`, `tanne-3`, `pappel-2`, `pappel-3`, `birke-2`, `birke-3`, je mit Looks Wald und Höhle) und der Busch (`busch`, 1 × 1) liegen als Daten-JSON mit PNGs in `public/grafik/gebaeude/` und erscheinen in der Vorschau; die Regeldatei kennt die Biom-Looks und die neuen Materialien.

## Kontext

- **Spec:** GEB6 AC-01, AC-04, › Anforderungen (Bäume, Busch, Biom-Look), Offene Fragen 2, 4. Regeln: `docs/standards/gebaeude-grafik.md` (Raster § 1, Palette § 2, Material § 3, Vorlage § 5, Stufen § 6, Dichte § 7, Ablauf § 9).
- **Vorbild:** `public/grafik/gebaeude/tanne.json`, `pappel.json`, `birke.json` (1 × 2,25 Kacheln, Klasse `baum`, Stamm mittig bei x 14–18 px) und für Stufen `mauer.json` (`farben` = Farbtausch, `zusaetze`). `tools/gebaeude/stufen.go`: Stufe n ≥ 2 tauscht nur Farben, die in der Grundform vorkommen (sonst bricht `task gebaeude` ab); Ausgabe `<id>-<stufe>-<dichte>.png`, `dateien` im Index zeigt die Grundform.
- **Biom-Look (Entscheidung dieses Sprints, Annahme):** Für Natur-Objekte ist **Stufe = Look**: Stufe 1 = Wald (Grundform), Stufe 2 = Höhle, Stufe 3 = Mine; `aenderung` nennt den Look („Höhle: kühleres, dunkleres Laub“). Bäume stehen nur im Wald und in der Höhle, deshalb haben die Formen 2 und 3 nur die Stufen 1 und 2. Vorschlag Höhle: Laub eine Stufe dunkler und kühler (`christi` → `elf-green`, `atlantis` → `christi`, `dell` → `opal`), Nadeln der Tanne (`elf-green`, `christi`, `opal`) entsprechend dunkler (`opal`, `elf-green`, `deep-koamaru`).
- **Bestehende Bäume bleiben unverändert** (`tanne.json`, `pappel.json`, `birke.json` und ihre `-<d>.png`): GEB5.4 nutzt `pappel-32.png` für die Plantage, und ihr md5 ist Teil der Prüfung. Neue ids sind `tanne-2`, `tanne-3`, `pappel-2`, `pappel-3`, `birke-2`, `birke-3`; Dateiname = id + `.json`; Klasse `baum`.
- **Formen** (Vorschlag, frei, solange der Umriss unterscheidbar ist): Tanne 2 breit mit tiefen unteren Ästen, Tanne 3 schlank mit schiefer Spitze; Pappel 2 schmaler mit Lücken, Pappel 3 mit Zwiegabel; Birke 2 mit hängenden Zweigen, Birke 3 niedriger und breiter.
- **Busch** (`busch`, `klasse` leer): 1 × 1 Kachel, rundlich, Material Laub (`christi`, `atlantis`, `dell`), nur Look Wald (Busch steht nur im Wald, `data/biomes/forest.json`), keine Stufen.
- **Materialien (Offene Frage 4):** Mit der Spec-Freigabe (oder der Antwort von 🧑) trägt diese Session in § 3 die Zeilen Fels, Zelttuch, Erde und Gold nach dem Vorschlag ein; GEB6.2 und GEB6.4 nutzen sie.
- **Vorschau:** `src/tools/assetvorschauLogic.ts` › `baumReihe(index, anzahl = 6)` wechselt alle Einträge der Klasse `baum` reihum und zeigt bei neun Bäumen nicht alle; `stufenKarten` zeigt Stufen. Alle Formen und beide Looks sollen sichtbar sein (`src/tools/assetvorschau.ts`, Test `assetvorschauLogic.test.ts`).
- **Parallel:** GEB5.3 (DEV) muss `fertig` sein (Sprint-Regeln); GEB5 hat Lager, Kaserne, Farm und Setzling nach `public/grafik/gebaeude/index.json` und Regeldatei § 1 geschrieben: nach `git merge origin/develop` `task gebaeude` neu laufen lassen und § 1 zusammenführen.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `tanne-2.json` … `birke-3.json`, `busch.json` und ihre PNGs; `index.json`)
- `tools/gebaeude/` (nur nötige Erweiterungen, mit Test)
- `src/tools/assetvorschau*.ts` (Baum-Reihe mit allen Formen und Looks, mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Änderung an `tanne.json`, `pappel.json`, `birke.json` und ihren PNGs; keine Darstellung im Spiel und keine Register-Änderung (GEB6.5); kein Fels, Erz, Adern, Truhe, Portal, Lager; keine Looks für Mine.

## Schritte

1. Prüfen, dass GEB6 aktiviert ist (Spec freigegeben, Ordner `aktiv/`) und GEB5.3 (DEV) `fertig` ist (`git ls-remote --heads origin 'sprint/*'`, Sprint-Dateien dort lesen); sonst nichts tun und melden. `sprint/geb6` auschecken (fehlt er: von `origin/develop` anlegen, `Start-Commit` setzen), `Status: in Arbeit`, committen, pushen.
2. md5 aller vorhandenen PNGs in `public/grafik/gebaeude/` sichern (z. B. `md5sum public/grafik/gebaeude/*.png` in eine Datei im Scratchpad).
3. Regeldatei § 3: Zeilen Fels (= Stein), Zelttuch (= Stoff hell), Erde, Gold nach dem Vorschlag der Offenen Frage 4 bzw. der Antwort von 🧑 eintragen.
4. Vorlage zuerst (§ 5), dann Bausteine: `tanne-2`, `tanne-3`, `pappel-2`, `pappel-3`, `birke-2`, `birke-3` (je 1 × 2,25, Klasse `baum`) mit `stufen` Stufe 2 = Höhle (Farbtausch, `aenderung`); `busch` (1 × 1, ohne Stufen).
5. `task gebaeude`; PNGs sichten (Wald und Höhle je Baum, Busch); md5 der vorhandenen PNGs gleich; zweiter Lauf ohne Änderung.
6. Regeldatei § 1 (Maße: Baumformen, Busch 1 × 1), § 4 (neue Bausteinarten), § 6 (Absatz „Biom-Look: Stufe = Look“, Höhle, Mine).
7. Vorschau: alle neun Bäume und beide Looks sichtbar (`baumReihe`/Stufenkarten erweitern), Test dazu.
8. Code-Review (getrennter Agent), Ergebnis ausfüllen, `Status: fertig`, ein Commit `feat(dev): Baum-Formen, Biom-Looks und Busch (GEB6.1)`, `git fetch`, `git merge origin/develop`, push auf `sprint/geb6`.

## Fertig, wenn

- [ ] AC-01/AC-04: `tanne-2`, `tanne-3`, `pappel-2`, `pappel-3`, `birke-2`, `birke-3` (je 1 × 2,25, mit `vorlage`, Stufen Wald und Höhle) erzeugen `<id>-1-32.png` und `<id>-2-32.png` (je 32 × 72 px) plus 16/64/128; `busch.json` (1 × 1, mit `vorlage`) erzeugt `busch-32.png` (32 × 32 px) plus 16/64/128.
- [ ] AC-04: md5 aller vorher vorhandenen PNGs gleich (insbesondere `tanne-32.png`, `pappel-32.png`, `birke-32.png`); ein zweiter `task gebaeude`-Lauf ändert nichts.
- [ ] AC-04: Regeldatei § 1 (Maße), § 3 (Fels, Zelttuch, Erde, Gold), § 4 (Bausteine) und § 6 (Stufe = Look bei Natur-Objekten) nachgetragen.
- [ ] AC-04: Vorschau zeigt alle neun Bäume und beide Looks (Vitest für die erweiterte Auswahl).
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
