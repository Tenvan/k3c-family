# BS9.1 · Bots bauen das Schiff, Bericht Bau-Tag

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bs9/1-bots-bauen-schiff
- **Abhängig von:** –
- **Tickets:** B-637
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
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

**Fertig (2026-10-29, Claude).** AC-01: `shipX` (bot_wirtschaft.go) zahlt den nächsten Schiffsabschnitt, wenn Hub ≥ 2, Heer ≥ `fighterWant` und das Material im Vorrat liegt; `sim.ShipNextCost` (ship.go) liefert den Preis. Test `TestBotBautSchiff`. Ohne die Heer-Bedingung kaufte der Bot den Kiel an Tag 1 und die Wirtschaft brach zusammen (Hub 1, 0 Kämpfer). Kennzahl „Bau-Tag“: der Bericht kennt `affordableDay` je Abschnitt (Kiel Tag 1, Rumpf Tag 16, Segel nie); ein eigener Bau-Tag-Zeitstempel fehlt (Ticket-Kandidat).
