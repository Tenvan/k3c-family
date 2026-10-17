# B-452 · GEB7: Abnahme von Vorschau und Spiel durch Ralf

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** GEB7
- **Projekt:** GEB
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GEB7 ist umgesetzt und geprüft (GEB7.1–GEB7.5 fertig, Code auf `develop`, Review GEB7.6 ohne schwere Befunde); AC-08 verlangt die Abnahme der 15 Hintergrund-Schichten in Vorschau und Spiel durch 🧑. Agenten nehmen nicht ab. Das Wald-Gate (Wald vor GEB7.2/7.3 prüfen) entfiel im autonomen Lauf (Spec Revision 2); 🧑 prüft alle fünf Biome gemeinsam.

## Ziel

🧑 hat die Hintergründe gesehen und bestätigt oder Nachbesserungen benannt, dazu die Annahmen 1–9 aus GEB7 › Offene Fragen bestätigt oder ersetzt; GEB7 kann `erledigt` werden.

## Beteiligte und Zielgruppen

🧑 entscheidet; Agenten setzen Nachbesserungen um (Farbtausch oder Neuzeichnen der JSON, kein Code).

## Anforderungen

- Vorschau `assetvorschau.html?szene=gebaeude` › „Hintergründe“: je Biom drei Schichten, 32 px ×1 und 16 px ×2.
- Spiel: je Biom (Tiefe 0–4) drei Schichten in Maßstab 1, Fluss unter dem Hub ohne Spalt.
- Pages-Seite: Hero und Tiefen-Kacheln.
- Auffälligkeiten laut Session-Ergebnissen: Mine und Kristall in `mitte`/`nah` trennen sich nur schwach, Eisenstollen `fern`/`mitte` grob, Kristall `fern` wirkt rötlich-violett, Wald regelmäßig.

## Nicht-Ziele

Neue Bausteinarten im Generator (nur bei Antwort auf Frage 7), Animationen, Himmel und Paletten (eigenes REG-Ticket).

## Regeln und Einschränkungen

Abnahme gehört 🧑 (`docs/arbeitsweise.md` › SDD); Claude nimmt nicht selbst ab und prüft im Browser nur mit Freigabe für den Lauf.

## Beispiele

🧑 sagt „Abnahme“ → GEB7 AC-08 gilt; 🧑 nennt „Mine nah zu hell“ → Farbtausch in `hg-mine-nah.json`, neuer `task gebaeude`-Lauf.

## Ausnahme- und Fehlerfälle

nicht relevant (reine Abnahme).

## Akzeptanzkriterien

- **AC-01** 🧑 bestätigt GEB7 AC-08 (Vorschau, Spiel, Pages-Seite) und die Annahmen aus GEB7 › Offene Fragen, oder nennt Nachbesserungen.

## Offene Fragen

Sind Motive, Farben und Dichte der 15 Schichten so abgenommen, und gelten die Annahmen 1–9 (besonders Frage 5: Pages-Seite umgestellt, Frage 7: nur Rechtecke genügen)? Entscheidet 🧑.

## Notizen

Entspricht dem Ticket B-450 auf `origin/sprint/geb7` (nur dort angelegt, nie auf `develop`; die Nummer ist dort nicht mehr frei zu führen, deshalb B-452). Vorschau: `svc_start`, dann `assetvorschau.html?szene=gebaeude`.
