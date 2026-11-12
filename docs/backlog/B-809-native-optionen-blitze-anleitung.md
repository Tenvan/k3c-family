# B-809 · Die Optionen des nativen Spiels bieten Blitze und Anleitung an und merken sich „Anleitung gesehen“

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** NG13
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Der Browser-Client bietet in den Optionen die Schalter „Blitze“ und „Anleitung an/aus“ (`optionsLogic.ts:13`, `:29`) und merkt sich „Anleitung gesehen“. Ralf hat die Optionen des nativen Spiels am 2026-11-11 auf Lautstärke, Vollbild und Sprache festgelegt (B-800); die zwei Schalter fehlen deshalb.

## Ziel

Die Optionen des nativen Spiels bieten zusätzlich Blitze und Anleitung an, und „Anleitung gesehen“ bleibt dauerhaft in den Einstellungen gespeichert.

## Beteiligte und Zielgruppen

Ralf entscheidet, Spieler nutzen, Claude setzt um.

## Anforderungen

- Schalter „Blitze“ (Bildschirmblitze aus/an).
- Schalter „Anleitung“ an/aus.
- „Anleitung gesehen“ wird im Einstellungsspeicher des Spiels abgelegt.

## Nicht-Ziele

Weitere Optionen über B-800 hinaus.

## Regeln und Einschränkungen

Die Optionen aus B-800 (Lautstärke, Vollbild, Sprache) werden um genau diese zwei Schalter erweitert (entschieden, siehe Offene Fragen). Einstellungen nur additiv in `view.Settings` und im Dateiformat `_settings.json` (`local/settings.go`); fehlende Felder behalten den Standard (Blitze an, Anleitung an, nicht gesehen).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/menu/**`, `native/k3c-game/view/settings.go`, `native/k3c-game/view/settings_test.go`, `native/k3c-game/local/settings.go`, `native/k3c-game/local/settings_test.go`, `native/k3c-game/fx/**`, `native/k3c-game/hud/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Blitze aus → Nachtwelle ohne Bildschirmblitz. Anleitung aus → keine Hinweise, auch nach Neustart.

## Ausnahme- und Fehlerfälle

Einstellungsdatei fehlt oder ist defekt → Standardwerte, Log-Zeile.

## Akzeptanzkriterien

- **AC-01** Test: beide Schalter ändern den Zustand, Wert bleibt nach Speichern und Laden.
- **AC-02** Test: bei „Anleitung aus“ erscheint kein Hinweis.
- **AC-03** Test: „Anleitung gesehen“ überlebt einen Neustart (Einstellungsdatei).

## Offene Fragen

Entschieden autonom, Claude, 2026-11-12: Ja, die Schalter „Blitze“ und „Anleitung an/aus“ kommen zu den Optionen aus B-800 dazu; „Anleitung gesehen“ wird dauerhaft in den Einstellungen (`_settings.json`) gespeichert. Begründung: Gleichstand mit dem Browser-Client, Blitze sind eine Frage der Verträglichkeit. Der Pfad `settings/**` aus Revision 1 gab es nicht; Einstellungen liegen in `view/settings.go` und `local/settings.go`, die Wirkung in `fx` (Blitze) und `hud` (Anleitung).

## Notizen

Gefunden im Funktionsabgleich NG9.1 (2026-11-11). Bereich in Revision 2 an den tatsächlichen Ort der Einstellungen angepasst.
