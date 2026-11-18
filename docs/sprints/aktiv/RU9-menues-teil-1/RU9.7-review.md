# RU9.7 · Review Menüs Teil 1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru9
- **Abhängig von:** RU9.6
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Der Sprint ist abgenommen, der PR gegen `main` ist offen.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Zusätzlich: Jede `_test.go` aus `native/k3c-game/menu/` deren Datei in RU9 portiert wurde (`art`, `controls`, `keys`, `lobby`, `main`, `options`, `table` und die Tests aus `paper_test.go`, `fake_test.go`) ist in Rust portiert oder mit Grund im Ergebnis einer Session genannt; die Tests, die auf Screens aus RU12 warten, tragen `#[ignore = "wartet auf RU12"]` und stehen in der Liste für RU12 (`pending.rs` enthält nur noch Platzhalter für RU12: `new_pause`, `new_saves`, `new_world_pick`, `new_testing`, Lizenz-Liste, `version_text`; `grep -rn "ignore" rust/crates/k3c-game/src/menu` nennt nur `RU12`); kein Fachmodul importiert den Zeichen-Stack (Leitfaden § 8); `native/` und `engine/` im Diff unverändert; `testdata/golden` unverändert; `testdata/rust/bild/menu/` nur neue Dateien. B und Esc: pro Screen von RU9 (Lobby, Optionen, Steuerung, Tasten, Lizenzen) ein grüner Test, im Hauptmenü ohne Wirkung. Zwei Spieler: Lobby-Tests mit zwei Geräten grün. `Rc`/`RefCell` in `menu/` (Vorentscheidung RU9.1) mit Fundstelle im Ergebnis auflisten. Reviewer-Agent `k3c-reviewer` (Sonnet) läuft getrennt vom Schreiben; wegen der HUD-Eingabe-Pfade zusätzlich `k3c-hud`.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/menu/**`
- `rust/crates/k3c-game/tests/menu_bild.rs`
- `testdata/rust/bild/menu/**`

## Nicht-Ziele

Neue Funktionen. Befunde über 30 Zeilen Fix werden Tickets.

## Schritte

1. Jedes Kriterium (AC-01 bis AC-03) gegen die Ergebnisse der Sessions RU9.1 bis RU9.6 nachprüfen.
2. Dateigrößen (≤ 400 Zeilen), Funktionslänge (≤ 60 Zeilen), `grep -rn -E "HashMap|Rc<|RefCell|unsafe" rust/crates/k3c-game/src/menu` auswerten.
3. `task check` und `task check:go` nacheinander.
4. Abnahme der Sprint-README füllen, das Abnahme-Ticket für Sim, Balance und Gen anlegen (genau eins; die Sichtprüfung am PC mit Tastatur und Pad, zwei Spieler in der Lobby, gehört als Zeile „Gerät: PC“ hinein), PR gegen `main` öffnen.

## Fertig, wenn

- [ ] Jedes Kriterium ist im Abschnitt Abnahme mit Nachweis genannt.
- [ ] `task check` und `task check:go` grün.
- [ ] PR offen.

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
