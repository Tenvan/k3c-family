# NG9.1 · Ende-zu-Ende-Test und Funktionsabgleich mit dem Browser-Client

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng9/1-ende-zu-ende
- **Abhängig von:** NG2, NG3, NG4, NG5, NG6, NG7, NG8
- **Tickets:** B-802
- **Kriterien:** AC-01, AC-02

## Ziel

Ein Ende-zu-Ende-Test ohne Fenster läuft vom Start bis zum Laden eines Spielstands, und eine Liste zeigt, was nativ gegenüber `src/scenes/` fehlt.

## Kontext

Spec: B-802. Alle Fachpakete sind gemergt (NG2–NG8). Der Test treibt `game` mit festen Eingaben (Typen aus `view`) ohne Fenster: Hauptmenü → Lobby mit 2 Spielern → kurzes Spiel (einige hundert Takte) → Speichern → Hauptmenü → Weiter → gleicher Welt-Hash. Das ist ein Integrationstest, kein Sim- oder Balancing-Lauf. Funktionsabgleich: alle Darstellungs-Dateien in `src/scenes/` (render, hud, hints, effects, Szenen) mit Gegenstück nativ ja/nein; jedes Nein als Ticket im Projekt NAT (`plan_create`, Vorlage). Opus wegen Zusammenspiel aller Pakete.

## Erlaubte Dateien

- `native/k3c-game/game/**`
- `native/k3c-game/e2e/**`
- `native/Taskfile.yml`
- Planung NG9, B-802, neue Tickets (über MCP)

## Nicht-Ziele

Lücken selbst schließen (Tickets); Änderungen in den Fachpaketen (Befund → Ticket); Sim-, Balance- und Lasttest-Läufe.

## Schritte

1. Test zuerst in `e2e/` (rot).
2. In `game` nur Verdrahtung nachziehen, die das Zusammenspiel braucht.
3. Task `native:game:e2e`.
4. Funktionsabgleich als Liste ins Ergebnis, Lücken als Tickets.

## Fertig, wenn

- [ ] AC-01: `task native:game:e2e` grün.
- [ ] AC-02: Liste im Ergebnis, Tickets angelegt.
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

AC-01 umgesetzt: Ende-zu-Ende-Lauf `native/k3c-game/e2e` (`TestFlow`) treibt die echte Schleife ohne Fenster: Hauptmenü, Lobby mit zwei Tastatur-Hälften, laufen, Münzen an die Mauer, Mauer gebaut, Skill über das Skill-Menü gelernt, Dämmerung, Nacht und Welle, Pause, Spiel verlassen mit Speichern, Weiter (Welt-Hash gleich dem direkt über `engine/room` geladenen Stand), Spieler 2 tritt wieder bei, Niederlage, Hauptmenü. Zwei Läufe gleich in allen 13 Stationen, auch im Bild (`TestFlowImages`, `task native:game:e2e`). Bilder `reports/native-game-e2e/a/`. AC-02 umgesetzt: Funktionsabgleich mit dem Browser-Client (Ereignisse, Texte, Effekte, Ton, Menüs ohne Abweichung; Lücken als Tickets B-806 bis B-816).
