# GEB3.3 · Turm 1 bis 5 mit Zaubertum

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb3/3-turm-stufen
- **Abhängig von:** GEB3.2
- **Tickets:** B-435
- **Kriterien:** AC-01

## Ziel

Der Turm liegt in den Stufen 1–5 (3 × 8 Kacheln) als Daten und PNG vor; Stufe 5 ist als Zaubertum erkennbar; die Vorschau zeigt seine Stufenreihe.

## Kontext

- **Spec:** GEB3 AC-01 (Teil Turm); Mechanik `stufen` aus GEB3.2 (Regeldatei § 6), Stilvorlage `mauer.json`.
- **Turm** (`turm.json`, 3 × 8 Kacheln): schmaler Wachturm, oben Plattform mit Brüstung für Bogenschützen (Turm hat `archerSlots: 2`, `data/buildings.json`), darunter Schaft, unten Tür. Stufe 1 Holz (Gerüst, Plankenbrüstung), 2 Stein (Quader, Zinnen), 3 Kupfer, 4 Eisen, 5 **Zaubertum**: Kristall-Materialien plus Zusätze, die ihn klar abheben (z. B. Kristallspitze über der Plattform, leuchtende Runen oder Fenster); er wirkt laut `data/buildings.json` › `tower.spell` ab Stufe 5 einen Zauber.
- Vorlage (Pflicht) und `aenderung` je Stufe genau beschreiben.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `turm.json` und PNGs, `index.json`)
- `tools/gebaeude/` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md` (§ 1 Maße, § 3/§ 4 neue Materialien und Bausteine)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Zuordnung ins Spiel (GEB3.4), keine Zauber-Animation, keine Änderung an der Mauer.

## Schritte

1. `sprint/geb3` auschecken, `Status: in Arbeit`, committen, pushen.
2. `turm.json` nach Kontext; `task gebaeude`.
3. PNGs sichten (Read auf `turm-*-32.png` und `-16.png`), neben der Mauer stimmig; nachbessern.
4. Regeldatei ergänzen (Maße Turm, neue Materialien und Bausteine).
5. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-01 (Turm): `turm-1-32.png` … `turm-5-32.png` je 96 × 256 px, plus 16/64/128; zweiter `task gebaeude`-Lauf ändert nichts.
- [ ] Stufe 5 trägt Zusätze, die nur der Zaubertum hat (in `aenderung` beschrieben).
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
