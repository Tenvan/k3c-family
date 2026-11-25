# B-1064 · RU16 Abnahme: Sim, Balance, Gen und Sichtprüfung Gebäude und Objekte

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** BAL11
- **Projekt:** BAL
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU16 (Rust-Port `props`, Standbild, Bildvergleich an sieben Ständen) schließt ohne Sim-, Balance- und Gen-Läufe ab (B-742).

## Ziel

Die gesperrten Läufe und die Sichtprüfung im Spiel sind nachgeholt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

`check:race`; Golden-Vergleiche nur als Tests (Ausnahme B-1014); Port-Tests grün.

Prüfpunkte für RU39 (ungemessen): Allokationen je Frame in `props/paint.rs` (`label`: Text-Klon, wachsender Breiten-Cache), `RefCell<Library>` über das ganze Zeichnen (Review MITTEL/NIEDRIG).

Bekannt: `new_pending` ruft in 8 Methoden `unimplemented!()`, nicht verdrahten, bis RU15/RU18 die Naht ersetzen (B-1053); Text 2 px tiefer als Go (B-1063); acht Tests `ignore = "wartet auf RU10"` (`island()`); Bildtests laufen nur mit gerenderter Grafik (`K3C_GRAFIK_DIR`, `K3C_STILL_REQUIRE=1`).

## Nicht-Ziele

Neue Funktionen; Nachzüge B-997, B-998, B-1001, B-994.

## Regeln und Einschränkungen

Sim-Sperre vom 2026-11-11 gilt, Golden nie neu schreiben.

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `task check:race` ist grün oder als nicht lauffähig begründet (kein gcc); die acht ignorierten Tests laufen, sobald RU10 `island()` liefert.
- **AC-02** Ralf hat im Spiel geprüft: halbtransparente Figuren und Gefechtsmarken heller (B-1061), Deckkraft von Glut und Kristall-Schein, Tönung und Text der Gebäude gegen die Go-Version, Baustellen und Mauer-Ruine.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Lane `zweit`: Der Second Worker fährt dieses Ticket (Sim, Balance und Gen; die Sichtprüfung bleibt bei Ralf).
