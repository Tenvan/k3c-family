# B-628 · TestSimTestMitCheckout scheitert auf dem Windows-Runner an 8.3-Kurzpfaden

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-25
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der CI-Job „k3c-workbench · Frontend · Tests · Lint · Wails (Windows)“ ist auf `develop` seit mindestens drei Push-Läufen rot (u. a. 819c53d3, Lauf 40135355697) und ebenso im manuellen Lauf auf `sprint/wz29` (40169301354). Ursache: `TestSimTestMitCheckout` (`tools/k3c-workbench/internal/mcpsrv/simtest_checkout_test.go:44/50`) vergleicht den Langpfad `C:\Users\runneradmin\…` mit dem 8.3-Kurzpfad `C:\Users\RUNNER~1\…` aus `t.TempDir()`; `samePath` gleicht nur Groß-/Kleinschreibung bzw. Trenner ab, keine Kurznamen.

## Ziel

Der Job ist auf dem Windows-Runner wieder grün, ohne die Prüfung (Build im Worktree, `K3C_DATA_DIR` des Worktrees) aufzuweichen.

## Beteiligte und Zielgruppen

Agent DEV; 🧑 Ralf entscheidet über die Einplanung.

## Anforderungen

Pfadvergleich im Test (oder in `samePath`) normalisiert Kurz- und Langnamen, z. B. über `filepath.EvalSymlinks` beider Seiten oder `windows.GetLongPathName`.

## Nicht-Ziele

Umbau der CI, damit PRs die vollen Jobs fahren (eigenes Ticket, falls gewünscht).

## Regeln und Einschränkungen

Domäne DEV, nur Testcode bzw. `samePath` in `tools/k3c-workbench/internal/mcpsrv`.

## Beispiele

`t.TempDir()` liefert `C:\Users\RUNNER~1\…`, der Build meldet `C:\Users\runneradmin\…` → Vergleich gilt als gleich.

## Ausnahme- und Fehlerfälle

Pfad existiert nicht (noch nicht angelegt) → Vergleich fällt auf den bisherigen Stringvergleich zurück.

## Akzeptanzkriterien

- **AC-01** `TestSimTestMitCheckout` ist im CI-Job auf `windows-latest` grün.
- **AC-02** Die Erwartung (Build-Ordner und `K3C_DATA_DIR` im Worktree) bleibt unverändert streng.

## Offene Fragen

keine

## Notizen

Gefunden 2026-10-25 beim vollen CI-Lauf für WZ29 (`workflow_dispatch`). Bei PRs laufen die vollen Jobs nicht (`if: github.event_name != 'pull_request'`), deshalb fiel es erst nach dem Merge auf `develop` auf.

Umgesetzt 2026-10-25: `canon` (`tools/k3c-workbench/internal/mcpsrv/workspace.go`) löst bei einem (noch) fehlenden Pfad den längsten vorhandenen Anfang auf und hängt den Rest an; vorher fiel es auf `filepath.Clean` zurück und ließ `RUNNER~1` stehen. Nachweis: `TestSamePathKurznameOhneVorhandenesEnde` (echter Kurzname über `GetShortPathName`, rot mit der alten Fassung), `TestSimTestMitCheckout` grün.
