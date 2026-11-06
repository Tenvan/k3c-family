# MC1.5 · Review MC1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** mcp
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mc1/5-review
- **Abhängig von:** MC1.4
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Sprint MC1 ist geprüft: alle Kriterien AC-01 bis AC-06 mit Nachweis, Befunde behoben oder als Ticket im Projekt PAP, Abnahme in der Sprint-README eingetragen.

## Kontext

- Spec: Sprint-README `docs/sprints/*/MC1-asset-mcp-tools/README.md`, Vertrag `docs/standards/paper-mcp.md`, Beispiele `tools/paper/fixtures/mcp/`, Entscheidungen B-701 und die Zeilen D-MC1-nn der Sprint-README.
- Code: `tools/k3c-workbench/internal/paperassets/`, `internal/mcpsrv/tools_assets*.go` (ohne `tools_assets_research*`), `tools.go`, `checkout.go`, `workspace.go`, `instructions.md`.
- Live-Prüfung (Tool in Claude Code aufrufen) geht erst nach Merge und Neustart der Workbench aus der Repo-Wurzel (B-701 D-10); diese Session prüft in-process. Die sichtbare Prüfung „`assets_preview {id: monarch}` zeigt ein Bild, `asset-material-aendern` erscheint als Prompt“ macht 🧑 nach dem Merge.
- Ist RV1.2 vor dieser Session gemergt, zeigt `newAssetReview` (`tools_assets_review.go`) noch auf `assetsui.Stubs`: umstellen auf die echte Implementierung (eine Zeile) oder Ticket anlegen.

## Erlaubte Dateien

- Alle erlaubten Dateien von MC1.1–MC1.4 (nur Befund-Fixes unter etwa 30 Zeilen; größere Befunde → Ticket)
- `docs/sprints/*/MC1-asset-mcp-tools/` (Abnahme, Ergebnisse)
- `docs/backlog/` (neue Tickets nur über `plan_create`)

## Nicht-Ziele

- Keine neuen Funktionen; keine Änderung am Vertrag (`docs/standards/paper-*.md`, `tools/paper/`).

## Schritte

1. Diff des Sprint-Branches gegen `origin/develop` lesen; je Kriterium AC-01 bis AC-06 den Test bzw. Befehl aus den Session-Ergebnissen nachvollziehen.
2. AC-01 querprüfen: Liste der Tools im Vertrag (› 2) gegen `ListTools` und gegen die Tests – jedes Tool hat mindestens einen In-Process-Test, die Zeilen entsprechen `tools/paper/fixtures/mcp/*.txt` (alle 17 Dateien durchgehen).
3. Gegenprobe Atomarität (AC-02) von Hand im Test-Repo: Render-Fehler (Exit 2) und ungültiger Patch lassen Spec und Ausgabe unverändert, `.cache/paper/tmp/` bleibt leer.
4. Regeln prüfen: jedes `assets_*` hat `checkout` im Schema, schreibende enden mit `Checkout:`; Beschreibungen „Nutze es bei: … Statt: …“; Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen; Log-Meldungen mit Emoji; kein `t.Skip`, kein TODO.
5. Reviewer-Agent (Opus, getrennt vom Schreiben) über den ganzen Sprint-Diff; schwere Befunde beheben, Rest als Ticket (Projekt PAP).
6. `task check:dev`, dann `task check:diff`; `task check:race` nur, falls `paperassets` Goroutinen hat (Jobs) – ohne gcc läuft er lokal still durch, dann in der Abnahme als „übersprungen“ nennen.
7. Abnahme in der Sprint-README (höchstens fünf Zeilen), Status der D-MC1-Zeilen auf `bestätigt` oder `geändert` (mit Grund).

## Fertig, wenn

- [ ] AC-01 bis AC-06: je Kriterium ein Nachweis (Testname bzw. Befehl mit Ergebnis) im Ergebnis dieser Session.
- [ ] `task check:dev` und `task check:diff` grün.
- [ ] Abnahme der Sprint-README ausgefüllt; offene Befunde als Ticket.

## Prüfen

```bash
task check:dev
task check:diff
task check:race
```

Nacheinander. Manuelle Live-Prüfung nur durch 🧑 nach dem Merge.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
