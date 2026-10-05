# B-283 · Das Protokoll kennt Beruf ausbilden, Tauschen, Berufe der Bürger und Grabstein/Wiederbeleben

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** W5
- **Erstellt:** 2026-10-05
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, mit Sprint W5

## Ausgangslage

B-123 wurde in S2 nur für Schlag, Skills, Pool, `learn`, `respec` und die Aktionsliste umgesetzt (Protokoll v4). Die
Teile Beruf ausbilden, Tauschen (c2s), Berufe der Bürger und Grabstein/Wiederbeleben (s2c) fehlen, weil die Simulation
sie noch nicht hat (B-120, B-121). B-153 (W5) setzt voraus, dass B-123 die Eingaben für Berufe und Tausch liefert, und
B-120 verweist für das Protokoll auf B-123. Gefunden im Review S2.5.

## Ziel

Die in B-123 offenen Teile haben ein Ticket, das nach B-120 und B-121 eingeplant wird; nichts fällt zwischen B-123 und B-153 durch.

## Beteiligte und Zielgruppen

Entwickler (Client und Server); 🧑 entscheidet, ob das in B-153 aufgeht.

## Anforderungen

- c2s: Beruf ausbilden und Tauschen, serverseitig geprüft (`bad_request` ohne Zustandsänderung).
- s2c: Berufe der Bürger, Grabstein und Wiederbeleben je Monarch.
- `docs/protocol.md`, Beispiele unter `testdata/protocol/`, beide Enden parsen sie.

## Nicht-Ziele

Simulation (B-120, B-121), Darstellung (B-125, B-126).

## Regeln und Einschränkungen

Protokolländerung in einer eigenen Session, beide Enden gemeinsam (`docs/arbeitsweise.md` › Domänen).

## Beispiele

Ein Gerät bildet einen Bürger zum Bergmann aus → der Server prüft Gold und Ort, der Beruf steht im nächsten `delta`.

## Ausnahme- und Fehlerfälle

Unbekannter Beruf, fremder Slot oder kein Gold → `bad_request`, Zustand unverändert.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt die Felder, `testdata/protocol/` hat Beispiele, beide Enden parsen sie (Tests).
- **AC-02** Test: Der Server lehnt ungültigen Beruf und ungültigen Tausch mit `bad_request` ab.

## Offene Fragen

Aufgehen in B-153 statt eigenem Ticket? Entscheidet 🧑.

## Notizen

Rest von B-123/AC-02 (Beruf, Tausch) und B-123 › Anforderungen (Berufe, Grabstein).
