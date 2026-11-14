# LT1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Lasttest-Werkzeug
- **Projekt:** TST

## Ziel

Ein Befehl `task load` misst Tick-Dauer und CPU gegen das Ziel und bewertet es. Am Ende sichtbar: Ein Messlauf am Pi mit Tabelle und Bewertung („erreicht“, „knapp“, „verfehlt“).

## Ergebnis

- 2026-10-04, Review LT1.4: AC-01, AC-02, AC-05 (LT1.1) sowie AC-03, AC-04 (LT1.2) sind mit Tests nachgewiesen (`TestLaufSammeltTicksUndRaeumtAuf`, `TestGleicherSeedGleicheEingaben`, `TestBewertungUndExitCode`, `TestStatusNenntCPU`); AC-05 mit der Abweichung B-204 (Test-Räume schließen erst nach der Leer-Frist). - AC-06: angenommen, Validierung offen (LT1.3, 🧑 am Pi; B-042 bleibt eingeplant bis zur Messung, angenommen laut Messung 2026-10-03: Nacht 10,2 bis 10,3 ms, Ziel < 10 ms). - Ein Befund behoben (SRV): Der Poller konnte nach dem Ende des Laufs noch Proben anhängen, während der Bericht la …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Das Werkzeug erzeugt gegen einen In-Prozess-Server einen Bericht mit Tick-Reihe je Raum (B … |
| AC-02 | geprüft | Gleicher Seed ergibt dieselbe Eingabefolge (B-175/AC-02). |
| AC-03 | geprüft | Bewertung und Exit-Code folgen den drei Stufen (B-175/AC-03). |
| AC-04 | geprüft | `/api/status` liefert `cpu`, wo die Quelle existiert (B-175/AC-04). |
| AC-05 | geprüft | Keine Test-Räume bleiben offen, kein Token in der Ausgabe (B-175/AC-05). |
| AC-06 | – | Der Messlauf am Pi ist bewertet; B-042 ist archiviert (B-175/AC-06). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-175 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 – | Ein Lasttest-Werkzeug misst Tick-Dauer und CPU gegen das Pi-Ziel |
| B-042 | verworfen | AC-06 – | Pi-Modell und Leistungsziel sind festgelegt |
