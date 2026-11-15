# GS1.3 · Spiel: Geschichtszeile je Spielerhälfte an den Wendepunkten (B-974)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs1
- **Abhängig von:** GS1.2
- **Tickets:** B-974
- **Kriterien:** AC-04

## Ziel

Das Spiel blendet die Zeilen der Geschichte an den Wendepunkten in jeder Spielerhälfte ein und aus, ohne Eingabe zu verbrauchen.

## Kontext

Spec: `docs/backlog/B-974-geschichte-einblendung-spiel.md` (R1–R7, AC-01 bis AC-04). Die Engine liefert die Zeilen über die Wendepunkt-Funktionen aus GS1.2 (`engine/sim/story.go`); das Spiel rechnet keine Wendepunkte selbst. HUD: `native/k3c-game/hud/` – Meldungen `banner.go` (Warteschlange je Spieler, `bannerSeconds`, Zeit nur aus `Frame.Clock`, Verkleinern `fitPx`), Zeichnen `draw.go`/`drawworld.go`, Zustand `state.go`, Schrift `fonts.go`/`glyphs.go`, Layout `layout.go`. Ereignisse kommen als `view.Event` (`native/k3c-game/view/`), die Welt des Raums über die lokale Sitzung (`native/k3c-game/local/`), Spielstart und Laden in `native/k3c-game/game/` (`play.go`, `loading.go`). Ereignisse einer anderen Stufe müssen alle Spielerhälften erreichen (Wendepunkte gelten für die Insel). Mit 2 Spielern gleichzeitig; keine Allokation je Frame im Ruhezustand.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- `native/k3c-game/view/**`
- `native/k3c-game/game/**`
- `native/k3c-game/local/**`

## Nicht-Ziele

Regeln der Wendepunkte (Engine), Texte, Vertonung, Einblendung über die ganze Breite, Flaschenpost (GS4).

## Schritte

1. Tests zuerst (`hud/story_test.go`): Einreihen je Ereignis, Ablauf nach `Frame.Clock`, 2 Spieler in verschiedenen Stufen, Laden löst nichts aus, 0 Allokationen.
2. Zustand „Geschichtszeile“ je Spieler (eigene Warteschlange neben `bannerQueue`), neue Datei `hud/story.go`.
3. Anbindung: neue Kampagne, `islandSwitch`, `bossSpawned`/`bossDefeated` des Endbosses → Zeilen der Engine an alle Spielerhälften.
4. Zeichnen im oberen Drittel der Zelle mit Ein-/Ausblenden (0,5 s / 4 s / 0,5 s), Verkleinern statt Kürzen.
5. Test: Alle Zeilen der Welt-Dateien unter `data/worlds/` sind mit der HUD-Schrift darstellbar; fehlende Zeichen in `glyphs.go` ergänzen.
6. Fachprüfung `k3c-hud`, dann `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-04: Einblendung an allen Wendepunkten in jeder Spielerhälfte, 2 Spieler, Ablauf nach Uhr, keine Eingabe verbraucht, 0 Allokationen, alle Zeilen darstellbar (B-974/AC-01 bis AC-04).
- [ ] B-974/AC-05 (TV) bleibt offen bei 🧑.
- [ ] `check_run task:check:diff` grün.

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
