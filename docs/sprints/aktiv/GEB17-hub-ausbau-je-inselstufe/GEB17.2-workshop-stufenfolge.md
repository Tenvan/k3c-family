# GEB17.2 · Workshop: Ausbauschritte je Inselstufe, Kosten und Material-Tor

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** geb17/2-workshop-stufenfolge
- **Abhängig von:** –
- **Tickets:** B-592
- **Kriterien:** AC-02

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

- Vorentschieden (autonom, Claude, 2026-10-25) laut Ralfs Ziel „ohne Nachfragen, kritische Nachfragen in KRT“. Ralf bestätigt über B-623 (KRT).
- Die 5 Hub-Stufen bleiben, je Stufe 3 Schritte (`hubStep`). Schritt 3 braucht das Material der nächsten Inselstufe (Material-Tor). Die bisherigen Kosten werden auf die Schritte verteilt (ca. 25/25/50 %). Protokoll additiv. Je Schritt eine eigene Burg-Ansicht.
