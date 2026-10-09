# LP1.3 · Review

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** lp1/3-review
- **Abhängig von:** LP1.1, LP1.2
- **Tickets:** B-335, B-292
- **Kriterien:** alle

## Ziel

Sprint LP1 ist nach `docs/arbeitsweise.md` › Review-Session abgenommen, der eine PR des Sprints ist offen.

## Kontext

Diff `origin/develop...origin/sprint/lp1`. AC-04 ist eine Abnahme durch 🧑 (LP1.4) und keine Abhängigkeit des Reviews: als `angenommen, Validierung offen (LP1.4)` führen. Grenzfall `tests/projectRules.test.ts` (INF) laut Sprint-README prüfen: nur die Seiten-Prüfung geändert.

## Erlaubte Dateien

- Dateien aus LP1.1 (nur schwere Befunde)
- Planungsdateien

## Nicht-Ziele

Stil, Benennung, Vereinfachungen; Server.

## Schritte

1. `task check` grün.
2. Diff lesen, nur schwere Befunde nach `docs/arbeitsweise.md` beheben oder als Ticket anlegen.
3. Abnahme in der Sprint-README, Fahrplan (LP1.4 unter „Offen am Gerät“, falls noch offen), Versionsvorschlag; B-292 archivieren.
4. Commit, `git merge origin/develop`, push, PR des Sprints öffnen.

## Fertig, wenn

- [x] Alle Kriterien mit Nachweis oder `angenommen`/`verschoben` in der Abnahme.
- [x] `task check` grün, PR offen.

## Prüfen

```bash
task check
```

## Ergebnis

2026-10-07, Agent (Claude Opus 5.5), Diff `19c176e..sprint/lp1` (PR #200 enthielt LP1.1 bereits, gemergt).

- `task check` grün. Alle Kriterien in der Abnahme der Sprint-README: AC-01, AC-02, AC-03, AC-05 mit Nachweis (LP1.1, LP1.2), AC-04 angenommen, Validierung offen (LP1.4).
- **Schwerer Befund behoben:** `src/tools/dev.ts` wertete nur den Controller aus; LP1.4 verlangt Pfeiltasten. Jetzt wählen ←/↑ und →/↓ wie der Controller (`moveFocus` mit `nextFocus`), Enter öffnet. Im Browser-Pane geprüft (Fokus wandert Testing → Level-Betrachter → Gamepad-Test und zurück, keine Konsolenfehler).
- Geprüft ohne Befund: Seiten-Regeln (`installPageChrome()`, `openPage()`, B unbelegt, View + Menu über die Shell), nur `textContent` für Kacheltexte, Grenzfall `tests/projectRules.test.ts` (nur Seiten-Prüfung), Dateien ≤ 100 Zeilen.
- B-292 archiviert. LP1.4 im Fahrplan unter „Offen am Gerät“. Neues Ticket B-339 (Tastensymbole je Controller-Familie; Frage von 🧑 während des Reviews, kein Befund aus dem Diff).
