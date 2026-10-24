# GEB18.1 · Generator erzeugt Hintergründe in vierfacher Auflösung und mit Varianten

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb18/1-generator-dichte-varianten
- **Abhängig von:** –
- **Tickets:** B-618
- **Kriterien:** AC-01

## Ziel

`tools/buildings` erzeugt Hintergrund-Schichten in Dichte 128 (vierfache Auflösung) und kennt Varianten je Biom (Ablage `hg-<teil>-<schicht>-v<n>-128.png`).

## Kontext

Generator `tools/buildings` (`buildings.go`: `erlaubt(d)`, `render`, `renderDichten`; `background_test.go`), Vorlagen und Ausgaben unter `public/grafik/gebaeude/hg-*`; Client-Abbildung `src/scenes/render/worldSprites.ts` (`BACKGROUND_LAYERS`, Slots `bg:<biom>:<n>`); Spec B-618, GEB7-Regeln in `docs/rules`.

## Erlaubte Dateien

- `tools/buildings/**`
- `Taskfile`-Eintrag in `tools/` nur, falls ein neuer Generator-Aufruf nötig ist

## Nicht-Ziele

Neue Vorlagen-Inhalte (GEB18.2, GEB18.3), Client, Level-Generator.

## Schritte

1. Dichte 128 für Hintergrund-Klassen erlauben, Ausgabe-Namen mit Variante.
2. Vorlagen-Format für Varianten (eine Vorlage je Variante oder Varianten-Liste) festlegen und in `codemap.md` beschreiben.
3. Go-Tests: Größe ×4 gegenüber 32, Varianten-Namen, Prüfung „fünf Varianten je Biom“ als Hilfsfunktion für GEB18.3.

## Fertig, wenn

- [ ] AC-01: `go test ./tools/buildings/...` grün mit Tests für Dichte 128 und Varianten.

## Prüfen

`check_run go:test` mit Muster `buildings`, vor dem Commit `task check:diff`.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
