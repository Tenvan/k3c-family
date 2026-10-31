# GEB17.1 · Ausbau-Angebot erscheint vor der Burg

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb17/1-ausbau-angebot-burg
- **Abhängig von:** –
- **Tickets:** B-591
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

- AC-01: umgesetzt (Geräte-Abnahme offen).
  - Ursache im Client: Die Burg (`hubSite`) steht nicht in `world.sites`, deshalb fand `priceTagSites` sie nie. Die Burg-View hatte kein Schild. Der Server meldet `hubUpgrade` korrekt.
  - Fix: Die Burg nimmt an „nur das nächste Preisschild“ teil (`offersHubUpgrade` aus `hubUpgrade`). Das Schild zeigt Gold und Material bzw. den Wartegrund, das Material-Label kennt Eisen und Kristall.
- Geprüft mit `task check` (grün). Neuer Vitest mit echten Snapshot-Daten (Angebot vor der Burg, näherer Platz gewinnt, kein Angebot auf Stufe 5, bei laufendem Ausbau oder bezahltem Gold).
- Hinweis: Das Schild erscheint ab 6 Units, der Server nimmt Münzen ab 2 Units an. Mit GEB16 (`payable`) kann die Burg später auch auf ein Server-Feld umziehen.
