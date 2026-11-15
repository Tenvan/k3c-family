# GS2.2 · Spiel: Seekarten-Overlay als Diorama (B-964)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs2
- **Abhängig von:** GS2.1
- **Tickets:** B-964
- **Kriterien:** AC-02, AC-03

## Ziel

Das Seekarten-Overlay zeichnet das Diorama aus den Assets, mit Varianten je Insel und Rückfall auf die bisherigen Formen.

## Kontext

Spec: `docs/backlog/B-964-seekarte-spiel-grafik.md` (R2–R6, AC-02 bis AC-04). Bestehend (B-962): `native/k3c-game/hud/seekarte.go` (`seaView`, Knoten, `refresh`/`build`), `seekarte_input.go` (Bedienung, bleibt unverändert), `drawsea.go` (`drawSea`, `drawSeaNode`, `drawSeaMark`, Formen – bleibt als Rückfall), Tests `seekarte_test.go`. Route: `view.RouteIsland` (`native/k3c-game/view/frame.go`: ID, Name, Etappe) – für Größe und Schichtzahl um Stufenzahl und Inselgröße erweitern (Quelle: Welt-Datei über die lokale Sitzung). Assets und Zuordnung aus GS2.1 (`data/paper.json` › `seekarte`), Laden wie die Weltbilder (`native/k3c-game/art/worldimages.go`, `Library.Usable`). Aufsatz aus einem festen Hash der Insel-ID, kein Zufall zur Laufzeit. Overlay läuft je Spielerhälfte, der andere spielt weiter; keine Allokation je Frame.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- `native/k3c-game/art/**`
- `native/k3c-game/view/**`

## Nicht-Ziele

Bedienung und Zielwahl, Schiffsfahrt im Vollbild (GS2.3), Andenken (GS3), neue Assets.

## Schritte

1. Tests zuerst: Wahl von Miniatur, Größe, Schichten, Aufsatz je Insel (deterministisch, 2 Spieler gleich), Zustandszeichen, Rückfall ohne Assets, 0 Allokationen.
2. `art`: Zuordnung `seekarte` lesen (analog `WorldImages`).
3. `view.RouteIsland` um Stufenzahl und Größe ergänzen und füllen.
4. Neues Zeichnen in `hud/drawsea_paper.go`: Meer-Schichten, Miniaturen in Etappen-Spalten, Schiff an der aktuellen Insel, Zeichen; `drawSea` wählt Diorama oder Formen.
5. Fachprüfung `k3c-hud`, `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-02: Diorama mit Varianten und Zuständen, deterministisch, Tests aus B-962 grün (B-964/AC-02, AC-03).
- [ ] AC-03: Rückfall auf Formen, keine Allokation je Frame (B-964/AC-04).
- [ ] B-964/AC-05 (TV) bleibt offen bei 🧑.

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
