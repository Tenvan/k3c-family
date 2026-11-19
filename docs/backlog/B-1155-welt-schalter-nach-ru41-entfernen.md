# B-1155 · Welt-Schalter der Features nach RU41 als Felder der Welt-Datei führen

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU62
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Features von RU62 gelten nur in Welten, die in `data/cargo.json`, `coop.json`, `festival.json` › `worlds` (und Wetter, Reittier, Einsiedler in ihren Dateien) stehen, damit Go-Golden und `testdata/` unverändert bleiben.

## Ziel

Nach RU41 (Go-Engine entfernen) führen die Welt-Dateien `weather`, `mount`, `hermit`, `festival` selbst; die Schalter `worlds` entfallen; Golden wird mit dem Frieden neu geschrieben.

## Beteiligte und Zielgruppen

Claude autonom, Golden-Neuschreiben mit Begründung.

## Anforderungen

- Zuordnung Welt auf Feature als Felder der Welt-Datei; Schalter `worlds` entfernt; Golden neu mit Begründung im Commit.

## Nicht-Ziele

Neue Mechaniken.

## Regeln und Einschränkungen

Erst nach RU41.

- **Bereich:** `data/**`, `rust/crates/k3c-data/**`, `rust/crates/k3c-sim/**`
- **Abhängig von:** RU41
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** golden

## Beispiele

nicht relevant: Schuld.

## Ausnahme- und Fehlerfälle

nicht relevant: Schuld.

## Akzeptanzkriterien

- **AC-01** Kein `worlds`-Schalter mehr in `data/*.json`; Tests grün; Golden neu begründet.

## Offene Fragen

keine

## Notizen

Aus der Review RU62.14.
