# GEB1.3 · Drei schmale Bäume als Daten und PNG

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb1/3-baeume
- **Abhängig von:** GEB1.2
- **Tickets:** B-435
- **Kriterien:** AC-05

## Ziel

Tanne, Pappel und Birke liegen als Daten und PNG (je 1 Kachel breit) in `public/grafik/gebaeude/` und erscheinen in der Asset-Vorschau als überlappende Reihe, gebaut mit `tools/gebaeude` nach `docs/standards/gebaeude-grafik.md`.

## Kontext

- **Spec:** Sprint-README GEB1 AC-05; B-435 › Beispiele: drei schmale Arten, je 1 Kachel breit (32×72 px bei 32 px je Kachel, also 1×2,25 Kacheln), stehen überlappend in Reihen als kleiner Wald wie in Kingdom Two Crowns. Genaue Varianten feilt 🧑 später aus.
- **Grundlage aus GEB1.2:** Werkzeug `tools/gebaeude` (`task gebaeude`), Datenformat und Regeln in `docs/standards/gebaeude-grafik.md`, Vorschau-Streifen „Neue Gebäude (GEB)“ aus `public/grafik/gebaeude/index.json`.
- **Formen:** Tanne = schmaler Kegel, dunkles Grün, Stamm kaum sichtbar; Pappel = hohe schmale Säule, helleres Grün; Birke = weißer Stamm mit dunklen Querstrichen (`fein`), lockere Krone. Licht links oben wie bei der Werkstatt.
- **Reihe:** Bäume im Abstand von ca. ½ Kachel, sodass sie sich überlappen (hintere Bäume zuerst gezeichnet). Wenn das Datenformat Unterkategorien braucht (z. B. `klasse: "baum"`), in Werkzeug und Regeldatei ergänzen.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `tanne.json`, `pappel.json`, `birke.json` und deren PNGs, `index.json`)
- `tools/gebaeude/` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `src/tools/assetvorschau.ts`, `src/tools/assetvorschauLogic.ts`, `src/tools/assetvorschauLogic.test.ts`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine weiteren Baumvarianten oder Level-Paletten, keine Zuordnung zum Slot `node:tree` (`data/assets.json` bleibt unberührt), keine Plantage, keine Animation (Wind).

## Schritte

1. `Status: in Arbeit` auf `sprint/geb1`.
2. Drei Daten-JSON nach Kontext anlegen; `task gebaeude`.
3. Vorschau: Reihe der drei Bäume überlappend (Logik der Anordnung in `assetvorschauLogic.ts` mit Test), 32 px ×1 und 16 px ×2.
4. Regeldatei um Bäume ergänzen (Maße 1×2,25 Kacheln, Farben, Überlappung).
5. Ergebnis ausfüllen, `Status: fertig`.

## Fertig, wenn

- [x] AC-05: `tanne-32.png`, `pappel-32.png`, `birke-32.png` mit je 32×72 px liegen im Repo; zweiter `task gebaeude`-Lauf ändert nichts.
- [x] AC-05: Test in `assetvorschauLogic.test.ts` prüft die überlappende Anordnung der Reihe.
- [x] `task check` und `task check:go` grün. (In GEB1.3: `task check`, `go test ./tools/gebaeude` und `golangci-lint`; das ganze `task check:go` lief in GEB1.4 grün.)

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Keine manuellen Prüfungen in dieser Session; die Sichtprüfung gehört GEB1.4.

## Ergebnis

2026-10-16, Claude (autonom, Ralf im Chat).
- AC-05 geprüft: `tanne`, `pappel`, `birke` (`klasse: baum`, 1 × 2,25 Kacheln) → `*-32.png` je 32×72 px, dazu 16/64/128; zweiter `task gebaeude`-Lauf → md5 gleich.
- AC-05 umgesetzt: `baumReihe` (½ Kachel Abstand, überlappend) mit Test in `assetvorschauLogic.test.ts`; Vorschau zeichnet die Reihe in 32 px ×1 und 16 px ×2 vor den Gebäude-Karten. Sichtprüfung in GEB1.4.
- Werkzeug: Feld `klasse`, Gebäudemaße als Bruch im 1/32-Raster; Review-Befund behoben: erst alles rendern, dann alte PNGs löschen (Test `TestRunLaesstOrdnerBeiFehlerUnveraendert`).
- Checks: `task check` grün (k3c-dev), `go test ./tools/gebaeude`, `golangci-lint` 0 Befunde. Review (Sonnet-Agent): ein Befund, behoben.
- Regeldatei um Bäume (Maße, Material, Reihe) ergänzt. Formen sind ein erster Wurf; Varianten feilt 🧑 später aus (B-435).
