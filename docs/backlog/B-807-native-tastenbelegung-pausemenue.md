# B-807 · Das Pause-Menü des nativen Spiels zeigt die Tastenbelegung

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** NG13
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Der Browser-Client zeigt die Tastenbelegung als Seite des Pause-Menüs (`src/scenes/keyTable.ts`). Das native Spiel hat sie nicht, der Hinweis `native/k3c-game/hud/hints.go:5` verweist aber darauf. Die Daten liegen in `native/k3c-game/input/bindings.go` und `hud` (KeyName); das Menü (`native/k3c-game/menu`) importiert diese Pakete nicht. Verwandt: B-741 (HUD ohne Tastenhinweise, Belegung im Menü) gilt dem Browser-Client.

## Ziel

Im Pause-Menü des nativen Spiels ist eine Seite mit der Tastenbelegung (Tastatur, beide Hälften, Gamepad) erreichbar.

## Beteiligte und Zielgruppen

Spieler am PC und Mini-PC, Claude (Umsetzung), Ralf (Abnahme).

## Anforderungen

- Seite „Tasten“ im Pause-Menü, Inhalt aus den Bindings, nicht doppelt gepflegt.
- Zurück per Esc und Gamepad B wie in den übrigen Menüseiten.
- Hinweis in `hud/hints.go` stimmt danach.

## Nicht-Ziele

Umbelegen von Tasten, Tastenhinweise im HUD (B-741, B-755).

## Regeln und Einschränkungen

Importregeln der Pakete: `menu` darf `input` und `hud` nicht direkt importieren; die Belegung kommt über den Vertrag `view` (entschieden, siehe Offene Fragen). Datei ≤ 400 Zeilen.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/menu/**`, `native/k3c-game/view/**`, `native/k3c-game/game/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Pause → Tasten → Liste je Aktion und Spielerhälfte; B → zurück zum Pause-Menü.

## Ausnahme- und Fehlerfälle

Kein Gamepad angeschlossen → Tastaturspalten bleiben, Gamepad-Spalte zeigt die Standardbelegung.

## Akzeptanzkriterien

- **AC-01** Test in `menu`: Seite zeigt zu jeder Aktion der Bindings eine Zeile, Zurück führt ins Pause-Menü.
- **AC-02** Importregel-Test des Moduls bleibt grün.
- **AC-03** Ende-zu-Ende-Lauf (`task native:game:e2e`) bleibt in allen Stationen gleich.

## Offene Fragen

Entschieden autonom, Claude, 2026-11-12: Über den Vertrag `view`, nur additiv: `view` bekommt einen Typ für die Zeilen der Tastenbelegung (Aktion, Tastatur links, Tastatur rechts, Gamepad), `game` (kennt alle Pakete) baut die Zeilen aus `input/bindings.go` und den Beschriftungen aus `hud` und reicht sie an `menu` weiter. Begründung: hält die Importregel (`game/imports_test.go`) ein, eine Quelle für die Belegung, kein Eingriff in `input` und `hud`.

## Notizen

Gefunden im Funktionsabgleich NG9.1 (2026-11-11).
