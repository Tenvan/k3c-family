# GEB2.2 · Schmiede und Rüstkammer als Daten und PNG

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb2/2-schmiede-ruestkammer
- **Abhängig von:** GEB2.1
- **Tickets:** B-435
- **Kriterien:** AC-01

## Ziel

Schmiede und Rüstkammer liegen als Daten-JSON und PNG (6 × 4 Kacheln) in `public/grafik/gebaeude/`, gebaut mit `tools/gebaeude` nach `docs/standards/gebaeude-grafik.md`, und erscheinen im Vorschau-Streifen.

## Kontext

- **Spec:** Sprint-README GEB2 AC-01 und › Anforderungen (Motive). Vorlagen: `werkstatt.json` und die Ergebnisse von GEB2.1 (`taverne.json`, `heilplatz.json`, neue Materialien in `docs/standards/gebaeude-grafik.md`).
- **Schmiede** (`schmiede.json`, Slot `smithy`): Holz-Vordach über gemauerter Esse mit Glut (warme Farben), Kamin aus Stein, Amboss auf Holzklotz vorn, Wassertrog. Feine Details: Zange und Hammer am Amboss, Funken als Einzelpixel, Steinfugen.
- **Rüstkammer** (`ruestkammer.json`, Slot `armory`): wuchtiger als die anderen, Steinsockel mit Holzaufbau, Ständer mit Rüstung (Metall) und Schild (Stoff/Wappen), Waffenregal. Feine Details: Nieten, Wappenzeichen, Metallglanz.
- **Material Stein und Glut** gibt es vermutlich noch nicht (ungeprüft, hängt an GEB2.1): mit DB32-Farben festlegen und in § 3 eintragen; Metall nach § 3 (`heather`, `dim-gray`).

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `schmiede.json`, `ruestkammer.json` und deren PNGs, `index.json`)
- `tools/gebaeude/` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Zuordnung ins Spiel (GEB2.3), keine Glut- oder Rauch-Animation, keine Ausbaustufen, keine Änderung an Werkstatt, Taverne oder Heilplatz.

## Schritte

1. `sprint/geb2` auschecken, `Status: in Arbeit`, committen, pushen.
2. `schmiede.json` und `ruestkammer.json` nach Kontext anlegen; `task gebaeude`.
3. PNGs ansehen (Read auf `*-32.png` und `*-16.png`): Motiv erkennbar, neben den anderen vier stimmig; nachbessern.
4. Regeldatei um Stein, Glut und neue Bausteine ergänzen; Tabelle § 1 um Schmiede und Rüstkammer.
5. Code-Review der geänderten Dateien, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-01: `schmiede-32.png` und `ruestkammer-32.png` mit je 192×128 px liegen im Repo, dazu 16/64/128; zweiter `task gebaeude`-Lauf ändert nichts.
- [ ] AC-01: `index.json` führt alle sechs Gebäude-Dateien plus Bäume; Regeldatei nennt Stein und Glut.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Keine manuellen Prüfungen in dieser Session; die Sichtprüfung gehört GEB2.4.

## Ergebnis

2026-10-16, Claude (autonom, Ralf im Chat).
- AC-01 umgesetzt (Teil Schmiede, Rüstkammer): `schmiede.json` (Holz-Vordach, Kamin und Esse aus Stein mit Glut und Funken, Amboss auf Klotz, Werkzeug an der Wand, Wassertrog) und `ruestkammer.json` (Steinsockel, Holzaufbau, Schieferdach, Tor mit Eisenbändern, Rüstungsständer, Wappenschild, Speere am Regal), je 6 × 4 Kacheln; `*-32.png` 192×128 px plus 16/64/128; zweiter `task gebaeude`-Lauf → md5 gleich; `index.json` führt alle acht Einträge.
- Regeldatei: Stein mit Fuge, Glut, Schiefer, Eisen, Wappen, Wasser in § 3, Bausteine in § 4, Klasse 6 × 4 um beide ergänzt.
- Checks: `task check` und `task check:go` grün (k3c-dev). Review (Sonnet-Agent, Skriptprüfung): ein leichter Befund (Material Wasser fehlte), behoben. Sichtprüfung gehört GEB2.4.
