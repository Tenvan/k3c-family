# PL1.5 · Abnahme am PC und an der Xbox

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** Mensch
- **Umgebung:** live
- **Branch:** pl1/5-abnahme
- **Abhängig von:** PL1.4
- **Tickets:** B-292, B-316, B-195, B-215
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

🧑 hat die manuellen Kriterien am PC und an der Xbox geprüft; die Ergebnisse stehen hier, der Sprint kann danach abgeschlossen werden.

## Kontext

- Geprüft wird der Stand von `develop` nach dem Merge des Sprint-PRs (Main Checkout, Heimnetz-Server).
- B-292/AC-02: Server mit vorhandenem Stand `familie`; „Neues Spiel“ erreicht die Lobby ohne Fehlerhinweis, „Spielen“ erzeugt einen Raum (Server-Log `🏰 Raum erstellt`, kein `save_exists`).
- B-316/AC-02: Belegung laut PL1.2 › Ergebnis (Spieler 1 links, Spieler 2 rechts mit Pfeiltasten, Strg rechts, Enter, Ziffernblock).
- B-195/AC-02: Gesten laut PL1.1 › Ergebnis und B-195 › Notizen (heute: RB 3 s halten = Diagnose, LB + RB 3 s halten = Cheat-Dialog); Seite und `?dev=` notieren.
- B-215/AC-02: Sprache in den Optionen auf English, neu laden; Home-Button und Touch-Tasten (Touch per `?touch=1` erzwingbar) zeigen Englisch.

## Erlaubte Dateien

- `docs/sprints/aktiv/PL1-start-tastatur-koop-texte/`, `docs/sprints/erledigt/`, `docs/sprints/README.md`, `docs/backlog/` (Status, Ergebnisse, neue Tickets)

## Nicht-Ziele

Code ändern; Mängel hier beheben (→ Ticket).

## Schritte

1. Am PC: „Neues Spiel“ mit vorhandenem Stand `familie` starten (B-292/AC-02).
2. Am PC: zwei Spieler an einer Tastatur (Leertaste, Enter), beide bewegen und handeln unabhängig (B-316/AC-02).
3. An der Xbox: Debug-Overlay mit dem Controller öffnen und schließen (B-195/AC-02).
4. Sprache auf English, neu laden: Home-Button und Touch-Tasten prüfen (B-215/AC-02).
5. Ergebnis je Punkt schreiben, Mängel als Ticket. Sind alle Kriterien belegt: Tickets archivieren, Sprint nach `docs/sprints/erledigt/`, Fahrplan in `docs/sprints/README.md` anpassen.

## Fertig, wenn

- [ ] AC-01: „Neues Spiel“ startet bei vorhandenem Stand `familie` ohne `save_exists` (B-292/AC-02, Beobachtung 🧑).
- [ ] AC-02: Zwei Spieler spielen am PC an einer Tastatur im Split-Screen unabhängig (B-316/AC-02, Beobachtung 🧑).
- [ ] AC-03: 🧑 öffnet und schließt das Overlay auf der Xbox mit dem Controller (B-195/AC-02).
- [ ] AC-04: Home-Button und Touch-Tasten zeigen nach Sprachwechsel und Neuladen Englisch (B-215/AC-02, Beobachtung 🧑).

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
