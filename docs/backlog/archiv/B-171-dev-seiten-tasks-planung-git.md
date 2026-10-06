# B-171 · k3c-dev zeigt Tasks, Planung und Git wie die Workbench der ErpApi

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** M7
- **Erstellt:** 2026-10-03
- **Spec:** rückwirkend
- **Revision:** 2
- **Freigabe:** – (rückwirkend: aus der Umsetzung vom 2026-10-03 abgeleitet, ohne Freigabe)

## Ausgangslage

`tools/k3c-dev` hat die Seiten Dienste, Logs und MCP (M4, M5). Die Dev-Workbench der ErpApi (`../ErpApi/tools/go/dev-workbench`) kennt zusätzlich Tasks, Planung und Git. Ihr Code hängt an ErpApi-Eigenheiten (Planungsformat, Towncrier, Confluence, Allow-Liste für Agenten) und am gemeinsamen Paket `@tools/wails-ui`; er lässt sich nicht kopieren, nur nachbauen.

## Ziel

Entwickler sehen und bedienen in k3c-dev die Tasks des `Taskfile.yml`, die Planung des Repos (Sprints, Backlog, Plan, Fragenkatalog) und den Git-Stand samt Commit, ohne Shell und ohne Datei-Suche. Dazu nutzt die Oberfläche das Standard-Theme von Radix in Dark und Light.

## Beteiligte und Zielgruppen

Entwickler und 🧑, die k3c-dev im Fenster nutzen; Coding-Agenten bleiben bei den MCP-Tools (nicht Teil dieses Tickets).

## Anforderungen

- Tasks: Katalog aus `task --list-all --json --no-status` nach Namensräumen mit Filter; Start und Stopp je Task mit Zusatzargumenten; Ausgabe live als Konsolen-Quelle `task:<name>`; höchstens ein Lauf je Task; beim Beenden von k3c-dev enden laufende Tasks.
- Planung: aktive und geplante Sprints mit Session-Fortschritt, offene Tickets nach Domäne mit Filter; ein Umschalter zeigt zusätzlich `docs/plan-weiterentwicklung.md` und `docs/fragenkatalog.md` als Markdown (inklusive Tabellen). Gelesen wird frisch von der Platte, nur aus `docs/`. Ein Wächter in k3c-dev (Polling alle 2 s, unabhängig von der offenen Seite) meldet Änderungen an Sprints, Tickets, Plan und Fragenkatalog; die Seite lädt dann selbst neu.
- Git: Stand von Index und Arbeitsbaum, Stagen und Unstagen, Commit mit Typ, Domäne, Betreff (≤ 72 Zeichen) und Rumpf im Format `typ(domäne): Betreff`; letzte Commits. Verändert werden nur Index und Historie.
- Alle drei Seiten laufen ohne Wails gegen das Mock-Backend (`npx vite` im Frontend).
- Theme: Radix-Standard, Hell/Dunkel über `appearance`, Vorgabe nach Systemeinstellung.

## Nicht-Ziele

Historie, Pins und Allow-Liste für Tasks sowie die `task_*`-MCP-Tools; KI-Vorschlag für die Commit-Nachricht, Towncrier-Fragment, Pending-Queue, Release-Bereich und das MCP-Tool `git_commit`; Radar-Wächter über Worktrees und Änderungs-Historie der Planung; Push, Discard und Stash. Bei Bedarf als eigene Tickets.

## Regeln und Einschränkungen

Domäne SRV: Änderungen nur in `tools/k3c-dev/`. Datei ≤ 400 Zeilen, Funktion ≤ 60 (`golangci-lint`, Oxlint). Argumente für Tasks werden abgelehnt statt bereinigt (Shell-Zeichen). Pfade beim Staging nur relativ zum Repo, NUL-getrennt über stdin. Planungs-Dokumente nur aus einer festen Liste, nie über einen Pfad der Oberfläche. Keine neue Abhängigkeit.

## Beispiele

`test` wählen und starten → Zeile zeigt „läuft“, die Ausgabe erscheint live, am Ende „erfolgreich“ mit Dauer. Planung → „Fragenkatalog“ → die Tabelle der offenen Fragen. Git: Datei stagen, Betreff `Tasks-Seite`, Typ `feat`, Domäne `srv` → Vorschau `feat(srv): Tasks-Seite`, Commit legt den Hash an.

## Ausnahme- und Fehlerfälle

`task` nicht im PATH oder Taskfile kaputt → Karte mit Kommando und Ursache, ein alter Baum bleibt sichtbar. Argument mit `;` → abgelehnt. Leerer Index → „nichts gestaged“, Knopf gesperrt. Commit-Hook scheitert → Fehlertext aus Git an der Oberfläche. Falsche Repo-Wurzel → Planung meldet den Fehler statt einer leeren Liste.

## Akzeptanzkriterien

- **AC-01** Die Seite Tasks zeigt den Katalog, startet und stoppt Tasks und zeigt die Ausgabe live; Go-Tests für Katalog, Lauf (Erfolg, Fehler) und Argumentprüfung sind grün.
- **AC-02** Die Seite Planung zeigt Sprints und Tickets des Repos und per Umschalter Plan und Fragenkatalog; ein Go-Test liest die echten `docs/` des Repos ohne Format-Ausfall; ein Go-Test belegt, dass der Wächter eine Änderung meldet, den Ausgangsstand, das Archiv und fremde Dateien aber nicht.
- **AC-03** Die Seite Git stagt, unstagt und committet im Repo-Format; ein Go-Test geht den ganzen Weg in einem echten temporären Repo (inklusive Zustand vor dem ersten Commit).
- **AC-04** Das Theme ist das Radix-Standard-Theme in Dark und Light; es gibt keine eigene Farbpalette mehr.
- **AC-05** Alle drei Seiten laufen im Mock ohne Wails (Browser-Pane).
- **AC-06** `task check:dev` ist grün.
- **AC-07** 🧑 hat die drei Seiten im echten Fenster (`task k3c-dev`) abgenommen.

## Offene Fragen

keine

## Notizen

Vorlage: `../ErpApi/tools/go/dev-workbench` (Tasks: `internal/taskcat`, `taskrun`; Git: `internal/gitcommit`; Planung: `internal/radar`, dort ErpApi-Format). Umsetzung am 2026-10-03 im Branch `claude/dev-workbench-migration-07abcf`.
