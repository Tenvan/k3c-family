# WZ26 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** CI rot beheben, Prüf- und Planungswerkzeuge härten
- **Projekt:** WZG

## Ziel

develop ist wieder grün und die Planungs- und Prüfwerkzeuge liefern verlässliche Antworten. Am Ende sichtbar: grüne CI, `check_run` mit Lauf-ID.

## Ergebnis

- 2026-10-23, Claude autonom: AC-01 bis AC-03 erfüllt, `task check:diff` grün. CI-Rot auf develop nachgezogen: Sound-Test (Archiv-Tickets), `.dockerignore` (pipeline-liste.json), `TestSimTestMitCheckout` (Kurzpfade), `TestDownloadHit` (ohne Umgebungs-Proxy). - AC-04 angenommen, Validierung offen: Fensterposition des Studios am PC (B-496) und Live-Messung `check_run` (B-520) liegen bei 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | `task lint` und die CI auf develop sind grün (B-465/AC, B-473/AC). |
| AC-02 | erfüllt | `check:all` scheitert nicht mehr am TLS-Zertifikat (B-474/AC). |
| AC-03 | erfüllt | `plan_next` beachtet Sprint-Branches auf origin, `check_run` liefert eine Lauf-ID (B-519/A … |
| AC-04 | – | Das Studio stellt Fensterposition und Größe wieder her (B-496/AC). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-465 | erledigt | AC-01 erfüllt | task lint ist auf develop rot, weil SprintsBacklog 61 Zeilen hat |
| B-473 | erledigt | AC-01 erfüllt | CI auf develop rot: k3c-workbench internal/planning Worktree-Tests scheitern auf … |
| B-474 | erledigt | AC-02 erfüllt | TestDownloadHit scheitert lokal am TLS-Zertifikat (Norton), check:all rot |
| B-519 | erledigt | AC-03 erfüllt | plan_next berücksichtigt den Stand der Sprint-Branches auf origin |
| B-520 | erledigt | AC-03 erfüllt | check_run wartet bis zum Ende des Laufs und meldet Fortschritt statt Lauf-ID zum … |
| B-496 | erledigt | AC-04 – | Das Asset-Studio merkt sich Fensterposition und Größe und stellt sie sicher wied … |
