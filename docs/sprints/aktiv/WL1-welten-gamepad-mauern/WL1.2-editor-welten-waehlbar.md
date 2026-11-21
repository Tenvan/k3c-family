# WL1.2 · Welten-Editor: alle Welten anwählbar

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wl1/2-editor-welten-waehlbar
- **Abhängig von:** –
- **Tickets:** B-1211
- **Kriterien:** AC-02

## Ziel

Ein bis zwei Sätze: welches Ergebnis nach dieser Session auf `develop` liegt.

## Kontext

Alles, was ein Agent **ohne Vorwissen** braucht: Stand des Codes, relevante Dateien mit Pfad, Entscheidungen
(`docs/decisions/…`), Fallstricke. Keine Verweise auf Chat-Verläufe.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.
Ein Pfad oder Glob je Zeile in Backticks: daraus entsteht die Kollisionsprüfung paralleler Sessions. Sammeldateien (`docs/backlog/README.md`,
`docs/sprints/README.md`, `docs/projekte/README.md`, `docs/glossar.md`) stehen hier nicht; sie ändert `plan_*` oder der letzte Commit des Sprints.

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
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Ursache: In data/worlds/ liegt nur standard.json; Sengoku, Nil und Britannien sind feste Welten ohne Datei und standen gesperrt in der Liste (B-1168). Fix: Ein Klick auf eine solche Welt legt sie aus der Standard-Welt an (worldCreate) und öffnet sie; alle Welten sind damit anwählbar. Test in WorldPage.test.tsx angepasst, vitest (41) und tsc grün. Die Welten bleiben inhaltlich Kopien von Standard, bis sie gestaltet werden.
