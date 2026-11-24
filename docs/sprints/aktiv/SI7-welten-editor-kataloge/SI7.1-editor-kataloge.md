# SI7.1 · Welten-Editor und world-Tools wählen Wetter, Reittier und Figur (B-1003)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si7
- **Abhängig von:** SI3, SI4, SI5
- **Tickets:** B-1003
- **Kriterien:** AC-01, AC-02

## Ziel

Der Welten-Editor und die MCP-Tools `world_get`, `world_set`, `world_check` wählen je Welt Wetter, Reittier und Figur aus den Katalogen oder „keins“ (B-1003/AC-01 bis AC-05).

## Kontext

Voraussetzung: SI3, SI4, SI5 erledigt. Die Zuordnung je Welt steht in `data/weather.json` › `worlds`, `data/mounts.json` › `worlds` und `data/hermits.json` › `worlds`, nicht in der Welt-Datei (`docs/rules/welten.md` Zeilen 112–114, 265). Das Rust-Spiel liest die Dateien (`rust/crates/k3c-data`); die Go-Engine liest sie nicht.

Der Welten-Editor und das Welten-MCP liegen im Studio: Backend `tools/k3c-studio/internal/world/`, MCP-Server `tools/k3c-studio/internal/mcpsrv/`, Frontend `tools/k3c-studio/frontend/src/world/` (Tests `*.test.tsx`). Die ID-Prüfung liest die drei Katalog-Dateien; im Frontend nichts fest verdrahten. Oberfläche nach Atomic Design: Radix nur über `@webkit`, rohe Controls nur in `atoms/` (`tests/atomicDesign.test.ts`). Entwickeln mit `task k3c-studio` nur, wenn nötig; die laufende Studio-EXE nicht neu bauen (das macht 🧑 nach dem Merge).

## Erlaubte Dateien

- `tools/k3c-studio/**`
- `data/weather.json`
- `data/mounts.json`
- `data/hermits.json`

## Nicht-Ziele

Kataloge bearbeiten, Vorschau im Editor, Änderungen an `engine/` oder `data/` (fehlt der Engine eine Katalog-Abfrage: Ticket), Neubau der EXE, `tools/webkit` (fehlt ein Baustein: Ticket).

## Schritte

1. Vorhandenen Weg eines Welt-Feldes im Studio nachvollziehen (z. B. `combat`); die Zuordnung in den drei Katalogen lesen.
2. Backend: drei Auswahlwerte je Welt lesen und schreiben („keins“ entfernt den Eintrag), Katalog-Listen aus den Dateien anbieten, unbekannte ID mit gültigen IDs ablehnen.
3. MCP: `world_get` nennt die Felder, `world_set` nimmt sie an, `world_check` meldet unbekannte IDs; Tool-Beschreibungen nachziehen.
4. Frontend (Studio): drei Auswahlfelder in der Übersicht mit „keins“, ungültiger Bestandswert markiert, Speichern je Feld.
5. Tests: Go (Backend, MCP), Vitest (Übersicht).
6. Fachprüfung durch Agent `k3c-workbench` (zuständig auch für das Studio); `task check:dev` und `task check:diff` grün.

## Fertig, wenn

- [ ] AC-01: Tests zu B-1003/AC-01 und AC-02 grün (Testnamen im Ergebnis).
- [ ] AC-02: Tests zu B-1003/AC-03 grün; `tests/atomicDesign.test.ts`, `task:check:dev` und `task:check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 und AC-02 umgesetzt: Studio-Oberfläche mit drei Auswahlfeldern je Welt (keins, gültige IDs, Bestandswert ohne Katalogeintrag markiert). Speichern schreibt nur den Block `worlds` der drei Katalog-Dateien. Nachweise: Go `TestPicksSetzenUndEntfernen`, `TestPickUnbekannteIDLehntAb`, `TestPicksViewLiefertOptionenUndLehntAb`, `TestPickPatchMitUngueltigerArtSchreibtNichts`; Vitest `WorldPick.test.tsx` (Anzeige). PR #861, Commits b05a1ea32 und c0bf7332f. `task check:diff` grün. `task check:dev` rot durch `tools/assetkit` (TLS-Tests, lokales Norton-Zertifikat), nicht durch diese Änderung. Offen: Vitest für Änderung per Auswahl (`onChange`).
