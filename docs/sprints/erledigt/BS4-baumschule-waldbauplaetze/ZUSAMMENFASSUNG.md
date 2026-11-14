# BS4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Baumschule auf gerodeten Waldbauplätzen
- **Projekt:** BAS

## Ziel

Die Baumschule ersetzt die Farm im Server: Der Generator legt Baumschul-Plätze in Wald-Chunks an, kaufbar erst nach dem Roden; das Protokoll trägt den Platz-Zustand. Werte und Bots folgen in BS8 (Meilenstein 2). **Am Ende sichtbar:** Generator- und Sim-Tests grün, Protokoll-Golden aktualisiert.

## Ergebnis

2026-10-28 (Claude, autonom): AC-01 bis AC-04 nachgewiesen (BS4.1), Review ohne schwere Befunde (BS4.2). Neue Tickets: keine. Offen: Bots (BS8), Darstellung „erst roden“ (BS6).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `farm` heißt in Daten und Go-Code `nursery`, Golden begründet erneuert (B-636/AC-02, Serve … |
| AC-02 | geprüft | Generator-Test: Baumschul-Plätze mit Bäumen in Wald-Chunks, deterministisch (B-636/AC-03). |
| AC-03 | geprüft | Sim-Test: Kauf erst nach dem Roden, Plantage wächst (B-636/AC-04). |
| AC-04 | geprüft | Protokoll trägt Platz-Zustand „erst roden“, Golden `testdata/protocol/` aktualisiert. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-636 | eingeplant | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft | Die Farm heißt Baumschule und steht auf gerodeten Waldbauplätzen |
