# GEB18.2 · Fünf detailreiche Varianten für Wald, Höhle und Mine

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
