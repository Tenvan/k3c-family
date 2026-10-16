# B-448 · dev:test TestRepo scheitert an Entwurfs-Sprints ohne Sessions (GEB7)

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`tools/k3c-dev/internal/planning/planning_test.go` › `TestRepo` verlangt für jeden Sprint außer `erledigt` mindestens eine Session. Der geplante Entwurfs-Sprint GEB7 (`docs/sprints/geplant/GEB7-hintergruende/README.md`, Reife Entwurf, seit #302) hat keine Session-Tabelle; `check_run dev:test` scheitert dadurch auf `develop`, auch ohne eigene Änderung (aufgefallen in GEB5.4). Entwürfe dürfen laut `arbeitsweise.md` › Sprint-Lebenslauf Stichpunkte statt Sessions haben.

## Ziel

`dev:test` ist auf `develop` grün, auch wenn ein Sprint mit `Reife: Entwurf` noch keine Session-Dateien hat.

## Beteiligte und Zielgruppen

Entwickler (🧑 und Agenten), die `check_run dev:test` vor einem Abschluss ausführen.

## Anforderungen

- `TestRepo` prüft Sessions nur für Sprints mit `Reife: bereit` (oder für aktive und erledigte Sprints), nicht für Entwürfe.
- Der Test fängt weiterhin fehlende Titel und ein aus dem Parser fallendes Format.

## Nicht-Ziele

Keine Änderung an GEB7 oder an anderen Sprints; keine Änderung am Planungs-Parser.

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-dev/`); `arbeitsweise.md` › Sprint-Lebenslauf (Reife `Entwurf` erlaubt Stichpunkte statt Session-Dateien).

## Beispiele

Geplanter Sprint mit `Reife: Entwurf` und leerer Session-Tabelle → `TestRepo` grün. Aktiver Sprint ohne Sessions → weiter rot.

## Ausnahme- und Fehlerfälle

Sprint ohne Titel → weiter rot, auch als Entwurf.

## Akzeptanzkriterien

- **AC-01** `check_run dev:test` mit `pattern: TestRepo` ist grün, solange nur Entwurfs-Sprints (`Reife: Entwurf`) ohne Sessions in `docs/sprints/geplant/` liegen.

## Offene Fragen

keine

## Notizen

Erledigt ohne Codeänderung: GEB7 hat mit #315 Sessions bekommen (Sprint GEB7 geplant, Session-Tabelle gefüllt), `TestRepo` scheitert nicht mehr. Beleg: `check_run dev:test` auf `origin/develop` (b9a22a82) grün, 2026-10-16, vor dem Tag v0.18.0. Die Anforderung (Entwürfe ohne Sessions dürfen `TestRepo` nicht rot machen) bleibt ungelöst: ein künftiger Entwurfs-Sprint ohne Sessions würde den Test wieder auslösen. AC-01 ist für den heutigen Stand erfüllt.
