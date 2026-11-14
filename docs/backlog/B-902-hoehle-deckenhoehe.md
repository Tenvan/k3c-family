# B-902 · Die Höhle zeigt ihre Deckenhöhe (Natur-Eigenschaft ceilingHeight)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BI2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Höhle hat seit B-891 die Natur-Eigenschaft `ceilingHeight` (Deckenhöhe 1–3, nur Darstellung). Der Wert steht in `Layout.Nature`; das native Spiel liest ihn noch nicht.

## Ziel

Das native Spiel zeichnet die Höhlendecke niedrig, mittel oder hoch nach dem Wert der Stufe.

## Beteiligte und Zielgruppen

Spieler der Höhle; Claude setzt um; Ralf 🧑 prüft im Abnahme-Ticket.

## Anforderungen

- Biome mit der Eigenschaft `ceilingHeight` zeichnen oben im Welt-Teil eine Felsdecke (Band über die sichtbare Breite) mit gezackter Unterkante.
- Unterkante der Decke in Units über dem Boden: sichtbare Höhe (420/32 Units) minus Dicke; Dicke 3,5 Units bei Wert 1, 2,15 bei 2, 0,8 bei 3 (linear).
- Zacken an der Unterkante alle 1,5 Units, Tiefe 0,3–0,8 Units aus der Position (`mix`).
- Gezeichnet hinter allen anderen props (zuerst in der Zeichenliste).

## Nicht-Ziele

Kollision oder Spielregeln der Decke; Papierschnitt-Asset der Decke.

## Regeln und Einschränkungen

Wert aus `World.Level.Nature`, fehlt er: `default` des Bioms. Formen der Zeichenliste (`shapeRect`, `shapeTri`), keine neue Abhängigkeit.

- **Bereich:** `native/k3c-game/props/decor.go`, `native/k3c-game/props/decor_test.go`, `native/k3c-game/props/plan.go`
- **Abhängig von:** B-891
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Höhle mit Wert 1 → Decke reicht bis 9,6 Units über dem Boden; mit 3 → bis 12,3 Units.

## Ausnahme- und Fehlerfälle

Biom ohne `ceilingHeight` (Wald, Mine …) → keine Decke. Wert außerhalb 1–3 → auf den Bereich begrenzt.

## Akzeptanzkriterien

- **AC-01** In der Höhle plant props eine Decke; in Biomen ohne `ceilingHeight` nicht (Unit-Test).
- **AC-02** Die Unterkante der Decke steigt mit dem Wert: Wert 1 < Wert 2 < Wert 3; ohne Wert im Level gilt der Default (Unit-Test).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
