# B-209 · `src/model/data.ts` kennt alle Platz-Arten aus `hub.json`

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`src/model/data.ts` (Zeile 65–66) typisiert `HUB.sites[].kind` fest als `'wall' | 'tower' | 'workshop' | 'storage' | 'stairsUp' | 'stairsDown'`. Mit festen Bauplätzen (Q43, B-206) kommen Kaserne, Taverne, Heilplatz, Schmiede, Rüstkammer, Händler, Tor, Farm-Weltplatz, Mauerlinien und Angebots-Anhänge in die Daten; der Typ würde sie verschweigen oder der Import bräche.

## Ziel

Der Client-Typ passt zu den Daten, neue Platz-Arten fallen beim Typecheck auf statt am TV.

## Beteiligte und Zielgruppen

Entwickler CLI.

## Anforderungen

- Der Typ von `HUB` deckt alle Platz-Arten und -Felder aus `data/hub.json` ab (auch Linien und Anhänge, sobald B-206 sie anlegt).
- Ein Test prüft, dass jede `kind` aus `data/hub.json` im Typ bzw. einer Liste des Clients bekannt ist.

## Nicht-Ziele

Anzeige (B-207); Protokoll (B-208); Regel-Logik im Client.

## Regeln und Einschränkungen

`src/model/` enthält nur Typen und Daten, keine Logik (`CLAUDE.md` › Struktur). Datei ≤ 400 Zeilen.

## Beispiele

B-206 ergänzt `{ "kind": "barracks", … }` in `data/hub.json` → der Test schlägt fehl, bis `data.ts` die Art kennt.

## Ausnahme- und Fehlerfälle

Unbekannte Art in den Daten → Test rot mit Name der Art.

## Akzeptanzkriterien

- **AC-01** Test: Jede `kind` aus `data/hub.json` ist im Client bekannt.
- **AC-02** `task check` grün.

## Offene Fragen

keine

## Notizen

Folge aus Beschluss Q43 (2026-10-04, zweite Runde). Nach B-206 umsetzen.
