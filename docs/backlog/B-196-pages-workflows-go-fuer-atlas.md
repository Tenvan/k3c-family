# B-196 · Die Pages-Workflows bauen mit Go, weil `task build` den Atlas packt

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`task build` ruft seit GR4.1 `task atlas` auf (Go-Werkzeug `tools/atlas`, Ausgabe `public/atlas/`, nicht eingecheckt). Der Job `check` in `.github/workflows/ci.yml` und der Job `build` in `.github/workflows/deploy-pages.yml` führen `task pages` (und damit `build`) aus, ohne vorher `actions/setup-go` zu laufen. Die gehosteten Runner haben zwar ein Go, aber ggf. nicht die Version aus `go.mod` (1.27); `release.yml` führt `task build` ebenfalls vor `setup-go` aus.

## Ziel

Alle Workflows, die `task build` oder `task pages` ausführen, installieren vorher Go in der Version aus `go.mod`.

## Beteiligte und Zielgruppen

Entwickler und Agenten (CI), 🧑 (Release, GitHub Pages).

## Anforderungen

- `ci.yml` (Job `check`), `deploy-pages.yml` und `release.yml` haben vor dem Build einen Schritt `actions/setup-go` mit `go-version-file: go.mod`.

## Nicht-Ziele

Atlas einchecken oder das Werkzeug in TypeScript neu schreiben (B-163).

## Regeln und Einschränkungen

`.github/workflows/` gehörte nicht zu den erlaubten Dateien von GR4.1; deshalb ein eigenes Ticket. Der PR von GR4 wird erst grün, wenn das behoben ist (oder die Runner-Go-Version reicht).

## Beispiele

PR öffnen → Job `check` baut `_site/` inklusive `app/atlas/atlas.json`.

## Ausnahme- und Fehlerfälle

nicht relevant, reine CI-Konfiguration.

## Akzeptanzkriterien

- **AC-01** Die drei Workflows enthalten `setup-go` vor dem ersten `task build`/`task pages`; die CI des PR ist grün.

## Offene Fragen

keine

## Notizen

Aufgefallen in GR4.1. Sinnvoll vor dem PR von GR4 (Review-Session GR4.4) umzusetzen.
