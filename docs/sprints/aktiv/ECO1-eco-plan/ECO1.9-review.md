# ECO1.9 · Review und Abnahme-Ticket ECO1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** eco1/9-review
- **Abhängig von:** ECO1.2, ECO1.5, ECO1.7, ECO1.8
- **Tickets:** B-1191, B-1192, B-1193, B-1194, B-1195, B-1196, B-1197, B-1198
- **Kriterien:** AC-09

## Ziel

ECO1 ist geprüft; ein Abnahme-Ticket sammelt PC-Abnahme, Gen-, Sim- und Race-Läufe.

## Kontext

Review der Sessions ECO1.1 bis ECO1.8 gegen die Tickets B-1191 bis B-1198. Sim, Balance, Gen und `check:race` nur im Abnahme-Ticket (B-742).

## Erlaubte Dateien

- `docs/sprints/aktiv/ECO1-*/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Diff lesen, Fachagenten (k3c-welten, k3c-hud, k3c-balancing) laufen lassen.
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen (PC-Abnahme von Ralf).

## Fertig, wenn

- [ ] AC-09: Review ohne offene schwere Befunde, Abnahme-Ticket angelegt.
- [ ] `task check` und `task check:go` grün.

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

AC-09 geprüft: Review der Sessions ECO1.1 bis ECO1.8 durch Reviewer-Agent; schwere Befunde keine, drei mittlere zu ECO1.7 (Pendel-Schleife, Limit nur Stufe 0, Kämpfer-Seite) behoben (Wartezeit `FUTILE_WAIT`, `surface_gear` mit Zielstufe, Anker-Seite der schwächeren Seite); `cargo test -p k3c-sim --lib errand` 5/5 grün. `task check` und `task check:go` grün.

Abnahme-Ticket: B-1202 (Sprint ECO1, Spec freigegeben autonom, Claude).

Abweichungen: ECO1.2, ECO1.6 und ECO1.7 änderten Dateien außerhalb der erlaubten Liste (Importregel: `view/seal.rs`; Go-Struct und Golden für `topOnly`; Armory-Tests in Go, Rust und hubplan).

Bekannt rot (B-1190 AC-04): `alloc_je_bild`, `eingebettete_welten_gueltig`, `insel_nach_1000_schritten_wie_go`, `e2e_flow`; zusätzlich `still_hud pad-tastatur` (4,020 > 4,0) und `ablauf_wie_go_zwei_laeufe_gleich`, beide in B-1202 AC-07.

### Für den Hauptrechner

- 50:50-Entscheidungen für Ralf (B-1202 Offene Fragen): Taverne als Produktion (`topOnly`) und Bogner unten erlaubt; Rückkehr der Besorger ohne sichtbaren Weg; Truppen-Limit unten.
