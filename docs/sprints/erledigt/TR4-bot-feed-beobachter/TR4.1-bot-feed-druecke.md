# TR4.1 · Bot-Eingabe verliert keine kurzen Drücke und hält bei stummem Feed an

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** offline
- **Branch:** tr4/1-bot-feed-druecke
- **Abhängig von:** –
- **Tickets:** B-354
- **Kriterien:** AC-01

## Ziel

Ein bis zwei Sätze: welches Ergebnis nach dieser Session auf `develop` liegt.

## Kontext

Alles, was ein Agent **ohne Vorwissen** braucht: Stand des Codes, relevante Dateien mit Pfad, Entscheidungen
(`docs/decisions/…`), Fallstricke. Keine Verweise auf Chat-Verläufe.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `pfad/…`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Konkrete, nummerierte Schritte in sinnvoller Reihenfolge.

## Fertig, wenn

- [ ] AC-01: Jeder Punkt ist prüfbar: ein Befehl mit erwartetem Ergebnis, ein Test, eine Datei, eine Beobachtung.
- [ ] Punkte ohne Kriterium sind technische Voraussetzungen (z. B. CI grün).

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01: umgesetzt. `BotFeed.takePressed` behält einen Druck auch zwischen zwei Frames. Nach 1000 ms ohne Nachricht liefert der Slot keine Eingabe mehr; einmal geloggt (🤒 nie verbunden, 👋 nach Abbruch).
- Geprüft mit Vitest (4 neue Fälle mit Fake-Timern), `tsc` und `task check`. Review: Es bleibt keine Taste hängen, die Slots sind für 2 Spieler getrennt.
