# F1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Zielkorridore und Bedienungsregeln
- **Projekt:** –

## Ziel

Jede Kennzahl hat einen Zielkorridor als Zahl, und Pause, Schriftgröße, Verbindungsverlust und Sprache sind als Regel mit Zahl beschlossen. Am Ende sichtbar: `docs/rules/zielkorridore.md` und `docs/rules/bedienung.md` mit Datum der Bestätigung durch 🧑.

## Ergebnis

2026-10-03 (F1.5): Doku-Sprint ohne Review, `task check` grün. AC-01 bis AC-07, AC-09: Nachweise in den Ergebnissen von F1.1 bis F1.4 (Workshop F1.4: alle Vorschläge von 🧑 unverändert bestätigt); AC-08: Ergebnis F1.5, B-134, B-135, B-136, B-144, B-145 archiviert. Nichts verschoben, keine behobenen Befunde; neue Tickets aus dem Sprint: B-181 (Latenz-Anzeige), B-185 (Verluste je Welle in `wirtschaft.md`). Version: v0.4.1 vorgeschlagen (Patch: nur Regeln und Planung, keine neue Funktion); gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `docs/rules/zielkorridore.md` existiert mit Kennzahlen, Szenario, Unter- und Obergrenze un … |
| AC-02 | – | 🧑 hat die Zielkorridore bestätigt, Datum im Kopf der Datei; `docs/game-design.md` verweist … |
| AC-03 | geprüft | Abschnitt „Pause“ in `docs/rules/bedienung.md` beantwortet Q01 mit Zahl, `docs/game-design … |
| AC-04 | geprüft | Abschnitt „Schriftgröße“ nennt je Layout 1 bis 4 Spieler eine Mindestgröße in px und das P … |
| AC-05 | geprüft | Abschnitt „Verbindung“ nennt Reservierungszeit, Verhalten des Monarchen und Latenz-Ziel in … |
| AC-06 | geprüft | Abschnitt „Sprache“ hält die Entscheidung mit Datum fest (B-145/AC-01). |
| AC-07 | geprüft | Der Abschnitt „Reittier“ in `docs/rules/monarch.md` nennt Standard-Reittier und Faktoren ( … |
| AC-08 | geprüft | `task check` ist grün, und die Tickets B-134, B-135, B-136, B-144, B-145 sind nach `docs/b … |
| AC-09 | – | 🧑 hat die Vorschläge in `bedienung.md` und im Abschnitt Reittier bestätigt, Datum im Kopf … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-134 | erledigt | AC-01 geprüft, AC-02 –, AC-08 geprüft | Jede Kennzahl des Spiels hat einen Zielkorridor als Zahl |
| B-135 | erledigt | AC-03 geprüft, AC-08 geprüft, AC-09 – | Pause im gemeinsamen Raum ist als Regel festgelegt |
| B-136 | erledigt | AC-04 geprüft, AC-08 geprüft, AC-09 – | Die Mindest-Schriftgröße je Split-Viertel ist festgelegt |
| B-144 | erledigt | AC-05 geprüft, AC-08 geprüft, AC-09 – | Verbindungsverlust und Eingabe-Latenz haben eine Regel mit Zahlen |
| B-145 | erledigt | AC-06 geprüft, AC-08 geprüft | Das Spiel bleibt dauerhaft deutschsprachig, oder die Lokalisierung ist geplant |
