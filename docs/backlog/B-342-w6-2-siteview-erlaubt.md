# B-342 · W6.2 darf den Bauplatz-Wartegrund in `siteView.ts` anbinden

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WRT
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

W6.2 (`docs/sprints/aktiv/W6-anzeige-wirtschaft/W6.2-hud-bauplaetze.md`) beschreibt den Bauplatz-Code in `src/scenes/worldRenderer.ts` (395 Zeilen, `missing()`, `key`). Seit GR3.1 (#136) liegt dieser Code in `src/scenes/siteView.ts` (209 Zeilen): `updateSiteView` baut den `key`, setzt den Text für `waitingMaterial` über `missing()` (Zeile ~156) und `missing()` selbst steht am Dateiende (Zeile ~204). `worldRenderer.ts` (302 Zeilen) enthält weder `missing()` noch den `key`. `siteView.ts` steht nicht in den Erlaubten Dateien von W6.2.

## Ziel

W6.2 kann den Wartegrund aus dem Server-Feld am Bauplatz zeigen und `missing()` entfernen, ohne die Erlaubten Dateien zu verlassen.

## Beteiligte und Zielgruppen

🧑 entscheidet; der nächste autonome Lauf von W6.2 setzt um.

## Anforderungen

- Erlaubte Dateien von W6.2 nennen `src/scenes/siteView.ts` (nur Anbindung: Aufruf von `siteStatusView.ts`, Wartegrund im `key`, `missing()` entfällt).
- Das Kriterium in „Fertig, wenn“ prüft `grep -n "missing(" src/scenes/siteView.ts src/scenes/worldRenderer.ts` (heute ist der Grep auf `worldRenderer.ts` schon leer und beweist nichts).

## Nicht-Ziele

Grafik und Sprites (GR3), weitere Änderungen an `siteView.ts` (Preisschild, `canAfford`).

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Autonomer Ablauf: nötige Datei nicht erlaubt → `blockiert`. `siteView.ts` gehört zu GR3; GR3 läuft nicht gleichzeitig mit W6.

## Beispiele

Bauplatz bezahlt, Material fehlt → `siteView.ts` ruft `siteStatusView.ts` mit `siteWait()` aus `wirtschaftAnzeige.ts`, Text „wartet auf Material (Stein)“ und Symbol am Platz.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Planungsfrage.

## Akzeptanzkriterien

- **AC-01** W6.2 nennt `src/scenes/siteView.ts` in den Erlaubten Dateien, der Kontext beschreibt `siteView.ts` statt `worldRenderer.ts`, und die Spec ist von 🧑 neu freigegeben.

## Offene Fragen

- Darf W6.2 `src/scenes/siteView.ts` ändern (Anbindung, `missing()` entfernen)? Entscheidet 🧑.

## Notizen

Gefunden im autonomen Lauf W6.2 am 2026-10-07 (Session `blockiert`).
