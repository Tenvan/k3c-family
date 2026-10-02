# B-104 · Das Protokoll kennt die Stufe je Spieler und die Raum-Optionen

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Protokoll v2 (`docs/protocol.md`, `engine/net/protocol*.go`, `src/online/protocol.ts`) schickt jedem Gerät den Zustand einer Welt; es kennt weder Stufen je Spieler noch Raum-Optionen.

## Ziel

Snapshots enthalten je Spieler seine Stufe (und die Stufen, die ein Gerät sehen muss); `create` nimmt Raum-Optionen (Grad, Ziel, Niederlage-Modus) entgegen. Nutzen: Client kann Kamera und Anlegen-Dialog bauen.

## Beteiligte und Zielgruppen

Entwickler (Client und Server); eigene Session laut Arbeitsweise › Protokoll.

## Anforderungen

- Neue oder erweiterte Nachrichten für Stufe je Spieler, Insel, Stufenwechsel und Raum-Optionen; Testdaten in `testdata/protocol/`.
- Abwärtsverhalten: Version erhöhen, ältere Clients bekommen `version`.
- Sicherheit: Optionen werden serverseitig geprüft (Dev nur im Dev-Mode, Werte in den Grenzen).

## Nicht-Ziele

Darstellung (B-105, B-106), Simulation (B-100, B-101).

## Regeln und Einschränkungen

`docs/protocol.md` ist die Quelle; nur Protokoll und beide Enden in einer Session. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

`create` mit `{grade:"hard", goal:"endboss", defeat:"stage"}` → Raum mit diesen Optionen; `snap` nennt je Spieler `stage`.

## Ausnahme- und Fehlerfälle

Unbekannte Option → Fehler `bad_request`; Dev in Live → Fehler.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt die neuen Felder und Nachrichten, `testdata/protocol/` hat Beispiele, beide Enden parsen sie (Tests).
- **AC-02** Der Server lehnt ungültige Optionen ab (Test).
- **AC-03** Protokollversion ist erhöht, ein alter Client erhält `version`.

## Offene Fragen

Wie viel Zustand der anderen Stufen sieht ein Gerät (nur Stufen mit eigenen Spielern)?

## Notizen

Aus R1.3. Vor B-105 und B-106.

SP12 (2026-10-02): `Island.StageOf(index)` und `Island.Players()` liefern die Stufe je Spieler; das Protokoll muss sie ausgeben.
