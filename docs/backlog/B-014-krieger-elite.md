# B-014 · Krieger und Elite-Truppen sind umgesetzt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** W4
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Es gibt Bauern und Bogenschützen, keine Nahkämpfer und keine Elite-Truppen.

## Ziel

Krieger und Elite-Truppen sind umgesetzt. Nutzen: Truppen-Management ist die Hauptrolle des Monarchen.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen gleichzeitig, lokal und online); Umsetzung durch Entwickler oder Agent in Go.

## Anforderungen

- Krieger (Schwert, Nahkampf): Posten hinter der äußersten gebauten Sperre (mit Tor zwischen Mauer und Tor, ohne Tor hinter der Mauer), Reichweite 1; Seitenverteilung wie bei Bogenschützen als Startwert (Beschlüsse Q46, 2026-10-04, ändert Q38). Schwert-Zahlziel an der Werkstatt, `dx +4` (Beschluss Q53).
- Elite-Upgrades mit Stein/Kupfer.

## Nicht-Ziele

Weitere Truppentypen.

## Regeln und Einschränkungen

Neue Mechaniken nur in Go (Entscheidung 001, Feature-Stopp in `src/world/`); deterministisch, nur der Seed-RNG, kein `Math.random()`; Werte in `data/`; Komplexitäts-Budget.

## Beispiele

Schwert in der Werkstatt → eine Einheit wird Krieger und kämpft im Nahkampf.

## Ausnahme- und Fehlerfälle

Wird im Regelwerk festgelegt (siehe Offene Fragen).

## Akzeptanzkriterien

- **AC-01** Die Werkstatt bietet Schwerter.
- **AC-02** Das Elite-Upgrade wirkt laut Daten.
- **AC-03** Tests decken beides ab.

## Offene Fragen

Verhalten bei fehlendem Stein/Kupfer (Regelwerk, 🧑).

## Notizen

–

R3 (2026-10-02): Krieger, Elite und Rüstung sind in `docs/rules/buerger.md` beschlossen; die Umsetzung steht in B-122.
