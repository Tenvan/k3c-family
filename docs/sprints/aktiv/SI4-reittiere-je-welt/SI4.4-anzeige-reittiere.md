# SI4.4 · Gatter und Reittier je Monarch im Spiel (B-1000)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si4
- **Abhängig von:** SI4.3
- **Tickets:** B-1000
- **Kriterien:** AC-04

## Ziel

Das Spiel zeichnet das Gatter mit dem Welt-Reittier und jeden Monarchen mit seinem eigenen Reittier (B-1000/AC-02, AC-03, AC-05).

## Kontext

Das native Spiel (`native/k3c-game`) zeichnet nur; Zustand aus der Engine (SI4.3: Reittier-ID je Spieler, Gatter mit Position). Monarch und Reittier: `native/k3c-game/actors/` (Rig, Reittier über den Sprite-Schlüssel aus `data/paper.json`, heute für alle `horse`); Plätze mit Zahlstellen-Anzeige: `native/k3c-game/props/` (`sites.go`, `plan_sites.go`). Assets aus SI4.2 (`camel`, `sengokuHorse`, Gatter). Split-Screen: beide Hälften zeichnen beide Monarchen. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Fachprüfung: Agent `k3c-grafik` (Rig, `actors/`).

## Erlaubte Dateien

- `native/k3c-game/**`

## Nicht-Ziele

Umstiegs-Animation, HUD-Anzeige, neue Sounds, Engine-Änderungen (fehlt ein Zustand: Ticket).

## Schritte

1. Zustand aus SI4.3 lesen.
2. Reittier je Monarch aus seiner Reittier-ID wählen statt für alle gleich; unbekannte ID ist ein Testfehler.
3. Gatter zeichnen, mit stehendem Welt-Reittier, solange ein gesteuerter Monarch es nicht hat; Preis über die Zahlstellen-Anzeige.
4. Go-Tests: zwei Monarchen mit verschiedenen Tieren, Gatter mit und ohne Tier.
5. Fachprüfung `k3c-grafik`, `check:diff`.

## Fertig, wenn

- [ ] AC-04: Tests zu B-1000/AC-02 und AC-03 grün (Testnamen im Ergebnis); `check_run task:check:diff` grün.
- [ ] Die Beobachtung am TV (B-1000/AC-04) bleibt bei 🧑 und ist im Ergebnis als offen genannt.

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
