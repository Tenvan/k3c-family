# NG1.4 · Spielschleife, Split-Screen-Zellen und Platzhalter

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng1/4-spielschleife-zellen
- **Abhängig von:** NG1.2, NG1.3
- **Tickets:** B-792
- **Kriterien:** AC-07

## Ziel

Das Fenster zeigt 1–2 Zellen im Split-Screen mit Platzhaltern an den Positionen der echten Simulation und großer FPS-Anzeige.

## Kontext

Spec: B-792 (Schleife). Baut auf `local` (NG1.2) und `art` (NG1.3). Aufteilung der Zelle wie NV3: Welt in den oberen zwei Dritteln, Fluss im unteren Drittel (`native/k3c-ebiten/river.go`, `level.go`). Split-Screen-Regeln des Browsers: `src/scenes/layout.ts` (zwei Spieler übereinander oder nebeneinander, siehe Datei). FPS-Anzeige wie `native/k3c-ebiten/fps.go`. Taktung: Simulation 30 Hz über `local`, Zeichnen mit Bildrate; vorerst ohne Zwischenbilder (B-793 › Offene Fragen). Eingabe vorläufig direkt über Ebitengine im Platzhalter von `input` (zwei Pads oder Tastatur), bis NG5 sie ersetzt. Lage der Zellen und Ausschnitt als reine Funktionen testen.

## Erlaubte Dateien

- `native/k3c-game/main.go`
- `native/k3c-game/game/**`
- `native/k3c-game/input/**`
- `native/Taskfile.yml`
- Planung NG1, B-792 (über MCP)

## Nicht-Ziele

Echte Darstellung der Fachbereiche (NG2–NG8); Menü (Start direkt ins Spiel mit Schalter `--seed`).

## Schritte

1. Test zuerst: Zellen-Lage für 1 und 2 Spieler bei 1920×1080 und 1280×720, Welt-/Fluss-Anteil je Zelle.
2. `game`: Ebitengine-Schleife, Update taktet `local`, Draw ruft die Drawer je Zelle in Ebenen-Reihenfolge.
3. Platzhalter zeichnen Figuren und Gebäude aus `*sim.World` als Rechtecke.
4. FPS-Anzeige groß oben.
5. Task `native:game` startet das Spiel; Fenster einmal sehen und Screenshot ins Ergebnis.

## Fertig, wenn

- [ ] AC-07: Lage-Tests grün; `task native:game` zeigt zwei Zellen mit sich bewegenden Platzhaltern (Screenshot im Ergebnis).
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
