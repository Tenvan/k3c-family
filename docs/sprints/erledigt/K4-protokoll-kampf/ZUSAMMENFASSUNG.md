# K4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Protokoll für Bosse, Events und Inselwechsel
- **Projekt:** KMP

## Ziel

Der Client erhält Boss-HP, Phase, Warnkreis, Event und Inselwechsel-Zustand; die Wechsel-Bestätigung ist serverseitig geprüft. Am Ende sichtbar: `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün.

## Ergebnis

2026-10-10, Review K4.3: AC-01 bis AC-04, AC-06 und AC-07 umgesetzt und geprüft (Ergebnisse K4.1a, K4.1, K4.2). AC-05 verschoben → B-386: `snap` Ø 10,2 KB und `delta` Ø 913 B überschreiten die Grenze aus Q08 für den Kampf-Zustand; 🧑 entscheidet, ob Q08 den ganzen Zustand umfasst. B-154 bleibt offen. `task check` und `task check:go` grün (Shell, B-275). Behoben: `docs/protocol.md` › Kampf nannte noch Version 5 und eine Wechsel-Eingabe. Neu: B-385 (`islandSwitch`-Ereignis geht beim Inseltausch verloren). B-080 bleibt offen (AC-01 nicht im Sprint). Version: v0.16.0 vorgeschlagen (Minor: Protokoll …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `docs/protocol.md` beschreibt die neuen Felder, `testdata/protocol/` hat Beispiele, beide … |
| AC-02 | – | Der Raum tauscht die Insel erst, wenn nach dem Sieg über den Endboss alle lebenden Spieler … |
| AC-03 | – | Der Snapshot enthält Boss-HP, Phase, Warnkreis und das aktive Event mit Restzeit (Test auf … |
| AC-04 | – | Protokollversion erhöht, ältere Clients erhalten `version` (Test) (B-154/AC-04). |
| AC-05 | verschoben | Bytes je Tick in einer Bosswelle mit 4 Spielern gemessen und notiert, höchstens 200 Byte j … |
| AC-06 | – | Eine Dev-Aktion wechselt den Schwierigkeitsgrad eines laufenden Raums ab der nächsten Well … |
| AC-07 | – | Die Welt stellt Endboss-Phase, Warnkreis, aktives Event mit Restzeit und Inselwechsel als … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-154 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 verschoben | Das Protokoll kennt Bosse, Phasen, Events und den Inselwechsel |
| B-080 | erledigt | AC-06 – | Dev-Tasten (Gold, Stufe, Neustart) wirken über den Server |
| B-383 | erledigt | AC-07 – | Die Welt stellt Boss-Phase, Warnkreis, Event-Restzeit und Inselwechsel für das P … |
| B-345 | erledigt | AC-02 – | Der Raum tauscht die Insel bei `SwitchReady` |
