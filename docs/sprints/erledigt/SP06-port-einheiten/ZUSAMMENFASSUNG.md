# SP06 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Port II – Gegner, Wellen, Reisen, Kampagne
- **Projekt:** –

## Ziel

Die Go-Simulation kann alles, was die TS-Simulation kann, und jeder Pfad ist gegen TS geprüft. Am Ende sichtbar: `go test -cover ./engine/sim/` ist grün, alle Golden-Läufe stimmen vollständig, auch Nacht, Stufenwechsel und Spielstand, und die Abdeckung liegt bei mindestens 90 %.

## Ergebnis

- 2026-10-01, leichtes Review über `520e39d..main` per unabhängigem Subagent (auf Auftrag von 🧑 im selben Lauf). - Kriterien: AC-01 (SP06.1), AC-02, AC-03 und AC-04 (SP06.2), AC-05 (SP06.3), alle mit Gegentest bzw. Messung belegt. B-043, B-059 und B-074 sind erledigt. - Behobene Befunde: keine (keine schweren). Neues Ticket: B-075 (Golden-Spielstand mit ausgebautem Hub).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Gegner, Wellen, Kampf und Burg-Fall: `sim-forest-nacht` und `sim-cave-aggression` sind in … |
| AC-02 | geprüft | Reisen und Kampagne: `campaign-abstieg` ist in allen Snapshots grün, auch nach dem Laden ( … |
| AC-03 | geprüft | Spielstand Version 1: Go liest den TS-Spielstand aus `campaign-abstieg.json` verlustfrei |
| AC-04 | geprüft | Freie Monarchen: B-059/AC-01 und B-059/AC-02 als Go-Tests grün; die Golden-Läufe bleiben u … |
| AC-05 | geprüft | Abdeckung: `go test -cover ./engine/sim/` meldet mindestens 90 % der Anweisungen (B-074/AC … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-043 | erledigt | AC-02 geprüft | Portierungs-Fallen sind durch Golden-Tests abgedeckt |
| B-074 | erledigt | AC-05 geprüft | Golden-Läufe decken Tragen, Bauen, Bögen, Truhen und Münz-Rückgabe ab |
| B-059 | erledigt | AC-04 geprüft | Nur gesteuerte Monarchen entscheiden über den Stufenwechsel |
