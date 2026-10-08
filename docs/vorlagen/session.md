# SP00.1 · Titel der Session

- **Status:** offen | in Arbeit | fertig | blockiert | verworfen (Grund im Ergebnis)
- **Typ:** Umsetzung | Review | Workshop
- **Agent:** autonom | Mensch
- **Domäne:** REG | SIM | SRV | CLI | PLAT | INF | DEV (genau eine; die Session ändert nur deren Dateien)
- **Umgebung:** offline | live | ? (offline: ohne laufende Dienste prüfbar – Code, Unit-/Mock-Tests, Werkzeuge ohne Serverzugriff, worktree-tauglich; live: braucht laufenden Server, Browser oder Gerät)
- **Branch:** sp00/1-kurzname
- **Abhängig von:** – (oder SP00.1, B-000)
- **Tickets:** B-000
- **Kriterien:** AC-01, AC-02 (Akzeptanzkriterien der Sprint-README, die diese Session erfüllt; Review: alle)

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
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
