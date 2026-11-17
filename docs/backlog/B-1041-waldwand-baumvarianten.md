# B-1041 · Waldbäume in mehreren Höhen und Tönen ergeben eine geschlossene Waldwand

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** BB1
- **Projekt:** PAP
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`docs/standards/bildaufbau.md` § 5: In K2C schließt der dichte Wald das Bild nach oben ab, viele Stämme stehen in mehreren Höhen und Tönen hintereinander. Unsere Waldbäume (`objekte/tanne`, `pappel`, `birke` mit Varianten) erreichen keine geschlossene Wand.

## Ziel

Ungerodeter Wald wirkt dicht und dunkel; der Himmel ist nur in Lücken zu sehen.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (PAP), Fachprüfung `k3c-grafik`. Abnahme: 🧑.

## Anforderungen

- Je Baumart mindestens drei Höhen (Nadelbaum 4–6 M, Laubbaum 2,5–3,5 M) und eine dunkle Variante für die hintere Reihe.
- Ein Mittelgrund-Asset „Waldwand“ (Hintergrund-Rig) mit Stämmen und geschlossenem Kronendach.
- Texturbudget einhalten (`k3c-grafik`).

## Nicht-Ziele

Platzierung im Level-Generator, Wechsel nach Rodung (B-1047).

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/objekte/**`, `public/grafik/hintergruende/**`, `data/paper.json`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Monarch reitet am Inselrand: hinter ihm dunkle Stämme in drei Reihen, oben nur Kronen.

## Ausnahme- und Fehlerfälle

Tiefe Stufen (Höhle, Mine) haben keinen Wald und brauchen das nicht.

## Akzeptanzkriterien

- **AC-01** Varianten und Waldwand sind über `assets_*` angelegt und in `data/paper.json` zugeordnet, `k3c-grafik` ohne schweren Befund.
- **AC-02** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Notizen

Gedächtnis `reference_k2c-bildauswertung` (Bilder 1, 13).
