# GR5 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Juice: Treffer, Screenshake, Münzen
- **Projekt:** –

## Ziel

Treffer, Münzen, Tod und Bauen haben sichtbare Effekte, Screenshake und Blitz sind abschaltbar. Am Ende sichtbar: Effekte am TV und eine Option, die sie ausschaltet.

## Ergebnis

- 2026-10-04, Review GR5.3: AC-01 (GR5.1: `effects.test.ts`, 7 Event-Arten), AC-03 (`effectFor` rein, `noSim.test.ts` grün), AC-04, AC-05, AC-06, AC-07 (GR5.2: Tests `shakeCell`, `flashAllowed` im 16-ms-Takt, `rumblePad`; `task check` und `task check:go` grün) sind umgesetzt und im Diff geprüft. - AC-02 (Schalter per Test; Aussehen und Blitz-Eindruck) sowie die Sicht auf AC-01 und AC-04: angenommen, Validierung offen (🧑, TV; im Fahrplan unter „Offen am Gerät“); Vibration auf der Xbox angenommen. Nichts davon im Browser gesehen. - Keine schweren Befunde: Effekte nutzen `frame.state.events` (nic …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Jedes Feedback-Event aus der Liste (Treffer, Kill, Münze aufheben, Münze geben, Bau fertig … |
| AC-02 | – | Mit Screenshake und Blitz „aus“ treten beide nicht auf (B-164/AC-02). |
| AC-03 | – | Die Effekt-Auslösung ändert keinen Spielzustand, `noSim.test.ts` bleibt grün (B-164/AC-03) … |
| AC-04 | geprüft | Screenshake wirkt im Split-Screen nur in der Kamera des betroffenen Spielers (B-164/AC-04) … |
| AC-05 | geprüft | Die Blitzfrequenz liegt bei höchstens 3 pro Sekunde (B-164/AC-05). |
| AC-06 | geprüft | Auf einem Controller ohne Vibration läuft das Spiel ohne Fehler weiter (B-164/AC-06). |
| AC-07 | – | `task check` ist grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-164 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 geprüft, AC-05 geprüft, AC-06 geprüft | Treffer, Münzen und Bauen haben sichtbare Rückmeldung, Screenshake und Blitze si … |
