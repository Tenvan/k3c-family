# B-159 · Ein Lauf ist als Datei aus Seed und Eingaben wiederholbar

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** BAL1
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑 (mit BAL1)

## Ausgangslage

Die Simulation ist deterministisch (`engine/sim`, `engine/rng`) und läuft headless (`sim.CreateWorld`, `sim.Step`; Werkzeug `sim_run` in `tools/k3c-dev/internal/enginetools/run.go`). Ein Fehler oder eine Balancing-Auffälligkeit lässt sich nur nachspielen, wenn man Seed und Eingaben von Hand kennt; ein Dateiformat dafür fehlt.

## Ziel

Ein Lauf lässt sich als Datei speichern (Seed, Parameter, Datenstand, Eingaben je Tick) und exakt wieder abspielen. Nutzen: Bugs und verletzte Balancing-Ziele sind mit einer Datei reproduzierbar; Bots (B-099) liefern die Aufzeichnung.

## Beteiligte und Zielgruppen

Entwickler und Agenten (Fehlersuche, Balancing); 🧑 bekommt Repro-Dateien aus dem Tester.

## Anforderungen

- Format: Seed, Spieleranzahl, Szenario-Parameter, Hash des Datenstands (`data/*.json`), Liste der `PlayerCommand` je Tick; Textformat (JSON), Version im Dateikopf.
- Aufnahme: Der Tester (Bots) schreibt auf Wunsch eine Datei je Lauf; Wiedergabe in `engine/sim` ohne Bot.
- Wiedergabe liefert den Endzustand-Hash des Originals; bei anderem Datenstand eine Warnung.
- Wiedergabe-Werkzeug in k3c-dev neben `sim_run`.

## Nicht-Ziele

Aufnahme echter Spielsitzungen aus dem Browser (später), Video-Wiedergabe, Schnittstelle im Protokoll v2.

## Regeln und Einschränkungen

Deterministisch (`engine/rng`, keine Wanduhr), Eingaben nur als `PlayerCommand`; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`; Spielstand-Versionen aus F2 (B-137) bleiben unberührt.

## Beispiele

Bot „sparsam“, Seed 12, 2 Spieler → Datei; Wiedergabe → derselbe Burgfall-Tick und derselbe Endzustand-Hash.

## Ausnahme- und Fehlerfälle

Datei mit unbekannter Version → Fehler mit Versionsangabe. Abweichender Datenstand → Warnung, Wiedergabe läuft trotzdem. Beschädigte Datei → Fehler mit Zeilennummer.

## Akzeptanzkriterien

- **AC-01** Test: Aufnahme und Wiedergabe eines Bot-Laufs liefern denselben Endzustand-Hash.
- **AC-02** Der Tester schreibt auf Wunsch je Lauf eine Replay-Datei mit Seed, Parametern, Datenstand-Hash und Eingaben je Tick.
- **AC-03** Test: Eine Datei mit unbekannter Version und eine mit anderem Datenstand-Hash ergeben den dokumentierten Fehler beziehungsweise die Warnung.
- **AC-04** k3c-dev spielt eine Replay-Datei ab und nennt Endzustand-Hash und Burgfall-Tick.

## Offene Fragen

keine

## Notizen

Lücke 23 aus `docs/plan-weiterentwicklung.md` § 4. Bots liefern die Aufnahme (BAL1).
