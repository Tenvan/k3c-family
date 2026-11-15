# SI5.1 · Regeltext besondere Figur und Bauwerk der Welt (B-972)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si5
- **Abhängig von:** –
- **Tickets:** B-972
- **Kriterien:** AC-01

## Ziel

`docs/rules/buerger.md` beschreibt die besondere Figur der Welt, `docs/rules/materialien-gebaeude.md` ihr Bauwerk mit Dauerbonus (B-972/AC-01).

## Kontext

Quelle ist `B-972 › Anforderungen` samt Defaults in `B-972 › Offene Fragen` (Entscheidungen 🧑 2026-11-15; Defaults vorläufig). Heute: Figuren und der Händler als Besucher in `docs/rules/buerger.md` § 1 (Händler tauscht Material gegen Gold), Gebäude je Hub-Stufe in `docs/rules/materialien-gebaeude.md` § 3, Bau, Zerstörung und Reparatur in § 4, feste Hub-Plätze in `data/hub.json`. Eine Welt Polar gibt es nicht; Walfänger und Tranhütte stehen als geplant im Text. Tabellenform der Dateien beibehalten (Regel, Begründung, Daten, Zielkorridor).

## Erlaubte Dateien

- `docs/rules/buerger.md`
- `docs/rules/materialien-gebaeude.md`

## Nicht-Ziele

Code, Daten-Dateien, `welten.md` (Feld `hermit` trägt SI5.2 ein), Sichtregel, Werte abstimmen.

## Schritte

1. `B-972` und die genannten Abschnitte vollständig lesen.
2. `buerger.md` § 1: besondere Figur je Welt (Schwertschmied, Priesterin; Walfänger geplant), Hütte in Stufe 2 am fernen Rand, Zahlung, einmalig je Insel, Abgrenzung zum Händler, Weg in den Hub nur Anzeige.
3. `materialien-gebaeude.md` § 3: Bauwerk je Welt (Dojo, Schrein; Tranhütte geplant), fester Hub-Platz `hermitHall`, Bonus für die ganze Insel, kein Ausbau; § 4: zerstörbar und reparierbar, Bonus ruht.
4. Daten-Spalte: `data/hermits.json`, `data/hub.json` › `islandSites`; alle Zahlen als vorläufig.
5. Ausnahmefälle aus B-972; Stand-Zeilen nachziehen.

## Fertig, wenn

- [ ] AC-01: Beide Dateien enthalten die Regeln aus B-972 in Tabellenform.
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

–
