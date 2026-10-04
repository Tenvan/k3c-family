# B-156 · Kampf, Gegner und Bosse sind in einer Balancing-Runde gegen die Zielkorridore abgestimmt

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** BR2
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint BR2

## Ausgangslage

Nach K1 bis K5 sind Traits, neue Gegner, Bosse, Events und Siegvarianten spielbar; die Zahlen in `data/enemies.json`, `data/waves.json`, `data/difficulty.json` und den Biomen (`data/biomes/`) sind Startwerte. Zielkorridore (B-134) und Spieleabende (B-008, B-155) liefern die Maßstäbe.

## Ziel

Die Werte von Kampf, Gegnern und Bossen liegen in den Zielkorridoren und sind in `docs/rules/` begründet; ein dritter Spieleabend hat sie geprüft. Nutzen: Bosse und Events sind schaffbar, aber spannend, in allen Schwierigkeitsgraden.

## Beteiligte und Zielgruppen

Spieler (Familie, Kinder); 🧑 spielt den Abend und entscheidet über Werte; der Agent misst und wertet aus.

## Anforderungen

- Messung der Kennzahlen aus den Zielkorridoren vor und nach jeder Wertänderung (Simulator B-099, soweit verfügbar, sonst Läufe über die Go-Tests), je Schwierigkeitsgrad.
- Wertänderungen nur in `data/`, jede mit Begründung in `docs/rules/gegner.md` oder `docs/rules/bosse.md`.
- Spieleabend 3 mit Protokoll nach `docs/playtests/`.
- Nach Spieleabend 3 ein Tag mit Release-Checkliste (Q20).

## Nicht-Ziele

Wirtschaft (B-155), neue Mechaniken, Ausbau des Testers (BAL1 bis BAL3), Release (RL1).

## Regeln und Einschränkungen

Werte gehören nach `data/`, Golden-Daten werden nach dem Golden-Ablauf (B-137) aktualisiert; die manuelle Abnahme und Freigabe liegen bei 🧑. Voraussetzung: B-155 (BR1).

## Beispiele

Endboss fällt mit 2 Spielern in unter einer Minute → HP oder Phasenwerte in `data/` anheben, Messung wiederholen, Begründung eintragen.

## Ausnahme- und Fehlerfälle

Ein Zielkorridor lässt sich nicht erreichen → Abweichung als Ticket anlegen, Korridor oder Mechanik wird von 🧑 neu entschieden.

## Akzeptanzkriterien

- **AC-01** Die Zielkorridore für Kampf und Bosse stehen vor der Runde als Zahlen in `docs/rules/` (Beobachtung: Datei enthält Zahlen je Kennzahl).
- **AC-02** Für jede Wertänderung in `data/` nennt der Commit die betroffenen Kennzahlen mit Wert vor und nach der Änderung.
- **AC-03** Für jeden geänderten Wert aus `data/enemies.json`, `data/waves.json`, `data/difficulty.json` und `data/biomes/` steht eine Begründung in `docs/rules/` (Stichprobe von 🧑).
- **AC-04** Spieleabend 3 hat stattgefunden, das Protokoll liegt in `docs/playtests/`.
- **AC-05** Je Zielkorridor für Kampf und Bosse ist Pass oder Fail festgehalten; jede Abweichung hat ein Ticket; `task check` und `task check:go` grün.
- **AC-06** Nach Spieleabend 3 ist ein Tag mit Release-Checkliste gesetzt, nach Bestätigung durch 🧑 (Q20).

## Offene Fragen

Termin und Teilnehmer des Spieleabends: `docs/fragenkatalog.md Q24` (🧑).

## Notizen

Aus Plan Phase 3 (B2). Nach B-155.
