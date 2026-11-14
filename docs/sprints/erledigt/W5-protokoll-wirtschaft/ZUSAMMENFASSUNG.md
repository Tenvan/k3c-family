# W5 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Protokoll für Berufe, Händler, Lager und Hub-Stufe
- **Projekt:** –

## Ziel

Der Client erhält Hub-Stufe, Lagerstand, Wartegrund und Händler-Zustand; die Protokollversion ist dafür erhöht. Hub-Ausbau, Tausch beim Händler und Berufswahl bleiben Bezahlen am Ort (`input.pay`, A halten), es gibt dafür keine neuen Eingaben (B-330, Variante B). Am Ende sichtbar: `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün.

## Ergebnis

- 2026-10-06 (W5.3, autonom): AC-01, AC-03, AC-06 (W5.1: `TestWirtschaftZustand`, `TestFormWieBeispiele`, `clientWirtschaft.test.ts`), AC-04, AC-05 (W5.2: `TestHandschlagFalsch` mit v4, Bytes vorher/nachher in `docs/protocol.md`) mit Nachweis; AC-02 verworfen (B-330, Variante B). - Review des Diffs: keine schweren Befunde, keine behoben; `engine/sim/` unverändert, Version in Server, Client, Dokument und Beispielen 5. Die Änderung an `src/online/clientSkills.test.ts` (nur Version 4 → 5, außerhalb der erlaubten Dateien) ist als zwingende Folge des Versionssprungs abgenommen. - Neue Tickets: kein …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `docs/protocol.md` beschreibt die neuen Felder, `testdata/protocol/` hat Beispiele, beide … |
| AC-02 | verworfen | verworfen (B-330, Entscheidung 🧑 2026-10-06): ~~Der Server lehnt ungültige Eingaben mit `b … |
| AC-03 | – | Der Snapshot enthält Hub-Stufe, Lagerstand mit Maximum, Wartegrund und Händler-Zustand (Te … |
| AC-04 | – | Protokollversion erhöht (wegen der Felder aus W5.1), ältere Clients erhalten `version` (Te … |
| AC-05 | – | Bytes je Tick mit 4 Spielern und 3 Stufen gemessen und notiert, `task check:go` grün (B-15 … |
| AC-06 | – | Die Ereignisse `revived` (Wiederbeleben durch einen Mitspieler, Q62), `disarmed` (Bürger v … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-153 | erledigt | AC-01 –, AC-02 verworfen, AC-03 –, AC-04 –, AC-05 – | Das Protokoll kennt Berufe, Händler, Lagerstand, Hub-Stufe und Wartegrund |
| B-283 | erledigt | AC-01 –, AC-02 verworfen | Das Protokoll kennt Beruf ausbilden, Tauschen, Berufe der Bürger und Grabstein/W … |
