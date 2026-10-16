# GEB8.1 · Vorlage im Werkzeug und in allen acht Gebäude-JSONs

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb8/1-vorlage-werkzeug-daten
- **Abhängig von:** GEB2
- **Tickets:** B-441
- **Kriterien:** AC-01, AC-02

## Ziel

`tools/gebaeude` verlangt und übernimmt die Gebäude-Vorlage, und alle acht Gebäude-JSONs tragen eine Vorlage, die ihr heutiges PNG beschreibt. Die PNGs bleiben byte-gleich.

## Kontext

- **Spec:** Sprint-README GEB8 AC-01, AC-02; B-441 › Anforderungen (Format). Begriff „Gebäude-Vorlage“ im Glossar.
- **Werkzeug:** `tools/gebaeude/gebaeude.go` (Struktur `building`, `check`, `run` schreibt `index.json` über `eintrag`), Tests in `gebaeude_test.go` (Muster: Tabelle mit kaputten Varianten, z. B. „zu dünn bei 16“). JSON mit `DisallowUnknownFields` für die Vorlage, damit Tippfehler auffallen.
- **Daten:** `public/grafik/gebaeude/{werkstatt,tanne,pappel,birke,taverne,heilplatz,schmiede,ruestkammer}.json`. Quellen für die Texte: Bausteine (Arten, Farben, Lage) und Doku — GEB1 README › Ausgangslage (Werkstatt: Handelsstand mit Tuchdach in Rot/Hell, Tresen, Gestell rechts mit Hämmern und Bögen), GEB1.3 › Kontext (Bäume), GEB2 › Anforderungen und GEB2.1/GEB2.2 › Kontext (vier Gebäude). Materialnamen aus Regeldatei § 3. Teile von links nach rechts, hinten nach vorn, Lage grob in Kacheln.
- PNG-md5 vor Beginn sichern und nach `task gebaeude` vergleichen.

## Erlaubte Dateien

- `tools/gebaeude/`
- `public/grafik/gebaeude/*.json` (nur Feld `vorlage`), `public/grafik/gebaeude/index.json`
- `docs/standards/gebaeude-grafik.md` (§ 5 Datenformat, § 9 Ablauf)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Änderung an Bausteinen oder PNGs, keine Vorschau (GEB8.2), keine KI-Erzeugung aus der Vorlage.

## Schritte

1. Sprint-Branch `sprint/geb8` (erste Session: von `origin/develop`, `Start-Commit` setzen), `Status: in Arbeit`, committen, pushen.
2. md5 aller `public/grafik/gebaeude/*.png` sichern.
3. Tests zuerst: fehlende Vorlage, leeres `motiv`/`teile`, unbekanntes Feld → Fehler mit Datei und Feld; Vorlage landet im `index.json`.
4. Werkzeug umsetzen; Regeldatei § 5 um das Format, § 9 um „Vorlage zuerst schreiben, dann Bausteine“.
5. Acht Vorlagen nach Kontext schreiben; `task gebaeude`; md5 vergleichen (gleich).
6. Code-Review der geänderten Dateien, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-01: `go test ./tools/gebaeude` deckt Pflicht, Ablehnung und Übernahme ins `index.json` ab; Format in § 5.
- [ ] AC-02: acht JSONs mit `vorlage`; PNG-md5 vor/nach gleich; `index.json` enthält die Vorlagen.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Keine manuellen Prüfungen in dieser Session.

## Ergebnis

2026-10-16, Claude (autonom, Ralf im Chat).
- AC-01 geprüft: `tools/gebaeude` liest `vorlage` (`motiv`, `teile`, `materialien`, `stimmung`, `fein`), verlangt `motiv` und `teile`, liest die Datei mit `DisallowUnknownFields` (unbekannte Felder → Abbruch mit Feldname) und schreibt die Vorlage ins `index.json`. Tests: `TestVorlagePflicht` (fehlt, Motiv leer, Teile leer, unbekanntes Feld; erst rot, dann grün), `TestRunSchreibtIndexMitKlasse` prüft die Übernahme, `TestEchteVorlagen` baut alle Gebäude-JSONs des Repos und prüft die Materialnamen gegen § 3. Format in Regeldatei § 5, Ablauf § 9 („Vorlage zuerst“).
- AC-02 geprüft: alle acht JSONs (Werkstatt, Tanne, Pappel, Birke, Taverne, Heilplatz, Schmiede, Rüstkammer) tragen ihre Vorlage, abgeleitet aus Bausteinen, PNG-Sichtung und Sprint-Doku; PNG-md5 vor/nach gleich.
- Checks: `task check` und `task check:go` grün. Review (Sonnet-Agent): keine schweren Befunde; leichte behoben (Materialnamen exakt nach § 3, Lage der Wasserschale, Test über echte Dateien). Nicht übernommen: Holz für Baumstämme in `materialien` (Stamm ist kaum sichtbar).
