# GEB16.7 · Abnahme am Gerät: Schilder, Wald, Gerüst, Abbau-Symbole

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb16/7-abnahme-geraet
- **Abhängig von:** GEB16.6
- **Tickets:** B-584, B-585
- **Kriterien:** AC-05

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

- [x] AC-05: Ralf hat Schilder, Wald, Gerüst und Abbau-Symbole am Gerät abgenommen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Abnahme durch Ralf (🧑), bestätigt im Chat am 2026-10-31; Claude hat die Prüfung nicht selbst gesehen.

- AC-05 abgenommen (Ralf, am Gerät): Bauschilder, dichterer Wald, Baugerüst und Abbau-Symbole (Axt, Spitzhacke).
- Befunde: Claude wurden keine genannt, es entstehen keine neuen Tickets.
