# RU0.1 · Messung je Bild mit GC-Marken im Go-Spiel

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/ru0
- **Abhängig von:** –
- **Tickets:** B-1019
- **Kriterien:** AC-01

## Ziel

Das Go-Spiel schreibt mit `--measure-frames <datei>` je Bild eine Zeile: Zeit, Dauer von Update und Draw, Takte, alpha, GC.

## Kontext

Der Messmodus steht in `native/k3c-game/game/measure.go` (Schalter `--measure`, `--warmup`, `--seed`, `--tick`, `--input`; Bericht `Report` mit `FrametimeMs`, `BilderOhneTakt`, `TaktAbstandMs`, `GcLaeufe`, `GcPauseMs`). Die Sim taktet mit 30 Hz (`game.TPS`, `game.go:17`, gesetzt in `native/k3c-game/main.go:112`), gezeichnet wird mit Bildrate; `blend.go` schiebt die Lage um alpha aus der Wanduhr. Die genaue Uhr steht in `clock_windows.go`.

Fallstricke: `runtime.ReadMemStats` hält alle Goroutinen an und darf nicht je Bild laufen; `runtime/metrics.Read` mit vorab angelegten Proben ist dafür gemacht (`/gc/cycles/total:gc-cycles`, `/sched/pauses/total/gc:seconds`). Die Messung darf je Bild nichts anlegen: Puffer für die ganze Messdauer vorab (5 Minuten × 240 Bilder je Sekunde), geschrieben wird erst nach der Messung.

## Erlaubte Dateien

- `native/k3c-game/game/measure*.go`
- `native/k3c-game/main.go`

## Nicht-Ziele

Am Takt oder an den Zwischenbildern etwas ändern (RU0.2). Der Fix für die Ruckler (B-943).

## Schritte

1. `measure.go`, `game.go` (Update und Draw), `blend.go`, `main.go` (Schalter) lesen.
2. Neue Datei `measure_frames.go`: Probe je Bild (Zeit seit Start in ns, Update in ms, Draw in ms, Takte in diesem Bild, alpha, GC-Läufe seit dem Bild davor) in einem vorab angelegten Puffer; Lesen der GC-Zähler über `runtime/metrics` mit wiederverwendeten Proben.
3. Schalter `--measure-frames <datei>` in `main.go`; nach der Messung schreibt das Spiel je Probe eine JSON-Zeile.
4. Bericht erweitern: p99,9, längstes Bild, Zahl der Bilder über dem 1,5-Fachen des Medians, Anteil davon mit GC, Anteil mit null und mit zwei Takten. Die Rechnung als reine Funktion mit Test (`measure_frames_test.go`).
5. `go test ./game/ -run Measure` in `native/k3c-game`, `task native:game:build`, ein Probelauf von 60 s; `task check:diff`.

## Fertig, wenn

- [ ] AC-01: Der Probelauf erzeugt eine Datei mit einer Zeile je Bild und einen Bericht mit den neuen Feldern (Auszug im Ergebnis).
- [ ] Der Test der Kennzahlen ist grün; ohne `--measure-frames` zeigt `git diff` keinen geänderten Pfad im normalen Ablauf von Update und Draw außer einer Abfrage auf den Schalter.

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

**Zwischenstand 2026-11-16 (k3c-08), vorläufig: Der volle Lauf folgt (🧑). Status bleibt `in Arbeit`.**

### Umgesetzt

- AC-01 erfüllt: `--measure-frames <datei>` schreibt je Bild Zeit, Bilddauer, Draw-Zeit, Takte, alpha und GC-Läufe (`native/k3c-game/game/measure_frames.go`, Kennwerte im Bericht unter `bilder`). GC-Zähler aus `runtime/metrics`, Puffer vorab, geschrieben wird nach der Messung. `go test -short ./game/` grün.
- Zusätzlich auf Wunsch 🧑 (Szene zu leicht): Last-Szene `--measure-load <ausbau>,<truppen>,<gegner>` über die Dev-Aktionen des Raums (`measure_load.go`). An der Sim ist nichts geändert.
- „GC aus“ ohne Codeänderung: `GOGC=off`, `GOMEMLIMIT=8GiB`.
- Abweichungen vom Plan: Änderungen auch in `game/modes.go` (Schalter) und `game/game.go` (alpha merken, Last-Szene); Arbeit im Hauptcheckout auf `sprint/planung-w1`, nicht committet; `task check:diff` steht noch aus; Läufe je 120 s statt 5 Minuten.

### Läufe (1920×1080, 2 Spieler, je 120 s nach 5 s Aufwärmen, Energieschema „Ausbalanciert“)

Leichte Szene: `--seed k3c --tick 15600`, Nacht, beide Spieler stehen an der Burg. Last-Szene: `--tick 60 --measure-load 5,160,120`, beide Spieler reiten im Sprint gegenläufig, Wende alle 15 s. „Lang“ = Bild über dem 1,5-Fachen des Medians.

| Lauf | GC | VSync | Bilder | Median ms | p99 ms | p99,9 ms | längstes ms | lang | lang mit GC | Bilder > 33 ms | Bilder > 50 ms | GC-Läufe | Draw Mittel / max ms | Takt Mittel / max ms |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
| leicht | an | an | 7 109 | 16,7 | 36,4 | 73,3 | 155,9 | 242 | 0 | 97 | 32 | 1 | 0,92 / 15,5 | 0,30 / 14,5 |
| leicht | **aus** | an | 7 073 | 16,7 | 42,3 | 87,2 | 215,2 | 277 | 0 | 125 | 39 | 0 | 0,91 / 27,8 | 0,30 / 5,9 |
| leicht | an | aus | 121 000 | 0,6 | 3,1 | 12,9 | 141,0 | – | 21 | 11 | 7 | 38 | 0,44 / 102,8 | 0,20 / 5,8 |
| leicht | **aus** | aus | 79 727 | 1,0 | 8,6 | 32,9 | 409,1 | – | 0 | 76 | 34 | 0 | 0,56 / 405,3 | 0,30 / 32,9 |
| Last | an | an | 7 013 | 16,7 | 45,9 | 106,7 | 296,1 | 345 | 1 | 161 | 54 | 8 | 2,36 / 158,5 | 0,94 / 231,3 |

Die übrigen drei Läufe der Last-Szene (GC aus, VSync aus) wurden auf Wunsch 🧑 nach dem ersten gestoppt.

### Erste Auswertung

1. **GC ist nach diesen Läufen nicht die Ursache.** Mit „GC aus“ lief kein einziger GC-Lauf, und es gab eher mehr lange Bilder (277 statt 242). Von 242 bzw. 345 langen Bildern fiel 0 bzw. 1 mit einem GC-Lauf zusammen. Bilder mit GC-Lauf dauerten höchstens 26 ms; die GC-Pausen summieren sich auf 0 bis 7 ms je Lauf.
2. **Die langen Bilder entstehen überwiegend außerhalb von Update und Draw.** In den längsten Bildern (80 bis 300 ms) liegt Draw bei 0,5 bis 2,5 ms; danach holt Ebitengine 2 bis 5 Takte auf einmal nach. Das Bild kommt also zu spät an (Warten auf die Bildausgabe oder auf Rechenzeit vom Betriebssystem), nicht weil das Spiel rechnet. Vermutet, ungeprüft: Scheduling von Windows (Energieschema „Ausbalanciert“, vgl. Entscheidung 006) oder die Bildausgabe (VSync, DWM).
3. **Schon die leichte Szene ruckelt:** 97 Bilder über 33 ms in 2 Minuten bei 0,9 ms Draw. Mit Last zu wenig hat es also nichts zu tun.
4. **Einzelne Ausreißer im Programm** gibt es zusätzlich: ein Draw mit 158 ms und ein Takt mit 231 ms in der Last-Szene (Takt 3005, direkt nach dem Auffüllen der Gegner durch die Last-Szene; vermutlich von der Szene selbst verursacht, ungeprüft), ein Draw mit 405 ms im Lauf „GC aus, VSync aus“.

### Einschränkungen (wichtig für die Bewertung)

- **Der PC war nicht in Ruhe.** Während der Läufe der leichten Szene liefen die Claude-App, die Workbench und Shell-Befehle von Claude (Dateien lesen); 🧑 hat in der Zeit getippt. Das kann die späten Bilder mit verursacht haben. Der volle Lauf muss auf dem ruhenden PC stattfinden.
- **Die Last-Szene ist schwächer als gewollt:** Die 168 eigenen Einheiten erschlagen die Gegner in Sekunden; die meiste Zeit lebten 0 bis 60 Gegner. Gegner entstehen an der Burg, weil sie beim Spieler sofort die Krone rauben (Spielende nach 7 s).
- Je Fall nur ein Lauf von 2 Minuten; keine Wiederholung, keine Sichtprüfung durch 🧑, kein schwacher PC.
- Ob das Fenster im Vordergrund war, ist nicht festgehalten.

### Nächste Schritte

- Voller Lauf auf ruhendem PC (🧑 startet oder gibt das Zeitfenster): fünf Läufe je 5 Minuten, Last-Szene mit überlebenden Gegnern, dazu ein Lauf mit Energieschema „Höchstleistung“ und erhöhter Prozess-Priorität.
- RU0.2 wie geplant: `--pacing frame` (Takt aus eigener Zeitsammlung).
- Rohdaten: `.work/messung/*.json` (Berichte), `*.jsonl` (je Bild), `*.log`.
