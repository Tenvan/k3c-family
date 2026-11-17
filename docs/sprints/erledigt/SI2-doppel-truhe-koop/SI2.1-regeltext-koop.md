# SI2.1 · Regeltext Koop-Aktionen und Doppel-Truhe (B-968)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si2
- **Abhängig von:** –
- **Tickets:** B-968
- **Kriterien:** AC-01

## Ziel

Die neue Regeldatei `docs/rules/koop.md` beschreibt Koop-Aktionen und die Doppel-Truhe (B-968/AC-01).

## Kontext

Quelle ist `B-968 › Anforderungen` samt Defaults in `B-968 › Offene Fragen` (Entscheidungen 🧑 2026-11-15; Defaults vorläufig). Heute: Truhen geben Gold aus `data/economy.json` › `chestGold` und werfen es als Fontäne aus (B-953); Koop und Besitz in `docs/rules/wirtschaft.md` § 2 (nur lesen). Aufbau der Regeldateien an `docs/rules/monarch.md` oder `bosse.md` ablesen: Kopf mit Stand, Abschnitte mit Tabellen (Regel, Begründung, Daten, Zielkorridor), Abschnitt „Offen und Annahmen“. Koop-Werte laufen nur über Multiplikator-Felder (Basiswerte = Solo). Felder in `data/economy.json` legt SI2.3 an.

## Erlaubte Dateien

- `docs/rules/koop.md`

## Nicht-Ziele

Code, Daten-Dateien, weitere Koop-Aktionen ausarbeiten (nur als spätere Kandidaten nennen), Verweise in anderen Regeldateien (setzt SI2.3 nicht; bei Bedarf Ticket).

## Schritte

1. `B-968` vollständig lesen.
2. `docs/rules/koop.md` anlegen: Grundsatz Koop-Aktionen, Doppel-Truhe (Kennzeichen aus Seed und Position, wirkt nur bei zwei gesteuerten Monarchen, gleichzeitig davor, Gold × Faktor als Fontäne, Zusatz für beide), Solo-Verhalten, Ausnahmefälle.
3. Daten-Spalte: `data/economy.json` › `doubleChest.everyNth`, `doubleChest.goldFactor`; Werte als vorläufig.
4. Abschnitt „Offen und Annahmen“ mit den Defaults und den späteren Kandidaten.

## Fertig, wenn

- [ ] AC-01: `docs/rules/koop.md` existiert in Tabellenform mit allen Regeln aus B-968.
- [ ] `check_run task:check:diff` grün.

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

AC-01: umgesetzt, `docs/rules/koop.md` mit Grundsatz, Doppel-Truhe, Offen und Annahmen (Tabellenform, Werte vorläufig).
check:diff: geprüft (siehe Commit).
