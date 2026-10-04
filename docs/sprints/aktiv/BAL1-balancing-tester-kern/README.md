# BAL1 · SIM · Balancing-Tester: Kern und Replay

- **Status:** aktiv
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-099, B-159
- **Start-Commit:** 9e6849e
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑; umfasst B-099, B-159 und die Domänen-Ausnahme `tools/k3c-dev/`; mit Änderungen aus dem Spec-Review (Werkzeug als Tool in k3c-dev, veraltete Fragen in B-099 gestrichen)

## Ausgangslage

Die Simulation läuft headless (`engine/sim`, Werkzeug `sim_run` in k3c-dev), aber es gibt keine Bots, keine Szenario-Matrix und kein Dateiformat für wiederholbare Läufe. Details in B-099 und B-159.

## Ziel

Ein Werkzeug spielt viele deterministische Läufe mit Bots und liefert Kennzahlen als JSON; jeder Lauf ist als Datei wiederholbar. Am Ende sichtbar: ein Befehl im Terminal erzeugt den Kennzahlen-Report, eine Replay-Datei lässt sich in k3c-dev abspielen und liefert denselben Endzustand-Hash.

## Beteiligte und Zielgruppen

Entwickler und Agenten nutzen das Werkzeug. Es lebt als Tool in k3c-dev (`tools/k3c-dev/internal/`), beschlossen von 🧑 am 2026-10-04.

## Anforderungen

B-099 › Anforderungen (Szenario-Matrix, Bot-Profile, Kennzahlen je Lauf; Bericht und Sensitivität folgen in BAL2 und BAL3) und B-159 › Anforderungen.

## Nicht-Ziele

Zielkorridore und `task balance` (BAL2, B-157), weitere Profile und Sensitivität (BAL3, B-158), Abgleich mit echten Abenden (BAL4, B-160).

## Regeln und Einschränkungen

**Domänen-Ausnahme (Freigabe dieser Spec erlaubt sie):** Der Tester (Bots, Kennzahlen, Replay) liegt in k3c-dev unter `tools/k3c-dev/internal/` (Domäne SRV); BAL1.1 bis BAL1.3 dürfen dort und am Task in `Taskfile.yml` ändern. Die Simulation selbst bleibt in `engine/sim` und wird nur benutzt.

Deterministisch (`engine/rng`, keine Wanduhr); Bots nur über `PlayerCommand`; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`; Aufgaben nur über `task`. Einschiebbar zwischen den Phasen, kein zweiter aktiver Nicht-Einschiebbar-Sprint.

## Beispiele

Bot „sparsam“, 100 Seeds, 2 Spieler, 5 Tage → JSON mit Kennzahlen je Lauf; Seed 12 als Replay-Datei gespeichert → Wiedergabe liefert denselben Burgfall-Tick.

## Ausnahme- und Fehlerfälle

Lauf bricht ab → „ungültig“ mit Seed im Report. Replay-Datei mit unbekannter Version → Fehler mit Versionsangabe (B-159).

## Akzeptanzkriterien

- **AC-01** Ein Lauf mit denselben Seeds, Parametern und Daten liefert byte-gleiche Kennzahlen (B-099/AC-01).
- **AC-02** Mindestens zwei Bot-Profile und die Kennzahlen aus B-099 › Anforderungen sind umgesetzt und getestet (B-099/AC-02).
- **AC-03** Aufnahme und Wiedergabe eines Bot-Laufs liefern denselben Endzustand-Hash (B-159/AC-01).
- **AC-04** Der Tester schreibt auf Wunsch je Lauf eine Replay-Datei mit Seed, Parametern, Datenstand-Hash und Eingaben (B-159/AC-02).
- **AC-05** Unbekannte Version und abweichender Datenstand ergeben Fehler beziehungsweise Warnung (B-159/AC-03).
- **AC-06** k3c-dev spielt eine Replay-Datei ab und nennt Endzustand-Hash und Burgfall-Tick (B-159/AC-04).
- **AC-07** `task check:go` ist grün.

## Offene Fragen

keine (Ort des Werkzeugs: Tool in k3c-dev, 🧑 2026-10-04)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BAL1.1 | `BAL1.1-bots-kennzahlen.md` | Umsetzung | autonom | fertig |
| BAL1.2 | `BAL1.2-replay-format.md` | Umsetzung | autonom | fertig |
| BAL1.3 | `BAL1.3-replay-k3c-dev.md` | Umsetzung | autonom | in Arbeit |
| BAL1.4 | `BAL1.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
