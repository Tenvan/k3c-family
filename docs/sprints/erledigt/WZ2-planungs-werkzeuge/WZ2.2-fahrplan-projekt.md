# WZ2.2 · Fahrplan: Spalte Projekt und Einsortieren nach Rang

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** sprint/wz2
- **Abhängig von:** WZ2.1
- **Tickets:** B-366
- **Kriterien:** AC-03

## Ziel

k3c-dev schreibt in der Fahrplan-Zeile eines Sprints das Projekt und setzt die Zeile nach Projekt-Rang und Sprint-Tabelle des Projekts ein.

## Kontext

- Spec: `docs/backlog/B-366-fahrplan-spalte-projekt.md` (AC-01, AC-02).
- `syncRoadmap` und `placeRow` in `tools/k3c-dev/internal/planning/tables.go` kennen nur `Sprint`, `Domäne`, `Prio`, `Reife`, `Ordner`; neue Zeilen bekommen `Projekt: –` und landen am Tabellenende.
- `docs/sprints/README.md` gliedert nach Projekt-Rang mit Spalte `Projekt`; Rang steht in `docs/projekte/README.md`, die Reihenfolge der Sprints in der Sprint-Tabelle des Projekts (`projects.go`, `projects_tables.go`).
- Der Marker `**Einschiebbar**` zeigt noch auf dieselbe Tabelle und muss weiter treffen (entfällt mit B-361).
- ABN und ruhende Projekte stehen zuletzt. Sprint ohne Projekt oder Projekt nicht in der Übersicht: Zeile ans Tabellenende, `plan_create` meldet den Grund.

## Erlaubte Dateien

- `tools/k3c-dev/internal/planning/tables.go`, `projects.go`, `projects_tables.go` und zugehörige `*_test.go`
- Testdaten (Kopie von `docs/`) im Test-Ordner

## Nicht-Ziele

Felder `Prio` und `Einschiebbar` entfernen (B-361); Planungsseite umbauen; Tabellen-Überschriften im Fahrplan ändern.

## Schritte

1. Test zuerst (rot): `plan_create` eines Sprints mit Projekt GRA schreibt die Zeile mit `Projekt` an die Stelle nach Rang (B-366/AC-01).
2. `syncRoadmap` füllt die Spalte `Projekt` aus dem Sprint-Feld.
3. Einsortieren nach Projekt-Rang und Sprint-Tabelle des Projekts; ABN und ruhende Projekte zuletzt.
4. Fallback ans Tabellenende mit Grund in der Antwort.
5. Test: `plan_set` mit `Status: aktiv` verschiebt die Zeile in „Aktiv“ an die Rang-Stelle (B-366/AC-02).
6. `task check:dev` ausführen, Ergebnis eintragen.

## Fertig, wenn

- [ ] AC-03: Go-Tests für `plan_create` und `plan_set Status: aktiv` grün; Zeile trägt Projekt und steht nach Rang (`B-366/AC-01`, `AC-02`).
- [ ] `task test -- planning` und `task check:dev` grün; keine Datei > 400 Zeilen.

## Prüfen

```bash
task check:dev
```

Bei laufendem k3c-dev stattdessen `check_run` mit `task:check:dev`. Manuelle Prüfungen: keine.

## Ergebnis

- AC-03 umgesetzt, geprüft: `TestFahrplanZeileNachProjektRang` (B-366/AC-01), `TestFahrplanAktivNachRang` (B-366/AC-02) und `TestFahrplanOhneProjektAmEnde` grün; ohne die Änderung an `tables.go` rot. `task check:dev` grün (go test, golangci-lint 0 Issues).
- Neu: `tools/k3c-dev/internal/planning/roadmap_order.go` (Schlüssel Projekt-Rang, dann Platz in der Sprint-Tabelle des Projekts); `syncRoadmap` füllt die Spalte `Projekt` und setzt die Zeile nach Rang ein, wenn sie neu ist, die Tabelle wechselt oder das Projekt wechselt; Sprint ohne bekanntes Projekt steht am Tabellenende, die Antwort nennt den Grund. Tabellen ohne Spalte `Projekt` und unveränderte Zeilen behalten ihr Verhalten.
- Abweichungen vom Plan: keine. Neue Tickets: keine. Manuell geprüft: nichts.
