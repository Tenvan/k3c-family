# GEB17.8 · Abnahme am Gerät: Angebot und Wachstumsreihe

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb17/8-abnahme-geraet
- **Abhängig von:** GEB17.7
- **Tickets:** B-590, B-591
- **Kriterien:** AC-06

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

- [x] AC-06: 🧑 sieht im Spiel Angebot und Reihe als echtes Wachstum.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Abnahme durch Ralf (🧑), bestätigt im Chat am 2026-10-31; Claude hat die Prüfung nicht selbst gesehen.

- AC-06 abgenommen (Ralf, am Gerät): Ausbau-Angebot vor der Burg und unterscheidbare Wachstumsreihe im Spiel.
- Nicht geprüft und weiter offen: die Bestätigung der Stufenfolge (AC-02) über B-623 (KRT, Status offen); dazu hat Ralf im Chat nichts gemeldet.
- Befunde: Claude wurden keine genannt, es entstehen keine neuen Tickets.
