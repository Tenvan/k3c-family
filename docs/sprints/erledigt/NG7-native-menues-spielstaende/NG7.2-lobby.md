# NG7.2 · Lobby: Beitritt von 1–2 Spielern, Welt und Seed

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** menue
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng7/2-lobby
- **Abhängig von:** NG7.1
- **Tickets:** B-800
- **Kriterien:** AC-02

## Ziel

In der Lobby treten 1–2 Spieler mit A ihres Pads bei, wählen Welt und Seed und starten das Spiel.

## Kontext

Spec: B-800. Vorbild: `src/scenes/lobbyLogic.ts`, `lobbyOptions.ts`, `lobbyPad.ts`, `localSlots.ts`, `runStart.ts` samt Tests. Start über `view.Session` (Neues Spiel mit Spielern, Welt, Seed). Vertrag und Importregel wie NG7.1.

## Erlaubte Dateien

- `native/k3c-game/menu/**`
- Planung NG7, B-800 (über MCP)

## Nicht-Ziele

Online-Beitritt, Raum-Code, mehr als 2 Spieler.

## Schritte

1. Fälle aus den TS-Tests als Go-Tests (rot): Beitritt 1 und 2 Spieler, Austritt mit B, Welt und Seed.
2. Lobby-Bildschirm.
3. Start über eine Schein-`view.Session` im Test.

## Fertig, wenn

- [ ] AC-02: Tests grün.
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

AC-02 umgesetzt: Lobby mit Beitritt eines zweiten Geräts per A, drittes abgelehnt, Spieler 2 geht mit B; wählbar Grad, Ziel, Niederlage, Welt, Seed; Start über `Session.New` mit den Geräten in Beitrittsreihenfolge. Geprüft mit `lobby_test.go` und am Bild `ng7-menus/ng7-2-lobby.png` (Agent und Claude: zwei Spieler mit Gerät sichtbar, Auswahl klar).
Abweichung: Seed ist eine Zahl statt Freitext.
