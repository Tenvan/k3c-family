# B-194 · Der Split-Screen läuft auf der Xbox flüssig

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** PF1
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beim Test am 2026-10-03 (X1.2) hat 🧑 auf der Xbox (Edge 150, Server `pi-gaming`) im Split-Screen der Testseite (`testing.html`) sehr starkes Ruckeln gesehen. Der Sprite-Test derselben Sitzung lief mit 2000 Sprites bei 60 FPS (`docs/game-design.md` › Xbox-Messung); die reine Zeichenlast ist also vermutlich nicht die Ursache (ungeprüft).

## Ziel

Zwei Spieler im Split-Screen spielen auf der Xbox ohne sichtbares Ruckeln.

## Beteiligte und Zielgruppen

🧑 und die Familie am TV (Xbox, Edge), Entwickler im Client.

## Anforderungen

- Ursache gemessen: Client (FPS, Zeichenzeit je Kamera) oder Netz/Server (Snapshot-Takt, Latenz, Pi-Last).
- Split-Screen mit 2 Spielern läuft auf der Xbox flüssig.

## Nicht-Ziele

Leistungsziel des Servers (B-042, LT1).

## Regeln und Einschränkungen

`CLAUDE.md` › Seiten & Navigation und Regeln (B nicht belegen, View + Menu reserviert, 2 Spieler gleichzeitig). Komplexitäts-Budget.

## Beispiele

nicht relevant, die Beobachtung steht in der Ausgangslage.

## Ausnahme- und Fehlerfälle

nicht relevant, Ursache noch offen.

## Akzeptanzkriterien

- **AC-01** Eine Messung am Gerät nennt FPS im Split-Screen und Snapshot-Abstand, Ursache steht im Ticket.
- **AC-02** Nach der Korrektur zeigt das Debug-Overlay im Split-Screen mit 2 Spielern auf der Xbox im Mittel ≥ 55 FPS (🧑 am Gerät).

## Offene Fragen

Welche Szenario-Einstellung lief (Mock-Spieler, Layout, Stufe)? Ruckelt auch `game.html` mit zwei Controllern oder nur die Testseite? Klärt 🧑.

## Notizen

Gemeldet von 🧑 nach X1.2. Sprite-Budget und Audio: `docs/game-design.md` › Xbox-Messung.
