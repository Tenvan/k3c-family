# B-629 · task buildall baut die Web-EXEs von Studio und Workbench nicht mit

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-25
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (2026-10-25): Nach `task buildall` zeigte das Studio auf der Seite „Quellen“ weiter den Stub „Quellen und Downloads kommen mit WZ29.6“. Ursache: `buildall` (`Taskfile.yml`) baut nur die Fenster-EXEs (`k3c-workbench:build`, `k3c-studio:build`, …). Die Web-EXEs `bin/k3c-studio-web.exe` und `bin/k3c-workbench-web.exe` aus dem Web-Client-first-Weg (B-614) entstehen nur beim Start von `<app>:web` und blieben auf einem alten Stand (24.10., 23:36). Die laufende Web-EXE kannte die RPC-Methode `Sources` nicht, das neue Frontend aus `frontend/dist/` lief also gegen ein altes Backend.

## Ziel

Nach `task buildall` sind alle EXEs in `bin/` auf dem Stand des Checkouts, auch die Web-EXEs.

## Beteiligte und Zielgruppen

Ralf (baut und startet die EXEs), Agent INF.

## Anforderungen

- `buildall` baut zusätzlich `bin/k3c-studio-web` und `bin/k3c-workbench-web` (eigener `web:build`-Task je Projekt, den `<app>:web` wiederverwendet).
- `buildall` meldet am Ende kurz, welche EXE gebaut und welche übersprungen wurde (`sources`/`generates`).

## Nicht-Ziele

Abschaffen der Fenster-EXEs (bleibt laut B-614, bis sich der Web-Weg bewährt hat).

## Regeln und Einschränkungen

Neue Aufgaben in das Taskfile des Projekts (`tools/k3c-studio/Taskfile.yml`, `tools/k3c-workbench/Taskfile.yml`); `go build` mit festem Pfad (keine Firewall-Abfrage).

## Beispiele

Checkout auf neuem Stand, `task buildall` → `bin/k3c-studio-web.exe` hat ein aktuelles Änderungsdatum, das Studio im Browser kennt alle RPC-Methoden des Frontends.

## Ausnahme- und Fehlerfälle

Web-EXE läuft noch → Kopie scheitert mit klarer Meldung wie bei den anderen EXEs.

## Akzeptanzkriterien

- **AC-01** `task buildall` baut `bin/k3c-studio-web` und `bin/k3c-workbench-web` mit.
- **AC-02** `k3c-studio:web` und `k3c-workbench:web` nutzen denselben Build-Task.
- **AC-03** Die Beschreibung von `buildall` (und CLAUDE.md › Befehle) nennt die Web-EXEs.

## Offene Fragen

keine

## Notizen

Gefunden beim Fortsetzen von WZ29; der Stub-Text stammt aus `tools/k3c-studio/frontend/src/api/http.ts` auf `develop` (vor dem Merge von WZ29).
