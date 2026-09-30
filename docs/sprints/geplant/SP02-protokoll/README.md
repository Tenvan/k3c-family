# SP02 · SRV · Protokoll v2 & Raummodell

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-030, B-036, B-038, B-039
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Online-Protokoll (`src/online/`) kennt einen Monarchen pro Gerät und ist nirgends beschrieben.

## Ziel

Protokoll und Raummodell für mehrere Räume, mehrere lokale Spieler pro Gerät und gemischten Couch-/Online-Koop sind beschlossen. Am Ende sichtbar: `docs/protocol.md`, Beispiel-Nachrichten in `testdata/protocol/`, `docs/decisions/002-protokoll-v2.md`.

## Beteiligte und Zielgruppen

🧑 entscheidet das Raummodell; die Umsetzung in SP07 (Server) und SP08 (Client) arbeitet danach.

## Anforderungen

B-030, B-036 und B-038 › Anforderungen (Entwurf), B-039 (Snapshot-Takt). Sprint-eigen: Nachrichten für Eingaben pro lokalem Spieler, Snapshot (voll/Delta), Level-Übertragung, Takt 30 Hz, Versionsfeld.

## Nicht-Ziele

Engine-Code. Die Umsetzung folgt in SP07 (Server) und SP08 (Client).

## Regeln und Einschränkungen

Eine Protokoll-Änderung betrifft Client und Server (Grenzfall Protokoll in `docs/arbeitsweise.md`). Entscheidung 001.

## Beispiele

2 Controller an der Xbox + 1 Handy im selben Raum → je Nachricht ein JSON-Beispiel in `testdata/protocol/`.

## Ausnahme- und Fehlerfälle

Das Protokoll legt das Verhalten bei falscher Version, Abbruch und vollem Raum fest.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt Raum, Gerät und lokale Spieler sowie Beitreten, Verlassen und Wiederverbinden.
- **AC-02** Je Nachricht liegt ein JSON-Beispiel in `testdata/protocol/`; die Snapshot-Größe für 4 Spieler ist geschätzt.
- **AC-03** Entscheidung 002 ist beschlossen.

## Offene Fragen

Raumcode oder Lobby? Grenzen für Räume und Geräte? (🧑, SP02.1)

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SP02.1 🧑 Raummodell: Raum, Gerät, lokaler Spieler; Beitreten/Verlassen/Wiederverbinden; Raumcode oder Lobby; Grenzen; Spielstand pro Raum. Beispiel „2 an der Xbox + 1 Handy“ (AC-01).
- SP02.2 Nachrichten: Eingaben pro lokalem Spieler, Snapshot (voll/Delta), Level-Übertragung, Takt 30 Hz, Versionsfeld; je Nachricht ein JSON-Beispiel, Snapshot-Größe für 4 Spieler geschätzt (AC-02).
- SP02.3 🔍 Review + Entscheidung 002 (AC-03, alle).

## Abnahme

–
