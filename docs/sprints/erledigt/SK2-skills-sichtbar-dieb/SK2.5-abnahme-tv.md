# SK2.5 · Abnahme am TV

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** –
- **Abhängig von:** SK2.3
- **Tickets:** B-503
- **Kriterien:** AC-03

## Ziel

🧑 sieht am TV mit zwei Spielern die Ringe beim Wirken und Lernen und nimmt AC-03 ab.

## Kontext

Umsetzung SK2.3. Zwei Spieler, je ein Skill wirken (Tank Taunt, Zauberer Fireball), einen Passiven lernen.

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Neue Effekte.

## Schritte

1. Spiel am TV starten, zwei Spieler.
2. Skills wirken, Passiven lernen, Ringe beobachten.
3. Ergebnis eintragen, Befunde als Ticket.

## Fertig, wenn

- [x] AC-03 (B-503/AC-02): Effekt je Skill sichtbar, Farbe passt zur Klasse (2026-10-24, PC).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-03 geprüft (Ralf, 2026-10-24, PC mit Tastatur, zwei Spieler): Tank (taunt, shieldBash), Zauberer, Heiler und Dieb (backstab, smokeBomb) zeigen ihren Effekt, Farbe passt; passiven Skill lernen zeigt den Ring, zwei Spieler ohne Durcheinander. Dabei die Regel aus SK3.1 bestätigt: Unsichtbare werden nicht als Ziel gewählt, nehmen keinen Schaden, jeder eigene Schlag oder Skill beendet die Unsichtbarkeit, Backstab-Reichweite 2 Units. Keine Tickets. TV/Xbox nicht geprüft.
