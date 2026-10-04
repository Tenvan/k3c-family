# B-208 · Das Protokoll trägt die Bauplätze des Layouts sowie Platz- und Hub-Stufe zum Client

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** W5
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-206 legt die Bauplätze aus Daten und Seed fest (Linien 2–5 streuen je Seed), B-112 bringt Hub-Stufe und Platz-Stufen. Der Client kennt heute nur die festen Offsets aus `data/hub.json` (`src/model/data.ts`) und die gebauten Plätze aus dem Snapshot (`engine/net/`); die gestreuten Linien, gesperrten Plätze und Stufen erreichen ihn nicht.

## Ziel

Der Client zeichnet alle Plätze an der richtigen Stelle und kennt Hub- und Platz-Stufe, ohne die Regeln selbst nachzurechnen.

## Beteiligte und Zielgruppen

Entwickler SRV und CLI; Spieler profitieren über B-207.

## Anforderungen

- Das Layout (Plätze mit Art, Lage, Seite, Linie) geht einmal je Stufe zum Client, nicht in jedem Tick.
- Hub-Stufe und Platz-Stufe gehen im Snapshot bzw. Delta mit.
- Bandbreite bleibt im Budget aus Q08 (≤ 200 Byte je Tick und Client im Mittel).

## Nicht-Ziele

Layout und Regeln (B-206, B-112); Anzeige (B-207); weitere Wirtschafts-Felder des Protokolls (übrige W5-Tickets).

## Regeln und Einschränkungen

Entscheidung 001 (Server rechnet, Client zeichnet); Protokoll-Version erhöhen nach den Regeln in `docs/arbeitsweise.md`. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Client verbindet sich mit Seed A → erhält 5 Linien je Seite mit gestreuten Lagen; Hub-Stufe steigt auf 2 → der nächste Snapshot trägt Stufe 2.

## Ausnahme- und Fehlerfälle

Alter Client ohne Layout-Unterstützung → Versionsprüfung wie bei bisherigen Protokoll-Wechseln. Stufenwechsel eines Spielers → Layout der neuen Stufe wird gesendet.

## Akzeptanzkriterien

- **AC-01** Test (Go): Das Layout im Protokoll stimmt für einen Seed mit dem Layout der Sim überein.
- **AC-02** Test: Hub- und Platz-Stufe kommen nach einer Änderung beim Client an.
- **AC-03** Benchmark zeigt das Budget aus Q08 eingehalten; `task check` und `task check:go` grün.

## Offene Fragen

keine

## Notizen

Folge aus Beschluss Q43 und Q50 (2026-10-04, zweite Runde). Abhängig von B-206 und B-112.
