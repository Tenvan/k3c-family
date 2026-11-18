# B-1098 · Abnahme-Läufe RU21: Sim, Balance, Gen und Race für Rust-Store und Raum (Lane zweit)

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU21
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU21 hat Store und Raum nach Rust portiert (`rust/crates/k3c-room`). Die Abnahme-Läufe sind auf dem Hauptrechner gesperrt (Sim, Balance, Gen, Race). Lane: zweit (nur der Second Worker arbeitet dieses Ticket ab).

## Ziel

Die gesperrten Läufe sind für den Stand von RU21 durchgeführt und belegt.

## Beteiligte und Zielgruppen

Second Worker (Lane zweit); entwickelt wird nichts.

## Anforderungen

- Läufe laufen auf dem Stand von `main` nach dem Merge von RU21.
- Rote Läufe werden eigene Tickets.

## Nicht-Ziele

Neue Funktionen. Die ignorierten Tests (RU18, RU24) schaltet deren Sprint frei.

## Regeln und Einschränkungen

Sim und Balancing nur über `sim_test`; `docs/arbeitsweise.md` › Sim, Balance und Gen. Lane: zweit.

- **Bereich:** `rust/crates/k3c-room/**`
- **Abhängig von:** RU21
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: reines Abnahme-Ticket.

## Ausnahme- und Fehlerfälle

nicht relevant: ein roter Lauf wird ein neues Ticket.

## Akzeptanzkriterien

- **AC-01** Sim: `sim_test` auf dem Stand von RU21 ohne Befund (oder als nicht betroffen begründet: RU21 ändert keinen Sim-Code).
- **AC-02** Balance: kein Wert in `data/` geändert, Lauf nur falls nötig.
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Golden grün.
- **AC-04** Race: `check_run task:check:race` und `cargo test -p k3c-room` ohne Befund; die Tests mit `ignore` (RU18, RU24) bleiben außen vor.

Abnahme-Ticket eines Sprints (B-742), Einzelheiten in `docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

keine

## Notizen

Review-Befund RU21 (nicht behoben, Ticket-Stoff für RU24): `Manager::create` endet bis RU24 in `unimplemented!` (`create_in`/`load_in`); `Fs::log` Retry-Zeilen holt kein Aufrufer mit `take_log` ab.
