# SI5.4 · Hütte, Figur und Bauwerk im Spiel (B-1001)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si5
- **Abhängig von:** SI5.3
- **Tickets:** B-1001
- **Kriterien:** AC-04

## Ziel

Das Spiel zeichnet die Hütte mit Figur, den Weg der Figur in den Hub und das Bauwerk am Bauplatz (B-1001/AC-02, AC-04).

## Kontext

Das native Spiel (`native/k3c-game`) zeichnet nur; Zustand aus der Engine (SI5.2: Hütte belegt oder leer, Figur unterwegs, Bauplatz `hermitHall` mit Bau oder Bauwerk). Gebäude und Plätze: `native/k3c-game/props/` (`buildings.go`, `sites.go`, `plan_sites.go`), Figuren: `native/k3c-game/actors/`. Der Bau läuft in der Engine sofort; der Weg der Figur ist reine Anzeige (sie geht in Stufe 2 zum Aufgang und steht danach im Hub am Bauplatz). Assets aus SI5.3. Split-Screen: jede Hälfte zeichnet ihre Stufe. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `native/k3c-game/**`

## Nicht-Ziele

Bonus im HUD, Dialog oder Text der Figur, neue Sounds, Engine-Änderungen (fehlt ein Zustand: Ticket).

## Schritte

1. Zustand aus SI5.2 lesen.
2. Hütte zeichnen; Figur davor, solange nicht gezahlt; Preis über die Zahlstellen-Anzeige.
3. Nach der Zahlung: Figur geht zum Aufgang (Stufe 2) und steht am Bauplatz (Hub).
4. Bauplatz mit vorhandener Bau-Darstellung (Gerüst, Bauwerk, zerstört).
5. Go-Tests für die Zeichen-Einträge vor und nach der Zahlung; Fachprüfung `k3c-grafik`.

## Fertig, wenn

- [ ] AC-04: Tests zu B-1001/AC-02 grün (Testnamen im Ergebnis); `check_run task:check:diff` grün.
- [ ] Die Beobachtung am TV (B-1001/AC-03) bleibt bei 🧑 und ist im Ergebnis als offen genannt.

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
