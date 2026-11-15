# GS3.3 · Spiel: Andenken, Abschlusszeile und Zahlen auf der Seekarte (B-966)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs3
- **Abhängig von:** GS3.2
- **Tickets:** B-966
- **Kriterien:** AC-03

## Ziel

Die Seekarte im Spiel zeigt auf abgeschlossenen Inseln Trophäe und Fahne und, mit dem Zeiger darauf, Abschlusszeile und Zahlen.

## Kontext

Spec: `docs/backlog/B-966-chronik-der-reise.md` (R3–R6, AC-05, AC-06). Chronik aus GS3.1 (Lesefunktion der Engine), Trophäen aus GS3.2 (`data/paper.json` › `seekarte` › `trophaeen`), Abschlusszeile = erste Zeile von `story.done` der Insel (GS1, `engine/sim/story.go`). Overlay: `native/k3c-game/hud/seekarte.go` (Knoten, Zeiger), `seekarte_input.go` (heute springt der Zeiger nur auf wählbare Inseln – neu darf er abgeschlossene ansteuern, A wirkt dort nicht), `drawsea_paper.go` (Diorama aus GS2), Rückfall `drawsea.go`. Route und Zustand kommen über `native/k3c-game/view/` und `native/k3c-game/local/`. Text „<Tage> Tage · <n> Monarchen“ mit Einzahl; Texte in `hud/texts.go`. Mit 2 Spielern, keine Allokation je Frame.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- `native/k3c-game/view/**`
- `native/k3c-game/local/**`

## Nicht-Ziele

Vorschau im Menü (GS3.5), Format der Chronik, neue Assets, Nachlesen aller Zeilen.

## Schritte

1. Tests zuerst: Trophäe auf abgeschlossener Insel; Zeiger darauf zeigt Zeile und Zahlen; ohne `done` nur Zahlen; ohne Chronik nur Trophäe; A wählt nichts; 2 Hälften gleich; 0 Allokationen.
2. Chronik und Abschlusszeile in den Frame (`view`) reichen.
3. Zeiger-Logik erweitern, Bedienung der Wahl unverändert.
4. Zeichnen: Trophäe samt Fahne, Zeile und Zahlen unter der Miniatur; Rückfall ohne Asset.
5. Fachprüfung `k3c-hud`, `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-03: Trophäe, Abschlusszeile und Zahlen für abgeschlossene Inseln, für beide Spieler gleich (B-966/AC-05, AC-06).
- [ ] B-966/AC-07 (TV) bleibt offen bei 🧑.

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
