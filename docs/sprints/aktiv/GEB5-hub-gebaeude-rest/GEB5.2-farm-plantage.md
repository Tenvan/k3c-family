# GEB5.2 · Farm und Setzling der Plantage

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb5/2-farm-plantage
- **Abhängig von:** GEB5.1
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-02, AC-04

## Ziel

`farm.json` (4 × 3 Kacheln) und `setzling.json` (1 × 1 Kachel, Phase „leerer Plantage-Platz“) liegen mit Vorlage, ohne `stufen`, als PNGs in `public/grafik/gebaeude/` und erscheinen in der Vorschau, der Setzling neben der Pappel aus GEB1.

## Kontext

- **Spec:** GEB5 AC-01, AC-02, AC-04, › Anforderungen (Farm, Plantage), › Offene Fragen 3–5; Regeldatei § 1, § 3, § 5.
- **Farm:** Holzscheune mit Satteldach, Tor und kurzem Zaun, Material Holz (Farben wie Mauer-Stufe 1: rope/twine/oiled-cedar/loulou). Platz: ±52 zwischen Tor 1 (±48, 2 breit) und Turm 2 (≥ ±56, 3 breit), deshalb höchstens 4 Kacheln breit.
- **Plantage:** `engine/sim/plantation.go` setzt je Platz einen normalen Baum (`kind: tree`) an `plotX` (6 Plätze im Streifen ±3 um die Farm). Gewachsen zeigt die Plantage später die **Pappel** (`pappel.json`, 1 × 2,25, GEB1.3); diese Session baut nur den **Setzling** für den leeren Platz: kleiner Trieb mit Erdhügel und Stützstab, Farben der Pappel, Unterkante am Boden. Hinweis: Bei 16 px fallen `fein`-Bausteine weg (§ 7), der Setzling muss bei 16 px noch erkennbar sein.
- **Fallstricke:** md5 aller vorhandenen PNGs vor `task gebaeude` sichern und danach vergleichen (GEB3/GEB4); die Pappel-PNGs bleiben unverändert.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `farm.json`, `setzling.json` und PNGs, `index.json`)
- `tools/gebaeude/`, `src/tools/assetvorschau*.ts` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Darstellung im Spiel (GEB5.4), keine Farm-Stufen, keine weiteren Wachstumsphasen, keine Änderung an Pappel, Tanne, Birke oder `engine/`.

## Schritte

1. `sprint/geb5` auschecken, `Status: in Arbeit`, committen, pushen.
2. md5 der vorhandenen PNGs sichern.
3. `farm.json` und `setzling.json` nach Kontext; `task gebaeude`; PNGs sichten (32 und 16 px, Setzling neben `pappel-32.png`), nachbessern; md5 der übrigen PNGs gleich.
4. Regeldatei § 1 (Maße Farm, Setzling).
5. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-01/AC-04: `farm-32.png` 128 × 96 px und `setzling-32.png` 32 × 32 px plus 16/64/128; zweiter `task gebaeude`-Lauf ändert nichts; ältere PNGs md5-gleich; beide in der Vorschau.
- [ ] AC-02 (Grafik-Teil): Setzling liegt vor, die gewachsene Phase nutzt die vorhandene Pappel aus GEB1 (keine neue Baum-Grafik).
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Keine manuellen Prüfungen; die Sichtprüfung gehört GEB5.5.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
