# SI1.1 · Regeltext Laderaum und Fracht (B-967)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si1
- **Abhängig von:** –
- **Tickets:** B-967
- **Kriterien:** AC-01

## Ziel

Die Regel zu Laderaum, Ladeplätzen und Fracht steht in `docs/rules/stufen.md` § 1 und `docs/rules/wirtschaft.md` § 8 (B-967/AC-01).

## Kontext

Quelle ist `B-967 › Anforderungen` samt Defaults in `B-967 › Offene Fragen` (Entscheidungen 🧑 2026-11-15; Defaults vorläufig). Heute: Inselwechsel mit Startvorrat in `docs/rules/stufen.md` § 1 (Zeile „Inselwechsel (BAS, B-637)“), Schiff mit 3 Abschnitten in `docs/rules/wirtschaft.md` § 8 (Zeile „Schiff“, B-637, B-638), Daten in `data/ship.json`. Regeldateien nutzen Tabellen mit den Spalten Regel, Begründung, Daten, Zielkorridor; verbindliche Begriffe in `docs/glossar.md` (neue Begriffe Laderaum, Ladeplatz, Fracht, Beutel, Kiste dort im letzten Commit des Sprints eintragen, Sammeldatei). Nur Text: Felder in `data/ship.json` legt SI1.2 an.

## Erlaubte Dateien

- `docs/rules/stufen.md`
- `docs/rules/wirtschaft.md`

## Nicht-Ziele

Code, Daten-Dateien, Werte abstimmen (Balancing gesperrt), Anzeige.

## Schritte

1. `B-967` vollständig lesen, die betroffenen Zeilen in `stufen.md` § 1 und `wirtschaft.md` § 8 lesen.
2. `stufen.md` § 1: Zeilen zu Laderaum und Plätzen, den drei Ladeplätzen am Steg, Ankunft der Fracht (Material in den Vorrat, Truppen am Hafen, Beutel als Münzen am Steg), kein Entladen, letzte Insel ohne Ladeplätze.
3. `wirtschaft.md` § 8: vierter, freiwilliger Abschnitt „Laderaum“, Größe von Beutel und Kiste, Datenfelder `data/ship.json` › `cargo` und Abschnitt `hold`; alle Zahlen als vorläufig kennzeichnen.
4. Ausnahmefälle aus B-967 als Regelzeilen aufnehmen (voller Laderaum, Verlust des Laderaums, offener Träger-Auftrag bei Abfahrt).
5. Stand-Zeile im Kopf der Dateien nachziehen, Verweise auf B-967 setzen.

## Fertig, wenn

- [ ] AC-01: Beide Dateien enthalten die Regeln aus B-967 in Tabellenform, Daten-Spalte nennt `data/ship.json`-Felder, Werte als vorläufig markiert.
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

AC-01 geprüft: `docs/rules/stufen.md` § 1 (Laderaum und Plätze, drei Ladeplätze, Ankunft, Ausnahmen) und `docs/rules/wirtschaft.md` § 8 (Abschnitt `hold`, Beutel, Kiste, Koop). `task check:diff` und `vitest tests/` grün. Commit fafd5cf06 auf `sprint/si1`. Festlegungen für SI1.2 (vorläufig): `data/ship.json` › `cargo` mit `slots` 6, `holdSlots` 4, `purseCoins` 10, `crateUnits` 20, `callCoins` 1 (neu, nicht im Ticket); Abschnitt `hold` optional (15 Münzen + 60 Holz + 20 Stein); Ereignis `cargoFull`; `IslandSaveVersion` hochzählen. Glossar (Laderaum, Ladeplatz, Fracht, Beutel, Kiste) im letzten Commit des Sprints.
