# WZ35 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Workbench-Nachträge: Ticket-Nummern, Lauf-IDs, schmale Fenster, Auswertung
- **Projekt:** WZG

## Ziel

Die Workbench bleibt bei parallelen Aufrufen, Neustarts, schmalen Fenstern und ungewöhnlichen Berichten stabil. Am Ende sichtbar: Tests grün, Web-Client im halben Fenster ohne waagrechten Überlauf.

## Ergebnis

2026-10-31: AC-01 bis AC-05 belegt (WZ35.1 bis WZ35.3), `task check` und `task check:go` grün, `check:race` lokal ohne gcc übersprungen. Messung bei 600 px beobachtet, Ralf hat abgenommen. Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Parallel angelegte Tickets haben nie dieselbe Nummer (`B-615/AC-01`). |
| AC-02 | geprüft | Lauf-IDs und ihre Ordner unter `reports/` sind über Neustarts eindeutig (`B-616/AC-01`). |
| AC-03 | offen | Der Kopf der Workbench läuft in schmalen Fenstern ohne waagrechten Überlauf (`B-668/AC-01` … |
| AC-04 | geprüft | Die Auswertung stürzt bei Zielen ohne Grenzen nicht ab (`B-669/AC-01`). |
| AC-05 | geprüft | Übernehmen aus dem Vorrat ist atomar, die Abspielleiste spielt nie zwei Töne, die Download … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-615 | erledigt | AC-01 geprüft | Gleichzeitiges Anlegen von Tickets vergibt nie dieselbe Nummer |
| B-616 | erledigt | AC-02 geprüft | Lauf-IDs der Balancing-Seite bleiben über einen Neustart der Workbench eindeutig |
| B-668 | erledigt | AC-03 offen | Der Kopf der Workbench bricht in schmalen Fenstern um, statt die Seite waagrecht … |
| B-669 | erledigt | AC-04 geprüft | Die Auswertung eines Berichts stürzt nicht ab, wenn ein Ziel weder Unter- noch O … |
| B-678 | erledigt | AC-05 geprüft | Übernehmen aus dem Vorrat ist gegen parallele Aufrufe gesichert, und Vorschauen … |
