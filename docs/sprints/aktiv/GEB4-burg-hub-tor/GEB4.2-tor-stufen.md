# GEB4.2 · Tor 1 bis 5 passend zur Mauer

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb4/2-tor-stufen
- **Abhängig von:** GEB4.1
- **Tickets:** B-435
- **Kriterien:** AC-02, AC-04

## Ziel

`tor.json` (2 × 4 Kacheln, Vorlage, Stufen 2–5 in den Farbstufen der Mauer) liegt mit PNGs in `public/grafik/gebaeude/` und erscheint mit Stufenreihe in der Vorschau.

## Kontext

- **Spec:** GEB4 AC-02, AC-04 und › Anforderungen (Tor).
- Vorbild `mauer.json` (Farbtausch-Tabelle wie dort, damit Tor und Mauer derselben Stufe zusammenpassen). Das Tor steht im Spiel 4 Units außen vor der Mauer seiner Linie und hält Gegner auf (Q47); eigene Truppen passieren.
- **Motiv:** Torbogen mit zwei Flügeln, Querbalken und Beschlägen, niedriger als die Mauer (4 statt 5 Kacheln), Stufe 1 Holz, Stufe 2 Steinbogen, Stufe 3 Kupferbeschläge, Stufe 4 Eisenbänder und Nieten, Stufe 5 Kristallkanten.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `tor.json` und PNGs, `index.json`)
- `tools/gebaeude/`, `src/tools/assetvorschau*.ts` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Darstellung im Spiel (GEB4.3), keine offene/geschlossene Animation, keine Änderung an Mauer oder Burg.

## Schritte

1. `sprint/geb4` auschecken, `Status: in Arbeit`, committen, pushen.
2. `tor.json` nach Kontext; `task gebaeude`; PNGs neben `mauer-<n>-32.png` sichten, nachbessern; md5 der übrigen PNGs gleich.
3. Regeldatei § 1 (Maße), § 4 (Bausteine).
4. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-02/AC-04: `tor-1-32.png` … `tor-5-32.png` je 64 × 128 px plus 16/64/128; zweiter `task gebaeude`-Lauf ändert nichts; Farbstufen wie die Mauer.
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
