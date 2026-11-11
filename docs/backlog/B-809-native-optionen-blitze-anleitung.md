# B-809 · Die Optionen des nativen Spiels bieten Blitze und Anleitung an und merken sich „Anleitung gesehen“

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Browser-Client bietet in den Optionen die Schalter „Blitze“ und „Anleitung an/aus“ (`optionsLogic.ts:13`, `:29`) und merkt sich „Anleitung gesehen“. Ralf hat die Optionen des nativen Spiels am 2026-11-11 auf Lautstärke, Vollbild und Sprache festgelegt (B-800); die zwei Schalter fehlen deshalb.

## Ziel

Die Optionen des nativen Spiels bieten zusätzlich Blitze und Anleitung an, und „Anleitung gesehen“ bleibt dauerhaft gespeichert, falls Ralf das will.

## Beteiligte und Zielgruppen

Ralf entscheidet, Spieler nutzen, Claude setzt um.

## Anforderungen

- Schalter „Blitze“ (Bildschirmblitze aus/an).
- Schalter „Anleitung“ an/aus.
- „Anleitung gesehen“ wird im Einstellungsspeicher des Spiels abgelegt.

## Nicht-Ziele

Weitere Optionen über B-800 hinaus.

## Regeln und Einschränkungen

Entscheidung von Ralf zu B-800 (Optionen: Lautstärke, Vollbild, Sprache) darf nur mit seinem Ja erweitert werden.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/menu/**`, `native/k3c-game/settings/**`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
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

Sollen „Blitze“ und „Anleitung“ zu den am 2026-11-11 festgelegten Optionen (B-800) dazukommen? Entscheidet Ralf. Ohne Ja wird das Ticket verworfen.

## Notizen

Gefunden im Funktionsabgleich NG9.1 (2026-11-11). Der Pfad `native/k3c-game/settings/**` ist ungeprüft; Bereich bei der Freigabe an den tatsächlichen Ort der Einstellungen anpassen.
