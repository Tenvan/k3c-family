# B-103 · Inseln mit Endboss und gemeinsamem Inselwechsel sind spielbar

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** K2
- **Projekt:** KMP
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1

## Ausgangslage

Es gibt eine Kampagne mit drei Stufen (Wald, Höhle, Mine), keine Inseln und keine Bosse.

## Ziel

Eine Insel hat Minibosse je Stufe und einen Endboss in der tiefsten Stufe; dessen Sieg macht den Inselwechsel frei (gemeinsam), die nächste Insel ist eine neue Welt mit eigener Skalierungstabelle. Nutzen: Fortschritt über mehrere Inseln, Ziel „Endboss“.

## Beteiligte und Zielgruppen

Spieler; 🧑 legt Boss-Werte in Regelwerk III fest.

## Anforderungen

- Insel-Daten (Stufen, Skalierungstabelle je Insel); erste Ausbaustufe 1 Insel mit 3 Stufen.
- Miniboss je Stufe, Endboss in der tiefsten Stufe; Werte aus Regelwerk III (B-004/AC-03).
- Inselwechsel nur gemeinsam (alle lebenden Spieler am Boot/Portal) nach dem Endboss; neue Insel mit eigenen Hubs.

## Nicht-Ziele

Weitere Inseln über die erste hinaus (Inhalt), Boss-Grafik (CLI).

## Regeln und Einschränkungen

`docs/rules/stufen.md` §§ 1, 3; Entscheidung 003. Boss-Werte erst nach Regelwerk III. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Endboss der Mine fällt → Boot erscheint, alle stehen dort → Insel 2 beginnt mit Skalierung der Tabelle für Insel 2.

## Ausnahme- und Fehlerfälle

Spieler tot beim Wechsel zählt nicht (wie heute). Nur ein Spieler am Boot → Wechsel wartet.

## Akzeptanzkriterien

- **AC-01** Test: Endboss besiegt gibt den Inselwechsel frei, vorher nicht.
- **AC-02** Test: Wechsel nur mit allen lebenden Spielern am Punkt; neue Insel hat neue Hubs und die Tabelle der Insel.
- **AC-03** Test: je Stufe ein Miniboss, Endboss in der tiefsten Stufe.
- **AC-04** Spielstand speichert die aktuelle Insel und besiegte Bosse.

## Offene Fragen

Anzahl n der Inseln (offen, mit Insel 1; `docs/rules/stufen.md` § 7), nicht blockierend für K2.

Entschieden 2026-10-06 (🧑, `docs/rules/bosse.md` § 1 und § 1.1): Boss-Werte sind die vorläufigen Startwerte des Regelwerks (Miniboss etwa 8× HP und 2× Schaden, Endboss etwa 30× HP, 3 Phasen).

## Notizen

Aus R1.3 und B-108: klassisch Insel 1 bis n (`data/islands.json`), neue Insel mit leerem Material-Vorrat. Die Variante „Ebenen“ (mehrere Inseln je Ebene, nächste Ebene nach mindestens k besiegten Inseln) kommt später als eigenes Ticket. Abhängig von B-100 und Regelwerk III.

R4 (2026-10-02): Die Bosse selbst (Werte, Auslöser, Belohnung) stehen in `docs/rules/bosse.md` und werden in B-130 umgesetzt; B-103 bleibt für Inseln, Inselwechsel und den Endboss-Sieg als Auslöser.
