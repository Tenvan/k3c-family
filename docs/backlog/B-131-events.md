# B-131 · Vollmond, Blutmond und Händler-Überfall sind als Events umgesetzt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** K3
- **Projekt:** KMP
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, mit Sprint K3

## Ausgangslage

Es gibt keine Events; das GDD nannte nur „Wölfe bei Vollmond“ (`docs/rules/archiv/ist-gegner-bosse.md` § 3).

## Ziel

Jede 7. Nacht ist Vollmond (verstärkte Wolfswelle mit Alpha-Wolf), jede 13. Blutmond (+30 % Schaden, doppelter Drop), jeder 4. Händler-Besuch wird überfallen. Nutzen: Abwechslung im Rhythmus (`docs/rules/bosse.md` § 2).

## Beteiligte und Zielgruppen

Spieler; Werte pflegt REG.

## Anforderungen

- `data/events.json`: Vollmond (jede 7. Nacht, +50 % Wolfsanteil, Alpha-Wolf als Elite, 50 Gold), Blutmond (jede 13. Nacht, +30 % Schaden, doppelter Drop), Händler-Überfall (jeder 4. Besuch, 100 Material bei Schutz).
- Ereignisse `eventStarted`, `eventEnded` für Anzeige und Messung.
- Wirkung unten: Annahme, dass Events die Nacht des globalen Zyklus betreffen (Klärung im Ticket).

## Nicht-Ziele

Darstellung (B-132), Händler selbst (B-121), Alpha-Wolf-Sprite (B-010).

## Regeln und Einschränkungen

`docs/rules/bosse.md` § 2; deterministisch. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Nacht 7: Vollmond, das Banner „Vollmond!“ erscheint, die Welle hat mehr Wölfe und einen Alpha-Wolf.

## Ausnahme- und Fehlerfälle

Vollmond und Blutmond in derselben Nacht (Nacht 91) → beide wirken.

## Akzeptanzkriterien

- **AC-01** Test: Auslöser nach Rhythmus; Wirkung auf Wellen, Schaden und Drops laut Daten.
- **AC-02** Test: Händler-Überfall nur bei Besuch, Belohnung bei Schutz.
- **AC-03** Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

Wirkung unter Tage (Aggressionspool) und Rhythmus-Zähler im Spielstand.

## Notizen

Aus R4.3. Abhängig von B-121 (Händler).
