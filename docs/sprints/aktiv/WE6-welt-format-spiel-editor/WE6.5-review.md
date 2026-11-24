# WE6.5 · Review WE6 und Abnahme-Ticket

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/we6
- **Abhängig von:** WE6.6, WE6.3, WE6.4
- **Tickets:** B-987, B-993, B-994
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Der Sprint WE6 ist geprüft, Befunde sind behoben oder als Ticket angelegt, das Abnahme-Ticket steht.

## Kontext

Sprint-README `docs/sprints/*/WE6-welt-format-spiel-editor/README.md` mit AC-01 bis AC-04, Tickets B-987, B-993, B-994. Reviewer getrennt vom Schreiben: `k3c-hud`/`k3c-grafik` (Spiel), `k3c-workbench` (Editor, MCP). Gen und `check:race` nur im Abnahme-Ticket (B-742).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/**`
- `data/paper.json`
- `tools/k3c-workbench/internal/world/**`
- `tools/k3c-workbench/internal/mcpsrv/**`
- `tools/k3c-workbench/frontend/src/world/**`

## Nicht-Ziele

Neue Funktionen, Simulationen.

## Schritte

1. Diff des Sprints lesen, Reviewer-Agenten parallel starten.
2. AC-01 bis AC-04 mit Nachweis prüfen.
3. Schwere Befunde beheben, Rest als Ticket mit Projekt und Sprint.
4. `task check`, `task check:go`, `task check:dev` nacheinander.
5. Abnahme-Ticket anlegen, Abschnitt Abnahme füllen, offene Sichtprüfungen 🧑 nennen.

## Fertig, wenn

- [ ] AC-01 bis AC-04 mit Nachweis in der Sprint-README › Abnahme.
- [ ] Prüfläufe grün; genau ein Abnahme-Ticket angelegt.

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

Review WE6, Studio-Anteile (WE6.3, WE6.4) und PR #875 nur gelesen, ohne Prüfläufe (Tests ausstehend).

- AC-03 erfüllt: Tests in `tools/k3c-studio/internal/world/edit_endboss_skins_test.go` und `edit_override_lists_test.go`. Einschränkung: kein MCP-Roundtrip-Test, MCP-Beschreibung nennt `endBoss`, `skins`, `events` nicht.
- AC-04 erfüllt im Code (`frontend/src/world/WorldBoss.tsx`), `task check:dev` steht aus.
- AC-02 teilweise: #875 wendet Skins auf Gegner an (`actors/catalog.rs` › `skinned`), Rückfall auf das Basis-Asset ist da. Objekt-Skins und Log bei fehlendem Ersatz-Asset fehlen, Ticket nötig.
- AC-01 offen: Bau und Endboss je Biom, kein Standbild-Test. WE6.6 liefert ihn nach.
- Befund: „Erlaubte Dateien“ nennen `tools/k3c-workbench/internal/{world,mcpsrv}/**` und `frontend/src/world/**`; der Code liegt in `tools/k3c-studio/`. Vor dem Abnahme-Ticket korrigieren.
- Abnahme-Ticket erst nach AC-01. Sichtprüfung der Welten-Oberfläche und B-993/AC-03 durch 🧑.
