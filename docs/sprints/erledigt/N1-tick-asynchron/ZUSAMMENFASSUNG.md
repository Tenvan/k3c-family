# N1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Raum-Tick im Budget: Versand und Speichern asynchron
- **Projekt:** –

## Ziel

Unter der Raum-Sperre laufen nur Simulation und ein Zustandsaufbau je Stufe; Versand und Speichern sind asynchron. Am Ende sichtbar: Benchmark-Zahlen vorher/nachher im Session-Ergebnis, weniger `🐢 Tick zu langsam` im Log.

## Ergebnis

2026-10-04, Agent (Claude Opus 5.5) in N1.3, Review mit unabhängigem Reviewer. AC-01, AC-02, AC-04: Ergebnis N1.1; AC-03: Ergebnis N1.2; AC-05: Ergebnis N1.3 (`-race` nur in der CI, lokal kein cgo). Behoben: Panic in Schreib- und Speicher-Goroutine, `ack` beim Senden statt beim Aufbau, `closeCrashed` ohne Flush, Testlücken (siehe N1.3). Neu: B-280; B-278 ergänzt. Offen für 🧑: Messung am Pi (B-042). B-276 archiviert. Version: v0.6.1 vorgeschlagen (Patch: Leistung im Server, Protokoll unverändert); gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | B-276/AC-01 |
| AC-02 | – | B-276/AC-02 |
| AC-03 | – | B-276/AC-03 |
| AC-04 | geprüft | B-276/AC-04 |
| AC-05 | geprüft | B-276/AC-05 |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-276 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 geprüft, AC-05 geprüft | Der Raum-Tick bleibt im Budget, Kodierung und Speichern laufen außerhalb der Rau … |
