# B-1076 · Sim-Panics (Go-Nil-Dereferenz, Ereignis, Tag/Nacht-Zyklus) und Test-Fixtures in Result umbauen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU43
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU42 setzt `unwrap_used`, `expect_used`, `panic` auf `deny`. Drei Stellen in `k3c-sim` tragen `STAB(B-1076)`: `trade_at` (`expect` auf `w.merchant`, Go-Nil-Dereferenz), `event` (`panic` bei Nicht-Objekt), `first_day_night` (`panic` ohne Tag/Nacht-Biom).

## Ziel

Die drei Stellen geben `Result` oder `Option` zurück, die `STAB`-Markierungen sind entfernt.

## Beteiligte und Zielgruppen

Agenten setzen um.

## Anforderungen

- Fehlerweg nach Port-Orakel (Entscheidung 012) festlegen, Aufrufer anpassen.

## Nicht-Ziele

Neue Sim-Regeln.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-sim/src/state/**`
- **Abhängig von:** RU42
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant.

## Ausnahme- und Fehlerfälle

Golden bleiben unverändert.

## Akzeptanzkriterien

- **AC-01** Kein `STAB(B-1076)` mehr im Code, `task rust:lint` und `task rust:test` grün.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
