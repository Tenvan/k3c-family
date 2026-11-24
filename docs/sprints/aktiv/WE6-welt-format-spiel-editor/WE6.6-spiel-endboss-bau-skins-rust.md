# WE6.6 · Endboss, Bau und Skins je Welt im Rust-Spiel zeichnen (ersetzt WE6.1 und WE6.2)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we6
- **Abhängig von:** RU37, RU60
- **Tickets:** B-993, B-994
- **Kriterien:** AC-01, AC-02

## Ziel

Das Rust-Spiel zeigt Bau und Endboss in jedem Biom (ohne eigenes Asset: Ersatz der Basisart) und zeichnet Gegner und Objekte mit den Skins der Welt (Rückfall auf das Basis-Asset, Namen aus dem Wertesatz).

## Kontext

Spec: `B-993 › Anforderungen` (AC-01, AC-02) und `B-994 › Anforderungen` (AC-01 bis AC-03). Die Go-Sessions WE6.1 und WE6.2 sind verworfen; diese Session ersetzt sie in Rust (Entscheidung 013): Das Spiel ist `rust/crates/k3c-game` (`actors/`, `props/`, `world/`, `art/`), Zuordnung in `data/paper.json`. Endboss und Skins liefert die Sim aus WE5 (`rust/crates/k3c-sim`, belegt in RU60.4, B-981 bis B-984).

Das Spiel zeichnet nur. Texturbudget beachten (`k3c-grafik`). Jede Anzeige mit 2 Spielern prüfen.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/**`
- `rust/crates/k3c-game/tests/still_*.rs`
- `data/paper.json`

## Nicht-Ziele

Neue Spielregeln. Neue Assets (fehlt eines, vorläufig mit der Basisart zeichnen und melden). Welten-MCP und Editor (WE6.3, WE6.4).

## Schritte

1. Bau und Endboss je Biom zeichnen, Ersatz-Asset der Basisart, wenn keines vorhanden; Test.
2. Skins der Welt anwenden (Gegner, Objekte), Rückfall auf das Basis-Asset; Namen aus dem Wertesatz; Test.
3. Standbild-Test; `k3c-grafik` bei Bedarf.

## Fertig, wenn

- [ ] AC-01: Bau und Endboss in jedem Biom, Ersatz-Asset der Basisart (B-993/AC-01, AC-02).
- [ ] AC-02: Skins der Welt im Spiel, Rückfall auf das Basis-Asset, Namen aus dem Wertesatz (B-994/AC-01 bis AC-03).
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

- AC-02 geprüft: Skin-Tabelle der Welt wird beim Figurenaufbau gelesen; zugeordnete Assets ersetzen das Standard-Asset, ohne Eintrag bleibt alles beim Alten (Commit 7b37e4fec, rebased auf main als e3b1b49c1). `cargo test -p k3c-game`: 1073 Lib-Tests grün, Integrationstests grün, ohne Ignorierung.
- Umgebungsfehler, kein Codefehler: der Test `drei_stufen_mindestens_halbe_monarchenhoehe_auseinander` brauchte eine generierte Grafikdatei (`objekte/setzling`), die im Worktree fehlte.
- Objekt-Skins (Rest): Bäume und Felsen der Stufe nehmen das Ersatz-Asset aus der Skin-Tabelle, sonst das Basis-Asset (`props/plan_entities.rs`, Test `objekt_skin_ersetzt_das_asset_des_baums`). `cargo test -p k3c-game`: grün.
- Offen: Bau und Endboss je Biom (Standbild-Test, Gen-Lauf nötig).
- PR #875 gemergt (Squash).
