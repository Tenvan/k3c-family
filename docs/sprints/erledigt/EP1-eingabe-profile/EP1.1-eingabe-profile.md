# EP1.1 · Eingabe-Profile WASD, Pfeile, Gamepad je Spieler

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/ep1
- **Abhängig von:** –
- **Tickets:** B-893
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Eingabe-Profile `wasd`, `pfeile`, `gamepad` je Spieler in den Einstellungen; die Tastatur ist ein Gerät mit der Belegung des Tastatur-Profils; Optionen › Steuerung wählt das Profil je Spieler, höchstens einer hat die Tastatur.

## Kontext

- `native/k3c-game/input/bindings.go`: zwei Tastatur-Hälften (`halves`), `source.go` meldet beide als Geräte `keys1`, `keys2`.
- `native/k3c-game/view/settings.go`: `Settings` (vergleichbar per `==`), `local/settings.go` speichert sie als JSON.
- `native/k3c-game/menu/options.go`: Optionen-Menü; neue Unterseite Steuerung.
- `native/k3c-game/game/game.go` › `advance`: ruft `src.Poll()`; davor bekommt die Quelle das Tastatur-Profil.
- Szenen mit zwei Spielern (`game/scene.go`) und der Ende-zu-Ende-Test (`e2e/`) nutzten `keys2`: Spieler 2 wird `pad1`.
- HUD-Glyphen (`hud/glyphs.go`) folgen in EP1.2 dem Profil.

## Erlaubte Dateien

- `native/k3c-game/input/**`
- `native/k3c-game/view/**`
- `native/k3c-game/menu/**`
- `native/k3c-game/game/**`
- `native/k3c-game/hud/**`
- `native/k3c-game/devtools/**`
- `native/k3c-game/e2e/**`
- `native/k3c-game/local/settings*.go`

## Nicht-Ziele

- Icons je Profil (EP1.2).
- Rebinding einzelner Tasten.

## Schritte

1. `view`: Profile und `Settings.Controls` mit Standard, Normalisierung (höchstens ein Tastatur-Profil), `NextProfile`.
2. `input`: Belegungen `wasd` und `pfeile` für einen Spieler, ein Tastatur-Gerät, `SetKeyboard`.
3. `game`: Profil vor jedem Poll an die Quelle; Szenen und Tests ohne `keys2`.
4. `local`: `controls` speichern und laden.
5. `menu`: Optionen › Steuerung, Tastenbelegung mit Spalten WASD und Pfeile.
6. Tests, `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01: `input`-Test prüft je Profil alle Befehle und Menü-Aktionen.
- [ ] AC-02: `view`- und `menu`-Test: zweites Tastatur-Profil nicht wählbar.
- [ ] AC-03: `menu`-Test (sofort über `SetSettings`), `local`-Test (nach Neustart erhalten), `game`-Test (Quelle bekommt das Profil).
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

- **AC-01 geprüft:** `input/profile_test.go` › `TestEveryProfileHasAllActions` (jeder Befehl und jede Menü-Aktion in wasd und pfeile, Gamepad bis auf die Kombi Home), dazu die Tabellen-Tests `TestKeyCommandTable` und `TestKeyActionTable` je Profil.
- **AC-02 geprüft:** `view/profile_test.go` › `TestOneKeyboardPlayer` (Normalisierung und `NextProfile`), `menu/controls_test.go` › `TestControlsSwitchProfiles` (Spieler 2 bekommt die Tastatur erst, wenn Spieler 1 sie abgibt).
- **AC-03 geprüft:** sofort: `TestControlsSwitchProfiles` (SetSettings je Wechsel), `input` › `TestSetKeyboardSwitchesLayout`, `game/game.go` › `advance` stellt die Quelle vor jedem Poll um; nach Neustart: `local/settings_controls_test.go`.
- **Belegung (Vorschlag Claude, 🧑 bestätigt bei der Abnahme):** wasd wie bisher (A/D, Shift links, Leertaste, E, Q R T Z, K, Esc, Tab, Pos1); pfeile: ←/→, Shift rechts, Enter, Strg rechts oder Num 0, Skills Einfg/Entf/Ende/Bild ab oder Num 1–4, Skill-Menü ↓ oder Num 5, Esc/Rücktaste, Tab, Pos1, Bild ↑/↓. Tabelle in `native/k3c-game/input/bindings.go`. F9/F11 in beiden Profilen.
- **Entscheidungen:** Die Tastatur ist ein Gerät (`keys1`); `keys2` entfällt. Spielernummer folgt dem Lobby-Beitritt, das Profil bestimmt die Belegung. Ohne Tastatur-Profil bleibt die Tastatur mit WASD bedienbar. Szenen und Ende-zu-Ende-Lauf mit zwei Spielern nutzen jetzt Tastatur und Pad 1.
- **Abweichung:** `native/k3c-game/local/actions_test.go` angepasst (Erwartung mit normalisierten Einstellungen), außerhalb der Erlaubten Dateien, nur Test.
- **Befund (nicht aus dieser Session):** `e2e` › `TestFlow` (nur ohne `-short`) scheitert schon vor der Änderung bei „Spieler 2 findet einen Skill-Punkt“, `audio` › `TestEventCuesMatchBrowser` u. a. scheitern unverändert; beide vermutet vorbestehend auf `main`.
- `task check:diff` grün.
