# SI7.2 · Review Sprint SI7 Welten-Editor Kataloge

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si7
- **Abhängig von:** SI7.1
- **Tickets:** B-1003
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Der Sprint SI7 ist gegen seine Kriterien geprüft, schwere Befunde sind behoben.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Kriterien in der Sprint-README SI7 (AC-01 bis AC-03), Spec `B-1003`, Ergebnis der Session SI7.1. Der Sprint hat keinen Sim-, Balance- oder Gen-Bezug; ein Abnahme-Ticket entfällt und wird im Ergebnis so festgehalten. Der Neubau der Workbench-EXE (`task buildall`) bleibt bei 🧑.

## Erlaubte Dateien

- `tools/k3c-workbench/**`

## Nicht-Ziele

Neue Funktionen, Neubau oder Neustart der laufenden Workbench.

## Schritte

1. Diff des Sprint-Branches gegen `main` lesen; je Kriterium den Nachweis prüfen.
2. Fach-Review durch Agent `k3c-workbench`.
3. Schwere Befunde beheben, Rest als Tickets mit Projekt und Sprint.
4. `check_run task:check`, dann `check_run task:check:dev`.
5. Abschnitt Abnahme der Sprint-README füllen; Hinweis an 🧑: EXE neu bauen.

## Fertig, wenn

- [ ] AC-01, AC-02: Nachweise geprüft, Abweichungen benannt.
- [ ] AC-03: `task:check` und `task:check:dev` grün, Abnahme-Abschnitt gefüllt, Entfall des Abnahme-Tickets vermerkt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
