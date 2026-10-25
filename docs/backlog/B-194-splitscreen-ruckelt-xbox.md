# B-194 · Der Split-Screen läuft auf der Xbox flüssig

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** PF1
- **Projekt:** LST
- **Erstellt:** 2026-10-03
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25

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

Messung PF1.1 (Claude, 2026-10-25, Browser-Pane Desktop, WebGL, Spielstand `familie`, Wald, 2 Spieler per Tastatur). Die Game-Loop ist von Hand getaktet, gemessen ist die CPU-Zeit je `game.loop.step` über 600 Frames:

| Fall | Ø | p50 | p95 | p99 | davon Render | davon Update |
|---|---|---|---|---|---|---|
| 1 Kamera, Tag (1 Spieler) | 1,09 ms | 0,90 | 2,00 | 2,80 | – | – |
| 2 Kameras, Tag | 2,60 ms | 2,40 | 3,60 | 4,90 | 2,23 | 0,34 |
| 2 Kameras, Nacht (Welle) | 2,74 ms | 2,40 | 4,30 | 8,10 | 2,31 | 0,41 |
| Nacht, 2. Kamera ausgeblendet | 2,33 ms | 2,00 | 4,00 | 5,10 | – | – |
| 2 Kameras, Container außerhalb beider Sichten unsichtbar | 2,74 ms | 2,10 | 3,50 | 4,60 | – | – |

Zustands-Abstand: 29,6 Zustände/s, p50 33 ms, p95 45 ms, max 125 ms (das Pane läuft womöglich gedrosselt).
Einmalige Spitzen bis 316 ms (vermutet: Nachladen oder GC beim Beitritt).

Vermutete Hauptlast: Rendern (rund 85 % der Frame-Zeit). Die Spielszene hat rund 650 Objekte, davon 145 Welt-Container, von denen nur rund 25 in einer der Kameras liegen. Phaser zeichnet Container ohne Kamera-Culling in jeder Kamera, deshalb kostet die zweite Kamera am Desktop nur ca. 0,4 ms CPU.
Auf der Xbox zählt vermutlich die GPU-Füllrate (zwei Viewports mit je bis zu 7 bildschirmbreiten Parallax-Schichten) mehr; im Pane ist das nicht messbar (vermutet).
Senkbar im Client: Welt-Container außerhalb aller Kamerasichten nicht zeichnen (gemessen p50 −12 %).
