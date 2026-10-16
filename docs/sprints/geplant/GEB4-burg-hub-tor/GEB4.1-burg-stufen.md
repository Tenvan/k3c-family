# GEB4.1 · Burg mit Hub-Stufen 1 bis 5

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb4/1-burg-stufen
- **Abhängig von:** GEB3
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-04

## Ziel

`burg.json` (8 × 9 Kacheln, Vorlage, Grundform = Hub-Stufe 1, Stufen 2–5) liegt mit PNGs in `public/grafik/gebaeude/` und erscheint mit Stufenreihe in der Vorschau; die Burg wächst je Stufe sichtbar.

## Kontext

- **Spec:** GEB4 AC-01, AC-04 und › Anforderungen (Burg); Regeldatei § 5 (Vorlage), § 6 (Stufen).
- Vorbilder: `mauer.json`, `turm.json` (gleiche Farbtausch-Tabelle je Stufe: Holz rope/twine/oiled-cedar/loulou → Stein dim-gray/smokey-ash/valhalla/black → Kupfer tahiti-gold/twine/rope/oiled-cedar → Eisen heather/light-steel/dim-gray/valhalla → Kristall royal-blue/cornflower/deep-koamaru/viking).
- **Motiv:** Burg als Hub-Kern: Mittelbau mit großem Eingang (Zahlziel), Zinnen, Banner; Stufe 1 einfache Holzburg (Palisaden-Optik), ab Stufe 2 Ecktürme links und rechts, Stufe 3 höhere Mitte mit Fenstern, Stufe 4 Eisenbeschläge und zweites Banner, Stufe 5 Kristallspitzen auf den Türmen. Zusätze können die Grundform überdecken; alles innerhalb 8 × 9 Kacheln, Unterkante am Boden.
- Werkzeug, Vorschau und Stufenreihe gibt es seit GEB3; normalerweise reicht die Daten-JSON.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `burg.json` und PNGs, `index.json`)
- `tools/gebaeude/`, `src/tools/assetvorschau*.ts` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Kein Tor (GEB4.2), keine Darstellung im Spiel (GEB4.3), keine Animation.

## Schritte

1. Sprint-Branch `sprint/geb4` (erste Session: von `origin/develop`, `Start-Commit` setzen), `Status: in Arbeit`, committen, pushen.
2. md5 der vorhandenen PNGs sichern.
3. `burg.json` nach Kontext; `task gebaeude`; PNGs sichten (Read auf `burg-<n>-32.png` und `-16.png`), nachbessern; md5 der übrigen PNGs gleich.
4. Regeldatei § 1 (Maße), § 3/§ 4 (neue Materialien, Bausteine).
5. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-01/AC-04: `burg-1-32.png` … `burg-5-32.png` je 256 × 288 px plus 16/64/128; zweiter `task gebaeude`-Lauf ändert nichts; Stufen 2–5 haben Zusätze, nicht nur Farbtausch.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Keine manuellen Prüfungen; die Sichtprüfung gehört GEB4.4.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
