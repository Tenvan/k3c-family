# EQ1.5 · Besorger mit sichtbarem Weg über die Treppe

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
