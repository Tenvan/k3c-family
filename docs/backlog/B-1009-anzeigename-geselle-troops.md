# B-1009 · Der Beruf craftsman heißt in data/troops.json Geselle statt Handwerker

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RG2
- **Projekt:** BAL
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Befund aus RG2.2 (2026-11-15): Der Beruf `craftsman` trägt in `data/troops.json` (Zeile 4) den Anzeigenamen „Handwerker“. Seit B-950 ist der Handwerker der Bauer mit Hammer, der Beruf heißt laut Glossar „Geselle“.

## Ziel

Anzeigename und Glossar stimmen überein: `craftsman` heißt „Geselle“.

## Beteiligte und Zielgruppen

Spieler (Texte im Spiel), Entwickler.

## Anforderungen

- `data/troops.json`: `name` von `craftsman` ist „Geselle“.
- Texte im Spiel und Doku unter `docs/data/` nennen denselben Namen.

## Nicht-Ziele

ID `craftsman` umbenennen, Werte ändern.

## Regeln und Einschränkungen

- **Bereich:** `data/troops.json`, `docs/data/troops.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –
- Nur Anzeigename, kein Spielwert; Golden prüfen (ändert sich Golden, stoppen).

## Beispiele

Werkstatt-Angebot zeigt „Geselle“.

## Ausnahme- und Fehlerfälle

nicht relevant: reiner Text.

## Akzeptanzkriterien

- **AC-01** `grep -n '"Handwerker"' data/troops.json` ohne Treffer bei `craftsman`; `go test ./engine/...` und datadoc-Test grün, Golden unverändert.

## Offene Fragen

keine

## Notizen

Aus RG2.2, Befund 3.
