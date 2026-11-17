# SI2.4 · Doppel-Truhe im Spiel (B-998)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si2
- **Abhängig von:** SI2.3
- **Tickets:** B-998
- **Kriterien:** AC-04

## Ziel

Das Spiel zeichnet die Doppel-Truhe mit leuchtendem Schloss und die Fontäne beim Öffnen (B-998/AC-02, AC-04).

## Kontext

Das native Spiel (`native/k3c-game`) zeichnet nur; Zustand aus der Engine (SI2.3: Kennzeichen wirkt oder nicht, wartender Monarch). Truhen und Objekte: `native/k3c-game/props/` (`plan_objects.go`, `assets.go`, `data.go`), Fontäne `props/coin_fountain.go` (B-953), Ereignisse `props/listen.go`. Assets aus SI2.2 über `data/paper.json`. Linkes Schloss = Monarch mit kleinerem Spieler-Index (Spec `B-998`). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `native/k3c-game/**`

## Nicht-Ziele

Neue Sounds, HUD-Text, Wegweiser, Engine-Änderungen (fehlt ein Zustand: Ticket).

## Schritte

1. Zustand aus SI2.3 lesen.
2. Truhe mit wirkendem Kennzeichen als `doppel-truhe` zeichnen, sonst wie bisher.
3. Wartender Monarch → Zustand „Schloss leuchtet“ links oder rechts.
4. Öffnen → `doppel-truhe-offen` und vorhandene Fontäne.
5. Go-Tests für die Wahl des Bildes und des Schlosses; Fachprüfung `k3c-reviewer`.

## Fertig, wenn

- [ ] AC-04: Tests zu B-998/AC-02 grün (Testnamen im Ergebnis); `check_run task:check:diff` grün.
- [ ] Die Beobachtung am TV (B-998/AC-03) bleibt bei 🧑 und ist im Ergebnis als offen genannt.

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
