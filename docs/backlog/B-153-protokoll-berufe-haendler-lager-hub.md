# B-153 · Das Protokoll kennt Berufe, Händler, Lagerstand, Hub-Stufe und Wartegrund

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** W5
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-06, Chat, durch 🧑, Revision 2; B-330 Variante B

## Ausgangslage

Die Simulation bekommt in W1 bis W4 Hub-Stufen, Lager, Berufe und Händler (B-112, B-121); das Protokoll in `engine/net/protocol.go` und `docs/protocol.md` kennt davon nur, was B-123 liefert (Eingaben für Berufe und Tausch, Zustand der Berufe).

## Ziel

Der Client erhält Hub-Stufe mit Ausbaukosten, Lagerstand mit Maximum je Rohstoff, Wartegrund je Bauplatz und den Zustand des Händlers. Hub-Ausbau und Händlertausch bleiben Bezahlen am Ort über `input.pay` (B-330, Variante B). Nutzen: B-117 und B-126 können ohne Rechnen im Client zeichnen.

## Beteiligte und Zielgruppen

Entwickler (Client und Server); eigene Session laut `docs/arbeitsweise.md` › Protokoll.

## Anforderungen

- Zustand (s2c): Hub-Stufe und Ausbaukosten je Hub, Lagerstand und Maximum je Rohstoff, Wartegrund je Bauplatz (Bauer fehlt, Material fehlt, Gefahr), Händler (anwesend, Angebot, Rest der Zeit); Berufe der Bürger, soweit B-123 sie nicht schon liefert.
- Eingaben (c2s): keine neuen; Hub ausbauen, beim Händler tauschen und Berufswahl bleiben `input.pay` am Ort (B-330, Entscheidung 🧑 2026-10-06).
- Protokollversion erhöhen, Testdaten in `testdata/protocol/`.
- Snapshot-Größe messen (`engine/sim/island_bench_test.go`), Delta-tauglich (`engine/net/delta.go`).

## Nicht-Ziele

Darstellung (B-117, B-126), Simulation (B-112, B-121), Feedback-Events (B-140), Skills und Schlag (B-123).

## Regeln und Einschränkungen

`docs/protocol.md` ist die Quelle; Protokoll und beide Enden in einer Session. Der Client rechnet nichts (`src/scenes/noSim.test.ts`). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; 2+ Spieler.

## Beispiele

`snap` nennt je Bauplatz `wait:"material"` und im Lager `stone:{n:40,max:200}` → der Client zeigt den Wartegrund ohne eigene Rechnung.

## Ausnahme- und Fehlerfälle

nicht relevant: keine neuen Eingaben (B-330). Am Ort ohne Händler passiert nichts, ohne Material wartet der Hub-Ausbau (Sim, W1).

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt die neuen Felder (Hub-Stufe, Lager, Wartegrund, Händler); `testdata/protocol/` hat Beispiele; beide Enden parsen sie (Tests).
- **AC-02** verworfen (B-330, Entscheidung 🧑 2026-10-06): ~~Der Server lehnt ungültige Eingaben (Hub-Ausbau ohne Material, Tausch ohne Händler, unbekannter Beruf) mit `bad_request` ab (Test in `engine/net/`).~~
- **AC-03** Der Snapshot einer Insel mit Hub-Ausbau, gefülltem Lager und Händler enthält Hub-Stufe, Lagerstand mit Maximum, Wartegrund und Händler-Zustand (Test auf Testdaten).
- **AC-04** Protokollversion erhöht (wegen der neuen Felder); ein älterer Client erhält `version` (Test).
- **AC-05** Bytes je Tick mit 4 Spielern und 3 Stufen vor und nach der Änderung gemessen und in den Notizen festgehalten; `task check:go` grün.

## Offene Fragen

Abgrenzung zu B-123 bei Berufen (wer liefert welches Feld): beim gemeinsamen Planen der Protokoll-Sessions klären.

## Notizen

Aus Plan Phase 2 (W5). Setzt W1 bis W4 und B-123 voraus. Bandbreitenbudget kommt aus B-140.
