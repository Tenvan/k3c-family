# B-384 · Inselwechsel: Wechsel-Bestätigung (confirmIsland) passt nicht zum Presence-Wechsel der Sim

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

K4.2 plant die Nachricht `confirmIsland {slot}`, die eine Sim-Funktion „Wechsel bestätigen“ aufruft. Die Sim (K2, `engine/sim/island_switch.go`) kennt keine Bestätigung: `stepIslandSwitch` setzt `SwitchReady`, wenn alle lebenden, gesteuerten Spieler `hub.Travel.Seconds` im Bereich `hub.Travel.RangeUnits` des Wechselpunkts stehen (Anwesenheit); der Raum tauscht dann die Insel (B-345). Es gibt keine Eingabe, keinen Zustand je Spieler und keine Funktion, die der Server aufrufen könnte.

## Ziel

Entscheidung, ob und wie ein Gerät den Inselwechsel bestätigt.

## Beteiligte und Zielgruppen

Entwickler (SIM, SRV); 🧑 entscheidet.

## Anforderungen

- Der Server rechnet die Regel nicht nach (nur Sim).

## Nicht-Ziele

Darstellung (K5).

## Regeln und Einschränkungen

`docs/rules/stufen.md` § 1: Wechsel gemeinsam, alle lebenden Spieler am Punkt. B-154/AC-02: Bestätigung vor dem Endboss-Sieg → `bad_request`.

## Beispiele

Variante A: Anwesenheit bleibt der Wechsel, `confirmIsland` entfällt (B-154/AC-02 und K4.2 ändern sich). Variante B: SIM führt eine Bestätigung je Spieler ein (Zustand, Funktion, Test), `SwitchReady` verlangt sie zusätzlich zur Anwesenheit.

## Ausnahme- und Fehlerfälle

Bestätigung vor dem Sieg über den Endboss → `bad_request`.

## Akzeptanzkriterien

- **AC-01** Entscheidung 🧑 A oder B steht im Sprint K4 (Spec-Änderung) bzw. als SIM-Session.

## Offene Fragen

A oder B? 🧑. Ebenso: AC-06 (Dev-Aktion Schwierigkeitsgrad, B-080/AC-02) steht in der README, aber in keinem Schritt und keiner erlaubten Datei von K4.2.

## Notizen

Gefunden in K4.2 (Status `blockiert`).
