# B-520 · check_run wartet bis zum Ende des Laufs und meldet Fortschritt statt Lauf-ID zum Nachfragen

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ26
- **Projekt:** WZG
- **Erstellt:** 2026-10-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`check_run` wartete höchstens 45 s (B-372, 60-s-Frist je Aufruf bei HTTP-MCP) und nannte danach eine Lauf-ID; `check_run {run}` antwortete sofort mit „läuft noch“. Agenten mussten deshalb in Schleifen nachfragen oder Shell-Wächter bauen.

## Ziel

Ein `check_run`-Aufruf liefert das Ergebnis des Laufs. Lange Läufe macht Claude Code ab 2 min selbst zum Hintergrund-Task und meldet das Ende; Polling entfällt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `.mcp.json`: `timeout` 900000 ms für k3c-workbench (hebt die 60-s-Frist je Aufruf).
- `check_run` wartet bis zum Ende und sendet bei progressToken alle 20 s `notifications/progress` (Aktivität gegen die Leerlauf-Frist von 5 min).
- Ohne progressToken wartet es höchstens 4 min, dann Lauf-ID; `check_run {run}` wartet genauso statt sofort zu antworten.
- Anleitung, Tool-Beschreibung, `docs/arbeitsweise.md` und Skill `k3c-session` nennen das Verhalten.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Mit progressToken wartet `check_run` über `checkWait` hinaus bis zum Ende und meldet unterwegs Fortschritt (Go-Test).
- **AC-02** `check_run {run}` wartet auf das Ende des Laufs (Go-Test).
- **AC-03** Ohne progressToken kommt nach `checkWait` „läuft noch“ mit Lauf-ID (Go-Test).
- **AC-04** Live: ein `check_run` über 60 s endet in Claude Code mit dem Ergebnis statt einem Timeout (Messung nach Neustart der MCP-Verbindung, Ergebnis in Notizen).

## Offene Fragen

Ob Claude Code ein progressToken sendet und ob die Desktop-App die 60-s-Frist trotz `timeout` hält, ist ungeprüft; k3c-workbench loggt je Aufruf `⏳ check_run wartet auf Lauf` mit `progress` (true/false).

## Notizen

AC-01 bis AC-03 im Code mit Go-Tests belegt (`check_wait.go`, `check_wait_test.go`). AC-04 (Live-Messung über 60 s in Claude Code) bleibt bei 🧑: angenommen, Validierung offen. Beim Lauf in WZ26 lieferte `task:check:diff` (2 min 51 s) das Ergebnis direkt.
