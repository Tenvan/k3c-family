# B-914 · Ralf nimmt ST2 ab: Testing-Menü und Stresstest-Bericht am PC geprüft

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** ST2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

ST2 ist umgesetzt (ST2.1 Testing-Menü, ST2.2 Lauf und Bericht); alle Tests der Sessions grün. Offen ist die Sichtprüfung am Gerät.

## Ziel

Ralf bestätigt Testing-Menü, Lauf mit Vorgabe und den Bericht.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Hauptmenü › Testing (Entwickler-Build oder Optionen › Entwicklermodus) mit Pad und Tastatur bedienen.
- Vorgabe „Viele Gegner“ starten: F2 und F10 an, Ausbau und Truppen da, mehr Gegner je Welle; nach den Wellen steht das Spiel, Banner nennt den Bericht.
- Bericht `reports/native-stress-*.json` über `report_read` lesen.
- Manueller Stresstest durch Ralf ist von der Sperre für `sim_test` nicht betroffen (B-896); `gen:paper` und `check:race` nicht betroffen.

## Nicht-Ziele

Automatische Läufe (`sim_test`).

## Regeln und Einschränkungen

- **Bereich:** –
- **Abhängig von:** B-896
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Vorgabe wählen, A auf „Test starten“ → Spiel läuft mit den Werten.

## Ausnahme- und Fehlerfälle

Werte unpassend → `data/stresstests.json` anpassen oder Ticket.

## Akzeptanzkriterien

- **AC-01** Sichtprüfung durch 🧑 (`B-896/AC-04`).

## Offene Fragen

keine

## Notizen

Die Minutenfrist und `dauerSekunden` laufen nach Wanduhr samt Pausen (Review ST2, bewusst so).
