# GS4.5 · Review: Sprint-Abnahme GS4

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/gs4
- **Abhängig von:** GS4.7
- **Tickets:** B-971, B-980
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07

## Ziel

Der Sprint GS4 ist gegen seine Kriterien geprüft, Befunde sind behoben oder als Ticket angelegt, die Abnahme steht in der README.

## Kontext

Ablauf: `docs/arbeitsweise.md` › Review-Session. Kriterien: README des Sprints GS4 (`docs/sprints/geplant/GS4-flaschenpost/README.md`, nach dem Aktivieren unter `aktiv/`), Tickets B-971, B-980. Diff von `sprint/gs4` gegen den Start-Commit. Fachprüfer: `k3c-reviewer` (Sim, Determinismus, Spielstand, Golden-Diff), `k3c-balancing` (Werte in `data/bottle.json`, nur lesen), `k3c-grafik` (Asset), `k3c-hud` (Pfeil, Radar). TV-Abnahme (AC-07) bleibt bei 🧑. Keine Simulation, kein Balancing.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/world/**`
- `docs/rules/wirtschaft.md`

## Nicht-Ziele

Neue Funktionen, Abstimmen der Häufigkeit, Einsiedler und Gatter.

## Schritte

1. `check_run task:check`, danach `check_run task:check:go`.
2. Nachweise AC-01 bis AC-06 prüfen; Golden-Diff auf unerwartete Änderungen durchsehen.
3. Fachprüfer, schwere Befunde beheben, Rest als Tickets (z. B. Ziele Einsiedler und Gatter an B-972, B-970 melden).
4. Glossar-Einträge (Flaschenpost, Markierung) im letzten Commit.
5. Abnahme-Ticket anlegen (Sim, Balance, Gen, Race; B-742), Abschnitt Abnahme füllen, AC-07 als offen bei 🧑 nennen.

## Fertig, wenn

- [ ] AC-01 bis AC-06 belegt, AC-07 als offen bei 🧑 vermerkt.
- [ ] Abnahme-Ticket angelegt, Abschnitt Abnahme gefüllt.

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

Review GS4 (Flaschenpost), Session GS4.5. Tests: bottle 12 grün, k3c-data bottle 3 grün, save 77 grün (1 ignoriert). task check und check:go nicht gelaufen.

AC-01 erfüllt mit Abweichung: Regel § 9 in wirtschaft.md und Glossar vorhanden; Regeltext passt an einigen Stellen nicht mehr zu den Daten (edgeUnits, Reichweite, GS4.2-Verweis).
AC-02 teilweise: Morgengrauen, Abend und Aufheben deterministisch, aber Zielpool nur Doppel-Truhen mit mindestens zwei Monarchen, also mit einem Spieler nie eine Flasche (HOCH).
AC-03 nicht erfüllt: Ziel nur auf Stufe 1, Markierung ohne Zielart und Stufe, Ziel-Verlust ohne bottleTargetReached, Liste wird je Takt neu gefüllt (HOCH). Geschichtszeile reihum erfüllt.
AC-04 teilweise: Version 14, Laden, Fixture v14 ohne Feld bottle; Zähler zieht beim Inselwechsel nicht mit (island/switch.rs). Golden verschoben auf RU41 (Go-Engine eingefroren).
AC-05 verschoben (GS4.3 offen), AC-06 verschoben (GS4.7 offen; k3c-game liest bottle nicht), AC-07 offen bei Ralf (Sichtprüfung am TV).

Befunde: HOCH Zielpool nur Doppel-Truhen; HOCH Markierung verschwindet ohne Ereignis; MITTEL Markierung nur Stufe 0; MITTEL Zähler beim Inselwechsel; MITTEL Gleichstand bei Berührung nicht höchster Index; NIEDRIG Regeltext-Abgleich, Fixture ohne bottle, Allokation je Takt, GS4.6 noch als offen markiert.

Gesamturteil: GS4 nicht abnahmereif. Sprint bleibt aktiv. Vor GS4.7 die beiden HOCH-Befunde beheben.
