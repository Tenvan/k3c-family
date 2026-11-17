# SI5.5 · Review Sprint SI5 Besondere Figur der Welt

- **Status:** blockiert
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si5
- **Abhängig von:** SI5.4
- **Tickets:** B-972, B-1001
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Der Sprint SI5 ist gegen seine Kriterien geprüft, schwere Befunde sind behoben, das Abnahme-Ticket ist angelegt.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Kriterien in der Sprint-README SI5 (AC-01 bis AC-06), Specs `B-972` und `B-1001`, Ergebnisse der Sessions SI5.1 bis SI5.4. Die Abnahme am TV (AC-05) macht 🧑. Sim, Balance, Gen und Race laufen im Abnahme-Ticket (B-742; Sim und Balancing sind gesperrt).

## Erlaubte Dateien

- `docs/rules/buerger.md`
- `docs/rules/materialien-gebaeude.md`
- `docs/rules/welten.md`
- `engine/sim/**`
- `data/hermits.json`
- `data/hub.json`
- `data/buildings.json`
- `data/paper.json`
- `native/k3c-game/**`

## Nicht-Ziele

Neue Funktionen, Simulationsläufe, Balancing, Grafik-Golden.

## Schritte

1. Diff des Sprint-Branches gegen `main` lesen; je Kriterium den Nachweis prüfen.
2. Fach-Reviews: `k3c-welten` (Engine, Hub-Platz, Welt-Datei), `k3c-grafik` (Assets), `k3c-balancing` (neue Werte, ohne Simulation).
3. Schwere Befunde beheben, Rest als Tickets mit Projekt und Sprint.
4. `check_run task:check`, dann `check_run task:check:go`.
5. Abnahme-Ticket anlegen (Sim, Balance, Gen, Race).
6. Glossar-Begriffe (besondere Figur, Einsiedler-Hütte, Dojo, Schrein) im letzten Commit eintragen; Abschnitt Abnahme der Sprint-README füllen.

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

Pausiert mit SI5.3 (Ralf, 2026-11-17): Review erst, wenn die Welt-Assets zum Schluss kommen.
