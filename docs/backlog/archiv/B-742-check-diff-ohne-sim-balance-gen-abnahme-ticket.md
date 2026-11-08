# B-742 · check:diff ohne Sim-, Balance- und Gen-Läufe; Abnahme-Ticket je Sprint/Projekt

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** RST
- **Erstellt:** 2026-11-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Chat (Ralf): Anpassung check:diff ohne sim/balance/gen, Abnahme-Ticket

## Ausgangslage

Lange Läufe bremsen den Entwicklungsflow: Beim Abschluss von Sessions, Sprints und Projekten laufen Sim-, Balance- und Gen-Skripte (gen:paper, Grafik-Golden, Balancing-Tests) mit, auch wenn sie mit der Änderung nichts zu tun haben. Diese Läufe können autonom nachts laufen.

## Ziel

Der normale Workflow (check:diff, Abschluss von Sessions, Sprints, Projekten) ruft keine Sim-, Balance- und Gen-Skripte mehr auf. Sie laufen nur direkt bei der Entwicklung der betroffenen Features. Zum Abschluss eines Sprints oder Projekts gibt es genau ein autonomes Abnahme-Ticket für diese Läufe, das Claude nachts abarbeiten kann.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Taskfile.yml check:diff ohne Schritte gen-paper, grafik-golden, Balance, Sim, check:race. docs/arbeitsweise.md und CLAUDE.md beschreiben die Regel. Sprint-/Session-Vorlagen und Skill k3c-session verweisen auf das Abnahme-Ticket statt auf Sim-Sessions vor der Abnahme. Tests, die gen:paper-Bilder brauchen, laufen weiter in task test/build/dev und im Abnahme-Ticket.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** check:diff enthält keinen gen-paper-, Grafik-Golden-, Balance- oder Sim-Schritt, das Schritt-Protokoll (B-739) bleibt stimmig und check:diff bleibt grün.
- **AC-02** docs/arbeitsweise.md und CLAUDE.md nennen die Regel: Abschluss ohne Sim/Balance/Gen, genau ein Abnahme-Ticket je Sprint/Projekt.
- **AC-03** Sprint-/Session-Vorlagen und Skill k3c-session verweisen auf das Abnahme-Ticket statt auf Sim-Sessions vor der Abnahme.
- **AC-04** Eine Vorlage oder Anleitung für das Abnahme-Ticket existiert und listet die Läufe (sim_test, Balance, gen:paper samt Golden, check:race).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Umgesetzt 2026-11-08 (Claude):

- AC-01: `Taskfile.yml` › `check:diff` ohne Schritte `gen-paper` und `grafik-golden`, Flags `a`, `ga`, `gr` und `GOLDEN_GRUPPEN` entfernt (nichts hing mehr daran); `public/grafik/*` und `tools/paper/*` setzen nur noch `c`. `vitest-changed` schließt die Tests aus, die gerenderte PNGs oder Golden brauchen (`tests/paperAssets`, `tests/buildingTargets`, `tools/paper/golden/**`, `src/scenes/render/*Sprites`, `hubGrowth`). Schritt-Protokoll (⏱-Marker) unverändert. Sim, Balance und `check:race` waren nie Teil von `check:diff`.
- AC-02: `docs/arbeitsweise.md` › „Sim, Balance und Gen (B-742)“, Tempo im Lauf, Review-Session (Schritt 1 ohne `check:race`, Schritt 4 Abnahme-Ticket), Sprint-Lebenslauf; `CLAUDE.md` Regel und Befehle.
- AC-03: Vorlagen `session.md`, `sprint.md`, `projekt.md`, `ticket.md` und Skill `.claude/skills/k3c-session` verweisen auf das Abnahme-Ticket.
- AC-04: Anleitung in `docs/arbeitsweise.md` › Sim, Balance und Gen (Felder, Kriterien AC-01 Sim, AC-02 Balance, AC-03 Gen samt Golden, AC-04 Race), Kurzform in `docs/vorlagen/ticket.md`.
- Geprüft: `task:test` planning und projectRules grün; `task check:diff` grün in 4 min 4 s (Go und Workbench voll wegen `Taskfile.yml`), Vitest-Ausschlüsse im Lauf bestätigt, kein gen:paper-/Golden-Schritt mehr.
