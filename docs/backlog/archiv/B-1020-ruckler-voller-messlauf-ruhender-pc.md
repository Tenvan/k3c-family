# B-1020 · Der volle Ruckler-Messlauf läuft auf dem ruhenden PC mit einer Last-Szene, in der die Gegner überleben

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** RU0
- **Projekt:** RSU
- **Erstellt:** 2026-11-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU0.1 hat am 2026-11-16 fünf Läufe von je 120 s gemessen (Tabelle in `RU0.1 › Ergebnis`). Erste Auswertung: GC ist nicht die Ursache der Ruckler; die langen Bilder entstehen außerhalb von Update und Draw. Das Ergebnis ist vorläufig: Der PC war nicht in Ruhe (Claude-App, Workbench, Shell-Befehle, Tippen), die Last-Szene war zu schwach (168 eigene Einheiten erschlagen die Gegner in Sekunden, meist lebten 0 bis 60), je Fall gab es nur einen kurzen Lauf, und drei Läufe der Last-Szene wurden gestoppt. 🧑: Ruckler fallen vor allem beim Durchreiten einer voll ausgebauten Welt mit vielen Soldaten, Bürgern und Gegnern auf.

## Ziel

Belastbare Zahlen aus der Szene, in der 🧑 die Ruckler sieht, gemessen auf dem ruhenden PC. Damit ist die erste Auswertung bestätigt oder widerlegt.

## Beteiligte und Zielgruppen

🧑 gibt das Zeitfenster (PC bleibt unberührt) und sieht die Läufe an. Ein Agent startet und wertet aus.

## Anforderungen

- Last-Szene (`--measure-load`, `native/k3c-game/game/measure_load.go`): voll ausgebaute Welt, viele Kämpfer, Schützen und Bürger, dazu Gegner, die über die ganze Messdauer leben und in der Landschaft stehen, nicht nur an der Burg. Die Spieler reiten im Sprint durch die ganze Welt, ohne dass das Spiel endet.
- Je Lauf 5 Minuten nach 5 s Aufwärmen, 1920×1080, zwei Spieler, Messung je Bild (`--measure-frames`).
- Läufe: normal; GC aus (`GOGC=off`, `GOMEMLIMIT=8GiB`); VSync aus; Energieschema „Höchstleistung“ mit erhöhter Prozess-Priorität; nach RU0.2 zusätzlich `--pacing frame`.
- Ruhender PC: Claude-App und Workbench geschlossen oder untätig, keine Eingaben, Spielfenster im Vordergrund. Der Start läuft über ein Skript, das nach dem Start nichts mehr tut, bis alle Läufe fertig sind.
- Je Lauf festhalten: Energieschema, Prozess-Priorität, Fenster im Vordergrund, lebende Gegner und Truppen im Mittel.
- Kennzahlen wie in RU0.1: Median, p99, p99,9, längstes Bild, lange Bilder, davon mit GC, Bilder über 33 und 50 ms, Draw und Takt (Mittel und Maximum).

## Nicht-Ziele

Der Fix der Ruckler (B-943). Änderungen an der Sim. Rust-Code.

## Regeln und Einschränkungen

Die Last-Szene nutzt nur Dev-Aktionen des Raums; die Sim bleibt unverändert. Kein Lauf über `sim_test`.

- **Bereich:** `native/k3c-game/game/measure_load*.go`, `native/k3c-game/game/measure_frames*.go`, `native/measure.ps1`, `docs/messung/ruckler-go.md`
- **Abhängig von:** RU0.1
- **Braucht 🧑:** Gerät
- **Größe:** S
- **Sperre:** –

## Beispiele

Auf dem ruhenden PC bleiben in der leichten Szene nur noch eine Handvoll Bilder über 33 ms → die 97 aus RU0.1 kamen vom belegten PC. Mit „Höchstleistung“ und hoher Priorität verschwinden die späten Bilder → Ursache ist das Scheduling von Windows.

## Ausnahme- und Fehlerfälle

Das Spiel endet in der Last-Szene (Krone geraubt) → Lauf verwerfen, Szene anpassen, nicht mit der Reststrecke rechnen. 🧑 nutzt den PC während eines Laufs → Lauf wiederholen.

## Akzeptanzkriterien

- **AC-01** Die Last-Szene hält über 5 Minuten im Mittel mindestens 100 lebende Gegner und 150 eigene Einheiten, und beide Spieler reiten die ganze Zeit; das Spiel endet nicht.
- **AC-02** Die Läufe liegen als Bericht und Bilddatei unter `.work/messung/`; die Tabelle der Kennzahlen steht im Ergebnis der Session und in `docs/messung/ruckler-go.md`.
- **AC-03** Je Lauf sind Energieschema, Priorität und Vordergrund festgehalten; 🧑 bestätigt, dass der PC unberührt war.
- **AC-04** Die Auswertung sagt, ob die erste Auswertung aus RU0.1 hält (GC nicht die Ursache; späte Bilder außerhalb von Update und Draw).

## Offene Fragen

1. Wann ist der PC frei (🧑)? 2. Wie überleben die Gegner: stärkere Sorte, weniger Kämpfer, oder Nachschub in kurzen Abständen an den Portalen (Vorschlag: Wellen über die Dev-Aktion `wave` plus Nachschub)? 3. Zusätzlich auf dem schwachen PC messen (🧑, Gerät)?

## Notizen

Aufruf der Last-Szene aus RU0.1: `k3c-game --seed k3c --players 2 --tick 60 --input <Sprint, Wende alle 15 s> --size 1920x1080 --measure-load 5,160,120 --measure 120 --warmup 5 --report … --measure-frames …`. Auffällig dort: ein Takt mit 231 ms direkt nach dem Auffüllen der Gegner (vermutlich von der Szene selbst).
