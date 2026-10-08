# B-202 · S1 und W1 teilen sich die Spielstand-Version 3 eindeutig

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** W7
- **Projekt:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Code steht auf `IslandSaveVersion = 2` (`engine/sim/island_save.go`, Zeile 15), Fixtures gibt es für `testdata/saves/v1/` und `v2/`. Zwei geplante Sessions beanspruchen dieselbe nächste Version:

- `docs/sprints/geplant/S1-monarch-schlag-skills/S1.4-spielstand-golden.md`: Ziel „`IslandSaveVersion = 3`“ (Fund-Pool, Verteilung, Slots), Schritt 2 „Fehlermeldung `erwartet 1, 2 oder 3`“, Schritt 4 „Fixture `testdata/saves/v3/familie.json`“, Migration 2 → 3 und 1 → 3.
- `docs/sprints/geplant/W1-hub-ausbau/W1.3-zerstoerung-reparatur-spielstand.md`: Titel „Spielstand Version 3“, Schritt 4 „`IslandSaveVersion = 3`, Hub-Stufe in `HubSave`, Platz-Stufe in `SiteSave` … Fixture `testdata/saves/v3/familie.json`“; ebenso `W1.1-hub-ausbau.md` (Zeile 24, „das Format ändert erst W1.3 (Version 3)“).

Beide Sprints sind SIM und laufen nacheinander (ein aktiver Sprint je Domäne, B-174). Der Fahrplan (`docs/sprints/README.md` › Geplant) nennt S1 vor W1. Wer als Zweiter kommt, findet Version 3 belegt, sein Fixture-Pfad kollidiert, und seine Migration muss von der Version des Ersten ausgehen.

## Ziel

S1.4 und W1.3 ändern das Spielstand-Format in **einer gemeinsamen Version 3**, ohne dass die zweite Session die erste überschreibt; kein Stand geht verloren (Beschluss Q42, 2026-10-04).

## Beteiligte und Zielgruppen

Entwickler (SIM), Review-Sessions S1.5 und W1.4; 🧑 hat die gemeinsame Version beschlossen (Q42).

## Anforderungen

- Wer zuerst landet, hebt `IslandSaveVersion` auf 3 und legt die Fixture `testdata/saves/v3/` an (Migration 1 → 3 und 2 → 3) (Beschluss Q42, 2026-10-04).
- Die zweite Session ergänzt ihre Felder **optional** (`omitempty`) in Version 3, ohne neue Version, und erweitert die Fixture; ein v3-Stand ohne ihre Felder lädt mit Startzustand (Beschluss Q42, 2026-10-04).
- v3 trägt nur Hub-Stufe und Platz-Stufe; wandernde Offsets gibt es nicht, die Zuordnung der Plätze über `kind@x` bleibt stabil (Beschluss Q43, 2026-10-04; ersetzt Q26).
- Die Session-Dateien beider Sprints nennen Version 3, denselben Fixture-Ordner und die Regel „wer zuerst landet“.
- B-201 (Spielstand der Wirtschaft) folgt später mit Version 4 (Beschluss Q42, 2026-10-04).

## Nicht-Ziele

Inhalt der Formatänderungen (S1.4, W1.3, B-201); eine eigene Version je Sprint.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › „Spielstand-Format ändern“ (B-137); Ausnahme durch Beschluss Q42: beide Änderungen teilen Version 3, die zweite nur mit optionalen Feldern. Eine bereits gemergte Fixture wird nur erweitert, nie umnummeriert. Spec-Änderung der Session-Dateien nur über Revision und Freigabe durch 🧑.

## Beispiele

S1 landet zuerst → S1.4 schreibt v3 mit Skills; W1.3 ergänzt Hub- und Platz-Stufe als optionale Felder in v3 und erweitert `testdata/saves/v3/`; ein v3-Stand aus S1.4 lädt mit Hub- und Platz-Stufe 1, Skills bleiben. Umgekehrt (W1 zuerst) genauso.

## Ausnahme- und Fehlerfälle

v3-Stand ohne die Felder der zweiten Session → Startzustand für diese Felder, kein Ladefehler. Eine Session würde ein Pflichtfeld in v3 einführen → nicht erlaubt, nur `omitempty`.

## Akzeptanzkriterien

- **AC-01** S1.4 und W1.3 (und W1.1, Zeile „Spielstand“) nennen beide Version 3, den Fixture-Ordner `testdata/saves/v3/` und die Regel „wer zuerst landet hebt, der Zweite ergänzt optional“.
- **AC-02** Die zweite Session nennt einen Test, der einen v3-Stand ohne ihre Felder lädt.
- **AC-03** `task test -- planning` grün.

## Offene Fragen

keine (Beschluss Q42, 2026-10-04).

## Notizen

Entstanden bei der Vorbereitung von W2 bis W4 (Fragenkatalog Block 5).

S1.5 (2026-10-04): S1.4 ist zuerst gelandet (Version 3, `testdata/saves/v3/familie.json`), AC-01 und AC-03 erfüllt. Offen bleibt AC-02: W1.3 nennt noch keinen eigenen Test, der einen v3-Stand ohne Hub- und Platz-Stufe lädt; das Ticket bleibt bis W1.3 offen.
