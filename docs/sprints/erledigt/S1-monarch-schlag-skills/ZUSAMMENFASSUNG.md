# S1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Monarch: Schlag, Fund-Pool und Skills
- **Projekt:** –

## Ziel

In der Simulation schlägt der Monarch zu, Skill-Punkte stammen aus einem Fund-Pool je Insel, jeder Spieler verteilt sie für sich, und die Skills von Tank, Zauberer und Heiler wirken. Pool und Verteilung stehen im Spielstand. Am Ende sichtbar: Go-Tests und Golden-Daten grün (Minimum der Phase 1: Schlag plus ein Skill je Klasse).

## Ergebnis

- **2026-10-04 (S1.5):** AC-01, AC-02 (S1.1), AC-03 (S1.2a, S1.2b), AC-04 (S1.2c), AC-07 (S1.3), AC-05, AC-06 (S1.4) mit Nachweis; `task check`, `task check:go` grün (`-race` ohne C-Compiler übersprungen). - **Befunde:** keine schweren. Golden: `sim-forest-monarch.json` neu, `campaign-abstieg.json` nur `skills`/`slots` plus Schlüsselreihenfolge eines Ereignisses, `rng.json` unverändert; Spielstand v3: v1/v2 laden, neuere Version abgelehnt, Quelle unverändert. - **Tickets:** B-118, B-119, B-022, B-152, B-222 erledigt; B-219 bleibt offen (`game-design.md`, `events.go` nicht erlaubt), B-202 offen …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Der Schlag trifft Gegner in Reichweite mit Abklingzeit (B-118/AC-01). |
| AC-02 | – | Fund-Pool und Verteilung je Spieler, Spätbeitretende, Tier-Gating und Respec nur am Tag (B … |
| AC-03 | – | Aktive Skills von Tank, Zauberer und Heiler wirken laut Daten, Gating verhindert Skills oh … |
| AC-04 | – | Passive wirken in 2-Spieler-Szenen (B-119/AC-03). |
| AC-05 | – | Spielstand speichert und lädt Pool, Verteilung und Skills; ein alter Stand lädt (B-118/AC- … |
| AC-06 | – | Golden-Daten sind aktualisiert und `task check:go` ist grün (B-118/AC-05, B-119/AC-04, B-1 … |
| AC-07 | – | Jeder Monarch hat das Standard-Reittier aus den Daten, seine Geschwindigkeit entspricht Ba … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-118 | erledigt | AC-01 –, AC-02 –, AC-05 –, AC-06 – | Der Monarch schlägt zu, Skill-Punkte kommen aus einem Fund-Pool und jeder Spiele … |
| B-119 | erledigt | AC-03 –, AC-04 –, AC-06 – | Die Skills von Tank, Zauberer und Heiler wirken in der Simulation |
| B-022 | erledigt | AC-05 – | Monarch-Level und Skills stehen im Spielstand |
| B-152 | erledigt | AC-06 –, AC-07 – | Jeder Monarch reitet von Anfang an auf einem Standard-Reittier |
