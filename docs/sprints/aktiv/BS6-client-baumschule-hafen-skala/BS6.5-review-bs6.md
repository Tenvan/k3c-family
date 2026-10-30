# BS6.5 · Review BS6

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** bs6/5-review-bs6
- **Abhängig von:** BS6.4
- **Tickets:** B-634, B-636, B-637
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

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
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

**Fertig (2026-10-29, Claude), Review durch getrennten Reviewer-Agenten (Sonnet) über 34ef14d2, fb2c50bb, ba556d4e.** Client unauffällig. Behoben (9b678135): Bot sucht Material für Rumpf und Segel (`shipNode`), wartet bei `waitingMaterial` wie bei Bauten, Positivtest für `shipX`. Offen: Schiff nimmt gemeinsames Material, das der Hub-Ausbau braucht (Ping-Pong möglich, ungeprüft); JSDoc von `drawPrice` hängt über der neuen Funktion (kosmetisch); Test `TestBotRodetBaumschule` prüft den Kauf nach dem Roden nicht. Ob der Bot die Hafen-Stufe je betritt, zeigt der Lauf: Rumpf ab Tag 16 bezahlbar, Segel nie. Abnahme am Gerät: BS6.6 (🧑).
