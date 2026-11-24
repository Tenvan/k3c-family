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

Review WE6 für die Studio-Anteile (WE6.3, WE6.4) durch Agent k3c-workbench: keine schweren Befunde. Behoben: unbekannte Insel-ID legte eine neue Insel an (jetzt Ablehnung vor dem Schreiben), unbekannter Endboss wird gegen data/bosses.json geprüft (Commit d669ed275, PR #865). Events und Plätze stehen in der Werte-Ansicht über die WE6.3-Tests. Nicht umgesetzt: Warnung bei fehlendem Asset und Bild-Vorschau der Skins (Engine prüft die Existenz nicht); Format-Feinheiten der Skin-Tabelle (leichte Befunde, im PR benannt). Abnahme: Sichtprüfung der Welten-Oberfläche durch 🧑. WE6.6 (Rust-Spiel) ist noch offen, Lane zweit.
