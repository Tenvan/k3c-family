# GEB18.2 · Fünf detailreiche Varianten für Wald, Höhle und Mine

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb18/2-varianten-wald-hoehle-mine
- **Abhängig von:** GEB18.1
- **Tickets:** B-618
- **Kriterien:** AC-02

## Ziel

Wald, Höhle und Mine haben je fünf detailreiche Varianten mit drei Schichten in Dichte 128, erzeugt mit dem Generator aus GEB18.1.

## Kontext

Vorlagen-Format aus GEB18.1 (`tools/buildings/codemap.md`), bestehende Schichten `public/grafik/gebaeude/hg-<teil>-<schicht>.json` als Ausgang, Palette DB32, GEB7-Regeln. Die alten Schichten bleiben bis GEB18.5 liegen (der Client nutzt sie noch).

## Erlaubte Dateien

- `public/grafik/gebaeude/hg-<wald|hoehle|mine>-*` (Vorlagen und erzeugte PNG)
- Credits nur, falls fremde Quellen (Default: selbst erzeugt)

## Nicht-Ziele

Eisenstollen und Kristall (GEB18.3), Katalog-Slots, Client.

## Schritte

1. Je Biom fünf Varianten entwerfen: mehr Silhouetten, Strukturen, Lichtakzente, untereinander deutlich verschieden.
2. Erzeugen, Kacheln nahtlos prüfen (`tile_test.go`).
3. Alte Schichten nicht löschen (GEB18.5).

## Fertig, wenn

- [ ] AC-02 (Teil): 3 Biome × 5 Varianten × 3 Schichten in Dichte 128 vorhanden, Generator-Tests grün.

## Prüfen

`check_run go:test` mit Muster `buildings`, vor dem Commit `task check:diff`.

## Ergebnis

- AC-02 (Teil) umgesetzt: Wald, Höhle und Mine haben je fünf Varianten (`hg-<biom>-<schicht>-v1` … `-v5`, 45 Vorlagen, Raster 128) mit eigenem Thema je Variante (Wald: Tannenhang, Laubwald, Lichtung, Mischwald, Birkenhügel; Höhle: Tropfsteinhalle, Spalt, Felsbögen, Pilzgrotte, See; Mine: Schachtwand, Stollen mit Schienen, Erzader, Förderturm, Geröll), gerendert in 32 und 128 px (90 PNG, zusammen 2,5 MB, größte 93 KB). Erzeugt mit einem Autoren-Skript aus festen Seeds; die Vorlagen sind statische Daten (Regeldatei § 10). Das Skript liegt lokal unter `.work/hg-erzeuger/` (gitignored).
- AC-02 geprüft: `go test ./tools/buildings/` grün (Naht, oberste Zeile, Bodenlinie, Helligkeitsstufen je Variante: Wald 43/57–61/70–71, Höhle 35–37/46–51/69–71, Mine 30–32/52–53/65–66, `nah` ≤ 75). Sichtprüfung durch Claude: Ebenen und Tiefe stimmen, die Varianten wirken aber noch schematisch und ähneln sich (vor allem Höhle und Mine) – Urteil und Feinschliff bei Ralf.
- Vorgezogen auf Wunsch von Ralf (Chat 2026-10-24) aus GEB18.3: Studio-Szene „Umgebung“ zeigt je Biom die bisherige Fassung und jede Variante in 128 px ×0,5 und 32 px (`hintergrundStreifen` mit Feld `variante`, `Backgrounds.tsx`); Studio-Vitest `graphics` (54), `tsc`, `task:lint` grün, im Browser geprüft (40 Bilder). `task check:diff` nicht gelaufen (Ralf: in dieser Claude-Session gesperrt).
- Offen: Autoren-Skript ins Repo übernehmen (Ort klären), sonst sind die Vorlagen nur von Hand änderbar.
