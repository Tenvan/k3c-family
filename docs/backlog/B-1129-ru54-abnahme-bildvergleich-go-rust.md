# B-1129 · RU54 Abnahme (Lane zweit): Bildvergleich Go gegen Rust, Farbe und Pause im Fenster

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU54
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU54 behebt zwei Smoke-Befunde (Pausenmenü verdeckt B-1127, Farben blass B-1128). Der Fensterweg (GPU) hat keine Aufnahme; B-1120 (RU32 Abnahme, Lane zweit) deckt den Bildvergleich Go/Rust über Aufnahmen ab.

## Ziel

Abnahme: Bildvergleich Go gegen Rust im Fenster (Farbe, Pause über dem Spiel) und Bericht.

## Beteiligte und Zielgruppen

Second Worker (Lane zweit), Ralf liest den Bericht.

## Anforderungen

- Farbwerte ausgewählter Pixel (Himmel, Papierkarte, HUD) im Rust-Fenster gegen Go vergleichen (Bildschirmaufnahme des Fensters).
- Pause im Fenster mit Zellen und HUD aufnehmen.

## Nicht-Ziele

Sim, Balancing, Gen, check:race.

## Regeln und Einschränkungen

Schreiben nur im Repo (.work/), Aufnahmen danach löschen. Lane: zweit.

- **Bereich:** `reports/**`
- **Abhängig von:** B-1127, B-1128, B-1120
- **Braucht 🧑:** Gerät
- **Größe:** S
- **Sperre:** –

## Beispiele

Himmel (0x0b1026) im Go-Fenster und im Rust-Fenster gleich bis auf +-2 je Kanal.

## Ausnahme- und Fehlerfälle

Weicht ein Wert ab: Ticket mit Ursache, kein Umbau in dieser Abnahme.

## Akzeptanzkriterien

- **AC-01** Farbwerte Rust gegen Go innerhalb +-2 je Kanal an den genannten Stellen.
- **AC-02** Pause im Fenster zeigt Karten über Welt und HUD.
- **AC-03** Ergebnis mit B-1120 abgestimmt.

## Offene Fragen

keine

## Notizen

–
