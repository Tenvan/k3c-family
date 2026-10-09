# B-279 · docs/protocol.md beschreibt den Eingabe-Takt so, wie der Client ihn seit N2 sendet

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** S2
- **Projekt:** –
- **Erstellt:** 2026-10-04
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`docs/protocol.md` › Takt sagt „höchstens eine `input` pro Tick“. Seit N2.2 (B-277) sendet der Client eine geänderte Eingabe sofort, mit mindestens 8 ms Abstand (`INPUT_MIN_GAP_MS` in `src/online/clientConnection.ts`), also bis zu einer je Bild. Der Server rechnet weiter mit der zuletzt empfangenen Eingabe und begrenzt die Rate nicht; die Protokoll-Datei lag nicht in den erlaubten Dateien von N2.

## Ziel

Vertrag und Verhalten stimmen überein; niemand baut am Server eine Grenze „eine je Tick“ ein, die die sofortige Eingabe wieder bremst.

## Beteiligte und Zielgruppen

Entwickler (SRV, CLI).

## Anforderungen

- `docs/protocol.md` › Takt nennt den neuen Eingabe-Takt (sofort bei Änderung, Mindestabstand im Client, Keepalive 500 ms); Protokollversion bleibt.

## Nicht-Ziele

Rate-Begrenzung am Server.

## Regeln und Einschränkungen

Vertrag `docs/protocol.md`; keine Protokolländerung.

## Beispiele

Spieler tippt links-rechts innerhalb eines Ticks → zwei `input`, der Server verrechnet die letzte.

## Ausnahme- und Fehlerfälle

nicht relevant (reine Doku).

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` › Takt beschreibt den Eingabe-Takt wie `sendInput` in `src/online/clientConnection.ts`.

## Offene Fragen

keine

## Notizen

Entstanden in N2.2.
