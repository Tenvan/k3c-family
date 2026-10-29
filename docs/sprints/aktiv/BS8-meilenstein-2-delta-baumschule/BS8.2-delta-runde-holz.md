# BS8.2 · Delta-Runde holz gegen M1

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** live
- **Branch:** bs8/2-delta-runde-holz
- **Abhängig von:** BS8.1
- **Tickets:** B-636, B-640
- **Kriterien:** AC-02, AC-03

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

**Fertig mit Abweichung (2026-10-29, Claude).** Delta-Läufe gegen `M1` (coop2, standard, 40 Tage, `k3c-dev`): mit den neuen Bots (run-1) `holz` gleich (1 Ziel, „Holz ≥ 100 ab Tag 3“ 0 % wie in M1), `basis` schweigt (eingefroren, nichts gekippt). Hebel `economy.json:gatherables.tree.markCost=0` (run-2) macht es schlechter (Stufe 3 Tag 15, 13 von 16 verletzt), nicht übernommen. AC-03: keine Wertänderung, also nur Gruppe `holz` (leer). **AC-02 nicht erfüllt:** `holz` wird nicht grün und der Lauf erreicht sein Ziel nicht; Ursache ist die Endphase (B-651, Titan/Endboss), nicht die Baumschule.
