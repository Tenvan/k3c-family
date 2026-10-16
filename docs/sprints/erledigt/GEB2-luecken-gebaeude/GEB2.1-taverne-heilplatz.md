# GEB2.1 · Taverne und Heilplatz als Daten und PNG

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb2/1-taverne-heilplatz
- **Abhängig von:** –
- **Tickets:** B-435
- **Kriterien:** AC-01

## Ziel

Taverne und Heilplatz liegen als Daten-JSON und PNG (6 × 4 Kacheln) in `public/grafik/gebaeude/`, gebaut mit `tools/gebaeude` nach `docs/standards/gebaeude-grafik.md`, und erscheinen im Vorschau-Streifen.

## Kontext

- **Spec:** Sprint-README GEB2 AC-01 und › Anforderungen (Motive). Vorlage für Aufbau und Stil: `public/grafik/gebaeude/werkstatt.json` (6 × 4, Pfosten, Tuch, Tresen, Gestell). Regeln: `docs/standards/gebaeude-grafik.md` (Raster 1/32 Kachel, DB32-Namen aus `tools/gebaeude/db32.go`, Licht links oben, Materialtabelle § 3, `fein` § 7).
- **Taverne** (`taverne.json`, Slot `tavern`): Holzhaus mit Satteldach, Tür mittig, ein bis zwei Fenster mit warmem Licht, Hängeschild mit Krug, Fass vor der Tür. Feine Details: Plankenfugen, Krug-Henkel, Fassreifen.
- **Heilplatz** (`heilplatz.json`, Slot `healer`): offenes Vordach/Zelt mit hellem Stoff auf zwei Pfosten, darunter Liege, daneben Kräuterkorb und Wasserschale; gut erkennbares Heil-Zeichen (Kreuz oder Blatt) auf dem Tuch. Feine Details: Faltenlicht, einzelne Kräuter.
- **Neue Materialien** (z. B. Fensterlicht, Dachschindeln): Farben aus DB32 wählen und in Tabelle § 3 nachtragen; neue Baustein-Arten in § 4.
- `task gebaeude` schreibt PNGs und `index.json`; die Vorschau (`assetvorschau.html?szene=gebaeude`) liest `index.json` automatisch, Code dort muss nicht geändert werden.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `taverne.json`, `heilplatz.json` und deren PNGs, `index.json`)
- `tools/gebaeude/` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Zuordnung ins Spiel (`data/assets.json`, `src/scenes/` → GEB2.3), keine Ausbaustufen, keine Animation, keine Änderung an der Werkstatt.

## Schritte

1. Sprint-Branch `sprint/geb2` (erste Session: von `origin/develop`, `Start-Commit` setzen), `Status: in Arbeit`, committen, pushen.
2. `taverne.json` und `heilplatz.json` nach Kontext anlegen; `task gebaeude`.
3. PNGs ansehen (Read auf `*-32.png` und `*-16.png`): Motiv erkennbar, 16 px ohne Löcher; nachbessern.
4. Neue Materialien und Baustein-Arten in die Regeldatei eintragen; Tabelle § 1 um Taverne und Heilplatz ergänzen.
5. Code-Review der geänderten Dateien, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-01: `taverne-32.png` und `heilplatz-32.png` mit je 192×128 px liegen im Repo, dazu 16/64/128; zweiter `task gebaeude`-Lauf ändert nichts (`git status` sauber).
- [ ] AC-01: `index.json` führt beide; Regeldatei nennt neue Materialien und Bausteine.
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
- AC-01 umgesetzt (Teil Taverne, Heilplatz): `taverne.json` (Fachwerk, Satteldach, Kamin, zwei Fenster mit Licht, Tür, Hängeschild mit Krug, Fass) und `heilplatz.json` (Vordach aus hellem Stoff mit grünem Kreuz, Liege, Schale, Kräuterkorb, Trockengestell), je 6 × 4 Kacheln; `*-32.png` 192×128 px plus 16/64/128; zweiter `task gebaeude`-Lauf → md5 gleich; `index.json` führt beide.
- Regeldatei: Klasse 6 × 4 um Taverne/Heilplatz, Materialien Putz, Dachziegel, Fensterlicht, Stein, Kräuter, Stoff blau, Heil-Zeichen grün, neue Bausteine in § 4.
- Checks: `task check` und `task check:go` grün (k3c-dev). Review (Sonnet-Agent, Skriptprüfung von Farben, Raster, Dicke, Regeldatei): keine Befunde. Sichtprüfung gehört GEB2.4.
