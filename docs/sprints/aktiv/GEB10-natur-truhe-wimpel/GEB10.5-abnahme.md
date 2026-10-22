# GEB10.5 · Abnahme in Vorschau und Spiel mit zwei Spielern, GEB6-Restpunkte

- **Status:** blockiert
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb10/5-abnahme
- **Abhängig von:** GEB10.4
- **Tickets:** B-454, B-456, B-457
- **Kriterien:** AC-04

## Ziel

🧑 hat Vorschau und Spiel mit zwei Spielern abgenommen oder Nachbesserungen als Ticket benannt (AC-04).

## Kontext

- Vorschau: Asset-Studio, Seite Grafik, Reiter Szenen, Szene Gebäude (Baum-Reihe, Höhlen-Flora, `truhe` neben `truhe-geschlossen`).
- Spiel mit zwei Spielern im Split-Screen: Wald, Höhle, Mine.
- Die autonom getroffenen Entscheidungen 1–6 der Sprint-README bestätigt oder ändert 🧑 hier.

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/` (Ergebnis, neue Tickets)

## Nicht-Ziele

Codeänderungen; Nachbesserungen werden Tickets.

## Schritte

Prüfliste für 🧑:

1. **Bäume (B-454/AC-03):** Wald – Bäume etwa ¾ der Szenenhöhe, dicht genug? Maß 11,25 Kacheln und Breiten 2–4 ok? Plantage-Pappel hinter der Farm ok?
2. **Höhlen-Flora:** Höhle – Wurzeln von der Decke bis in den Boden statt Bäumen; Höhlenbusch in der Vorschau; Mine wie Höhle.
3. **Truhe:** liegende Truhe geschlossen, beim Aufheben kurz offen; Vorschau zeigt beide.
4. **Wimpel (B-456/AC-02):** Spieler 1 und 2 markieren je einen Baum, Fels, eine Ader: Wimpel über dem Objekt, Farben unterscheidbar.
5. **Ausgang-Beschriftung (B-457/AC-01):** „Tiefer“/„Gesperrt“ lesbar über dem Ausgang.
6. **Split-Screen (B-457/AC-02):** dieselben Objekte in beiden Hälften, nichts flackert oder fehlt.
7. **Geschmack (B-457/AC-03):** Mine-Fels, Feuerstelle, Ausgang-Platz, Kristall-Ader bestätigt oder Ticket.

## Fertig, wenn

- [ ] AC-04: Punkte 1–7 von 🧑 abgenommen oder als Ticket benannt; Ergebnis mit Datum.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Blockiert: Abnahme durch Ralf (AC-04), Prüfliste unter Schritte. Kein Agent setzt diese Session um.
