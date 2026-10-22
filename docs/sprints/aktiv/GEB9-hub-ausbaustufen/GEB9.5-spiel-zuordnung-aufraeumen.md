# GEB9.5 · Spiel wählt Hub-Gebäude nach Hub-Stufe, Grundform-Dateien löschen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb9/5-spiel-zuordnung-aufraeumen
- **Abhängig von:** GEB9.3, GEB9.4
- **Tickets:** B-435
- **Kriterien:** AC-04, AC-05

## Ziel

Die acht Hub-Gebäude zeigen im Spiel die Grafik der aktuellen Hub-Stufe; die Grundform-Dateien `<id>-<dichte>.png` der GEB9-Gebäude und alle Verweise darauf sind gelöscht.

## Kontext

- Heute nutzt nur die Burg die Hub-Stufe: `src/scenes/render/siteView.ts` › `updateCastleView`, `src/scenes/render/buildingSprites.ts` › `hubTexture`, `stufen`. `siteTexture` bekommt sonst die Stufe des Bauplatzes. Die Hub-Stufe steht im Snapshot (`World.hubLevel`).
- Ziel: Werkstatt, Farm, Kaserne, Lager, Taverne, Schmiede, Heilplatz, Rüstkammer wählen `<id>-<hubLevel>-32.png`; fehlt die Stufe, gilt die höchste vorhandene, ohne Stufen die Grundform; nicht geladene Stufe fällt auf Stufe 1 zurück; Bau und Zerstörung unverändert (Platzhalter). Stufenzahl nicht verdrahten.
- Aufräum-Anforderung (Projekt GEB › Notizen): nach dem Umstellen die Dateien `public/grafik/gebaeude/<id>-<16|32|64|128>.png` der acht Gebäude, Einträge darauf in `public/grafik/index.json`/`gebaeude/index.json`, Register-Wahl in `data/assets.json`, `docs/assets/zuordnung-*.md`, Test-Fixtures, Mocks und CREDITS löschen bzw. anpassen; nur was keine andere Stelle mehr nutzt (vorher per Suche belegen). Der Übergang in `tools/buildings` (Übergangsliste aus GEB9.2) wird entfernt.
- Tests: `tests/projectRules.test.ts`, bestehende Tests zu `buildingSprites`; neue Zuordnungs-Tests dazu.

## Erlaubte Dateien

- `src/scenes/render/buildingSprites.ts`, `siteView.ts`, `natureSprites.ts` nur falls nötig, zugehörige Tests unter `tests/`
- `public/grafik/gebaeude/*.png` (Löschen der Grundformen der acht Gebäude), `public/grafik/**/index.json`, `data/assets.json`, `docs/assets/zuordnung-*.md`, `CREDITS`-Zeilen, Fixtures/Mocks
- `tools/buildings/` (Übergangsliste entfernen, Test anpassen), `docs/standards/gebaeude-grafik.md` (Übergang streichen)

## Nicht-Ziele

Camp, Burg/Mauer/Turm (unverändert), `engine/`, `data/hub.json`, Protokoll, neue Dichten.

## Schritte

1. Branch `geb9/5-spiel-zuordnung-aufraeumen`, `Status: in Arbeit`. Dateien oben lesen; Verwendung der Grundform-Dateien per Suche belegen.
2. Test zuerst: Zuordnung liefert je Hub-Gebäude und Hub-Stufe 1–5 den Texturnamen `<id>-<n>-32`; unbekannte Stufe → höchste vorhandene; ohne `stufen` → Grundform.
3. `siteView.ts` / `buildingSprites.ts` umstellen (Funktionen ≤ 60 Zeilen, Datei ≤ 400).
4. Grundform-Dateien, Verweise und Übergangsliste entfernen; `task gen:gebaeude` ohne Diff (außer den gelöschten Dateien).
5. Prüfen: `task check:diff`; Review-Agent (Sonnet) parallel bei ≥ 150 Code-Zeilen. Ergebnis, `Status: fertig`.

## Fertig, wenn

- [ ] AC-04: Zuordnungs-Test grün (Stufen 1–5, Fallbacks).
- [ ] AC-05: keine Datei `<id>-<dichte>.png` der acht Gebäude, keine Verweise (Suche belegt es im Ergebnis); `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-04 geprüft: `buildingSprites.test.ts` prüft für alle acht Hub-Gebäude Stufe 1–5 (Textur `grafik:<kind>-<n>` → `gebaeude/<id>-<n>-32.png`, Maßstab 1), Stufe 6 → höchste vorhandene, Stufe 0 und ohne Stufe → Grundform (Stufe 1), Baustelle → Platzhalter. `siteView.ts` wählt die Stufe über `siteStage` (Tor je Mauerlinie, Hub-Gebäude `World.hubLevel` mit Rückfall 1, sonst `Site.level`) und fällt auf die Grundform zurück, wenn die Stufen-Textur nicht geladen ist. AC-05 umgesetzt: 32 Grundform-Dateien `<id>-<dichte>.png` der acht Gebäude sind gelöscht (der Generator räumt beim Lauf auf), Übergangsliste und Kopie aus `tools/buildings` entfernt; die Suche nach `(werkstatt|…|kaserne)-(16|32|64|128).png` findet außer erledigten Sprints, Spec-Beispiel und dem Camp (`rekrutierungslager`, bleibt) nichts mehr. Register `data/assets.json`: Slot ohne Stufe zeigt `<id>-1-32.png`, neu `<kind>:1`–`<kind>:5` je Gebäude; `docs/assets/zuordnung-objekte.md` und § 6 nachgezogen. Keine Test-Fixtures, Mocks oder CREDITS-Zeilen betroffen (eigene Grafik, `catalog_test.go` schreibt seine Datei selbst).

Prüfung: `task test` (Client, 2348 Tests), `task typecheck`, `task lint` (nach Aufteilen von `updateSiteView` in `siteStage`, Komplexität war 17), `go test ./tools/buildings` grün; `task check:diff` als Ganzes kam wieder nicht durch das 10-Minuten-Zeitlimit (Studio-Vitest unter Last), das Review GEB9.6 fährt `task check`, `check:go`, `check:race` nacheinander. Kein Reviewer-Agent: Code-Änderung unter 100 Zeilen, nicht SIM/SRV. Nicht im Browser geprüft (manuelle Abnahme AC-06 bleibt bei Ralf).
