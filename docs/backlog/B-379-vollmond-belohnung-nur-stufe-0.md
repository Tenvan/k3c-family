# B-379 · Die Vollmond-Belohnung gilt für die Stufen, in denen gespielt wird

- **Domäne:** SIM
- **Typ:** Problem
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

Vollmond verstärkt die Welle in jeder Stufe der Insel (`fullMoonExtra`, `engine/sim/events_moon.go`), die Belohnung zahlt `nightEventsEnd` aber nur in Stufe 0: 50 Münzen am Hub der Stufe 0 und nur, wenn deren Burg hielt (`castleFellNight`). Spielen alle in einer tieferen Stufe, liegen die Münzen in einer leeren Stufe, und ein Burgfall dort zählt nicht (Review K3.3).

## Ziel

Die Belohnung erreicht die Spieler, die die Vollmondnacht verteidigt haben.

## Beteiligte und Zielgruppen

Spieler im Koop; 🧑 entscheidet die Auslegung.

## Anforderungen

- Belohnung und Burg-Bedingung je Stufe mit Spielern (oder eine bewusst andere Regel), deterministisch, 2+ Spieler in verschiedenen Stufen.

## Nicht-Ziele

Anzeige (K5), Werte (BR2).

## Regeln und Einschränkungen

`docs/rules/bosse.md` § 2; Golden-Läufe ohne Insel unverändert.

## Beispiele

Zwei Spieler in Stufe 1, Burg der Stufe 1 hält in Nacht 7 → Belohnung am Hub der Stufe 1.

## Ausnahme- und Fehlerfälle

Burg der Stufe 1 fällt, Stufe 0 hält → keine Belohnung für Stufe 1.

## Akzeptanzkriterien

- **AC-01** Test: Belohnung und Burg-Bedingung gelten für die Stufe, in der gespielt wird.

## Offene Fragen

Je Stufe mit Spielern oder nur die tiefste besetzte Stufe? (🧑)

## Notizen

Gefunden im Review K3.3.
