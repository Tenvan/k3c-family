# AZ1.4 · Workshop: Abnahme am PC

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** sprint/az1
- **Abhängig von:** AZ1.3
- **Tickets:** B-371, B-333
- **Kriterien:** AC-03, AC-04

## Ziel

🧑 nimmt am PC ab: zwei Spieler an einer Tastatur sehen je ihre Tasten, und eine Pause über `/dm` zeigt in jeder Zelle „Pausiert“ bei stillstehenden Figuren.

## Kontext

Stand nach AZ1.3 (PR des Sprints offen oder gemergt). Server im Dev-Mode (sonst kein `devPaused`), `game.html` am PC, `/dm` in einem zweiten Tab.

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Code ändern; Befunde werden Tickets.

## Schritte

1. Zwei Spieler an einer Tastatur beitreten (Leertaste, Enter). Spieler 2 an einen Bauplatz: Hinweis, Skill-Leiste und Skill-Menü (Ziffernblock 5) nennen seine Tasten.
2. `/dm` → „⏸ Pause“: in beiden Zellen „Pausiert“, alle Figuren stehen. „▶ Weiter“: Anzeige weg, Figuren laufen.
3. Ergebnis eintragen; ist dies die letzte offene Session, Sprint abschließen.

## Fertig, wenn

- [x] AC-03: 🧑 sieht die Pause-Anzeige in jeder Zelle und sie verschwindet nach „Weiter“.
- [x] AC-04: 🧑 sieht die Figuren während der Pause stillstehen und danach wieder laufen.

## Prüfen

Beobachtung am PC durch 🧑.

## Ergebnis

Beobachtung 🧑 am PC, 2 Spieler an einer Tastatur, Server im Dev-Mode, `/dm` in einem zweiten Tab (2026-10-11); Agent hat die Dienste gestartet und protokolliert.

- **Schritt 1** (B-371/AC-02, Gegenprobe zu AZ1.1): ok. Hinweis, Skill-Leiste und Skill-Menü von Spieler 2 nennen seine Tasten.
- **AC-03** (B-333/AC-01): ok. „⏸ Pause“ zeigt in beiden Zellen „Pausiert“; nach „▶ Weiter“ ist die Anzeige weg.
- **AC-04** (B-333/AC-02): ok. Die Figuren stehen während der Pause still und laufen danach wieder.
- **Zusatzprobe** (Review-Befund AZ1.3): ok. Spiel in der Pause über das Menü verlassen, neuen Raum betreten: Die Figuren laufen (Shutdown-Handler hebt `pauseAll` auf).

**Kriterien:** AC-03 und AC-04 erfüllt (🧑, PC). Abweichungen: keine. Neue Tickets: keine. B-371 und B-333 erledigt.
