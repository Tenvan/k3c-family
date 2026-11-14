# CH1.4 · Werte-Seite des Debug-Overlays

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/ch1
- **Abhängig von:** CH1.3
- **Tickets:** B-889
- **Kriterien:** AC-05

## Ziel

Das F10-Overlay hat eine zweite, durchschaltbare Seite mit den Werten der Sim (B-889 › Anzeige 1–5).

## Kontext

- Werte nur aus `*sim.World` und Frame lesen, nicht im Client rechnen.
- Auswahl eines Objekts per Klick (Maus) oder Pad (nächstes Objekt am Monarchen).

## Erlaubte Dateien

- `native/k3c-game/devtools/**`
- `native/k3c-game/view/**`
- `native/k3c-game/game/**`
- `native/k3c-game/local/**`

## Nicht-Ziele

- Logs (CH1.5).

## Schritte

1. Zusammenstellung der Zeilen rein und getestet.
2. Auswahl-Logik getestet.
3. Zeichnen.

## Fertig, wenn

- [ ] AC-05: Go-Tests der Zusammenstellung und Auswahl grün.
- [ ] `check_run task:check:diff` grün.

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

- AC-05 geprüft: F10 schaltet aus → Geometrie → Werte → aus; `devtools/values.go` stellt Tageszeit, Tag, Welle, Restzeit bis zur Nacht (`sim.SecondsToNight`, neu mit Test), je Spieler Gold, Vorrat der Stufe, x, Chunk, Stufe/Tiefe, Objektanzahl, Takt (TPS gemessen in `game/devvalues.go`, Rechenzeit `Frame.StepMs` aus local), Zeitraffer, Seed und das ausgewählte Objekt (Mausklick oder am Monarchen; Gegner/Truppe mit Zustand, Ziel, LP, KI) zusammen. Tests `TestValueLines`, `TestSelectAt`, `TestValuesDraw`, `TestTPSMeter`, `TestPickAt`, `TestSecondsToNight` grün.
- Pad-Auswahl = Objekt am eigenen Monarchen (mitlaufen statt Cursor).
- `check_run task:check:diff:go` grün.
