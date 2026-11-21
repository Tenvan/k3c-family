# EQ1.6 · Review und Abnahme-Ticket EQ1

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** ?
- **Branch:** eq1/6-review
- **Abhängig von:** EQ1.2, EQ1.5
- **Tickets:** B-1205, B-1206, B-1207, B-1208, B-1209
- **Kriterien:** AC-06

## Ziel

EQ1 ist geprüft; ein Abnahme-Ticket sammelt PC-Abnahme, Gen-, Sim- und Race-Läufe.

## Kontext

Review der Sessions EQ1.1 bis EQ1.5 gegen B-1205 bis B-1209. Sim, Balance, Gen und `check:race` nur im Abnahme-Ticket (B-742).

## Erlaubte Dateien

- `docs/sprints/aktiv/EQ1-*/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Diff lesen, Fachagenten (k3c-reviewer, k3c-hud) laufen lassen.
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen (PC-Abnahme von Ralf).

## Fertig, wenn

- [ ] AC-06: Review ohne offene schwere Befunde, Abnahme-Ticket angelegt.
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

- AC-06 geprüft: `task check` und `task check:go` grün (nacheinander, Claude). Review EQ1.1 (k3c-reviewer: zwei mittlere Befunde, behoben), EQ1.4 (k3c-reviewer: mittlerer Befund, behoben), EQ1.2 (k3c-hud: keine schweren Befunde, zwei mittlere Layout-Befunde unten), EQ1.3 und EQ1.5 (klein, ohne eigenes Review). Keine offenen schweren Befunde.
- Rust-Tests: nur die bekannten Roten (`alloc_je_bild`, `eingebettete_welten_gueltig`, `insel_nach_1000_schritten_wie_go`, B-1190 AC-04).

### Für den Hauptrechner

- **Abnahme-Ticket EQ1 anlegen** (Second Worker legt keine Tickets an): PC-Abnahme durch Ralf (Rüstung wird am Gebäude abgeholt, Icons am Regal und Preisschild, Bogner nur oben, Limit für die ganze Insel mit zwei Stufen, Besorger-Weg und Darstellung Ein-/Ausblenden oder Laufen); `task check:race`, `task gen:paper` samt Golden; Sim und Balance nur über `sim_test` (Rüstungspreis, Limit, Besorger ändern die Balance).
- **Befund Icons (mittel, vermutet, k3c-hud):** `props/plan_cost.rs` `shelf_icons` hat feste Pixelabstände ohne Obergrenze; bei Waffenschmied und Rüstungsschmied ist `shelf_rack` 0, die Reihe kann über das Gebäude hinauswachsen. `item_tag`: Icon fest bei x-56, kann in die Kostenreihe laufen. Im Spiel prüfen (voller Schmied, Preisschild mit mehreren Rohstoffen im Split-Screen); Regalgröße `armorRack` steht jetzt in den Daten (EQ1.3) und kann als Deckel dienen.
- **Befund (niedrig):** Platzart-zu-Icon-Tabelle steht doppelt (`props/plan_cost.rs` und `hud/icons.rs`), ein Vergleichstest fehlt; `shelf_icons` läuft je Bild auch bei leerem Regal (früher Rücksprung billiger); ohne Icon-Sheet haben Gürtel, Schwert und Rüstung keinen Ersatz.
- **Besorger-Darstellung:** Ereignisse `stairsLeave`/`stairsArrive` stehen bereit, die Blende in `k3c-game` fehlt noch; Ralf entscheidet bei der Abnahme.
- Zusatz: `weapons_in_flight` zählt nur die eigene Stufe, das Insel-Limit rechnet Holer anderer Stufen über `committed` mit (EQ1.4).
