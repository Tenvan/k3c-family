# B-756 · CI rendert die Papierschnitt-Grafik vor den Go-Tests, die sie lesen

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ41
- **Projekt:** WZG
- **Erstellt:** 2026-11-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Seit B-737 sind PNG und Manifeste der Papierschnitt-Grafik nicht mehr eingecheckt (`task gen:paper` rendert sie). Die CI-Jobs `go`, `go-arm64` und `k3c-workbench` rufen das nicht auf; Go-Tests lesen die Grafik (`cmd/k3c-server` `TestCheckPaperRepoStand`, Workbench `TestAssetsHintergrund`, `TestListeWieLoader`). Die CI auf develop ist deshalb seit da1f890 rot (B-737/AC-03 offen).

## Ziel

Die CI ist wieder grün: Jobs, deren Tests die Grafik lesen, rendern sie vorher.

## Beteiligte und Zielgruppen

Entwickler (🧑), Release.

## Anforderungen

- `go`, `go-arm64` und `k3c-workbench` richten bun ein, installieren die Abhängigkeiten und rufen `task gen:paper` vor den Go-Tests auf.
- Die Zeitgrenze der Jobs steigt auf 30 Minuten (Rendern ca. 4 Minuten).

## Nicht-Ziele

Kein gemeinsamer Render-Job mit Artefakt (zusätzliche Action-Versionen), kein Cache.

## Regeln und Einschränkungen

Tasks nur über `task`; Release-Checkliste (Golden amd64/arm64 grün). Umgesetzt mit #526, Beleg in WZ41.1.

- **Bereich:** `.github/workflows/ci.yml`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

Push auf develop → alle CI-Jobs grün.

## Ausnahme- und Fehlerfälle

Rendern schlägt auf arm64 oder Windows fehl: Befund als Ticket, Job bleibt rot.

## Akzeptanzkriterien

- **AC-01** Die CI auf develop ist nach dem Merge grün (alle Jobs).

## Offene Fragen

keine

## Notizen

Gefunden beim Release v0.22.0. Der Typecheck-Fehler `ebene-angabe` (B-752) war ein eigener Fehler, behoben in #525.
