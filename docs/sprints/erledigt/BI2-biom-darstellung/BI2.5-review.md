# BI2.5 · Sprint-Review BI2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/bi2
- **Abhängig von:** BI2.4
- **Tickets:** B-900, B-901, B-902, B-903, B-904, B-905, B-906
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07

## Ziel

Sprint-Review: Reviewer-Agent (k3c-reviewer, Sonnet) über den Sprint-Diff, schwere Befunde beheben, Abnahme-Ticket (Sichtprüfung + Gen) anlegen, Sprint abschließen.

## Kontext

Sessions BI2.1–BI2.4.

## Erlaubte Dateien

- Dateien der Sessions BI2.1–BI2.4 (nur Fixes aus dem Review)

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Review über `origin/main..HEAD`.
2. Schwere Befunde beheben.
3. Abnahme-Ticket mit Projekt NAT und Sprint BI2 (Sichtprüfung, `gen:paper`, Grafik-Golden, `check:race`).
4. Abnahme in der Sprint-README, `check:diff`, Commit.

## Fertig, wenn

Keine schweren Befunde offen, Abnahme-Ticket angelegt, `check:diff` grün.

## Prüfen

`check_run task:check:diff`.

## Ergebnis

Review über `6802a69d9..HEAD` (k3c-reviewer, Sonnet): keine schweren oder mittleren Befunde. Leicht: `viewTop` in `props/decor.go` dupliziert `game.ViewUnits` (bewusst, kein Import von game in props); Reihenfolge in `golden.json` vom Werkzeug. Abnahme-Ticket B-910 angelegt (Gen, Golden, `check:race`, Sichtprüfung durch Ralf).
