# B-913 · Ralf nimmt EP1 ab: Eingabe-Profile und Tasten-Icons an TV und PC geprüft

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** EP1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

EP1 ist umgesetzt (EP1.1 Eingabe-Profile, EP1.2 Tasten-Icons); alle Tests der Sessions grün. Offen sind die Sichtprüfung und der Spieltest am Gerät.

## Ziel

Ralf bestätigt Belegung, Menü Steuerung und Tasten-Hinweise an TV und PC.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Optionen › Steuerung: Profil je Spieler wählen, zweiter Spieler bekommt die Tastatur nicht.
- Belegung pfeile spielen (Tabelle in `native/k3c-game/input/bindings.go`) und bestätigen oder Änderung als Ticket.
- Skill-Menü an Tastatur (WASD, Pfeile) und Pad: Tastenkappe bzw. Xbox-Taste; F1 mit Tastenkappen.
- `check:race` und `gen:paper` laufen hier nicht: EP1 ändert keine Grafik und keine Nebenläufigkeit (Sim und Balance gesperrt).

## Nicht-Ziele

Papierschnitt-Icons (B-911).

## Regeln und Einschränkungen

- **Bereich:** –
- **Abhängig von:** B-893, B-894
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Spieler 1 auf Pfeile → Monarch läuft mit ←/→, Skill-Menü zeigt „Pfeil links / Pfeil rechts“.

## Ausnahme- und Fehlerfälle

Belegung unbequem → neues Ticket mit Wunschbelegung.

## Akzeptanzkriterien

- **AC-01** Sichtprüfung und Spieltest durch 🧑 an TV und PC (`B-893/AC-04`, `B-894/AC-03`).

## Offene Fragen

keine

## Notizen

–
