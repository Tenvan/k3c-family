# SK2.2 · Preset Dieb angepasst, Presets beim Laden geprüft

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** sk2/2-preset-dieb
- **Abhängig von:** – (Default aus Offene Fragen)
- **Tickets:** B-504
- **Kriterien:** AC-02

## Ziel

Das Preset Dieb nennt keine fehlenden Skills mehr, und das Laden der Daten scheitert mit Meldung, wenn ein Preset einen unbekannten Skill nennt.

## Kontext

`data/monarch.json › presets.thief.activeSkills` nennt `backstab`, `smokeBomb` (fehlen in `skills`). `ApplyPreset` (`engine/sim/monarch.go`) überspringt sie still. Geladen wird in `engine/sim/data.go` (`monarch = load[...]`, `UseData`), `-check-data` des Servers lädt denselben Code. Default aus Sprint-README › Offene Fragen: Preset leer.

## Erlaubte Dateien

- `data/monarch.json` (nur `presets.thief.activeSkills`)
- `engine/sim/data.go`, `engine/sim/monarch_data.go`, `engine/sim/monarch.go` (Kommentar), `engine/sim/monarch_*_test.go`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Dieb-Skills bauen, Basiswerte des Presets.

## Schritte

1. `checkPresets(m monarchData) error` in `monarch_data.go`; beim Laden (`load` und `UseData`) aufrufen.
2. Preset Dieb leer.
3. Go-Test: eingebettete Daten gültig, unbekannter Skill ergibt Fehler mit Preset und Skill.
4. `task check:diff`.

## Fertig, wenn

- [ ] AC-02 (B-504/AC-01): Go-Test grün.
- [ ] Golden unverändert (Preset Dieb wird von keinem Spieler genutzt).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

2026-10-23, autonom (Claude). Default aus SK2 › Offene Fragen (Bestätigung B-560).

- **AC-02 (B-504/AC-01) umgesetzt, geprüft:** `presets.thief.activeSkills` leer; `checkPresets` (`engine/sim/monarch_data.go`) läuft beim Start (`loadMonarch`, Panik mit Meldung wie jedes ungültige Datum) und in `UseData` (alter Stand bleibt). Test `TestPresetsNennenNurVorhandeneSkills` (eingebettete Daten gültig, unbekannter Skill → Fehler mit Preset und Skill).
- Golden unverändert (Preset Dieb nutzt kein Spieler). Studio-Tests der Monarch-Seite (`actions.test.ts`, `MonarchPage.test.tsx`, Kommentar `actions.ts`) nachgezogen: 24 Zeilen, keine Lücke „Skill fehlt“ (außerhalb der Liste, erzwungen durch die Daten).
- Reviewer (Sonnet): keine schweren Befunde. `task check:diff`: nur `engine/devrun` rot (Wanduhr-Tests unter Last, B-284; einzeln grün), Rest grün.
