# B-560 · Ralf entscheidet, ob die Dieb-Skills gebaut werden oder das Preset leer bleibt

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** KR5
- **Projekt:** KRT
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Preset Dieb nannte `backstab` und `smokeBomb`, die in `data/monarch.json › skills` fehlen (B-504). SK2.2 hat autonom den Default umgesetzt: `presets.thief.activeSkills` ist leer, das Laden prüft jetzt jedes Preset. `docs/rules/monarch.md`: „der Dieb folgt später als Inhalt“; kein Spieler startet heute als Dieb.

## Ziel

🧑 entscheidet, ob der Dieb-Inhalt jetzt gebaut wird.

## Beteiligte und Zielgruppen

🧑 entscheidet; SIM baut bei Bedarf.

## Anforderungen

- Optionen: (a) Preset leer lassen, Dieb später (Default, umgesetzt); (b) Backstab und Smoke Bomb jetzt bauen (game-design: Backstab 40 / 80 von hinten, CD 12 s; Smoke Bomb unsichtbar 5 s, CD 25 s) als neuer SIM-Sprint mit Balancing; (c) Preset Dieb ganz entfernen.
- Entscheidet 🧑 (b): neuer Sprint im Projekt SKL (Wirkungstypen `backstab`, `stealth`, Golden-Änderung, BAL-Lauf). (c): eine Zeile in `data/monarch.json`, Studio-Seite prüfen.

## Nicht-Ziele

Werte-Runde der Skills.

## Regeln und Einschränkungen

Regeländerung mit Balancing-Wirkung bleibt bei 🧑.

## Beispiele

nicht relevant: Entscheidungsfrage.

## Ausnahme- und Fehlerfälle

nicht relevant: Entscheidungsfrage.

## Akzeptanzkriterien

- **AC-01** Entscheidung von 🧑 steht hier und in SK2 › Offene Fragen; Regeltext nachgezogen (SK2/AC-01).

## Offene Fragen

(a), (b) oder (c)? Default (a).

## Notizen

Sprint KR5, Session KR5.1; Workshop SK2.1.

**Entscheidung 🧑 (KR5.1, 2026-10-23):** (b), Backstab und Smoke Bomb werden jetzt gebaut. Umsetzung: B-577 (SIM-Sprint im Projekt SKL mit Golden und BAL-Lauf). Bis dahin bleibt das Preset Dieb leer.
