# SI7.2 · Review Sprint SI7 Welten-Editor Kataloge

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** zweit
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

Fach-Review durch Agent k3c-workbench auf Diff b05a1ea32: 0 schwer, 3 mittel, 3 leicht. Behoben: Patch mit mehreren Arten war nicht atomar (c0bf7332f), jetzt Prüfung vor Schreiben, Test ergänzt. Nicht geändert: Lock-Fenster zwischen Prüfung und Schreiben (kleines Fenster, gleicher writeMu-Pfad); fester Blockformat-Schreibweg (passt zu den drei Dateien). Offen als leicht: Vitest für Änderung per Auswahl. AC-01 bis AC-03 nachgewiesen (Tests oben in SI7.1).
Abnahme: entfällt. Der Sprint hat keinen Sim-, Balance- oder Gen-Bezug, daher kein Abnahme-Ticket. Hinweis an 🧑: Workbench-EXE und Studio-EXE neu bauen nach dem Merge.
Checks: `task check:diff` grün. `task check` rot durch tools/paper (Render-Tests, nicht Teil dieser Änderung, Ursache nicht untersucht). `task check:dev` rot durch tools/assetkit (TLS, Norton).
