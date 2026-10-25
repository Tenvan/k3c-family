# B-623 · Stufenfolge des Hubs (3 Schritte je Hub-Stufe, Material-Tor) bestätigen

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** KRT
- **Erstellt:** 2026-10-25
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Workshop GEB17.2 (Stufenfolge des Hubs, B-592) war Ralf zugedacht. Laut Ralfs Ziel („ohne Nachfragen, kritische Nachfragen in KRT“) hat Claude am 2026-10-25 vorentschieden, damit GEB17.3 bis GEB17.7 laufen können.

## Ziel

Ralf bestätigt die Vorentscheidung oder ändert sie.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

Vorentscheidung (autonom, Claude):
- Die fünf Hub-Stufen und alles, was an ihnen hängt (Gebäude, Mauerlinien, Tor), bleiben.
- Jede Hub-Stufe hat **3 Schritte** (`hubStep` 1–3). Schritt 1 und 2 kosten Gold und Material der laufenden Stufe, Schritt 3 hebt die Hub-Stufe und braucht das Material der nächsten Inselstufe (Material-Tor, Wartegrund „Material der nächsten Insel“).
- Kosten: Die bisherigen Kosten je Hub-Stufe werden auf die drei Schritte verteilt (ca. 25/25/50 %), ohne Balancing.
- Protokoll: additives Feld `hubStep`, keine neue Version; keine Rücksicht auf alte Spielstände.
- Grafik: je Schritt eine unterscheidbare Ansicht der Burg (B-590).

## Nicht-Ziele

Balancing der Kosten (eigener Sprint).

## Regeln und Einschränkungen

Spec-Änderung nach SDD; die Umsetzung in GEB17 folgt der Vorentscheidung.

## Beispiele

Hub-Stufe 1, Schritt 3 → Ausbau auf Hub-Stufe 2 braucht Kupfer aus Tiefe 1.

## Ausnahme- und Fehlerfälle

nicht relevant

## Akzeptanzkriterien

- **AC-01** 🧑 hat die Stufenfolge bestätigt oder geändert; `docs/rules/stufen.md` spiegelt die Entscheidung.

## Offene Fragen

- Passen 3 Schritte je Hub-Stufe und das Material-Tor? Entscheidet 🧑.

## Notizen

Herkunft: GEB17.2, Sprint GEB17 (2026-10-25).
