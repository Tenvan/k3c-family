# B-741 · Verweise auf bin/k3c-balance und internal/enginetools außerhalb der Workbench nachziehen

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** RST
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

RS2.2 (B-726) hat `tools/k3c-workbench/internal/balance/cmd` (bin/k3c-balance) und `internal/enginetools` entfernt; gerechnet wird in `bin/k3c-sim` (`cmd/k3c-sim`, `engine/balance`). Außerhalb der erlaubten Dateien der Session verweisen noch Texte auf den alten Weg: `codemap.md` (Zeilen zu `internal/enginetools`, `internal/balance/cmd`), `data/codemap.md`, `docs/rules/zielkorridore.md` (`./bin/k3c-balance.exe …`), `docs/data/balance-*.md`, `docs/glossar.md` (Quelle `insights_plain.go` liegt jetzt in beiden Ordnern), `.github/workflows/ci.yml` (Artefakt-Name `k3c-balance`, Inhalt stimmt). Der Namensraum `balance` liegt weiter unter `tools/k3c-workbench/internal/balance/Taskfile.yml`, gehört inhaltlich aber zu `cmd/k3c-sim`.

## Ziel

Doku und Pfade nennen den aktuellen Weg (`bin/k3c-sim balance`, `engine/balance`); kein Leser sucht ein entferntes Paket.

## Beteiligte und Zielgruppen

Entwickler und Agenten (Claude), Ralf.

## Anforderungen

- Verweise auf `bin/k3c-balance`, `internal/balance/cmd`, `internal/enginetools` ersetzen oder als historisch markieren.
- Optional: Namensraum `balance` nach `cmd/k3c-sim/` oder `engine/balance/` verschieben (Wurzel-`Taskfile.yml`).

## Nicht-Ziele

Verhaltensänderung am Balancing.

## Regeln und Einschränkungen

Glossar zuerst; Prozess nur in `docs/arbeitsweise.md`.

## Beispiele

`git grep -n 'k3c-balance\|internal/enginetools'` findet nur noch erledigte Sprints und das Archiv.

## Ausnahme- und Fehlerfälle

nicht relevant: Doku-Pflege.

## Akzeptanzkriterien

- **AC-01** `git grep -n 'bin/k3c-balance\|internal/enginetools\|internal/balance/cmd' -- ':!docs/sprints/erledigt' ':!docs/backlog/archiv'` liefert nur begründete Treffer.

## Offene Fragen

keine

## Notizen

Gefunden in RS2.2.
