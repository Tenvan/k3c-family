# SI2.5 · Review Sprint SI2 Doppel-Truhe

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si2
- **Abhängig von:** SI2.4
- **Tickets:** B-968, B-998
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Der Sprint SI2 ist gegen seine Kriterien geprüft, schwere Befunde sind behoben, das Abnahme-Ticket ist angelegt.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Kriterien in der Sprint-README SI2 (AC-01 bis AC-06), Specs `B-968` und `B-998`, Ergebnisse der Sessions SI2.1 bis SI2.4. Die Abnahme am TV (AC-05) macht 🧑. Sim, Balance, Gen und Race laufen im Abnahme-Ticket (B-742; Sim und Balancing sind gesperrt).

## Erlaubte Dateien

- `docs/rules/koop.md`
- `engine/sim/**`
- `data/economy.json`
- `data/paper.json`
- `native/k3c-game/**`

## Nicht-Ziele

Neue Funktionen, Simulationsläufe, Balancing, Grafik-Golden.

## Schritte

1. Diff des Sprint-Branches gegen `main` lesen; je Kriterium den Nachweis prüfen.
2. Fach-Reviews: `k3c-reviewer` (Engine, Spiel), `k3c-grafik` (Assets), `k3c-balancing` (neue Werte in `data/economy.json`, ohne Simulation).
3. Schwere Befunde beheben, Rest als Tickets mit Projekt und Sprint.
4. `check_run task:check`, dann `check_run task:check:go`.
5. Abnahme-Ticket anlegen (Sim, Balance, Gen, Race).
6. Glossar-Begriffe (Koop-Aktion, Doppel-Truhe) im letzten Commit eintragen; Abschnitt Abnahme der Sprint-README füllen.

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
