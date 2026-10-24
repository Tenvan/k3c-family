# GEB10.2 · Geschlossene Truhe, Vorschau zeigt beide

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb10/2-truhe-geschlossen
- **Abhängig von:** GEB10.1
- **Tickets:** B-455
- **Kriterien:** AC-02

## Ziel

Neben der offenen Truhe (`truhe.json`) gibt es `truhe-geschlossen.json` mit PNG 16/32/64/128; die Vorschau zeigt beide nebeneinander.

## Kontext

- **Spec:** GEB10 AC-02 (B-455/AC-01), Entscheidung 4 der Sprint-README: eigene JSON statt Stufe, weil der Goldhaufen wegfällt (Stufen tauschen nur Farben und fügen hinzu, § 6).
- `truhe.json`: 1 × 1 Kachel, Klasse `beute`, Holz `rope`/`twine`/`oiled-cedar`, Eisenbänder, Gold. Die Vorschau zeigt jeden Eintrag des `index.json` als Karte (`gebaeudeKarten`), sortiert nach ID, also beide direkt nacheinander.

## Erlaubte Dateien

- `public/grafik/gebaeude/truhe-geschlossen.json` und ihre PNGs, `index.json`
- `tools/k3c-studio/frontend/src/graphics/` (nur falls die Vorschau es braucht, mit Test)
- `docs/standards/gebaeude-grafik.md` (§ 1)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Spiel-Zuordnung (GEB10.3); Truhen-Zustand in Simulation oder Protokoll.

## Schritte

1. Vorlage schreiben: geschlossener, gewölbter Deckel, Eisenbänder, Schloss, gleiche Maße und Materialien wie die offene Truhe.
2. Bausteine zeichnen, `task gen:gebaeude` zweimal, PNG (64 px) ansehen.
3. Regeldatei § 1 ergänzen.

## Fertig, wenn

- [x] AC-02: `truhe-geschlossen` im `index.json` mit vier Dichten, Vorschau zeigt sie neben `truhe` (Studio-Vitest grün).
- [x] Zweiter `task gen:gebaeude` byte-gleich; `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-02 (B-455/AC-01) umgesetzt, geprüft:** `truhe-geschlossen.json` (1 × 1, Klasse `beute`, Vorlage; Deckel gewölbt, Eisenbänder über Deckel und Körper, Körper aus `truhe` übernommen) mit PNG 16/32/64/128; `task gen:gebaeude --force` zweimal md5-gleich (53 Gebäude); 64 px und 16 px gesichtet.
- Vorschau: Karten aus dem echten `index.json` zeigen `truhe-geschlossen` direkt vor `truhe` (neuer Vitest „echte Grafik (GEB10)“, prüft auch Baumhöhen und Flora-Reihe); Studio-Vitest 26/26, `tsc` grün. Regeldatei § 1 ergänzt.
- B-455/AC-02 (Spiel-Zuordnung) folgt in GEB10.3.
