# GS2.4 · Review: Sprint-Abnahme GS2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/gs2
- **Abhängig von:** GS2.3
- **Tickets:** B-964, B-977
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

Der Sprint GS2 ist gegen seine Kriterien geprüft, Befunde sind behoben oder als Ticket angelegt, die Abnahme steht in der README.

## Kontext

Ablauf: `docs/arbeitsweise.md` › Review-Session. Kriterien: README des Sprints GS2 (`docs/sprints/geplant/GS2-seekarte-papierschnitt/README.md`, nach dem Aktivieren unter `aktiv/`), Tickets B-964, B-977. Diff von `sprint/gs2` gegen den Start-Commit. Fachprüfer: `k3c-grafik` (Assets, Texturbudget, `data/paper.json`), `k3c-hud` (Overlay, Fahrt-Szene). TV-Abnahmen (AC-05) bleiben bei 🧑.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- `native/k3c-game/game/**`
- `native/k3c-game/art/**`

## Nicht-Ziele

Neue Assets oder Funktionen, Andenken (GS3).

## Schritte

1. `check_run task:check`, danach `check_run task:check:go`.
2. Nachweise AC-01 bis AC-04 aus den Session-Ergebnissen prüfen.
3. Fachprüfer, schwere Befunde beheben, Rest als Tickets mit Projekt und Sprint.
4. Abnahme-Ticket anlegen (Gen samt Grafik-Golden, Race; B-742), Abschnitt Abnahme füllen, AC-05 als offen bei 🧑 nennen.

## Fertig, wenn

- [ ] AC-01 bis AC-04 belegt, AC-05 als offen bei 🧑 vermerkt.
- [ ] Abnahme-Ticket angelegt, Abschnitt Abnahme gefüllt.

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

AC-01 bis AC-04 belegt (Ergebnisse GS2.1 bis GS2.3; k3c-grafik ohne schwere oder mittlere Befunde, k3c-hud). Behoben: Fahrt blieb aus, wenn der Hafen nicht in Stufe 0 lag (`stillAt`, Test `TestVoyageStartsFromDeeperStage`); Schiff wechselte in der Streckenmitte die Zeichen-Reihe. Werte der Fahrt in `data/native.json` › `voyage`; Tests `TestVoyageLoadFailureIsFault`, `TestVoyageDeviceLost`. Commit 87fb7f6b1, PR #648. `go test`, Lint 0, `vitest tools/paper` (411), `check:go`, `check:diff` grün; `task check` einmal rot durch Timeout von `projectRules` › K3C_STUDIO_LAN unter Last, Einzellauf grün. AC-05 (TV) offen. Abnahme-Ticket B-1011, Nachträge B-1012.
