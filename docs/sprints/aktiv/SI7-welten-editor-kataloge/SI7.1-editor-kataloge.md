# SI7.1 · Welten-Editor und world-Tools wählen Wetter, Reittier und Figur (B-1003)

- **Status:** offen
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

Voraussetzung (gemergt): SI3, SI4, SI5 – die Welt-Datei hat die optionalen Felder `weather`, `mount`, `hermit`; Kataloge: `data/weather.json`, `data/monarch.json` › `mounts`, `data/hermits.json`; die Engine prüft die IDs beim Laden (`engine/sim/island_worlds.go`, `docs/rules/welten.md` § 2 und § 3). Workbench: Wails-App `tools/k3c-workbench` (eigenes Go-Modul), Backend der Welten in `tools/k3c-workbench/internal/world/`, MCP-Server in `internal/mcpsrv/`, Frontend in `tools/k3c-workbench/frontend/src/world/` (`Overview.tsx`, `NewWorld.tsx`, `WorldValues.tsx`, `useWorld.ts`, `worldOverrides.ts`, Tests `*.test.tsx`). Die Workbench rechnet die Engine im eigenen Prozess (B-924); Kataloge und Prüfung kommen von dort, nichts im Frontend fest verdrahten. Oberfläche nach Atomic Design: Radix nur über `@webkit`, rohe Controls nur in `atoms/` (`tests/atomicDesign.test.ts`). Entwickeln mit `task k3c-workbench` (Dev mit Vite 5191) nur, wenn nötig; die laufende Workbench-EXE nicht neu bauen (das macht 🧑 nach dem Merge).

## Erlaubte Dateien

- `tools/k3c-workbench/**`

## Nicht-Ziele

Kataloge bearbeiten, Vorschau im Editor, Änderungen an `engine/` oder `data/` (fehlt der Engine eine Katalog-Abfrage: Ticket), Neubau der EXE, `tools/webkit` (fehlt ein Baustein: Ticket).

## Schritte

1. Vorhandenen Weg eines Welt-Feldes vom Backend bis zur Übersicht nachvollziehen (z. B. `combat`).
2. Backend: drei Felder lesen und schreiben („keins“ entfernt das Feld), Katalog-Listen aus der Engine anbieten, unbekannte ID mit gültigen IDs ablehnen.
3. MCP: `world_get` nennt die Felder, `world_set` nimmt sie an, `world_check` meldet unbekannte IDs; Tool-Beschreibungen nachziehen.
4. Frontend: drei Auswahlfelder in der Übersicht mit „keins“, ungültiger Bestandswert markiert, Speichern je Feld.
5. Tests: Go (Backend, MCP), Vitest (Übersicht).
6. Fachprüfung durch Agent `k3c-workbench`; `check_run task:check:dev`, `check_run task:check:diff`.

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

Blockiert, nicht umgesetzt. Die Voraussetzung „SI3, SI4, SI5 gemergt“ trifft für die Engine nicht zu: `weather`, `mount` und `hermit` stehen nicht in der Welt-Datei-Dekodierung (`engine/sim/world_decode.go`, `island_worlds.go`), und es gibt keinen Go-Code, der `data/weather.json` und `data/hermits.json` liest. Git zeigt für SI3 nur Regeltext und Review, für SI4 Assets (teilweise), für SI5 einen pausierten Teilstand. Ohne diese Engine-Felder würde world_set Felder schreiben, die die Engine ablehnt. Nächster Schritt: SI3 bis SI5 als Umsetzung nachziehen oder SI7 zurückstellen. Außerdem zeigen die Erlaubten Dateien auf `tools/k3c-workbench/**`; der Welten-Editor liegt im Studio (`tools/k3c-studio`).
