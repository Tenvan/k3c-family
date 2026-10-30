# BT1.3 · Review

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bt1/3-review
- **Abhängig von:** BT1.1, BT1.2
- **Tickets:** B-041, B-095, B-048
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Ein getrennter Reviewer hat Diff und Kriterien von BT1 geprüft; Befunde sind behoben oder als Ticket festgehalten, der PR ist geöffnet. Die Abnahme am PC (B-041/AC-02) steht als `angenommen, Validierung offen` im Sprint, solange 🧑 sie nicht gemacht hat.

## Kontext

Review nach `docs/arbeitsweise.md` (getrennt vom Schreiben): Diff `origin/develop...sprint/bt1` lesen, AC-02 bis AC-04 an BT1.1 und BT1.2 halten, besonders Protokoll (Rückwärtskompatibilität, Seed-Länge, `bad_request`), Verhalten von `cmd/k3c-server` unverändert, Starter ohne Firewall-Abfrage (feste EXE, Loopback-Probe). Hardware-Abnahme (Doppelklick) ist keine Abhängigkeit; der Sprint bleibt `aktiv`, bis 🧑 sie gemacht hat. Version vorschlagen (Minor, neues Werkzeug und Protokollfeld).

## Erlaubte Dateien

- Befunde: nur kleine Korrekturen in den Dateien von BT1.1 und BT1.2; größere Befunde als Ticket
- Sprint-Dateien von BT1 (Abnahme, Status, Ordner), `docs/sprints/README.md`, `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Neue Funktionen, Ändern von Kriterien, manuelle Abnahme am PC durch den Agenten.

## Schritte

1. `task check`, `task check:go` (und `task check:all` soweit möglich) ausführen.
2. Diff `origin/develop...sprint/bt1` lesen, AC-02 bis AC-04 gegen die Ergebnisse halten.
3. Befunde beheben oder als Ticket anlegen.
4. Abnahme der Sprint-README füllen (höchstens fünf Zeilen, Versionsvorschlag), B-095 und B-048 `erledigt`, B-041 `erledigt` erst nach der Abnahme am PC; PR öffnen.

## Fertig, wenn

- [x] AC-02 bis AC-04 sind am Diff belegt (Nachweis im Ergebnis; AC-02 Abnahme am PC `angenommen, Validierung offen`).
- [x] Befunde sind behoben oder als Ticket festgehalten; `task check:go`, `task test` und `task typecheck` grün, `task check` nur durch den bestehenden Oxlint-Fehler (`src/tools/dm.ts`, auch auf `develop`) rot.
- [x] Abnahme gefüllt, B-095 und B-048 `erledigt`, PR geöffnet.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **Review (getrennter Reviewer, 2026-10-18):** keine schweren Befunde. AC-02: `tools/k3c-starter` samt Tests, Task `k3c-starter:build`, `check:starter` und CI-Job `k3c-starter` am Diff belegt; Job-Objekt-Layout gegen `winnt.h` und Ablauf des sauberen Beendens geprüft. AC-03: `create.seed`, `?seed=`, `?depth=`, Tests und Protokoll-Beispiel belegt. AC-04: Satz in Entscheidung 001. AC-01 entfällt (B-040). Keine Kriterien umformuliert (die Starter-Änderung in B-041 stammt aus der freigegebenen Revision 1).
- **Befund behoben:** Der Client verwarf ungültige Tiefe und zu langen Seed still (Spec: Fehlermeldung des Servers) → `parseStartParams` reicht beides unverändert durch, der Server antwortet `bad_request`; Vitest angepasst.
- **Offen für 🧑:** Abnahme am PC (B-041/AC-02: Doppelklick auf `bin/k3c-starter.exe`, Port belegt → Meldung), bis dahin `angenommen, Validierung offen`; Sprint bleibt `aktiv`, B-041 `eingeplant` (Arbeitsweise › Hardware entkoppelt).
- B-095 und B-048 `erledigt`; neue Tickets: keine.
