# BS3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Werte auf der Münzskala: Meilenstein-Lauf coop2 standard läuft durch
- **Projekt:** BAS

## Ziel

**Meilenstein 1 (Fassung vom 2026-10-28, 🧑):** Alle regulären Preise, Einnahmen und Ziele stehen auf der Münzskala 2–30, die Schiffskosten sind in `data/`, und ein Balancing-Lauf `coop2`, Welt standard, Seed `k3c-dev` läuft mit den neuen Werten **bis zum Sieg über den Splitter-Titan** (Miniboss der Kristallhöhle) durch. Den Endboss prüft vorerst der Stufen-Test (`task balance:fullgame`, mit Hilfen …

## Ergebnis

2026-10-28 (Claude, autonom): AC-01 bis AC-05 nachgewiesen (BS3.1–BS3.6, Ergebnisse dort); Meilenstein 1 in der Fassung „bis Splitter-Titan“ (🧑, Revision 2) erreicht: k3c-dev Tag 26, reproduzierbar. Befunde: B-645, B-650, B-651, B-652 (alle mit Notizen). Neue Tickets: B-649 bis B-654. `check:race` lokal übersprungen (kein C-Compiler).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Reguläre Goldpreise in `data/` liegen in 2…30, Mauer Stufe 1 = 2; ein Daten-Test prüft das … |
| AC-02 | geprüft | Einnahmen, Beute, Markieren und Ziele sind auf die Skala umgerechnet, Zielkorridore in `ba … |
| AC-03 | geprüft | Schiffskosten je Abschnitt stehen in `data/`, Daten-Test prüft Skala und Schema (B-638/AC- … |
| AC-04 | geprüft | Der Bericht nennt je Schiff-Abschnitt „bezahlbar ab Tag“ mit Ampel (B-638/AC-02). |
| AC-05 | geprüft | Meilenstein (Fassung 2026-10-28): Lauf `coop2`, Welt standard, Seed `k3c-dev` besiegt den … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-635 | eingeplant | AC-01 geprüft, AC-05 geprüft | Münzpreise liegen auf einer kleinen Skala von 2 bis 30 Gold |
| B-638 | verworfen | AC-03 geprüft, AC-04 geprüft | Das Balancing misst die Ressourcen für das Boot |
