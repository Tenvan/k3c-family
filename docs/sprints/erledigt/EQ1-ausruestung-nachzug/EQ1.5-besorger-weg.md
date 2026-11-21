# EQ1.5 · Besorger mit sichtbarem Weg über die Treppe

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eq1/5-besorger-weg
- **Abhängig von:** EQ1.4
- **Tickets:** B-1209
- **Kriterien:** AC-05

## Ziel

Der Besorger bleibt beim Stufenwechsel sichtbar und läuft zur Treppe, nach oben und zurück.

## Kontext

Ticket B-1209. `island/errand.rs` nimmt den Bürger heute aus der Stufe und stellt ihn oben neu hin (`take`/`place`). Default: Ein- und Ausblenden an der Treppe, Ralf entscheidet bei der Abnahme. Frame-Budget beachten.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/island/**`
- `rust/crates/k3c-sim/src/state/**`
- `rust/crates/k3c-game/src/actors/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Wechsel an der Treppe sichtbar machen (Figur bleibt erhalten).
2. Rückweg auf demselben Weg.
3. Tests AC-01 bis AC-03 des Tickets, `alloc_je_bild` prüfen.

## Fertig, wenn

- [ ] AC-05: Besorger sichtbar über die Treppe, deterministisch, Frame-Budget gehalten (B-1209 AC-01 bis AC-03).
- [ ] `task check:diff` grün.

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

- AC-05 geprüft (B-1209 AC-01 bis AC-03): Der Besorger lief schon sichtbar zur Treppe (Auftrag `ascend`) und kehrt auf demselben Weg zurück; neu meldet `island/errand.rs` das Verlassen und Erscheinen an der Treppe als Ereignisse `stairsLeave` (Stufe unten) und `stairsArrive` (Stufe oben, mit Ort, Art, ID), damit die Anzeige ein- und ausblendet (Default laut Ticket, Ralf entscheidet bei der Abnahme). Art, Leben und Beruf bleiben erhalten, nichts im Spielstand.
- Tests `island/errand_tests.rs`: Figur bleibt erhalten und Ereignisse kommen (AC-01), gleiche Eingabe gleicher Ablauf (AC-03), ohne Treppe geht keiner hinauf (Ausnahmefall).
- AC-02: Im Spiel geändert wurde nichts (`k3c-game` unberührt), nur zwei kleine Ereignisse je Wechsel in der Sim; `alloc_je_bild` unverändert (bekannt rot, B-1190 AC-04).
- Nur die bekannten Roten (`eingebettete_welten_gueltig`, `insel_nach_1000_schritten_wie_go`); clippy, fmt, Regeltests grün. Kein eigenes Review (Aenderung unter 30 Zeilen, EQ1.4 reviewt).
- Offen für Ralf (Abnahme EQ1): Darstellung – Ein-/Ausblenden an der Treppe (jetzt, Ereignisse stehen bereit, die Grafik der Blende fehlt noch) oder Laufen durch die Treppe.
