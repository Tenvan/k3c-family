# B-278 · docs/protocol.md beschreibt, wie der Server langsame Geräte behandelt

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit N1.1 (B-276) verwirft der Server veraltete Zustände eines Geräts, das mit dem Lesen nicht nachkommt: Ein noch
nicht gesendeter Zustand wird durch den neuesten ersetzt, dessen Delta zum zuletzt gesendeten Zustand geht, und die
Ereignisse der verworfenen Zustände (höchstens 256) wandern in `events` des neuesten. Getrennt wird nur noch, wenn sich
64 andere Nachrichten stauen. `docs/protocol.md` › Takt sagt dagegen noch „pro Tick genau einen Zustand“ und „Sendepuffer
voll → Server schließt die Verbindung“. Das Nachrichtenformat ist unverändert; nur die Beschreibung stimmt nicht mehr.

## Ziel

Das Protokoll-Dokument beschreibt das tatsächliche Verhalten, Client-Entwickler verlassen sich nicht auf lückenlose Ticks.

## Beteiligte und Zielgruppen

Client (CLI), Server (SRV); 🧑 gibt frei.

## Anforderungen

- `docs/protocol.md` › Takt: Ein langsames Gerät bekommt höchstens einen Zustand je Tick, `tick` darf springen; `events`
  eines Deltas kann Ereignisse übersprungener Ticks enthalten (höchstens 256).
- Ein Delta gilt relativ zum **zuletzt an dieses Gerät gesendeten** Zustand, nicht zum Zustand des vorigen Ticks.
- `ack` ist das höchste seq, das im gesendeten Zustand verrechnet ist (N1.3).
- Trennung nur bei Stau anderer Nachrichten (64).

## Nicht-Ziele

Änderung des Nachrichtenformats (B-263, B-208).

## Regeln und Einschränkungen

Grenzfall Protokoll (`docs/arbeitsweise.md`): eigene Session, die nur das Protokoll und beide Enden anpasst.

## Beispiele

WLAN hängt 2 s → das Gerät bekommt danach ein Delta mit `tick` + 60 und allen Münz-Ereignissen der Pause.

## Ausnahme- und Fehlerfälle

Mehr als 256 Ereignisse in der Pause → die ältesten fehlen.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` › Takt nennt Verwerfen, springenden `tick`, gesammelte `events` übersprungener Ticks, Delta relativ zum zuletzt gesendeten Zustand und die Trennung bei 64 anderen Nachrichten.

## Offene Fragen

Behandelt der Client springende Ticks und Ereignisse mehrerer Ticks richtig (ungeprüft)? Klärt die Protokoll-Session.

## Notizen

Angelegt in N1.1 (`engine/net/ws.go` › `push`, `mergeEvents`), im Review N1.3 ergänzt (Delta-Bezug, `ack`).
