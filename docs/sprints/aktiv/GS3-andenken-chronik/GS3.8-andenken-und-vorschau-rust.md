# GS3.8 · Andenken auf der Seekarte und Vorschau in der Spielstand-Auswahl im Rust-Spiel (ersetzt GS3.3 und GS3.5)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs3
- **Abhängig von:** GS3.2, GS3.7
- **Tickets:** B-966, B-978
- **Kriterien:** AC-03, AC-05

## Ziel

Die Seekarte im Rust-Spiel zeigt Trophäe, Abschlusszeile und Zahlen für abgeschlossene Inseln (für beide Spieler gleich), und die Spielstand-Auswahl zeigt die Vorschau des markierten Stands.

## Kontext

Spec: `B-966 › Anforderungen` (AC-05, AC-06) und `B-978 › Anforderungen` (AC-02, AC-03). Die Go-Sessions GS3.3 und GS3.5 sind verworfen; diese Session ersetzt sie in Rust (Entscheidung 013): Anzeige in `rust/crates/k3c-game/src/hud/`, `view/`, `local/`, `menu/`. Die Trophäen-Assets kommen aus GS3.2, die Chronik und die Liste aus GS3.7.

Laden, Löschen und die Taste B (zurück) im Menü bleiben unverändert. Jede Anzeige mit 2 Spielern prüfen; HUD-Prüfung durch `k3c-hud` vor dem Commit.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/view/**`
- `rust/crates/k3c-game/src/local/**`
- `rust/crates/k3c-game/src/menu/**`
- `rust/crates/k3c-game/tests/still_*.rs`

## Nicht-Ziele

Neue Spielregeln, Spielstand-Format (GS3.7). Neue Assets (GS3.2); fehlt eines, vorläufig zeichnen und melden.

## Schritte

1. Andenken (Trophäe, Abschlusszeile, Zahlen) auf der Seekarte für abgeschlossene Inseln zeichnen; Standbild-Test.
2. Vorschau des markierten Stands in der Spielstand-Auswahl; Laden, Löschen und B unverändert; Test.
3. `k3c-hud` vor dem Commit.

## Fertig, wenn

- [ ] AC-03: Die Seekarte zeigt Trophäe, Abschlusszeile und Zahlen für abgeschlossene Inseln, für beide Spieler gleich (B-966/AC-05, AC-06).
- [ ] AC-05: Die Spielstand-Auswahl zeigt die Vorschau des markierten Stands; Laden, Löschen und B unverändert (B-978/AC-02, AC-03).
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
