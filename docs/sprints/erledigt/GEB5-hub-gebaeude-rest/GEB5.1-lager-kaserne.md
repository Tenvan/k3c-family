# GEB5.1 · Lager und Kaserne

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb5/1-lager-kaserne
- **Abhängig von:** GEB4
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-04

## Ziel

`lager.json` und `kaserne.json` (je 6 × 4 Kacheln, Vorlage, ohne `stufen`) liegen mit PNGs in `public/grafik/gebaeude/` und erscheinen in der Vorschau.

## Kontext

- **Spec:** GEB5 AC-01, AC-04, › Anforderungen (Lager, Kaserne), › Offene Fragen 1, 2; Regeldatei `docs/standards/gebaeude-grafik.md` § 1 (Maße), § 3 (Materialsprache), § 5 (Vorlage).
- **Vorbilder:** `werkstatt.json`, `schmiede.json` (6 × 4, Vorlage) für Aufbau und Größe; `mauer.json` Stufe 2 für die Stein-Farben (Holz rope/twine/oiled-cedar/loulou → Stein dim-gray/smokey-ash/valhalla/black), damit Lager und Kaserne zur Steinmauer passen.
- **Motive:** Lager = Steinsockel mit Holzschuppen und offener Front, darin Kisten, Fässer und Säcke (Rohstoffe angedeutet); Kaserne = Steinhaus mit breitem Tor, Waffenständer (Speere/Schwerter) und Wimpel am Dach. Unterkante am Boden, alles innerhalb 6 × 4.
- **Fallstricke:** Die Kaserne steht bei +24 und überdeckt die Schmiede (+28) um 2 Kacheln (Offene Frage 2): Silhouette rechts eher niedrig halten. Vor `task gebaeude` md5 aller vorhandenen PNGs sichern und danach vergleichen (Lehre GEB3/GEB4: ältere PNGs bleiben byte-gleich).
- Werkzeug, Vorschau und Vorlage gibt es seit GEB3/GEB8; normalerweise reicht die Daten-JSON.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `lager.json`, `kaserne.json` und PNGs, `index.json`)
- `tools/gebaeude/`, `src/tools/assetvorschau*.ts` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Farm, keine Treppen (GEB5.2, GEB5.3), keine Darstellung im Spiel (GEB5.4), keine Stufen, keine Änderung an vorhandenen PNGs oder `data/`.

## Schritte

1. Sprint-Branch `sprint/geb5` (erste Session: von `origin/develop`; ist GEB4 noch nicht gemergt, von `origin/sprint/geb4` wie GEB4 auf GEB3), `Start-Commit` setzen, `Status: in Arbeit`, committen, pushen.
2. md5 der vorhandenen PNGs unter `public/grafik/gebaeude/` sichern.
3. `lager.json` und `kaserne.json` nach Kontext; `task gebaeude`; PNGs sichten (Read auf `<id>-32.png` und `-16.png`), nachbessern; md5 der übrigen PNGs gleich.
4. Regeldatei § 1 (Maße Lager, Kaserne), § 4 (neue Bausteine, falls nötig).
5. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-01/AC-04: `lager-32.png` und `kaserne-32.png` je 192 × 128 px plus 16/64/128; zweiter `task gebaeude`-Lauf ändert nichts; ältere PNGs md5-gleich; beide in der Vorschau (`assetvorschau.html?szene=gebaeude`, per Test oder `index.json` belegt).
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Keine manuellen Prüfungen; die Sichtprüfung gehört GEB5.5.

## Ergebnis

- **AC-01** umgesetzt: `lager.json`, `kaserne.json` mit PNGs 16/32/64/128 in `public/grafik/gebaeude/`; `lager-32.png`, `kaserne-32.png` je 192 × 128 px (`file`).
- **AC-04** geprüft (Teil Lager, Kaserne): je 6 × 4 mit Vorlage, ohne `stufen`; zweiter `task gebaeude`-Lauf md5-gleich; alle 132 älteren PNGs md5-gleich; beide in `index.json` (Vorschau liest sie generisch, `assetvorschau.ts`); Maße in Regeldatei § 1, neue Bausteine (`sack`, `holz`, `erz`, `schwert`) in § 4.
- `task check` und `task check:go` grün (erster Lauf fand Material „Stein (Sockel)“ nicht in § 3, korrigiert).
- Umsetzung im Team: zwei Agenten je Gebäude, Rendern im Scratch-Ordner; Review durch eigenen Agenten ohne schwere Befunde. Manuell geprüft: niemand (Sichtprüfung GEB5.5).
- Abweichungen: keine; neue Tickets: keine.
