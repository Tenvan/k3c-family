# GS3.1 · Engine: Chronik abgeschlossener Inseln im Spielstand (B-966)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs3
- **Abhängig von:** –
- **Tickets:** B-966
- **Kriterien:** AC-01

## Ziel

Die Engine hält beim Ablegen je abgeschlossener Insel Tage und Monarchen fest und trägt die Chronik im Spielstand mit; der Regeltext nennt sie.

## Kontext

**Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`):** `task k3c-workbench:dev:worktree`, MCP-Aufrufe mit `checkout`. Keine Simulation, kein Balancing.

Spec: `docs/backlog/B-966-chronik-der-reise.md` (R1, R2, R6, AC-01 bis AC-03). Code: `engine/sim/island_switch.go` (Inselwechsel, `NextIsland`), `engine/sim/island_targets.go` (`Completed`, `done`), `engine/sim/island_save.go` (`IslandSaveVersion = 9`, Felder `completed`, `chosen`, Prüfung unbekannter Inseln), Fixtures der Spielstand-Tests (`island_save_v8_test.go` als Muster). Ablauf Formatwechsel: `docs/arbeitsweise.md` › Spielstand-Format ändern (Version erhöhen, Fixture); keine Migration alter Stände nötig. Regeltext: `docs/rules/welten.md` § 5 und § 6.1. „Tage“ = Tage auf dieser Insel bis zum Ablegen; „Monarchen“ = lebende Spieler am Steg beim Wechsel. Sperre `spielstand`.

## Erlaubte Dateien

- `engine/sim/**`
- `docs/rules/welten.md`

## Nicht-Ziele

Anzeige (GS3.3), Liste der Spielstände (GS3.4), Assets, Migration alter Stände.

## Schritte

1. Tests zuerst: Wechsel am Tag 14 mit 2 Spielern → Eintrag; Mitziehen; zweiter Wechsel; Speichern/Laden byte-gleich; Stand ohne `chronicle`; Eintrag mit unbekannter Insel abgelehnt.
2. Typ und Feld `Chronicle` an der Insel, Eintrag beim Ablegen, Mitziehen in `NextIsland`.
3. Spielstand: Feld `chronicle`, `IslandSaveVersion` + 1, neue Fixture, Prüfung.
4. Zugriff für das Spiel (exportierte Lesefunktion).
5. `docs/rules/welten.md` § 5/§ 6.1 ergänzen; Glossar-Texte (Andenken, Chronik, Trophäe) für den letzten Commit vorbereiten.
6. Review-Agent (SIM, Spielstand), `check_run task:check:go` im Worktree, `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-01: Chronik entsteht, zieht mit, übersteht Speichern und Laden; Version erhöht, Fixture; Regeltext ergänzt (B-966/AC-01 bis AC-03).
- [ ] Dev-Workbench des Worktrees gestoppt.

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
