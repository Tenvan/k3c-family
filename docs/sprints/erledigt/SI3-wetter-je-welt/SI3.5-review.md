# SI3.5 · Review Sprint SI3 Wetter je Welt

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si3
- **Abhängig von:** SI3.4
- **Tickets:** B-969, B-999
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Der Sprint SI3 ist gegen seine Kriterien geprüft, schwere Befunde sind behoben, das Abnahme-Ticket ist angelegt.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Kriterien in der Sprint-README SI3 (AC-01 bis AC-06), Specs `B-969` und `B-999`, Ergebnisse der Sessions SI3.1 bis SI3.4. Die Abnahme am TV (AC-05) macht 🧑. Sim, Balance, Gen und Race laufen im Abnahme-Ticket (B-742; Sim und Balancing sind gesperrt).

## Erlaubte Dateien

- `docs/rules/wetter.md`
- `docs/rules/welten.md`
- `engine/sim/**`
- `data/weather.json`
- `data/paper.json`
- `data/sounds.json`
- `native/k3c-game/**`

## Nicht-Ziele

Neue Funktionen, Simulationsläufe, Balancing, Grafik-Golden.

## Schritte

1. Diff des Sprint-Branches gegen `main` lesen; je Kriterium den Nachweis prüfen.
2. Fach-Reviews: `k3c-welten` (Engine, Welt-Datei), `k3c-grafik` (Asset, Darstellung), `k3c-sound` (Ambient), `k3c-balancing` (neue Werte, ohne Simulation).
3. Schwere Befunde beheben, Rest als Tickets mit Projekt und Sprint.
4. `check_run task:check`, dann `check_run task:check:go`.
5. Abnahme-Ticket anlegen (Sim, Balance, Gen, Race).
6. Glossar-Begriffe (Wetter, Wetternacht, Vorboten, Taifun, Flut) im letzten Commit eintragen; Abschnitt Abnahme der Sprint-README füllen.

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

- `task check`: erst rot (`hintergruende.test.ts` zählte `wetter-wolkenbank` als Kulisse), Filter um `wetter-*` erweitert, Test grün.
- `task check:go`: rot nur in `engine/internal/detvec` (`doppel-truhe`, bekannt B-1030, nicht aus SI3).
- Diff: Asset Wolkenbank (Spec, Index, Golden, `data/paper.json`); Regeltext aus SI3.1; SI3.2/SI3.4 verworfen.
- Abnahme-Ticket B-1031 (Gen samt Golden, Race, check:go nach B-1030).
