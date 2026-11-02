# GEB18.7 · Abnahme am Gerät: Hintergründe, Abwechslung, Speicher auf der Xbox

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb18/7-abnahme-geraet
- **Abhängig von:** GEB18.6
- **Tickets:** B-618
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

- [x] AC-05: Ralf nimmt am Gerät ab, Xbox-Speicher und Ladezeit im Budget.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Abnahme durch Ralf (🧑), bestätigt im Chat am 2026-10-31; Claude hat die Prüfung nicht selbst gesehen.

- AC-05 abgenommen (Ralf, am Gerät): detailreichere, abwechselnde Hintergründe; Xbox-Speicher und Ladezeit laut Ralfs Bestätigung im Budget. Ob Ralf die Defaults (Dichte 128, Auswahl im Go-Level-Generator, Abschnittslänge = Kachelbreite der Schicht mitte) gesondert bestätigt hat, ist nicht festgehalten; sie gelten unverändert.
- Befunde: Claude wurden keine genannt, es entstehen keine neuen Tickets.
