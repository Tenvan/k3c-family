# B-195 · Das Debug-Overlay lässt sich auf der Xbox öffnen

- **Domäne:** PLAT
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** PL1
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Debug-Overlay (U4, DBG2) öffnet sich per Ö oder Klick auf den linken Stick (LS, Taste 10, `src/scenes/debugOverlayView.ts`). Beim Test am 2026-10-03 auf der Xbox ließ es sich laut 🧑 nicht aufrufen, obwohl die Gamepad-Testseite LS am Controller #0 erkannt hat (X1.2).

## Ziel

Das Debug-Overlay ist mit dem Controller auf der Xbox erreichbar.

## Beteiligte und Zielgruppen

🧑 und die Familie am TV (Xbox, Edge), Entwickler im Client.

## Anforderungen

- Öffnen und Schließen mit dem Controller auf der Xbox, ohne B, View + Menu oder eine Spieltaste zu belegen.

## Nicht-Ziele

Leistungsziel des Servers (B-042, LT1).

## Regeln und Einschränkungen

`CLAUDE.md` › Seiten & Navigation und Regeln (B nicht belegen, View + Menu reserviert, 2 Spieler gleichzeitig). Komplexitäts-Budget.

## Beispiele

nicht relevant, die Beobachtung steht in der Ausgangslage.

## Ausnahme- und Fehlerfälle

nicht relevant, Ursache noch offen.

## Akzeptanzkriterien

- **AC-01** Ursache steht im Ticket (Code-Pfad und Beobachtung).
- **AC-02** 🧑 öffnet und schließt das Overlay auf der Xbox mit dem Controller.

## Offene Fragen

Auf welcher Seite wurde es versucht (`game.html`, `testing.html`)? Mit `?dev=0` in der Adresse? Klärt 🧑. Nicht blockierend (🧑, Chat, 2026-10-06): PL1.1 stellt das Problem zuerst auf `game.html` und `testing.html`, mit und ohne `?dev=0`, nach.

## Notizen

Gemeldet von 🧑 nach X1.2. Die Domäne hängt von der Ursache ab (Eingabe PLAT oder Overlay CLI).
