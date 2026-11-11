# NG7.1 · Menü-Gerüst und Hauptmenü

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** menue
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng7/1-menue-geruest
- **Abhängig von:** NG1
- **Tickets:** B-800
- **Kriterien:** AC-01

## Ziel

Paket `menu` hat ein Menü-Gerüst mit Bildschirmen (`view.Screen`) und das Hauptmenü: Weiter, Neues Spiel, Laden, Optionen, Beenden, mit Pad bedienbar, B zurück.

## Kontext

Spec: B-800. Modul `native/k3c-game` (NG1): `view.Screen`, `view.Session` und Menü-Aktionen aus `view/`; Platzhalter in `menu/` ersetzen, `game/` und `view/` unverändert; kein Import von `local` oder `input`. Vorbild: `src/scenes/menuPages.ts`, `menuOpener.ts`, `paperCard.ts`, `src/scenes/LobbyScene.ts` (Menü-Teil). Online-, Raum-Code- und Werkzeug-Einträge entfallen. Superpowers TDD.

## Erlaubte Dateien

- `native/k3c-game/menu/**`
- Planung NG7, B-800 (über MCP)

## Nicht-Ziele

Lobby (NG7.2), Spielstände (NG7.3), Optionen und Pause (NG7.4).

## Schritte

1. Tests zuerst: Auswahl hoch/runter, A wählt, B zurück, Übergänge zwischen Bildschirmen.
2. Menü-Gerüst (Stapel von Bildschirmen).
3. Hauptmenü mit Papier-Karten-Look wie `paperCard.ts`.

## Fertig, wenn

- [ ] AC-01: Tests grün; Screenshot des Hauptmenüs im Ergebnis.
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
