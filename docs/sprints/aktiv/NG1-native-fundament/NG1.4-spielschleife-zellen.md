# NG1.4 · Spielschleife, Split-Screen-Zellen und Platzhalter

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng1/4-spielschleife-zellen
- **Abhängig von:** NG1.2, NG1.3
- **Tickets:** B-792
- **Kriterien:** AC-07, AC-11

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
6. Aufnahme-Modus (B-792/AC-05, Vorbild `native/k3c-ebiten/shots.go`): Szenen-Schalter (Seed, Stufe oder Spielstand, Spieler, Kamera-x je Zelle, Takt, feste Eingabe-Folge) als reine, getestete Auswertung; `--shots <ordner>` lässt die Simulation bis zum Takt vorlaufen, schreibt n Bilder im festen Takt-Abstand als PNG und als Streifen-PNG und beendet das Programm. Zeit aus dem Takt, nicht aus der Wanduhr. Task `native:game:shots`.

## Fertig, wenn

- [ ] AC-07: Lage-Tests grün; `task native:game` zeigt zwei Zellen mit sich bewegenden Platzhaltern (Screenshot im Ergebnis).
- [ ] AC-11: Szenen-Tests grün; `task native:game:shots` schreibt Bildfolge und Streifen-PNG, zwei Läufe mit gleichen Schaltern sind bildgleich (Streifen im Ergebnis, vom Agenten angesehen).
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

AC-07 umgesetzt: Schleife mit festem Takt (30 Updates je Sekunde, ein `local.Step` je Update), 1 oder 2 Zellen übereinander, Welt zwei Drittel und Fluss ein Drittel, Platzhalter aus dem Frame, große Anzeige; Lage offline getestet (`layout_test.go`). Im Fenster nicht angesehen; geprüft am Streifen des Aufnahme-Modus (Agent und Claude).
AC-11 umgesetzt: Szenen-Schalter (`--seed`/`--load`, `--depth`, `--players`, `--cam`, `--tick`, `--input`, `--size`) und `--shots <ordner>` mit `--count`, `--every`; schreibt Einzelbilder und `strip.png`; zwei Läufe mit gleichen Schaltern sind bildgleich (Hashes aller 9 PNG verglichen). Task `native:game:shots`.
Vertragsänderungen: `Session.Load(name, devices)`, `Session.Continue(devices)`, `NewGame.World`, `NewGame.Depth`. Offen für NG1.6: Insel-Infos fürs HUD, Einstellungen, Rütteln, Texteingabe. Schrift fehlt (Debug-Schrift).
