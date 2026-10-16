# GEB3.2 · Stufen im Werkzeug und Mauer 1 bis 5

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb3/2-stufen-werkzeug-mauer
- **Abhängig von:** GEB3.1
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-05

## Ziel

`tools/gebaeude` kann Ausbaustufen aus einer Gebäude-JSON rendern (Feld `stufen`), Regeldatei § 6 beschreibt die Mechanik, und die Mauer liegt in den Stufen 1–5 (Holz bis Kristall, 1 × 5 Kacheln) als Daten und PNG vor, mit Stufenreihe in der Vorschau.

## Kontext

- **Spec:** GEB3 AC-01 (Teil Mauer), AC-05; Sprint-README › Anforderungen (Stufen in der JSON, Format) und › Ausnahme- und Fehlerfälle.
- **Werkzeug:** `tools/gebaeude/gebaeude.go` (`building`, `vorlage`, `build`, `render`, `check`, `run`; Datei heute ~240 Zeilen, Limit 400 → Stufen-Logik in eigene Datei `stufen.go`). Format je Stufe: `{ "stufe": 2, "aenderung": "…", "farben": { "alt": "neu" }, "zusaetze": [Bausteine] }`. Stufe 1 = Grundform (keine Einträge nötig). Rendern: Grundform-Bausteine mit Farbtausch, dann Zusätze; alle Prüfungen aus `check` gelten auch für Zusätze. Ausgabe `<id>-<stufe>-<dichte>.png` für alle Stufen inkl. 1; Index-Eintrag bekommt `stufen` (Nummer, Änderung, Dateien). Gebäude ohne `stufen` bleiben byte-gleich (md5 vorher sichern).
- **Mauer** (`mauer.json`, 1 × 5 Kacheln): Stufe 1 Holz-Palisade (Pfähle mit Spitzen, Querriegel), Stufe 2 Stein (Quader, Zinnen), 3 Kupfer-Beschläge/Kupferplatten, 4 Eisen (Platten, Nieten), 5 Kristall (Kristallkanten, Leuchtfugen). Materialien § 3 (Holz, Stein, Eisen vorhanden; Kupfer und Kristall neu eintragen). Vorlage (Pflicht, GEB8) beschreibt die Grundform; `aenderung` jeder Stufe beschreibt genau, was sich ändert.
- **Vorschau:** `src/tools/assetvorschau*.ts`: Stufenreihe je Gebäude mit Stufen (Logik mit Vitest), Vorlage und Stufen-Änderungen darunter, Bodenlinie wie in GEB8.3.

## Erlaubte Dateien

- `tools/gebaeude/`
- `public/grafik/gebaeude/` (neu: `mauer.json` und PNGs, `index.json`)
- `src/tools/assetvorschau.ts`, `src/tools/assetvorschauLogic.ts`, `src/tools/assetvorschauLogic.test.ts`
- `docs/standards/gebaeude-grafik.md` (§ 1 Maße, § 3 Materialien, § 5 Format, § 6 Ausbaustufen), `docs/glossar.md` (falls „Ausbaustufe“ in der Grafik-Bedeutung fehlt)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Kein Turm (GEB3.3), keine Zuordnung ins Spiel (GEB3.4), keine Stufen für andere Gebäude, keine Animation.

## Schritte

1. `sprint/geb3` auschecken, `Status: in Arbeit`, committen, pushen.
2. md5 aller vorhandenen PNGs in `public/grafik/gebaeude/` sichern.
3. Tests zuerst (`tools/gebaeude/stufen_test.go`): Farbtausch, Zusätze, Dateinamen `<id>-<stufe>-<d>.png`, Index mit Stufen, Fehlerfälle (fehlende `aenderung`, doppelte/lückenhafte Stufe, Nicht-DB32-Farbe, Zusatz außerhalb).
4. Werkzeug umsetzen; Regeldatei § 5/§ 6.
5. `mauer.json` nach Kontext; `task gebaeude`; PNGs sichten (Read auf `mauer-*-32.png` und `-16.png`), nachbessern; md5 der alten PNGs vergleichen.
6. Vorschau: Stufenreihe mit Test.
7. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-05: `go test ./tools/gebaeude` deckt Stufen und Fehlerfälle ab; vorhandene PNGs md5-gleich; § 6 beschreibt die Mechanik.
- [ ] AC-01 (Mauer): `mauer-1-32.png` … `mauer-5-32.png` je 32 × 160 px, plus 16/64/128; zweiter `task gebaeude`-Lauf ändert nichts.
- [ ] Vitest für die Stufenreihe der Vorschau.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Keine manuellen Prüfungen; die Sichtprüfung gehört GEB3.5.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
