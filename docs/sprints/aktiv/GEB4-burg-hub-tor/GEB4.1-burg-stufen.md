# GEB4.1 · Burg mit Hub-Stufen 1 bis 5

- **Status:** fertig
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

2026-10-16, Claude (autonom, Ralf im Chat).
- Basis: GEB4 baut auf `sprint/geb3` auf, weil #310 noch offen war (Freigabe Ralf). Der Branch `sprint/geb4` wurde lokal neu von `origin/sprint/geb3` aufgesetzt und der Planungs-Commit übernommen; ein Fahrplan-Konflikt ist gelöst.
- AC-01/AC-04 umgesetzt: `burg.json` (8 × 9, Vorlage; Grundform = Hub-Stufe 1 Holzburg mit Zinnen, Tor und Banner). Stufe 2 bringt Stein mit zwei Ecktürmen, Stufe 3 Kupfer mit Obergeschoss, erleuchteten Fenstern und Banner obenauf, Stufe 4 Eisen mit Torbändern, Nieten und Bannern auf den Ecktürmen, Stufe 5 Kristall mit drei Kristallspitzen. Der Farbtausch ist identisch zu Mauer und Turm. `burg-<n>-32.png` haben je 256 × 288 px, dazu kommen 16/64/128. Ein zweiter Lauf ist md5-gleich, die älteren PNGs sind unverändert.
- Regeldatei: § 1 (Maße Burg), § 4 (Bausteine `turm`, `obergeschoss`, `mast`, `banner`).
- Checks: `task check` und `task check:go` grün. Review (Sonnet-Agent, Skriptprüfung): keine Befunde.
