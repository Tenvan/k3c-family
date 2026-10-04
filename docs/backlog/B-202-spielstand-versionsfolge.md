# B-202 · Die Spielstand-Versionen von S1 und W1 folgen eindeutig aufeinander

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
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

Jede Formatänderung hat eine eigene Versionsnummer und einen eigenen Fixture-Ordner; die zweite Session migriert von der ersten, kein Stand geht verloren.

## Beteiligte und Zielgruppen

Entwickler (SIM), Review-Sessions S1.5 und W1.4; 🧑 entscheidet die Reihenfolge.

## Anforderungen

- Die zuerst laufende Session behält Version 3, die zweite nutzt Version 4 mit Fixture `testdata/saves/v4/` und Migration 3 → 4 (sowie 1 → 4, 2 → 4).
- Die Session-Dateien beider Sprints nennen ihre Version, ihren Fixture-Ordner und die Fehlermeldung (`erwartet 1, 2, 3 oder 4`) passend.
- B-201 (Spielstand der Wirtschaft) schließt mit der nächsten freien Version an.

## Nicht-Ziele

Inhalt der Formatänderungen (S1.4, W1.3, B-201); Zusammenlegen beider Änderungen in eine Version.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › „Spielstand-Format ändern“ (B-137): je Formatänderung eine Version, alte Fixtures bleiben unverändert. Spec-Änderung der Session-Dateien nur über Revision und Freigabe durch 🧑.

## Beispiele

S1 läuft zuerst → S1.4 schreibt v3; W1.3 schreibt v4 und liest einen v3-Stand mit Skills, setzt Hub- und Platz-Stufe 1, Skills bleiben.

## Ausnahme- und Fehlerfälle

Reihenfolge ändert sich nach der Anpassung (W1 doch zuerst) → Versionen in beiden Dateien tauschen, bevor eine der Sessions startet; nach dem Merge einer Version wird sie nie umnummeriert.

## Akzeptanzkriterien

- **AC-01** S1.4 und W1.3 (und W1.1, Zeile „Spielstand“) nennen verschiedene Versionen und Fixture-Ordner; `grep -rn "IslandSaveVersion = 3" docs/sprints/geplant` trifft genau eine Session.
- **AC-02** Die Session mit Version 4 nennt die Migration aus Version 3 und einen Test, der das v3-Fixture lädt.
- **AC-03** `task test -- planning` grün.

## Offene Fragen

Welcher Sprint läuft zuerst, S1 oder W1 (🧑)? Vorschlag des Agenten: Reihenfolge des Fahrplans (S1 zuerst, W1.3 bekommt Version 4); siehe Fragenkatalog Q42.

## Notizen

Entstanden bei der Vorbereitung von W2 bis W4 (Fragenkatalog Block 5).
