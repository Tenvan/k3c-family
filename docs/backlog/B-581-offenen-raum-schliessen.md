# B-581 · Ein offener Raum lässt sich in der Lobby über Entfernen schließen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bei der Abnahme BD5.4 (2026-10-24, PC) zeigte sich: Offene Räume stehen in der Spielstand-Liste der Lobby, lassen sich aber nicht löschen (gewollt, BD5.1).

## Ziel

Bei einem offenen Raum bietet „Entfernen“ (X bzw. Entf) die Option, den Raum zu schließen; danach lässt er sich löschen.

## Beteiligte und Zielgruppen

Spieler am Gerät; 🧑 nimmt ab.

## Anforderungen

- Entfernen auf einem offenen Raum bietet „Raum schließen“ mit Rückfrage.
- Schließen speichert den Stand und wirft verbundene Clients sauber in die Lobby.

## Nicht-Ziele

Automatisches Löschen nach dem Schließen.

## Regeln und Einschränkungen

Protokoll laut `docs/protocol.md`; Server-Seite SRV, Client CLI; 2 Spieler.

## Beispiele

Offener Raum, X → „Raum schließen?“ → Ja → Raum zu, Spielstand bleibt, danach löschbar.

## Ausnahme- und Fehlerfälle

Mitspieler ist noch verbunden → Rückfrage nennt es.

## Akzeptanzkriterien

- **AC-01** Entfernen auf einem offenen Raum bietet „Schließen“ (Test und Beobachtung am PC).
- **AC-02** Nach dem Schließen lässt sich der Spielstand löschen.

## Offene Fragen

Wer darf schließen, wenn mehrere Geräte im Raum sind? Default: jedes Gerät mit Rückfrage; 🧑 entscheidet.

## Notizen

Aus BD5.4 (Ralf, 2026-10-24). Spec: Entwurf.
