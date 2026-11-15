# SI6.6 · Review Sprint SI6 Fest nach dem Sieg

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si6
- **Abhängig von:** SI6.5
- **Tickets:** B-973, B-1002
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07

## Ziel

Der Sprint SI6 ist gegen seine Kriterien geprüft, schwere Befunde sind behoben, das Abnahme-Ticket ist angelegt.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Kriterien in der Sprint-README SI6 (AC-01 bis AC-07), Specs `B-973` und `B-1002`, Ergebnisse der Sessions SI6.1 bis SI6.5. Die Abnahme am TV (AC-06) macht 🧑. Sim, Balance, Gen und Race laufen im Abnahme-Ticket (B-742; Sim und Balancing sind gesperrt). Besonders prüfen: Begründung der Golden-Änderung aus SI6.3 und dass kein Weg mehr Gegner nach dem Sieg erzeugt.

## Erlaubte Dateien

- `docs/rules/kampf.md`
- `docs/rules/bosse.md`
- `docs/rules/zielkorridore.md`
- `engine/sim/**`
- `data/native.json`
- `data/sounds.json`
- `data/paper.json`
- `native/k3c-game/**`

## Nicht-Ziele

Neue Funktionen, Simulationsläufe, Balancing, Grafik-Golden, erneutes Schreiben der Sim-Golden ohne Regelgrund.

## Schritte

1. Diff des Sprint-Branches gegen `main` lesen; je Kriterium den Nachweis prüfen.
2. Fach-Reviews: `k3c-reviewer` (Engine, Golden), `k3c-grafik` (Assets, Effekte), `k3c-sound` (Festmusik).
3. Schwere Befunde beheben, Rest als Tickets mit Projekt und Sprint.
4. `check_run task:check`, dann `check_run task:check:go`.
5. Abnahme-Ticket anlegen (Sim, Balance, Gen, Race; dazu Neubewertung des Korridors „Schiff fertig“ als Hinweis).
6. Glossar-Begriffe (Friedliche Insel, Fest) im letzten Commit eintragen; Abschnitt Abnahme der Sprint-README füllen.

## Fertig, wenn

- [ ] AC-01 bis AC-05: Nachweise geprüft, Abweichungen benannt.
- [ ] AC-07: `task:check` und `task:check:go` grün, Abnahme-Ticket angelegt, Abnahme-Abschnitt gefüllt.

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
