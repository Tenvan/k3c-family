# BT1.3 · Review

- **Status:** offen
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

- [ ] AC-02 bis AC-04 sind am Diff belegt (Nachweis im Ergebnis; AC-02 Abnahme am PC `angenommen, Validierung offen`).
- [ ] Befunde sind behoben oder als Ticket festgehalten; `task check` und `task check:go` grün.
- [ ] Abnahme gefüllt, PR geöffnet.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
