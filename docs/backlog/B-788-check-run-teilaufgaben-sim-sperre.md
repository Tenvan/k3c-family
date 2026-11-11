# B-788 · check_run kennt die Teilaufgaben von check:diff, sim_test und sim_run lehnen während der Sperre ab

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`check:diff` ist seit 2026-11-11 das Werkzeug `tools/checkdiff` mit den Teilaufgaben `check:diff:plan`, `check:diff:js` und `check:diff:go`. Der Katalog von `check_run` (`tools/k3c-workbench/internal/mcpsrv/check_targets.go`) kennt nur `task:check:diff`; die Beschreibung in `tools.go` nennt noch die Basis `origin/develop`. Simulationen und Balancing sind bis auf Widerruf gesperrt (Beschluss Ralf 2026-11-11), `sim_test` und `sim_run` führen sie aber weiter aus.

## Ziel

Agenten können nach einem roten Lauf über MCP nur die betroffene Bahn wiederholen, und die Sperre gilt im Werkzeug statt nur als Regeltext.

## Beteiligte und Zielgruppen

Claude und andere Agenten, 🧑 Ralf (hebt die Sperre auf).

## Anforderungen

- `check_run` nimmt `task:check:diff:plan`, `task:check:diff:js` und `task:check:diff:go` an.
- Die Beschreibung von `check_run` nennt die Basis `HEAD` und die zwei Stufen.
- `sim_test start` und `sim_run` lehnen während der Sperre mit Hinweis auf den Beschluss ab; ein Schalter (Datei oder Umgebung) hebt sie auf.

## Nicht-Ziele

Änderungen an `tools/checkdiff` selbst.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Tempo im Lauf und › Sim, Balance und Gen. Neubau der Workbench-EXE nötig.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `tools/k3c-workbench/internal/mcpsrv/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** exe

## Beispiele

`check_run task:check:diff:go` nach einem roten Go-Lint → nur `go vet`, Lint und Tests der geänderten Pakete laufen.

## Ausnahme- und Fehlerfälle

`sim_test start` während der Sperre → Ablehnung mit Verweis auf den Beschluss, kein Lauf.

## Akzeptanzkriterien

- **AC-01** `check_run` mit den drei neuen Zielen liefert Exit-Code und Dauer (Test in `mcpsrv`).
- **AC-02** `sim_test start` und `sim_run` lehnen während der Sperre ab; mit Schalter laufen sie (Test).

## Offene Fragen

Form des Schalters für die Sperre (Datei in `data/`, Umgebung, Einstellung der Workbench)? Entscheidet 🧑 bei der Freigabe.

## Notizen

–
