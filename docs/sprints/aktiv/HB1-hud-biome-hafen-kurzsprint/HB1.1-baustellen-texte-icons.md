# HB1.1 · HUD: Baustellen-Texte kleiner, Kosten als Icons, Ausbau x/n

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/hb1
- **Abhängig von:** –
- **Tickets:** B-1186
- **Kriterien:** AC-01

## Ziel

Baustellen im Spiel zeigen kleineren Text, Kosten als Ressourcen-Icons und den Ausbau als x/n.

## Kontext

HUD-Texte der Baustellen liegen in `rust/crates/k3c-game/src/hud/` (Text-Slots in `hud/paint.rs`, Zustand `hud/state.rs`, Zeichnen `hud/drawworld.rs`); Ressourcen-Icons gibt es als Papierschnitt-Icons (Gruppe `icons`). Ticket B-1186. Fallstricke: keine Allokation je Bild, Split-Screen mit zwei Spielern, Dateien ≤ 400 Zeilen. Zuerst die Stelle suchen, die den Baustellen-Text zeichnet (grep nach Kosten/Ausbau).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/props/**`

## Nicht-Ziele

Gebäudename (HB1.2), Händler (HB1.2), neue Icons (vorhandene nutzen).

## Schritte

1. Stelle des Baustellen-Texts finden, Schriftgröße verkleinern.
2. Kosten als Icon plus Zahl statt Wort zeichnen (Rückfall Kurztext ohne Icon-Sheet).
3. Ausbau-Text auf x/n kürzen.
4. Tests für Text und Icon-Wahl.

## Fertig, wenn

- [ ] AC-01: Baustellen-Texte kleiner, Kosten als Icons, Ausbau als x/n; Tests grün.
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
