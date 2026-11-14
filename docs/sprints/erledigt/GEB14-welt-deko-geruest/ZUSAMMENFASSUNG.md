# GEB14 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Wald und Höhle dichter, Gerüst bei Wartenden, Knoten-Markierer im Snapshot
- **Projekt:** GEB

## Ziel

Wald dichter, Höhle mit Büschen, Gerüst bei jedem wartenden Arbeiter, Markierer im Snapshot. Am Ende sichtbar: Level im Browser, Tests grün.

## Ergebnis

- 2026-10-23, Claude autonom: AC-01 bis AC-04 umgesetzt, Review (Sonnet, Spielstand und Golden) ohne schwere Befunde; `deco_test.go` nachgeschärft. Golden neu geschrieben (mehr Bäume verschieben den RNG-Strom), `campaign-ausbau` Eingaben angepasst. - Offen: `sim_test`-Holz-Balancing zu B-531 nicht gefahren (Balancing zurückgestellt); Anzeige der Büsche im Browser ungeprüft.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Wald hat mehr Bäume je Abschnitt, Höhle Deko-Büsche (B-531/AC). |
| AC-02 | – | `hubLookOf` zeigt das Gerüst bei jedem `waitingWorker` (B-572/AC). |
| AC-03 | – | Der Snapshot nennt den Markierer eines Knotens, Protokoll nachgezogen (B-532/AC). |
| AC-04 | – | Golden-Änderungen sind begründet. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-531 | erledigt | AC-01 – | Wald hat mehr Bäume je Abschnitt, Höhle bekommt Deko-Büsche |
| B-532 | erledigt | AC-03 – | Snapshot nennt, wer einen Knoten markiert hat |
| B-572 | erledigt | AC-02 – | Baugerüst steht immer bei waitingWorker, auch bei Gefahr |
