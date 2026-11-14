# US2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Spiel lädt Natur, Objekte, Münzen, Icons und Reittiere als Papierschnitt
- **Projekt:** PAP

## Ziel

Das Spiel lädt Natur, Objekte, Münzen und Icons aus den MG8-Gruppen; der berittene Monarch nutzt das Reittier aus `data/monarch.json` (Zuordnung nach `reittiere/*`), wenn das Monarch-Asset eine Animation dafür hat, sonst das Pferd.

## Ergebnis

2026-11-07, autonom (Claude). AC-01 US2.1 und US2.3, AC-02 US2.2, AC-03 US2.3 (`task check:diff` grün, Review ohne offene wichtige Befunde). Behoben aus dem Review: Befunde 1, 2, 3, 5, 7; zurückgestellt D-US2-04. Defaults D-US2-01 bis D-US2-04 in B-701. Neue Tickets: keine. Sichtprüfung in PS1; weitere Reittiere brauchen Gangarten im Monarch-Asset (MG1, D-US2-03).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Kein Lade-Pfad für Natur, Objekte, Münzen und Icons zeigt auf alte Packs oder `gebaeude/*- … |
| AC-02 | geprüft | Das Reittier aus `data/monarch.json` wählt die Monarch-Animation für `reittiere/<id>`; feh … |
| AC-03 | – | `task check:diff` grün, Review ohne offene wichtige Befunde. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
