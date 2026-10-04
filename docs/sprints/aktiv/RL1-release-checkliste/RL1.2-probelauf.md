# RL1.2 · Probelauf der Checkliste ohne Tag, Sprint abschließen

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** rl1/2-probelauf
- **Abhängig von:** RL1.1
- **Tickets:** B-170
- **Kriterien:** AC-03

## Ziel

Die Release-Checkliste ist einmal vollständig durchgegangen, ohne einen Tag zu setzen; das Ergebnis steht in der Abnahme der Sprint-README, rote Punkte sind Tickets, der Sprint liegt in `docs/sprints/erledigt/`.

## Kontext

Doku-Sprint ohne Review: diese Session schließt den Sprint ab (Schritte 4–5 der Review-Session, `docs/arbeitsweise.md` › Sprint-Lebenslauf). 🧑 führt den Probelauf, weil Punkte wie Pi-Image und Version auf der Xbox Geräte brauchen (Plan § 11.1: Sperre durch 🧑 bei RL1). Ein Agent darf mitlaufen: die Befehle der Liste ausführen, CI-Läufe nachsehen und Abnahme sowie Ordner-Verschiebung schreiben. Hardware-Punkte, die das Gerät gerade nicht erlaubt, gelten nach „Hardware entkoppelt“ als `angenommen, Validierung offen` und stehen so in der Abnahme.

## Erlaubte Dateien

- Sprint-README (Abnahme, Tabelle), diese Datei (Ergebnis)
- `docs/backlog/` (Status B-170, neue Tickets für rote Punkte), `docs/sprints/README.md` (Fahrplan), `docs/roadmap.md`

## Nicht-Ziele

Tag setzen, Befunde beheben (nur Tickets), Änderung der Liste (falls nötig: Ticket oder kurze Korrektur mit Begründung im Ergebnis).

## Schritte

1. Stand `origin/develop` nehmen, die Liste aus `docs/arbeitsweise.md` › „Release“ Punkt für Punkt abarbeiten (Befehle lokal, CI-Läufe des letzten Merge-Commits, Pi und Landingpage am Gerät, soweit verfügbar).
2. Je Punkt: grün, rot (Ticket) oder `angenommen, Validierung offen` mit Grund.
3. Abnahme (höchstens fünf Zeilen) in die Sprint-README: Datum, Ergebnis je Kriterium, Tickets, `Version: v… vorgeschlagen (Grund)` (Patch, Doku-Sprint).
4. B-170 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
5. Sprint-Ordner nach `docs/sprints/erledigt/`, `Status: erledigt`, Fahrplan anpassen, PR öffnen.

## Fertig, wenn

- [ ] AC-03: Probelauf ohne Tag durchgeführt, Ergebnis je Punkt in der Abnahme; rote Punkte als Tickets.
- [ ] Sprint liegt unter `docs/sprints/erledigt/`, B-170 archiviert.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Pi, Xbox) nur durch 🧑.

## Ergebnis

Agenten-Teil des Probelaufs, 2026-10-04, Stand `origin/develop` 46aa69b (CI-Lauf 37227816602). Kein Tag gesetzt.

| Punkt | Ergebnis | Beleg |
|---|---|---|
| Golden amd64 | grün | CI-Job „Go · Tests · Lint · Cross-Build“ |
| Golden arm64 | grün | CI-Job „Go · Tests arm64“ |
| Alles grün (`task check:all`) | rot (flackert) | Windows: `check`, `build`, `check:go`, `check:dev` je einzeln grün; in 1 von 2 Go-Läufen `TestRestore` rot, einzeln 53× grün → B-274. Weitere Fehlläufe ohne Befund im Projekt: Hook-Artefakt `.omc/` im Sprint-Ordner, fehlendes `npm ci` für das k3c-dev-Frontend im frischen Worktree (`task install`) |
| Spielstand-Migration | grün | `IslandSaveVersion = 4`, Fixtures `testdata/saves/v1`–`v4`, `TestJedeVersionHatFixture` in `go test` grün |
| Dev-Reste aus | rot | Server: Dev-Mode an, solange `K3C_DEV` ≠ `0`, `Dockerfile`/`compose.yaml` setzen nichts → B-273. Client: Overlay standardmäßig an (`?dev=0` schaltet ab, `src/scenes/debugOverlay.ts`) → bekannt als B-098 (K5); die Checkliste beschreibt den Zielzustand |
| Pi-Image | grün (CI), Gerät `angenommen, Validierung offen` | CI-Job „Docker · Image amd64/arm64 · Smoke-Test“; `release.yml` für v0.6.0 grün; Pull am Pi nur durch 🧑 |
| Version stimmt | `angenommen, Validierung offen` | `VERSION` aus `git describe` (Taskfile), `TestHealthMeldetVersion` grün, Landingpage zeigt `versionLine`/`versionMismatch`; Abgleich am Pi und auf der Xbox nur durch 🧑 |
| Credits vollständig | grün | `src/tools/credits.test.ts` in `task test` grün |
| Tag-Schema | grün | Verweis auf „Entscheidungen und Versionen“ vorhanden |

Ergebnis: Mit zwei roten Punkten (B-273, B-274) und dem bekannten B-098 wäre ein Tag blockiert. Die Liste selbst brauchte keine Korrektur.
