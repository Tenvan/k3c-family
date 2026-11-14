# BAL9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Meilenstein 1: Titan über acht Seeds, castleHeld im Fenster, Bot-Performance
- **Projekt:** BAL

## Ziel

Der Meilenstein-Lauf wird über 8 Seeds sauber beurteilt, `castleHeld` misst nur Nacht 1 bis 5, der Bot-Anteil ist schneller. Am Ende sichtbar: Bericht mit Pass je Kennzahl oder begründet neu gefassten Zielen.

## Ergebnis

2026-11-05: AC-01 (BAL9.3: run-18, 8 Seeds, `holz` und `schiff` neu gefasst, `frozen`, Baseline M5), AC-02 (BAL9.1), AC-03 (BAL9.2) nachgewiesen; `task check`, `check:go`, `check:dev`, `check:race` grün (`check:race` lokal ohne gcc übersprungen). Offen/ungeprüft: Meilenstein weiterhin rot (Endboss 0 von 8 Seeds), Ziele vorläufig gelockert bis zum Ressourcen-Balancing (B-698). Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Der Meilenstein-Lauf wird über 8 Seeds gegen `M4` beurteilt, `holz` und `schiff` grün oder … |
| AC-02 | geprüft | `castleHeld` wertet nur Nacht 1 bis 5 (`B-657/AC-01` bis `AC-03`). |
| AC-03 | geprüft | `wanted` und `nodeFor` ohne Map-Zugriffe je Tick, Lauf byte-gleich (`B-654/AC-01`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-656 | erledigt | AC-01 geprüft | Meilenstein 1 coop2 standard: Titan über mehrere Seeds, holz und schiff grün |
| B-657 | erledigt | AC-02 geprüft | Die Kennzahl castleHeld misst Nacht 1 bis 5 über den ganzen Lauf, nicht nur über … |
| B-654 | erledigt | AC-03 geprüft | Bot-Performance: bosses.json je Tick neu geparst, nodeFor/wanted mit Map-Zugriff … |
