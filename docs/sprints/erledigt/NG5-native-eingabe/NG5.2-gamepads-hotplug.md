# NG5.2 · Gamepads, Hot-Plug, Tastatur-Rückfall und Zurück-Tasten

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng5/2-gamepads-hotplug
- **Abhängig von:** NG5.1
- **Tickets:** B-798
- **Kriterien:** AC-02, AC-03

## Ziel

1–2 Gamepads werden Spielern zugeordnet, können während des Spiels kommen und gehen, die Tastatur springt ein; B und View + Menu wirken wie vorgeschrieben.

## Kontext

Spec: B-798. Ebitengine: `ebiten.AppendGamepadIDs`, `IsStandardGamepadLayoutAvailable`, `StandardGamepadButton`; Vorbild `native/k3c-ebiten/players.go`, `src/scenes/localSlots.ts`, `lobbyPad.ts`. Regeln `CLAUDE.md`: B in Menüs zurück, im Spiel unbelegt; View + Menu = zurück ins Menü. Zuordnung als reine Funktion über eine Pad-Liste testen (ohne echtes Gerät). Eingabe-Code: Reviewer (Sonnet) parallel zu `task check:diff`.

## Erlaubte Dateien

- `native/k3c-game/input/**`
- Planung NG5, B-798 (über MCP)

## Nicht-Ziele

Beitritt in der Lobby (NG7.2 nutzt die Aktionen); Controller-Familien.

## Schritte

1. Testfälle: Pad kommt, Pad geht, zwei Pads, kein Pad (Tastatur), Pad ohne Standard-Belegung (rot).
2. Pad-Verwaltung mit stabiler Zuordnung Spieler ↔ Pad.
3. B und View + Menu als Menü-Aktionen.
4. Einmal mit zwei echten Pads im Fenster prüfen, wenn vorhanden (sonst im Ergebnis vermerken).

## Fertig, wenn

- [ ] AC-02: Zuordnungs-Tests grün.
- [ ] AC-03: Tests für B und View + Menu grün.
- [ ] `task check:diff` grün, Review ohne schwere Befunde.

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

AC-02 umgesetzt: Pads nach `ebiten.GamepadID` sortiert, eigener Zustand je Pad, neu verbundenes Pad löst im ersten Update nichts aus, Pads ohne Standard-Belegung werden einmal gemeldet und ignoriert; geprüft mit `TestDevicesHotPlug`, `TestNoCrosstalk` (vier Geräte), `TestPollAllocs` (0 Allokationen).
AC-03 umgesetzt: View + Menu gemeinsam liefert nur `ActionHome`, Menu kurz beim Loslassen, B nur Zurück und nie ein Befehl; geprüft mit `TestPadMenuViewHome`.
Offen: Vertrag kennt keine Aktion für Skill-Menü und Vollbild; Haltezeiten rechnen mit 30 Updates je Sekunde. Am echten Gerät ungeprüft (zwei Pads, Ein- und Ausstecken, RT, Ziffernblock).
