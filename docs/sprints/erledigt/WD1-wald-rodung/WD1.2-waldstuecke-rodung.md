# WD1.2 · Sim: Waldstücke erkennen und Rodungsregel (äußere fünf)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wd1/2-waldstuecke-rodung
- **Abhängig von:** WD1.1
- **Tickets:** B-1204
- **Kriterien:** AC-02

## Ziel

Waldstücke werden erkannt; markierbar sind nur die äußeren 5 Bäume je Seite (nachrückend), kleine Gruppen und Einzelbäume höchstens 5 zugleich.

## Kontext

B-1204 AC-03/AC-04, Annahmen im Ticket. `node_markable` in `rust/crates/k3c-sim/src/state/node_mark.rs`, Markier-Zahlung in `economy/pay.rs`. Nur mit nativen Regeln.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-data/**`
- `data/sizes.json`
- `docs/rules/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Waldstück-Erkennung (Lücke ≤ 3 Units, Mindestlänge aus den Daten, Lichtungen trennen nicht, Hub trennt) als reine Funktion mit Geometrie für die Anzeige.
2. Markierregel in `node_markable` und im Pay-Ziel: äußere 5 je Seite, 5-zugleich-Limit für kleine Gruppen; Ablehnung ohne Abzug.
3. Tests inkl. 2 Spieler, Nachrücken, Teilung durch Rodung.
4. Regeltext in `docs/rules/materialien-gebaeude.md` § 3.3.

## Fertig, wenn

- [ ] AC-02: Erkennung und Rodungsregel laut B-1204/AC-03, AC-04 mit Tests (2 Spieler).
- [ ] `task check:diff` grün (außer vorher roten Tests).

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

AC-02 geprüft: `world/stands.rs` erkennt Waldstücke (Lücke ≤ 3 Units, Lichtungen überbrücken, Hub-Streifen trennt; Plantage-Bäume ausgenommen) und liefert `Stand` (Ausdehnung, groß, Bäume, markierbare äußere 5) für die Anzeige. `node_markable` fragt `tree_markable`: große Stücke nur äußere 5 je Seite (nachrückend), Einzelbäume und kleine Gruppen höchstens 5 zugleich je Stufe über alle Spieler; nur mit nativen Regeln auf der Oberfläche, sonst unverändert. Tests `stands_tests` (Trennung, Nachrücken, Limit, 2 Spieler über `find_pay_target`); `cargo test -p k3c-sim` grün bis auf den schon roten Insel-Takt-Test, clippy sauber. Regeltext in materialien-gebaeude.md § 3.3.

Abweichungen: Mindestlänge 0,25 statt 0,5 der Stufenbreite (der Hub-Streifen teilt den Wald in Hälften zu ca. 0,45, mit 0,5 wäre nie ein Waldstück groß; Konstante `MIN_STAND_FRACTION`, Annahme 2 im Ticket). Kein Hinweistext bei abgelehnter Markierung (Annahme 5): das Zahlziel erscheint einfach nicht; HUD-Hinweis bleibt offen.
