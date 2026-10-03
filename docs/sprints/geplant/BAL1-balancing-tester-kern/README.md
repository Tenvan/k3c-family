# BAL1 · SIM · Balancing-Tester: Kern und Replay

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-099, B-159
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Simulation läuft headless (`engine/sim`, Werkzeug `sim_run` in k3c-dev), aber es gibt keine Bots, keine Szenario-Matrix und kein Dateiformat für wiederholbare Läufe. Details in B-099 und B-159.

## Ziel

Ein Werkzeug spielt viele deterministische Läufe mit Bots und liefert Kennzahlen als JSON; jeder Lauf ist als Datei wiederholbar. Am Ende sichtbar: ein Befehl im Terminal erzeugt den Kennzahlen-Report, eine Replay-Datei lässt sich in k3c-dev abspielen und liefert denselben Endzustand-Hash.

## Beteiligte und Zielgruppen

Entwickler und Agenten nutzen das Werkzeug; 🧑 entscheidet, wo es lebt (siehe Offene Fragen).

## Anforderungen

B-099 › Anforderungen (Szenario-Matrix, Bot-Profile, Kennzahlen je Lauf; Bericht und Sensitivität folgen in BAL2 und BAL3) und B-159 › Anforderungen.

## Nicht-Ziele

Zielkorridore und `task balance` (BAL2, B-157), weitere Profile und Sensitivität (BAL3, B-158), Abgleich mit echten Abenden (BAL4, B-160).

## Regeln und Einschränkungen

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

- Wo lebt das Werkzeug (Paket `engine/balance` mit eigenem Befehl oder Tool in k3c-dev)? Entscheidet 🧑 bei der Aktivierung (B-099 › Offene Fragen).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BAL1.1 Szenario-Matrix, zwei Bot-Profile über `PlayerCommand`, Kennzahlen als JSON, Determinismus-Test (AC-01, AC-02).
- BAL1.2 Replay-Format: Aufnahme durch Bots, Wiedergabe, Versions- und Datenstand-Prüfung (AC-03, AC-04, AC-05).
- BAL1.3 Wiedergabe-Werkzeug in k3c-dev (AC-06).
- BAL1.4 Review (AC-07).

## Abnahme

–
