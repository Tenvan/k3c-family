# B-826 · Die Optionen des nativen Spiels schalten zwischen Fenster, rahmenlos und Vollbild um

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** NG19
- **Projekt:** NAT
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-13 (Dialog)

## Ausgangslage

Optionen › Vollbild ist ein Schalter an/aus (`native/k3c-game/menu/options.go`, `view.Settings.Fullscreen`, `game/game.go` `applyFullscreen`). Ein rahmenloses Fenster in Monitorgröße fehlt. Wunsch Ralf 2026-11-13.

## Ziel

Optionen › Anzeige wählt zwischen Fenster (mit Rahmen), rahmenlos (Fenster ohne Rahmen in Monitorgröße) und Vollbild; die Wahl wird gespeichert und sofort angewendet.

## Beteiligte und Zielgruppen

Spieler am PC und Mini-PC; 🧑 Ralf (Wunsch); Claude (Umsetzung).

## Anforderungen

- `view.Settings.Window`: "" = Fenster (Standard), `rahmenlos`, `vollbild`; ersetzt `Fullscreen`. Gespeichert als `window` in den Einstellungen (alte Dateien ohne Migration, Beschluss „Keine Rücksicht auf alte Spielstände“).
- Optionen: Zeile „Anzeige  < Vollbild >“, Links/Rechts/A wechseln zyklisch.
- F bzw. rechter Stick: Vollbild ↔ Fenster (aus rahmenlos → Vollbild).
- `game` wendet die Wahl an: Vollbild über `ebiten.SetFullscreen`; rahmenlos ohne Rahmen, Größe des Monitors, Position 0/0; Fenster mit Rahmen in der Startgröße, mittig. Nicht im Aufnahme-Modus.

## Nicht-Ziele

Exklusives Vollbild (bietet Ebitengine nicht), Auflösungswahl, Monitorwahl.

## Regeln und Einschränkungen

Entscheidung 008, Import-Regeln `game/imports_test.go`. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

- **Bereich:** `native/k3c-game/view/settings*.go`, `native/k3c-game/local/settings*.go`, `native/k3c-game/menu/options*.go`, `native/k3c-game/menu/texts.go`, `native/k3c-game/game/game.go`, `native/k3c-game/game/play_test.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Optionen › Anzeige, Rechts: Fenster → rahmenlos → Vollbild → Fenster.

## Ausnahme- und Fehlerfälle

Unbekannter Wert in der Datei → Fenster. Aufnahme-Modus ändert das Fenster nie.

## Akzeptanzkriterien

- **AC-01** Test in `menu`: Links/Rechts/A wechseln die drei Modi zyklisch, die Zeile zeigt den Modus in beiden Sprachen.
- **AC-02** Test in `view`/`local`: `Window` wird gespeichert und geladen, unbekannt → Fenster.
- **AC-03** Test in `game`: F wechselt Vollbild ↔ Fenster; `task native:game:e2e` grün.
- **AC-04** Sichtprüfung am PC durch 🧑 (alle drei Modi).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
