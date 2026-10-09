# B-366 · k3c-dev füllt im Fahrplan die Spalte Projekt und ordnet nach Rang

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit PJ3.3 gliedert `docs/sprints/README.md` die Sprints nach Projekt-Rang mit Spalte `Projekt` statt `Prio`; aktive und geplante (auch einschiebbare) stehen je in einer Tabelle. `syncRoadmap` in `tools/k3c-dev/internal/planning/tables.go` kennt nur `Sprint`, `Domäne`, `Prio`, `Reife`, `Ordner`: Eine neue Zeile bekommt `Projekt: –` und landet am Tabellenende statt an ihrem Platz im Projekt. Einschiebbare Sprints findet das Tool über die Marker-Zeile `**Einschiebbar**`, die jetzt nur noch auf dieselbe Tabelle zeigt.

## Ziel

k3c-dev schreibt in der Fahrplan-Zeile das Projekt des Sprints und setzt die Zeile nach Projekt-Rang und Sprint-Tabelle des Projekts ein, ohne Handarbeit.

## Beteiligte und Zielgruppen

Agenten und 🧑 beim Planen über die `plan_*`-Tools.

## Anforderungen

- Spalte `Projekt` aus dem Feld `Projekt` des Sprints füllen.
- Neue oder verschobene Zeile nach Projekt-Rang (`docs/projekte/README.md`) und Platz in der Sprint-Tabelle des Projekts einsortieren; ABN und ruhende Projekte zuletzt.
- Marker `**Einschiebbar**` entfällt mit B-361; bis dahin muss er weiter dieselbe Tabelle treffen.

## Nicht-Ziele

Felder `Prio`/`Einschiebbar` entfernen (B-361); Planungsseite der Workbench umbauen.

## Regeln und Einschränkungen

Domäne SRV (bzw. DEV nach DV1), nur `tools/k3c-dev/`. Tabellen-Überschriften im Fahrplan bleiben.

## Beispiele

Neuer Sprint `GR8` im Projekt GRA (Rang 4) nach GR7, M10 → Zeile `| GR8 | CLI | GRA | … |` direkt nach M10, vor SO2.

## Ausnahme- und Fehlerfälle

Sprint ohne Projekt oder Projekt nicht in der Übersicht: Zeile ans Tabellenende, `plan_create` meldet den Grund.

## Akzeptanzkriterien

- **AC-01** Go-Test in `tools/k3c-dev/internal/planning`: `plan_create` eines Sprints mit Projekt schreibt die Fahrplan-Zeile mit diesem Projekt an der Stelle nach Rang und Projekt-Reihenfolge.
- **AC-02** `plan_set` mit `Status: aktiv` verschiebt die Zeile in „Aktiv“ an die Rang-Stelle; `task test -- planning` bleibt grün.

## Offene Fragen

Keine.

## Notizen

Aus PJ3.3 (B-359). Andere Abschnitte nicht relevant bzw. offen bis zum Bereitmachen.
