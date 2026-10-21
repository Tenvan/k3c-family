# B-483 · Planungsseite: Einträge nur im Worktree sind nur lesbar (Detail, Menüs, Rang)

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ20
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus dem Review von WZ12.2: Einträge, die nur in einem Worktree existieren (Pill „neu“, `withWorktreeEntries`), sehen aus wie Wurzel-Einträge. Die Detailansicht (`Reader.tsx` › `planningGet`) und die Menüs für Prio, Projekt und Umgebung (in `Backlog.tsx` und in `Detail.tsx`, beide über `setField` und damit `planningSet` gegen die Wurzel) laufen gegen die Wurzel und scheitern. Die Rang-Knöpfe eines synthetischen Projekts sind sichtbar, aber deaktiviert (`rang` ist `–`). Vorschlag: synthetische Einträge markieren (`wt?: branch`), Menüs und Rang-Knöpfe ausblenden, Detail zeigt „nur im Worktree <branch>“. Geprüft am 2026-10-18: `plan_set` lehnt eine unbekannte, gültig geformte ID sauber ab („B-998 nicht gefunden“).

## Ziel

Einträge, die nur in einem Worktree existieren (Pill „neu“), sind auf der Planungsseite als solche erkennbar und nur lesend. Menüs, die gegen die Wurzel laufen und scheitern würden, und die deaktivierten Rang-Knöpfe erscheinen dort nicht; die Detailansicht nennt den Worktree-Branch statt eines Fehlers.

## Beteiligte und Zielgruppen

Ralf (🧑) liest und pflegt die Planung auf der Planungsseite, auch mit Einträgen aus Worktrees, und entscheidet, ob „nur lesend“ das gewollte Verhalten ist. Agenten erzeugen solche Einträge in Worktrees.

## Anforderungen

- Synthetische Einträge (aus `withWorktreeEntries`) tragen eine Markierung `wt?: branch` und lassen sich von Wurzel-Einträgen unterscheiden. Das Feld bekommen alle vier Typen: `PlanProject`, `PlanSprint`, `PlanTicket`, `PlanSession` (`api/types.ts`).
- Für markierte Einträge fehlen die Prio-, Projekt- und Umgebungs-Menüs in der Backlog-Liste (`Backlog.tsx`), die Prio- und Umgebungs-Menüs in `Detail.tsx` › `TicketDetail`, die Agent- und Umgebungs-Menüs in `SessionDetail` sowie die Rang-Knöpfe in `ProjectCard.tsx` (heute sichtbar, aber deaktiviert wegen `rang: –`; neu: ausgeblendet).
- Die Detailansicht (`Reader.tsx`) fragt für markierte Einträge die Wurzel nicht ab und zeigt „nur im Worktree <branch>“.
- Liegt derselbe neue Eintrag in mehreren Worktrees, gilt der erste Branch (wie `ADD` heute).
- Wurzel-Einträge verhalten sich unverändert, die Pill „neu“ bleibt.

## Nicht-Ziele

- Bearbeiten von Worktree-Einträgen über die Planungsseite (Schreiben läuft mit `checkout` über MCP).
- Neue Anzeigen für Worktrees; die Pills aus WZ12.2 bleiben.
- Der Zuordnungsfehler neuer Sprints (B-484).

## Regeln und Einschränkungen

Domäne DEV; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Planung nur über MCP: die Oberfläche darf keine Schreibaktion gegen die Wurzel auslösen, die für einen Worktree-Eintrag gedacht ist. Die Typen `PlanProject`, `PlanSprint`, `PlanTicket` und `PlanSession` bekommen ein optionales Feld `wt`; das Backend bleibt unverändert.

## Beispiele

- Ticket `B-900` existiert nur in `sprint/x` → Zeile mit Pill „neu“, Detail zeigt „nur im Worktree sprint/x“, kein Fehler, keine Prio-/Projekt-/Umgebungs-Menüs, weder in der Liste noch in der Detailansicht.
- Projekt, das nur im Worktree existiert → Rang-Knöpfe sind ausgeblendet (heute sichtbar, aber deaktiviert).
- Ticket `B-100` existiert in der Wurzel und im Worktree → alles wie bisher, Menüs vorhanden.

## Ausnahme- und Fehlerfälle

- Eintrag wird im Worktree erzeugt und später in die Wurzel übernommen → Markierung entfällt beim nächsten Laden, Menüs erscheinen.
- Worktree mit Lesefehler → trägt nichts bei, keine markierten Einträge.
- `plan_set` mit unbekannter, aber gültig geformter ID → das Backend lehnt sauber ab („B-998 nicht gefunden“, am 2026-10-18 geprüft); die Oberfläche soll es trotzdem nicht erst so weit kommen lassen.

## Akzeptanzkriterien

- **AC-01** Vitest in `tools/k3c-dev/frontend/src/planning` (`wtPills`): `withWorktreeEntries` markiert synthetische Ticket-, Sprint-, Projekt- und Session-Einträge mit `wt: <branch>` (bei mehreren Worktrees der erste); Wurzel-Einträge bleiben ohne Marke.
- **AC-02** Komponententest: Ein markiertes Ticket zeigt keine Prio-, Projekt- und Umgebungs-Menüs in der Backlog-Zeile (`Backlog.tsx`) und keine Prio- und Umgebungs-Menüs in `Detail.tsx` › `TicketDetail`; eine markierte Session zeigt keine Agent- und Umgebungs-Menüs in `SessionDetail`. Wurzel-Einträge zeigen sie. Sprints haben in `Detail.tsx` heute keine Menüs.
- **AC-03** Komponententest: Ein markiertes Projekt zeigt keine Rang-Knöpfe.
- **AC-04** Komponententest (`Reader`): Ein markierter Eintrag ruft `planningGet` nicht auf und zeigt „nur im Worktree <branch>“.
- **AC-05** `check_run` `task:check` und `dev:test` grün.

## Offene Fragen

1. Ist „nur lesend mit Hinweis“ das gewollte Verhalten für Worktree-Einträge? Entscheidet Ralf (🧑).
2. Annahme bis zur Antwort: ja; Menüs und Rang-Knöpfe werden ausgeblendet, nicht deaktiviert angezeigt.

## Notizen

Quelle: Review von WZ12.2. Code: `tools/k3c-dev/frontend/src/planning/` (`wtPills.ts` › `withWorktreeEntries`, `Reader.tsx`, `Backlog.tsx`, `Detail.tsx`, `ProjectCard.tsx`). Das Backend lehnt `plan_set` mit einer unbekannten, gültig geformten ID sauber ab (geprüft). Gilt auch für über B-484 in ein Projekt eingehängte Sprints: sie sind synthetisch, markiert und nur lesend. Verwandt: B-467, B-484.
