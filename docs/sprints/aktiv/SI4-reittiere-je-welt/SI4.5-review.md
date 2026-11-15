# SI4.5 · Review Sprint SI4 Reittiere je Welt

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si4
- **Abhängig von:** SI4.4
- **Tickets:** B-970, B-1000
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Der Sprint SI4 ist gegen seine Kriterien geprüft, schwere Befunde sind behoben, das Abnahme-Ticket ist angelegt.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Kriterien in der Sprint-README SI4 (AC-01 bis AC-06), Specs `B-970` und `B-1000`, Ergebnisse der Sessions SI4.1 bis SI4.4. Die Abnahme am TV (AC-05) macht 🧑. Sim, Balance, Gen und Race laufen im Abnahme-Ticket (B-742; Sim und Balancing sind gesperrt).

## Erlaubte Dateien

- `docs/rules/monarch.md`
- `docs/rules/welten.md`
- `engine/sim/**`
- `engine/room/**`
- `data/monarch.json`
- `data/paper.json`
- `native/k3c-game/**`

## Nicht-Ziele

Neue Funktionen, Simulationsläufe, Balancing, Grafik-Golden.

## Schritte

1. Diff des Sprint-Branches gegen `main` lesen; je Kriterium den Nachweis prüfen.
2. Fach-Reviews: `k3c-reviewer` (Engine, Spielstand), `k3c-grafik` (Assets, Rig), `k3c-balancing` (neue Werte, ohne Simulation).
3. Schwere Befunde beheben, Rest als Tickets mit Projekt und Sprint.
4. `check_run task:check`, dann `check_run task:check:go`.
5. Abnahme-Ticket anlegen (Sim, Balance, Gen, Race).
6. Glossar-Eintrag „Reittier“ anpassen, „Gatter“ eintragen (letzter Commit); Abschnitt Abnahme der Sprint-README füllen.

## Fertig, wenn

- [ ] AC-01 bis AC-04: Nachweise geprüft, Abweichungen benannt.
- [ ] AC-06: `task:check` und `task:check:go` grün, Abnahme-Ticket angelegt, Abnahme-Abschnitt gefüllt.

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
