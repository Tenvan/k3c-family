# RU0 · CLI · Ruckler im Go-Spiel messen: GC, Bildtakt, Laden (Gate G0)

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1019, B-1020
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-16 🧑 Chat (k3c-08)

## Ausgangslage

B-943 (offen, FB4): Die Grafik ruckelt trotz stabiler 60 FPS, auch auf schnellen Rechnern. Die Sim taktet mit 30 Hz (`game.TPS`), gezeichnet wird mit Bildrate und Zwischenbildern (`native/k3c-game/game/blend.go`, alpha aus der Wanduhr). Der Messmodus zählt schon GC-Läufe und die Summe der Pausen (`measure.go`), ordnet sie aber keinem Bild zu. 🧑 vermutet nicht steuerbare GC-Läufe als Ursache; das ist ungeprüft.

## Ziel

Die Ursache der Ruckler ist gemessen, nicht vermutet: GC, Bildtakt, Laden oder blockierende Aufrufe, je mit Anteil. Am Ende sichtbar: Bericht `docs/messung/ruckler-go.md` und die Antwort, ob ein Versuch in Go die Ruckler beseitigt.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1019 › Anforderungen` (Messung je Bild, vier Läufe, Bericht).

Welle 1 von 26, 3 Sessions (B-1015).

Modell-Empfehlung: `sonnet`.

### Sessions

1. Messung je Bild: Zeitstempel, Dauer von Update und Draw, Takte je Bild, alpha, GC-Läufe mit Pause im selben Bild (AC-01)
2. Versuchsschalter und vier Läufe derselben Szene: normal, GC aus, VSync, Takt aus eigener Zeitsammlung (AC-02)
3. Bericht, Sichtprüfung 🧑, Review (AC-03)

### Hinweise für die Umsetzung

- Entscheidend ist der Lauf „GC aus“: Ruckelt er genauso, ist GC nicht die Ursache, und Rust behebt die Ruckler nicht von selbst.
- Entscheidend ist ebenso `--pacing frame`: Verschwinden die Ruckler damit, liegt es am Zusammenspiel von 30-Hz-Takt, Bildrate und Wanduhr; das lässt sich in Go beheben (B-943) und muss ein Rust-Client genauso richtig machen (RU4).
- Entscheidung 006 fand bei den Takt-Ausreißern des Servers das Scheduling von Windows (EcoQoS, Priorität), nicht Go. Prozess-Priorität und Energieschema je Lauf notieren.
- Der Versuchsschalter ist ein Versuch, kein Fix: Standard bleibt das heutige Verhalten. Der Fix gehört zu B-943.
- B-943 liegt im aktiven Sprint FB4 und berührt dieselben Dateien; vor dem Start abstimmen, welcher Sprint zuerst läuft.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `native/k3c-game/game/measure*.go`, `native/k3c-game/game/pacing*.go`, `native/k3c-game/game/game.go`, `native/k3c-game/game/blend.go`, `native/k3c-game/main.go`, `native/measure.ps1`, `docs/messung/ruckler-go.md`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Sperre:** –

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Mit `--measure` schreibt das Go-Spiel je Bild eine Zeile nach `.work/messung/<lauf>.jsonl`: Zeitstempel, Dauer von Update und Draw, Zahl der Takte in diesem Bild, alpha, GC-Läufe und Pausen in diesem Bild (aus `runtime/metrics`). Ohne `--measure` ändert sich nichts.
- **AC-02** Vier Läufe derselben Szene (zwei Spieler, Nacht mit Gegnern, 5 Minuten): normal; GC aus (`GOGC=off` mit Speichergrenze); VSync an und aus; Versuchsschalter `--pacing frame` (ein Update je Bild, Sim-Takte aus eigener Zeitsammlung, alpha aus deren Rest). Je Lauf stehen Median, p99, p99,9, längstes Bild, Zahl der Bilder über dem 1,5-Fachen des Medians und der Anteil davon mit GC im selben Bild im Bericht.
- **AC-03** Der Bericht nennt die Ursachen mit Anteil und sagt, welcher Lauf sichtbar ruckelfrei ist; 🧑 hat die vier Läufe am PC angesehen und G0 entschieden.

## Offene Fragen

Entscheidung 🧑 am Gate bzw. vor dem Start (B-1014 › Offene Fragen).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU0.1 | `RU0.1-messung-je-bild.md` | Umsetzung | autonom | fertig |
| RU0.2 | `RU0.2-versuchsschalter-und-laeufe.md` | Umsetzung | autonom | fertig |
| RU0.3 | `RU0.3-bericht-und-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-16, k3c-10. Kriterien: RU0.1 und RU0.2 › Ergebnis (Messung je Bild, Schalter `--pacing frame|sync`, Vergleich Go/Ebitengine gegen Rust/wgpu, Lasttest). Befunde: 4 Lint-Befunde behoben, `check:diff` grün. G0 (🧑): Umstieg auf Rust. Neue Tickets: keine; B-1020 verworfen. Abnahme-Ticket Sim/Gen: entfällt (keine Sim- oder Gen-Änderung).
