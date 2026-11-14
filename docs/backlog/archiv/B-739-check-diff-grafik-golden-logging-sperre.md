# B-739 · check:diff: Grafik-Golden je Gruppe, Schritt-Logging und Sperre gegen parallele Läufe

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ41
- **Projekt:** WZG
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Chat (Ralf): "Ja, umsetzen", 2026-11-07

## Ausgangslage

Messung 2026-11-07 (warm, ohne parallele Last): Der Grafik-Test `tools/paper/golden/public.test.ts` braucht allein 189 s und läuft bei jeder Änderung unter `public/grafik/` (Flag `a`), obwohl `gen:paper` vorher schon rendert. Workbench-Go `-short` 90 s, Root-Go 18 s, Rest unter 10 s. Unter Last (mehrere Sessions) doppelt so lang, dazu Exit 201 durch Zeitlimits und rote Tests (B-738). `check_run` protokolliert nur eine Zeile je Lauf, Schritte sind nicht sichtbar.

## Ziel

`check:diff` bleibt bei Grafik-Änderungen im Minutenbereich, jeder Schritt ist im MCP-Log mit Dauer sichtbar, und gleichzeitige schwere Läufe verschiedener Sessions bremsen sich nicht gegenseitig aus.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Grafik-Golden: `GOLDEN_GRUPPEN` beschränkt `public.test.ts` auf Gruppen; `check:diff` setzt die Gruppen des Diffs, bei `stil.paper.json`, Dateien direkt unter `public/grafik/` und `tools/paper/**` alle.
- `check:diff` meldet Schritte als `⏱▶ name` und `⏱■ name exit=n`; `check_run` schreibt je Schritt einen Log-Eintrag (`ns=check`, `step`, `ms`, `exit`).
- `check_run` lässt höchstens einen schweren Lauf (`task:check:diff`, `task:check`) gleichzeitig zu, über Worktrees hinweg; Warten wird geloggt, das Zeitlimit beginnt nach dem Slot.
- Die Workbench-Go-Tests aus B-738 sind grün.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Domäne INF; Tasks nur über `task`. Umgesetzt mit #517, Beleg in WZ41.1.

- **Bereich:** `Taskfile.yml`, `tools/checkdiff/**`, `tools/k3c-workbench/internal/mcpsrv/check_steps*.go`, `tools/paper/golden/public.test.ts`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Bei einer Änderung in einer Grafik-Gruppe läuft der Golden-Vergleich nur für diese Gruppe (gemessen etwa 11 s statt 189 s); ohne `GOLDEN_GRUPPEN` prüft er alle Gruppen.
- **AC-02** `logs_query service=k3c-workbench pattern=schritt` zeigt je Schritt Start, Ende, `ms` und `exit`.
- **AC-03** Zwei gleichzeitige `check_run task:check:diff` aus verschiedenen Checkouts laufen nacheinander; der zweite loggt `wartet auf freien Prüf-Slot`.
- **AC-04** `go test -short` der Workbench ist grün (B-738).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
