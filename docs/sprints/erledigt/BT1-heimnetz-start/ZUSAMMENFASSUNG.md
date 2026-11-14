# BT1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Server im Heimnetz finden, Windows-Starter, Start mit Seed
- **Projekt:** REL

## Ziel

Der Server startet am PC per Doppelklick, ein Seed startet per URL. Am Ende sichtbar: Doppelklick startet den Server am PC; URL startet Seed und Tiefe. (Das Finden des Servers ohne IP per QR-Code für das Handy entfällt seit 2026-10-11, B-040 ist verworfen.)

## Ergebnis

- 2026-10-18, Review BT1.3 (getrennter Reviewer): AC-02 bis AC-04 am Diff belegt (BT1.1: `create.seed`, `?seed=`/`?depth=`, Entscheidung 001; BT1.2: `tools/k3c-starter`); AC-01 entfällt (B-040 verworfen). - Behoben: Der Client verwarf ungültige Tiefe und zu langen Seed still, die Spec nennt eine Fehlermeldung des Servers → Client reicht beides unverändert an `create` (`bad_request`). Beim Starter schon vorher: Job-Objekt, sauberes Beenden, `stopping`-Flag (Review BT1.2). - B-041/AC-02 (Doppelklick am PC, Port belegt): `angenommen, Validierung offen` (Abnahme durch 🧑, Agent hat die EXE nur auf …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | entfällt | Entfällt (Beschluss 🧑 2026-10-11, B-040 verworfen: QR-Code und Handy); ehemals „Geräte fin … |
| AC-02 | – | Der Windows-Starter `tools/k3c-starter` existiert und läuft per Doppelklick (B-041/AC-01, … |
| AC-03 | – | Ein neues Spiel startet per URL mit eigenem Seed und gewählter Tiefe (B-095/AC-01, B-095/A … |
| AC-04 | – | Die Wahl der Go-Standardbibliothek ist dort festgehalten, wo B-001 auf sie verweist (B-048 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-041 | eingeplant | AC-02 – | Wails-Starter für Windows existiert |
| B-095 | erledigt | AC-03 – | Ein neues Spiel startet per URL mit eigenem Seed und gewählter Tiefe |
| B-048 | erledigt | AC-04 – | Die Wahl der Go-Standardbibliothek ist dort festgehalten, wo B-001 auf sie verwe … |
