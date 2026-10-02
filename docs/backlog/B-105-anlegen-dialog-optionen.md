# B-105 · Der Anlegen-Dialog der Lobby wählt Grad, Ziel und Niederlage-Modus

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Lobby legt Räume mit Name und Spielstand an (`src/scenes/LobbyScene.ts`), ohne Optionen.

## Ziel

Beim Anlegen eines Raums wählt man Schwierigkeitsgrad, Ziel und Niederlage-Modus mit sinnvollen Standards. Nutzen: Familie stellt die Härte ein, ohne Entwickler-Werkzeug.

## Beteiligte und Zielgruppen

Spieler (Controller, Touch, Tastatur); 🧑 testet am Gerät.

## Anforderungen

- Auswahl Leicht bis Ultra (Standard Normal), Dev nur im Dev-Mode; Ziel und Niederlage-Modus mit Standard je Grad.
- Bedienung mit Controller (B bleibt unbelegt), Touch und Tastatur; 70 px oben frei.
- Anzeige der gewählten Optionen in der Raumliste und im Spiel.

## Nicht-Ziele

Debug-Panel (B-107), Simulation (B-101), Protokoll (B-104).

## Regeln und Einschränkungen

`CLAUDE.md` (Seiten, B-Taste, View + Menu), `docs/rules/stufen.md` § 5. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Raum anlegen: Grad Hart wählen → Niederlage-Modus springt auf Stufenverlust, lässt sich auf Komplett verloren ändern.

## Ausnahme- und Fehlerfälle

Server lehnt Option ab → Hinweis in der Lobby.

## Akzeptanzkriterien

- **AC-01** Testbare Auswahl-Logik (reine Funktion) für Standards je Grad und Dev-Sperre ist getestet.
- **AC-02** Die Lobby zeigt die Auswahl und sendet sie beim Anlegen.
- **AC-03** 🧑 hat den Dialog am Gerät abgenommen.

## Offene Fragen

keine

## Notizen

Aus R1.2 und R1.3. Abhängig von B-104.
