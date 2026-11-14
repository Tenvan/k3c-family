# RL1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Release-Checkliste
- **Projekt:** REL

## Ziel

Ein Abschnitt „Release“ in `docs/arbeitsweise.md` macht jeden Release prüfbar. Am Ende sichtbar: die Liste und ein Probelauf ohne Tag.

## Ergebnis

2026-10-04, Agent (Claude Opus 5.5) in RL1.2, Abschluss auf Anweisung von 🧑 (Chat). AC-01, AC-02, AC-04: Ergebnis RL1.1. AC-03: Probelauf ohne Tag durchgeführt (Ergebnis RL1.2, Stand `origin/develop` 46aa69b); Pi-Pull und Versionszeile auf der Xbox angenommen, Validierung offen. Rot: Dev-Mode im Release-Image an (B-273), `TestRestore` flackert unter Windows (B-274), Overlay standardmäßig an (B-098, K5). Ein Tag wäre damit blockiert. Version: v0.6.1 vorgeschlagen (Doku-Sprint, Patch); gesetzt erst nach Bestätigung durch 🧑 und grüner Checkliste. 2026-10-07, RL1.2: Versionszeile am PC grün (Clien …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `docs/arbeitsweise.md` enthält den Abschnitt „Release“, jeder Punkt nennt Befehl oder Date … |
| AC-02 | – | Die Liste verweist auf Golden amd64 und arm64, `task check:all`, Migration, Dev-Reste, Pi- … |
| AC-03 | – | Ein Probelauf ohne Tag ist durchgeführt und in der Abnahme festgehalten (B-170/AC-03). |
| AC-04 | – | Rhythmus und Auslöser sind von 🧑 beschlossen (B-170/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-170 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 – | Eine Release-Checkliste macht jeden Release prüfbar |
