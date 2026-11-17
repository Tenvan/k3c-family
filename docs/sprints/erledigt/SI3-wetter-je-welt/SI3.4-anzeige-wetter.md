# SI3.4 · Vorboten, Taifun und Flut im Spiel mit Ambient (B-999)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si3
- **Abhängig von:** SI3.3
- **Tickets:** B-999
- **Kriterien:** AC-04

## Ziel

Das Spiel zeigt Vorboten, Taifun und Flut und spielt je Art ein Wetter-Ambient (B-999/AC-02, AC-03, AC-05).

## Kontext

Das native Spiel (`native/k3c-game`) zeichnet nur; Zustand aus der Engine (SI3.2: `weather` mit Art, Phase `forecast` oder `active`, gesperrte Bereiche der Flut; Wirkung nur bei Tiefe 0). Himmel und Ebenen: `native/k3c-game/world/backdrop.go`, `layers.go`; Wasser: `world/water.go`, `world/water/`; Effekte: `native/k3c-game/fx/` (`effects.go`, `paint.go`); Ton: `native/k3c-game/audio/`, Slots in `data/sounds.json` (Kategorien sfx, musik, ambient; Mischpegel `docs/rules/klang.md`). Asset `wetter-wolkenbank` aus SI3.3. Split-Screen: jede Hälfte zeigt das Wetter nur, wenn ihr Monarch an der Oberfläche ist. Bild-Budget beachten (`world/measure_test.go`). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `native/k3c-game/**`
- `data/sounds.json`
- `public/audio/**`

## Nicht-Ziele

Schneesturm, HUD-Vorhersage, neue Animationen für Bäume oder Banner, Engine-Änderungen (fehlt ein Zustand: Ticket).

## Schritte

1. Zustand aus SI3.2 lesen.
2. Vorboten: Wolkenbank einblenden (Taifun), Wasserstand am Ufer anheben (Flut).
3. Taifun: Regen und Windstreifen in Windrichtung als Effekt.
4. Flut: gesperrte Randzonen unter Wasser zeichnen, Grenze klar erkennbar.
5. Ambient je Art: vorhandene Quelle wählen oder neuen Slot in `data/sounds.json` anlegen (Agent `k3c-sound` prüft); Start mit `active`, Ende am Morgen.
6. Go-Tests für Zeichen-Einträge je Art und Phase, unter Tage nichts, Ambient-Start und -Ende.
7. Fachprüfung `k3c-grafik`, `check:diff`.

## Fertig, wenn

- [ ] AC-04: Tests zu B-999/AC-02 und AC-03 grün (Testnamen im Ergebnis); `check_run task:check:diff` grün.
- [ ] Die Beobachtung am TV (B-999/AC-04) bleibt bei 🧑 und ist im Ergebnis als offen genannt.

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
