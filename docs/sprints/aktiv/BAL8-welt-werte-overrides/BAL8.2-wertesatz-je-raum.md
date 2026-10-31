# BAL8.2 · Wertesatz je Raum statt globaler Variablen, Go-Tests, Golden

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** bal8/2-wertesatz-je-raum
- **Abhängig von:** BAL8.1
- **Tickets:** B-681
- **Kriterien:** AC-02

## Ziel

Jeder Raum rechnet mit dem aufgelösten Wertesatz seiner Welt statt mit Paket-Variablen; Welten beeinflussen sich nicht.

## Kontext

Baut auf BAL8.1 (auflösende Funktion). Globale Werte in `engine/sim/data.go`: buildings, troops, economy, hub, monarch, enemyData, waves, goals (etwa 270 Zugriffsstellen in `engine/sim/*.go`, `engine/room/`, `engine/level/`); `UseData` ersetzt sie für den Balancing-Tester (`tools/k3c-workbench/internal/balance/vary.go`). Raum: `engine/room/room.go`, Welt: `engine/sim/types.go` (type World). Auflösung einmal je Raum beim Anlegen/Laden, kein Mehraufwand je Tick. Determinismus (`engine/rng`), 2 Spieler wie 1, Race-Detector (lokal ohne gcc nicht lauffähig, in CI).

## Erlaubte Dateien

- `engine/sim/**`, `engine/room/**`, `engine/level/**` (nur Zugriffe auf Wertesatz)
- `tools/k3c-workbench/internal/balance/vary.go` nur für `UseData`-Nachzug
- `docs/rules/welten.md` § 6

## Nicht-Ziele

Format und Prüfung (BAL8.1), Editor und Chat (WZ37), Protokoll- oder Spielstand-Änderung ohne Entscheidung von Ralf, neue Werte.

## Schritte

1. Wertesatz-Typ (`Values`) mit den Feldern der Paket-Variablen, am `World` gehalten; Standard-Wertesatz = heutige Variablen.
2. Zugriffe schrittweise von der Paket-Variable auf `w.vals`/`Values` umstellen (nach Datei-Gruppen), jede Gruppe mit gezieltem Test.
3. Raum-Anlegen/Laden löst die Welt auf (BAL8.1) und gibt den Wertesatz an die Sim.
4. `UseData` weiter für den Balancing-Tester bedienen.
5. Go-Tests: zwei Räume mit verschiedenen Welten und Kosten parallel, unabhängig; Raum ohne Overrides byte-gleich, Golden unverändert (B-681/AC-01, AC-02); `task check:race` in CI (AC-03).

## Fertig, wenn

- [ ] AC-02: Zwei Räume mit verschiedenen Welten rechnen mit eigenen Kosten (B-681/AC-01); ohne Overrides Golden unverändert (B-681/AC-02).
- [ ] B-681/AC-03: `task check:race` grün (lokal ohne gcc nicht prüfbar, dann als übersprungen nennen).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 (B-681/AC-01, AC-02): `TestWertesatzJeWeltParallelUnabhaengig` (vier nebenläufige Läufe byte-gleich zum Einzellauf, 2 Spieler), `TestWertesatzZweiRaeumeEigeneKosten` (room), `TestWertesatzOhneOverridesByteGleich`, Golden unverändert. Review (Sonnet, getrennt) fand 7 Stellen mit gemischter Quelle (HP/MaxHP, chestGold, dawnGold); behoben, `TestWertesatzEineQuelleJeWert` ergaenzt. `task check:diff` grün (3 min 36 s), `check:go` grün (2 min 20 s). B-681/AC-03: `check:race` lokal ohne gcc nicht lauffähig, in CI nachzuweisen. Mittlere Befunde als B-692 eingeplant. Abweichung: `engine/net/actions.go`, `payable.go` je eine Zeile (Stufe als Parameter), Protokoll unverändert.
