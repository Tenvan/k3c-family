# B-155 · Die Wirtschaft ist in einer Balancing-Runde gegen die Zielkorridore abgestimmt

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BR1
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint BR1

## Ausgangslage

Nach W1 bis W6 sind Hub-Ausbau, Material, Adern, Gebäude und Bürger spielbar; die Zahlen in `data/hub.json`, `data/buildings.json`, `data/economy.json` und `data/troops.json` sind Startwerte (B-015). Zielkorridore (B-134) und Spieleabende (B-008) liefern die Maßstäbe.

## Ziel

Die Werte der Wirtschaft liegen in den Zielkorridoren und sind in `docs/rules/` begründet; ein zweiter Spieleabend hat sie geprüft. Nutzen: Nächte sind weder zu leicht noch unschaffbar, Fortschritt fühlt sich verdient an.

## Beteiligte und Zielgruppen

Spieler (Familie, Kinder); 🧑 spielt den Abend und entscheidet über Werte; der Agent misst und wertet aus.

## Anforderungen

- Messung der Kennzahlen aus den Zielkorridoren vor und nach jeder Wertänderung (Simulator B-099, soweit verfügbar, sonst Läufe über die Go-Tests).
- Wertänderungen nur in `data/`, jede mit Begründung in `docs/rules/materialien-gebaeude.md` oder `docs/rules/wirtschaft.md`.
- Spieleabend 2 mit Protokoll nach `docs/playtests/` (Vorlage aus B-151).
- Vor der Runde werden die Zielkorridore in `docs/rules/zielkorridore.md` auf den 6-min-Tag (Q65) neu gefasst.
- Der Tageszyklus 6/2/4/2 min (Q65, Startwert) wird am Spieleabend 2 bestätigt oder mit Beschluss geändert.
- Nach Spieleabend 2 ein Tag mit Release-Checkliste (Q20).

## Nicht-Ziele

Kampf, Gegner und Bosse (B-156), neue Mechaniken, Ausbau des Testers (BAL1 bis BAL3).

## Regeln und Einschränkungen

Werte gehören nach `data/`, Golden-Daten werden nach dem Golden-Ablauf (B-137) aktualisiert; die Manuelle Abnahme und Freigabe liegen bei 🧑.

## Beispiele

Zeit bis zur Mauer liegt über dem Korridor → Kosten oder Bauzeit in `data/` senken, Messung wiederholen, Begründung eintragen.

## Ausnahme- und Fehlerfälle

Ein Zielkorridor lässt sich nicht erreichen → Abweichung als Ticket anlegen, Korridor oder Mechanik wird von 🧑 neu entschieden.

## Akzeptanzkriterien

- **AC-01** Die Zielkorridore der Wirtschaft stehen vor der Runde als Zahlen in `docs/rules/`, neu gefasst auf den 6-min-Tag (Q65) (Beobachtung: Datei enthält Zahlen je Kennzahl).
- **AC-02** Für jede Wertänderung in `data/` nennt der Commit die betroffenen Kennzahlen mit Wert vor und nach der Änderung.
- **AC-03** Für jeden geänderten Wert aus `data/hub.json`, `data/buildings.json`, `data/economy.json` und `data/troops.json` steht eine Begründung in `docs/rules/` (Stichprobe von 🧑).
- **AC-04** Spieleabend 2 hat stattgefunden, das Protokoll liegt in `docs/playtests/`.
- **AC-05** Je Zielkorridor der Wirtschaft ist Pass oder Fail festgehalten; jede Abweichung hat ein Ticket; `task check` und `task check:go` grün.
- **AC-06** Der Tageszyklus 6/2/4/2 min (Q65, Startwert) ist am Spieleabend 2 bestätigt oder mit Beschluss von 🧑 geändert.
- **AC-07** Nach Spieleabend 2 ist ein Tag mit Release-Checkliste gesetzt, nach Bestätigung durch 🧑 (Q20).

## Offene Fragen

Termin und Teilnehmer des Spieleabends: `docs/fragenkatalog.md Q24` (🧑).

## Notizen

Aus Plan Phase 2 (B1). Vorbild für B-156.
