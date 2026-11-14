# ST2.1 · Testing-Menü mit einstellbaren Stresstests und Vorgaben aus data/

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/st2
- **Abhängig von:** –
- **Tickets:** B-896
- **Kriterien:** AC-01

## Ziel

Hauptmenü-Karte „Testing“ (Entwickler-Build oder Entwicklermodus) mit einer Einstell-Seite für alle Werte aus B-896, Vorgaben aus `data/stresstests.json`, gemerkten eigenen Werten und dem Start über die Rolle `view.StressStarter`.

## Kontext

- Hauptmenü `native/k3c-game/menu/main_screen.go` (feste Karten `mainCount`, Illustrationen `menu/art.go`), Optionen `menu/options.go`, Lobby `menu/lobby.go` (Seed 0…999 als Vorbild).
- Einstellungen `view/settings.go`, gespeichert in `local/settings.go`.
- Vorgaben lesen über `k3c/data` (`data.Files`), Importregel `game/imports_test.go`: menu darf view, art, sim und data importieren.
- Den Start übernimmt `game` (ST2.2) über die optionale Rolle `view.StressStarter` der Session; fehlt sie, zeigt die Seite eine Meldung.

## Erlaubte Dateien

- `native/k3c-game/menu/**`
- `native/k3c-game/view/**`
- `native/k3c-game/local/settings*.go`
- `data/stresstests.json`

## Nicht-Ziele

- Lauf, Messung und Bericht (ST2.2).

## Schritte

1. `view/stress.go`: `StressTest` (vergleichbar), Gegner je Welle nach Kurve, `StressStarter`.
2. `data/stresstests.json` mit Vorgaben; Lesen in `menu`.
3. `Settings.DevMode` und `Settings.Stress` (gemerkt), Speichern in `local`.
4. Karte Testing im Hauptmenü (nur Entwickler), Schalter Entwicklermodus in den Optionen.
5. Seite `menu/testing.go`: Vorgabe, Spieler, Stufe, Seed, Gegner, Anstieg, Faktor, Ausbau, Truppen, Zeitraffer, Wellen, Dauer, Starten.
6. Tests, `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01: `menu`-Tests: Testing-Seite hat alle Zeilen, Pad und Tastatur ändern Werte, Vorgabe übernimmt Werte aus `data/stresstests.json`, Start ruft `StressStarter` mit den Werten; Karte nur im Entwickler-Modus.
- [ ] `task check:diff` grün.

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

- **AC-01 geprüft:** `menu/testing_test.go`: `TestTestingShowsAllRows` (alle Zeilen aus B-896, rollend mit höchstens `maxRows`), `TestTestingChangesValues` (Pad und Tastatur, Vorgabe übernimmt Werte aus `data/stresstests.json`, Änderung macht eigene Werte), `TestTestingStart` (Start über `view.StressStarter`, Werte gemerkt, ohne Rolle Meldung), `TestMainTestingOnlyForDevelopers`, `TestParseStressPresets`; Speichern: `local/settings_stress_test.go`.
- Neu: `view/stress.go` (`StressTest`, `EnemiesFor`, `StressStarter`), `view/stress_limits.go`, `Settings.DevMode` und `Settings.Stress`, Optionen › Entwicklermodus, Hauptmenü-Karte Testing (Entwickler-Build ohne Version oder Entwicklermodus), Seite `menu/testing.go`, Vorgaben `data/stresstests.json` (Viele Gegner, Volle Burg, Zwei Spieler Split-Screen, Tiefe Stufe).
- **Behoben (aus EP1.1):** Die Optionen hatten mit Steuerung acht Zeilen, die Seite zeigt höchstens sieben; Zurück war unsichtbar. Die Optionen rollen jetzt mit der Auswahl wie die Spielstände.
- Entscheidungen: Spieler 1 … `view.MaxPlayers` (heute 2), „Insel“ ist die eine Insel der Standard-Welt, einstellbar ist die Stufe.
- `task check:diff` grün.
