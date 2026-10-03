# B-181 · Das Debug-Overlay zeigt die Latenz von Eingabe bis Bild

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Regel „Verbindung“ in `docs/rules/bedienung.md` § 3 legt das Latenz-Ziel Eingabe → Bild fest (Mittel ≤ 100 ms beschlossen, p95 ≤ 150 ms Vorschlag) und nennt das Debug-Overlay als Messweg (Beschluss Q04). `src/scenes/debugOverlay.ts` zeigt heute Status, Protokoll, Snapshot-Rate, Alter des letzten Snapshots und FPS, aber keine Latenz. Grundlage wäre `ack` im Zustand (`docs/protocol.md`: höchstes `seq`, das der Server von dieser Verbindung verrechnet hat).

## Ziel

Die Latenz von Eingabe bis Bild ist am Gerät ablesbar, damit das Ziel aus `bedienung.md` § 3 geprüft werden kann.

## Beteiligte und Zielgruppen

Entwickler und 🧑 beim Test am TV; Familie nur mittelbar (spürbare Verzögerung).

## Anforderungen

- Das Overlay zeigt Mittel und p95 der Zeit von einer gesendeten `input` (`seq`) bis zum ersten Zustand mit `ack` ≥ `seq`, über ein gleitendes Fenster (z. B. 60 s).
- Funktioniert mit 2+ lokalen Spielern und online.

## Nicht-Ziele

Vorhersage des eigenen Monarchen (B-039), Messung am Server, Änderung des Protokolls.

## Regeln und Einschränkungen

Domäne CLI (`src/scenes/debugOverlay.ts`, `src/online/`); der Client rechnet nur Anzeige-Werte, keine Spiel-Logik (`CLAUDE.md`). Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Heim-WLAN, 2 Spieler, 60 s gespielt → Overlay zeigt „Latenz 62 ms (p95 110 ms)“.

## Ausnahme- und Fehlerfälle

Keine Eingabe im Fenster (Spieler steht) → Anzeige „Latenz –“ statt einer veralteten Zahl.

## Akzeptanzkriterien

- **AC-01** Test: Aus einer Folge gesendeter `seq` mit Zeitstempeln und empfangener `ack` berechnet die Funktion Mittel und p95 richtig (`task check`).
- **AC-02** Das Overlay zeigt Mittel und p95 in ms; ohne Eingaben im Fenster „–“.

## Offene Fragen

keine (Ziel und Messweg beschlossen in Q04; p95-Zahl bestätigt 🧑 in F1.4)

## Notizen

Entstanden in F1.1 beim Niederschreiben von `docs/rules/bedienung.md` § 3.
