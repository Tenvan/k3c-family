# B-1082 · Island::stage_ref klont den Grad (Grade mit String) je Stufenschritt und Tick

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU10
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`Island::stage_ref` (`rust/crates/k3c-sim/src/island/mod.rs`, RU10.1) baut je Stufe und Tick einen `IslandRef` und klont dabei `Grade` (enthält `default_defeat: String`) aus der Schwierigkeitstabelle. Das ist eine kleine Allokation je Stufenschritt im Takt-Pfad (Leitfaden § 11). `IslandRef.grade` ist als Wert aus RU11.1 vorgegeben.

## Ziel

Kein Allokieren je Stufenschritt beim Aufbau des Insel-Bezugs.

## Beteiligte und Zielgruppen

Entwickler der Rust-Sim (Projekt RSU); Leistungsmessung RU43.

## Anforderungen

- Gleiches Verhalten wie heute (deterministisch, 2+ Spieler).
- Messung vor Änderung (`alloc_je_bild`).

## Nicht-Ziele

Änderungen an den Spielwerten.

## Regeln und Einschränkungen

Leitfaden `docs/standards/rust-port.md` § 11.

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `stage_ref` allokiert nicht mehr je Aufruf (z. B. `Grade` ohne `String` oder Bezug per `Arc`), gemessen mit `alloc_je_bild`.

## Offene Fragen

keine

## Notizen

Gefunden in RU10.1.
