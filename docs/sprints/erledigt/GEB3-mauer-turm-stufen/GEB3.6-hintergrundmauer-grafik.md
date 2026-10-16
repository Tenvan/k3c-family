# GEB3.6 · Hintergrundmauer 1 bis 5 als Grafik

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb3/6-hintergrundmauer-grafik
- **Abhängig von:** GEB3.4
- **Tickets:** B-442
- **Kriterien:** AC-07

## Ziel

Die Hintergrundmauer liegt als `hintergrundmauer.json` mit Vorlage und Stufen 1–5 (1 × 4 Kacheln, nahtlos kachelbar) und PNG in `public/grafik/gebaeude/` vor und erscheint mit ihrer Stufenreihe in der Vorschau.

## Kontext

- **Spec:** GEB3 AC-07 und › Anforderungen (Hintergrundmauer, Revision 2); B-442; Glossar „Hintergrundmauer“.
- Vorbild `mauer.json` (gleiche Farbtausch-Tabelle je Stufe: Holz rope/twine/oiled-cedar/loulou → Stein dim-gray/smokey-ash/valhalla/black → Kupfer tahiti-gold/twine/rope/oiled-cedar → Eisen heather/light-steel/dim-gray/valhalla → Kristall royal-blue/cornflower/deep-koamaru/viking). Das Hilfsskript, das `mauer.json` erzeugt hat, liegt nicht im Repo; die JSON selbst ist die Quelle.
- **Hintergrund-Wirkung:** etwas dunkler und kontrastärmer als die Mauer (z. B. Licht-Kante weglassen, dunklere Grundfarbe als Zusatz-Überzug), damit sie hinter den Bauplätzen zurücktritt; Zinnen oben durchgehend.
- **Kachelbar:** Muster so wählen, dass links und rechts nahtlos aneinanderpassen (keine Schattenkante am Rand, Fugen-Raster in der Breite periodisch). In § 5/§ 6 der Regeldatei vermerken, was „kachelbar“ für die Quelle bedeutet.
- Die Vorschau zeigt Gebäude mit `stufen` schon als Reihe (GEB3.2); für die Hintergrundmauer zusätzlich drei Kacheln nebeneinander, damit die Naht sichtbar wird (Logik mit Vitest).

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `hintergrundmauer.json` und PNGs, `index.json`)
- `tools/gebaeude/` (nur nötige Erweiterungen mit Test)
- `src/tools/assetvorschau.ts`, `src/tools/assetvorschauLogic.ts`, `src/tools/assetvorschauLogic.test.ts`
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Darstellung im Spiel (GEB3.7), keine Änderung an Mauer oder Turm, keine Sim-Änderung.

## Schritte

1. `sprint/geb3` auschecken, `Status: in Arbeit`, committen, pushen.
2. `hintergrundmauer.json` nach Kontext (Vorlage, Grundform Holz, Stufen 2–5); `task gebaeude`.
3. PNGs sichten (Read; drei Kacheln nebeneinander prüfen), Naht und Zurücktreten nachbessern; md5 der übrigen PNGs unverändert.
4. Vorschau: Kachelreihe mit Test.
5. Regeldatei (Maße, Kachelbarkeit, Bausteine).
6. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-07: `hintergrundmauer-<n>-32.png` je 32 × 128 px plus 16/64/128; zweiter `task gebaeude`-Lauf ändert nichts; erste und letzte Pixelspalte passen nahtlos (Go-Test oder Vitest, der die Randspalten vergleicht).
- [ ] Vitest für die Kachelreihe der Vorschau.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Keine manuellen Prüfungen; die Sichtprüfung gehört GEB3.5.

## Ergebnis

2026-10-16, Claude (autonom, Ralf im Chat).
- AC-07 umgesetzt und geprüft: `hintergrundmauer.json` (`klasse: kachel`, 1 × 4 Kacheln, Vorlage, Stufen 2–5 mit den Farbstufen der Mauer, je eine Stufe dunkler, damit sie zurücktritt; zwei Zinnen je Kachel, keine Seitenkanten, Stoßfugen ab Stufe 2 nie auf der Naht). `hintergrundmauer-<n>-32.png` je 32 × 128 px plus 16/64/128; zweiter Lauf md5-gleich; ältere PNGs unverändert. Go-Test `TestHintergrundmauerKachelbar`: erste = letzte Pixelspalte in allen 5 Stufen × 4 Dichten (20 Bilder).
- Vorschau: `kachelAnzahl` (Vitest) – kachelbare Grafik erscheint dreimal nebeneinander, auch in der Stufenreihe.
- Regeldatei: § 1 Maße, § 4 Baustein `kante`, § 5 Klasse `kachel` und was kachelbar heißt.
- Checks: `task check` grün; `task check:go` grün (ein Lauf scheiterte vorher an einer Windows-Dateisperre in `engine/store`, ohne Bezug zu dieser Session; Wiederholung grün). Review (Sonnet-Agent): keine schweren Befunde; übernommen: Stoßfugen enden unter der Oberkante, Test prüft die Zahl der Bilder.
