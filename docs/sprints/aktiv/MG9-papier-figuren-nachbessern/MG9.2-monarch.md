# MG9.2 · Monarch: Reit-Idle, Gangarten je Reittier, Zustände, Spielerfarben

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** monarch
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/mg9
- **Abhängig von:** MG9.1
- **Tickets:** B-717
- **Kriterien:** AC-02

## Ziel

Der Monarch ist in jeder Lage animiert: Reit-Idle, Gangarten je Reittier, `give`, `hit`, `death`, `respawn`; die vier Spielerfarben sind klar unterscheidbar, die Satteldecke folgt dem Rock.

## Kontext

Ticket B-717 (schließt B-711). Spec `public/grafik/figuren/monarch.paper.json` (Rig `figur`, Reiten über `reittier`-Animationen, Varianten spieler1–4). Heute nur Pferd als Reittier (D-US2-03), `give/hit/death/respawn` fallen auf `idle` (D-US1-03), Spieler 4 nutzt `leder`. Nach MG9.1 (gleiche Reittier-Liste, einheitlicher Maßstab). Truppen-Zustände liegen in MG9.3.

## Erlaubte Dateien

- `public/grafik/figuren/**`
- `public/grafik/stil.paper.json` (nur neue Spielerfarben)
- `tools/paper/golden/**`
- `docs/sprints/aktiv/MG9-papier-figuren-nachbessern/**`

## Nicht-Ziele

Einbau ins Spiel (B-724, US3); Truppen (MG9.3).

## Schritte

1. Test zuerst: Animationen `reiten-idle` und `schritt|trab|galopp-<reittier>` für jedes Reittier der Gruppe, Keyframes `give`, `hit`, `death`, `respawn`, vier unterscheidbare Rock- und Satteldeckenfarben ohne Braun.
2. Animationen und Varianten in der Spec ergänzen.
3. `task gen:paper` für `figuren`, Vorschau je Reittier und Spieler ansehen, Golden schreiben.

## Fertig, wenn

- [ ] AC-02: `figuren/monarch` hat `reiten-idle` und je Reittier `schritt/trab/galopp-<id>` (B-717/AC-01).
- [ ] AC-02: `give`, `hit`, `death`, `respawn` vorhanden; Truppen siehe MG9.3 (B-717/AC-02).
- [ ] AC-02: Varianten spieler1–4 mit unterscheidbarer Rock- und Satteldeckenfarbe, keine auf Braun (B-717/AC-03).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
