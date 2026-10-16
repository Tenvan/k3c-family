# GEB8.1 · Vorlage im Werkzeug und in allen acht Gebäude-JSONs

- **Status:** in Arbeit
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
