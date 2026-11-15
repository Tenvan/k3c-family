# SI1.4 · Ladeplätze und Fracht an Deck im Spiel (B-997)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si1
- **Abhängig von:** SI1.3
- **Tickets:** B-997
- **Kriterien:** AC-04

## Ziel

Das Spiel zeichnet die Ladeplätze am Steg, die Fracht an Deck und die Ablehn-Rückmeldung in beiden Bildhälften (B-997/AC-02, AC-03, AC-05).

## Kontext

Das native Spiel (`native/k3c-game`, Go + Ebitengine) zeichnet nur; alle Zustände kommen aus der Engine (SI1.2: Plätze, Belegung je Platz, Ladeplätze, Ereignis `cargoFull`). Plätze und Gebäude: `native/k3c-game/props/` (`sites.go`, `plan_sites.go`, `buildings.go`, `assets.go`), Münz-Fontäne `props/coin_fountain.go`, Ereignisse hören `props/listen.go`, Ton `native/k3c-game/audio/`. Assets aus SI1.3 über `data/paper.json`. Split-Screen: jede Hälfte zeichnet die Stufe ihres Monarchen (`native/k3c-game/view/`). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Fachprüfung: Agent `k3c-hud` bei Änderungen an `hud/` oder `view/`, sonst `k3c-reviewer`.

## Erlaubte Dateien

- `native/k3c-game/**`

## Nicht-Ziele

Lade-Menü oder Overlay, neue Sounds, Engine-Änderungen (fehlt ein Zustand: Ticket, nicht ändern).

## Schritte

1. Zustand und Ereignisse aus SI1.2 lesen (Ergebnis der Session, `engine/sim`).
2. Ladeplätze am Steg als Plätze mit Zahlstellen-Anzeige zeichnen.
3. Fracht an Deck: je belegtem Platz Beutel, Kiste oder Truppe an fester Deck-Position; Laderaum-Stufe des Schiffs.
4. `cargoFull` → Ablehn-Rückmeldung mit vorhandenem Ton.
5. Go-Tests für Zeichen-Einträge und Rückmeldung.
6. Fachprüfung, `check:diff`.

## Fertig, wenn

- [ ] AC-04: Tests zu B-997/AC-02 und AC-03 grün (Testnamen im Ergebnis).
- [ ] B-997/AC-05: `check_run task:check:diff` grün.
- [ ] Die Beobachtung am TV (B-997/AC-04) bleibt bei 🧑 und ist im Ergebnis als offen genannt.

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
