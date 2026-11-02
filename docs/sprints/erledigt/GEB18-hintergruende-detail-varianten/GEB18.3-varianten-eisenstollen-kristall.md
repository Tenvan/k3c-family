# GEB18.3 · Fünf detailreiche Varianten für Eisenstollen und Kristallhöhle, Studio-Szene Umgebung

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb18/3-varianten-eisenstollen-kristall
- **Abhängig von:** GEB18.1
- **Tickets:** B-618
- **Kriterien:** AC-02

## Ziel

Eisenstollen und Kristallhöhle haben je fünf Varianten; alle Varianten aller fünf Stufen stehen als Slots im Katalog und in der Studio-Szene „Umgebung“.

## Kontext

Wie GEB18.2; Katalog `data/assets.json` (Slots `bg:<biom>:<n>`), Studio `tools/k3c-studio/frontend/src/graphics/Backgrounds.tsx` und `scenesLogic.ts` (`hintergrundStreifen`).

## Erlaubte Dateien

- `public/grafik/gebaeude/hg-<eisenstollen|kristall>-*`
- `data/assets.json` (Hintergrund-Slots je Variante)
- `tools/k3c-studio/frontend/src/graphics/**` (Szene „Umgebung“ mit Varianten, Tests)

## Nicht-Ziele

Client des Spiels, Level-Generator.

## Schritte

1. Varianten Eisenstollen und Kristall wie in GEB18.2.
2. Slots je Variante im Katalog, Test: jede Stufe hat fünf Varianten mit drei Schichten.
3. Studio-Szene „Umgebung“ zeigt je Biom alle Varianten, Vitest.

## Fertig, wenn

- [ ] AC-02: Katalog-Test und Studio-Vitest grün; alle 5 × 5 × 3 Schichten vorhanden.

## Prüfen

Gezielte Tests (`task:test` mit Muster `assets`, Studio-Vitest `graphics`), vor dem Commit `task check:diff`.

## Ergebnis

- AC-02 umgesetzt: Eisenstollen (Schmiedehalle, Trägerbrücke, Lavaspalte, Kettenschacht, Hochofen) und Kristallhöhle (Kristallhalle, Geodenwand, Kristallsee, Zapfenwald, leuchtende Adern) haben je fünf Varianten (30 Vorlagen, 60 PNG); damit alle 5 Stufen × 5 Varianten × 3 Schichten (150 PNG, 4,3 MB). Katalog: 75 Slots `bg:<biom>:<schicht>:v<n>` auf die 128-px-Datei, hinter `bg:crystal:3`; die alten `bg:<biom>:<n>` bleiben bis GEB18.5. Studio-Szene „Umgebung“ zeigte die Varianten schon seit GEB18.2.
- AC-02 geprüft: Go-Test `TestHintergrundVariantenJeStufe` (5 Biome, je mindestens 5 vollständige Varianten) und Helligkeit je Variante grün (Eisenstollen 24–27/51–54/72–75, Kristall 35/54–57/70); Vitest `assets`, `zuordnung`, `worldSprites` grün; `tools/assetkit` und `k3c-server -check-data` grün. `task check:diff` nicht gelaufen (Ralf: in dieser Claude-Session gesperrt).
- Fix aus GEB18.1: Eine ID ohne Endung zählte als Variante 1 und fiel mit `-v1` zusammen (Helligkeitstest hätte alte und neue Schichten gemischt); jetzt bisherige Fassung = Variante 0, zählt nicht mit. Test in `tools/buildings/` (außerhalb der erlaubten Dateien, nötig für den Katalog-Test).
- Sichtprüfung durch Claude: Regeln erfüllt, Motive aber noch schematisch (flache Träger, wenig Struktur); Urteil bei Ralf in GEB18.7.
