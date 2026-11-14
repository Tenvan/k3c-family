# WZ40 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** check:diff prüft nur den eigenen Diff, betroffene Pakete und parallel
- **Projekt:** WZG

## Ziel

B-730 ist belegt und geschlossen: Doku-Commits prüfen in Sekunden, Go-Änderungen nur betroffene Pakete. Am Ende sichtbar: `task check:diff` bei reinem Doku-Diff endet nach rund 8 s mit `· nur Doku`.

## Ergebnis

–

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Doku-Diff: `check:diff` unter 60 s, nur Planungs- und Regel-Tests (`B-730/AC-01`). |
| AC-02 | geprüft | Go-Änderung: nur Paket und Abnehmer, Ausgabe `🚦 check:diff` nennt sie (`B-730/AC-02`). |
| AC-03 | – | `docs/arbeitsweise.md` › Tempo im Lauf beschreibt den Takt (`B-730/AC-03`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-730 | offen | AC-01 geprüft, AC-02 geprüft, AC-03 – | check:diff prüft nur den eigenen Diff, betroffene Pakete und parallel |
