# B-588 · Abbau-Schilder zeigen eine Axt für Bäume und eine Spitzhacke für Felsen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** GEB16
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25

## Ausgangslage

Bauschilder (B-584) gibt es nur an Bauplätzen. Ralf (2026-10-24): Für den Abbau schön wären eine Axt und eine Spitzhacke als Symbol.

## Ziel

Ein Abbau-Schild zeigt eine Axt (Baum) bzw. eine Spitzhacke (Fels, Erz).

## Beteiligte und Zielgruppen

Spielende; 🧑 entscheidet über Aussehen und Ort; Grafik über das Asset-Studio.

## Anforderungen

- Zwei Symbole als Pixel-Grafik: Axt (Baum), Spitzhacke (Fels, Stein).
- Ein Abbau-Schild je abbaubarem Baum bzw. Fels (Entscheidung Ralf, 2026-10-24: pro Baum gewollt).
- Abbaubar sind nur Bäume an Waldrändern (Tiefe 5) und Felsen/Stein, wenn sie frei liegen; das Schild zeigt nur, was der Server als abbaubar meldet (Prinzip B-587).

## Nicht-Ziele

Neue Abbau-Regeln; Wimpel der Markierung (GEB10).

## Regeln und Einschränkungen

Nur zeichnen (ADR 001); Grafik über das Studio; Client zeigt nur, was der Server meldet (`marked`, B-587-Prinzip).

## Beispiele

Markierter Baum → Axt-Schild am Baum.

## Ausnahme- und Fehlerfälle

Grafik fehlt → kein Symbol (nie ein falsches).

## Akzeptanzkriterien

- **AC-01** Axt und Spitzhacke liegen als Grafik im Katalog und in der Studio-Vorschau.
- **AC-02** Das Schild erscheint am vorgesehenen Ort (Beobachtung am Gerät).

## Offene Fragen

Server-Seite: Meldet der Snapshot schon, welche Bäume (Waldrand, Tiefe 5) und Felsen (frei liegend) abbaubar sind? Falls nicht, braucht es ein Feld wie `payable` in B-587 (z. B. `mineable` je Knoten); in der Spec klären.

## Notizen

Aus der Abnahme am 2026-10-24. Ort entschieden: je Baum, nur abbaubare. Spec: Entwurf.
