# RV1.5 · Review des Sprints RV1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** review
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** rv1/5-review
- **Abhängig von:** RV1.1, RV1.2, RV1.3, RV1.4
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07

## Ziel

Der Sprint RV1 ist geprüft: alle Kriterien AC-01 bis AC-07 mit Nachweis, Befunde behoben oder als Ticket, Abnahme der Sprint-README ausgefüllt, ein PR `sprint/rv1` → `develop`.

## Kontext

- Sprint-README `docs/sprints/aktiv/RV1-asset-review/README.md` (Kriterien), Session-Ergebnisse RV1.1–RV1.4.
- Code: `tools/k3c-workbench/internal/review/`, `internal/assetsui/`, `app_assets.go`, `frontend/src/assets/review/`.
- Vertrag: `docs/standards/paper-mcp.md` › 1 und 4, `docs/standards/paper-spec.md` › 6 und 8. Abweichungen vom Vertrag sind Befunde, keine stillen Änderungen.
- Schwerpunkte: Shell-/Pfad-Code (`git`- und `bun`-Aufrufe ohne Shell, Pfade aus `id` nicht außerhalb von `public/grafik/`), atomares Schreiben des Caches, Fehlertexte wörtlich wie der Vertrag, keine Fixture-Abhängigkeit im Produktionscode, Dateigröße ≤ 400 und Funktionen ≤ 60 Zeilen, Atomic Design.
- Die sichtbare Abnahme (Monarch mit umrandetem Umhang nach Änderung am Umhang-Material) braucht den echten Renderer (RD1) und eine Monarch-Spec im Checkout; ohne beides bleibt sie offen (Sprint nicht erledigt, Domäne frei).

## Erlaubte Dateien

- `tools/k3c-workbench/internal/review/`, `tools/k3c-workbench/internal/assetsui/`, `tools/k3c-workbench/app_assets.go`, `tools/k3c-workbench/frontend/src/assets/review/` (nur Fixes aus dem Review)
- `docs/sprints/aktiv/RV1-asset-review/` (Ergebnis, Abnahme)

## Nicht-Ziele

- Keine neuen Funktionen; größere Befunde (über etwa 30 Zeilen) werden Tickets im Projekt PAP.
- Keine Änderung an Vertrag oder Fixtures (Ticket).

## Schritte

1. Diff des Sprints gegen den Start-Commit lesen; Code-Review mit getrenntem Reviewer-Agent (Opus) nach den Schwerpunkten aus dem Kontext.
2. Je Kriterium den Nachweis aus den Session-Ergebnissen nachprüfen: AC-01/02 `go test ./internal/review/`, AC-05/07 `go test ./internal/assetsui/`, AC-03–06 `task k3c-workbench:vitest`.
3. Befunde beheben (Fix mit Test) oder als Ticket anlegen.
4. Volle Prüfung nacheinander: `task check`, `task check:go`, `task check:race` (ohne gcc übersprungen, im Ergebnis nennen), `task check:dev`.
5. Manuelle Abnahme durch 🧑 Ralf (nur wenn RD1 und die Monarch-Spec da sind): `task k3c-workbench:web`, Assets › Review, Umhang-Material per `assets_set` ändern, Basis `HEAD` → Monarch mit Box um den Umhang, Animation läuft synchron, Prompt-Button kopiert.
6. Ergebnis und Abnahme der README (höchstens fünf Zeilen) füllen, Status der Sessions setzen, Commit `chore(dev): Review RV1`, PR `sprint/rv1` → `develop`.

## Fertig, wenn

- [ ] AC-01 bis AC-07: Nachweis je Kriterium im Ergebnis (Testname und Befehl).
- [ ] `task check`, `task check:go`, `task check:dev` grün; `task check:race` grün oder als übersprungen genannt.
- [ ] Keine offenen schweren Befunde; neue Tickets im Ergebnis genannt.
- [ ] Manuelle Abnahme durch Ralf erledigt oder als offen mit Grund (RD1 fehlt) vermerkt.

## Prüfen

```bash
task check
task check:go
task check:race
task check:dev
```

Nacheinander; manuelle Prüfung im Browser nur nach Freigabe durch 🧑 für diesen Lauf.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
