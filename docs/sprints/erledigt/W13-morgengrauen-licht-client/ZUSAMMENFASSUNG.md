# W13 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Morgengrauen im Client und Welt-Betrachter, dunklere Nacht mit Fackeln und Lagerfeuer als Licht
- **Projekt:** WRT

## Ziel

Spiel und Welt-Betrachter zeigen das Morgengrauen als eigene Phase, die Nacht ist deutlich dunkler und Lichtquellen hellen ihre Umgebung auf. Am Ende sichtbar: dunkle Nacht mit Lichtkegeln am TV, von 🧑 abgenommen.

## Ergebnis

2026-10-25, Claude (W13.3): `task check`, `task check:go` und das Studio-Vitest grün; `check:race` lokal übersprungen (kein C-Compiler). AC-01 erfüllt durch W13.1 (Nacht 0,3, Morgengrauen linear, Lagerfeuer leuchtet), AC-02 durch W13.2 (eine Darstellung für Spiel und Studio). Neues Ticket B-622: Fackeln als Spielobjekt. 2026-10-31: AC-03 von Ralf (🧑) am TV abgenommen (Helligkeit und Licht mit zwei Spielern), im Chat bestätigt; Claude hat die Prüfung nicht selbst gesehen. Alle Kriterien erfüllt, Sprint erledigt. Gestapelt auf W12 (#479).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Der Client zeichnet `dawn` als eigene Phase, Nacht und Morgengrauen sind dunkler, Fackeln … |
| AC-02 | geprüft | Der Welt-Betrachter setzt Morgengrauen als Phase `dawn` und nutzt dieselbe Darstellung ohn … |
| AC-03 | – | 🧑 hat Helligkeit und Licht am TV mit zwei Spielern abgenommen (B-472/AC-02). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-472 | erledigt | AC-01 erfüllt, AC-02 geprüft, AC-03 – | Nacht und Morgengrauen sind dunkler, Fackeln und Lagerfeuer machen Licht |
| B-470 | erledigt | AC-01 erfüllt, AC-02 geprüft | Der Tageszyklus hat eine eigene Phase Morgengrauen (Q65, 3:1:2:1) |
