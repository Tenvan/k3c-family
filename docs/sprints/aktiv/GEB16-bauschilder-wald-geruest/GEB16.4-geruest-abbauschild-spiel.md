# GEB16.4 · Gerüst im Bau und Abbau-Schilder im Spiel

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb16/4-geruest-abbauschild-spiel
- **Abhängig von:** GEB16.2, GEB16.3
- **Tickets:** B-586, B-588
- **Kriterien:** AC-03, AC-04

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

- AC-03 und AC-04 (Spiel): umgesetzt.
  - Im Bau steht das Gerüst der passenden Klasse, nie zugleich mit dem Schild (`scaffoldView.ts`). Fehlt die Grafik, zeichnet der Client eine einfache Form. Der Fortschritt ist ein Balken über dem Gerüst.
  - Abbau-Schild bei `minable` (`mineSignView.ts`): Axt am Baum, Spitzhacke an Fels, Erz und Ader, Busch ohne Symbol.
- Geprüft mit `task check` (grün), Vitest 2708 und oxlint ohne Fehler.
- Abweichung: Das Abbau-Schild gab es noch nicht, es ist neu als Brett am Fuß des Knotens. Im Spiel ist nichts geprüft (AC-05, Ralf).
