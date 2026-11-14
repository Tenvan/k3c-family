# B-893 · Ein Tastenlayout: jeder Spieler wählt WASD, Pfeiltasten oder Gamepad, die Tastatur hat höchstens ein Spieler

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** EP1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 (Defaults der offenen Fragen)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das native Spiel hat mehrere Tastenbelegungen (`native/k3c-game/input/bindings.go`); für zwei Spieler an einer Tastatur ist die Belegung unbequem. Ziel ist Couch-Koop: ein Spieler an der Tastatur, weitere am Gamepad.

## Ziel

Es gibt ein Tastenlayout. Jeder Spieler wählt sein Eingabegerät: Tastatur mit WASD, Tastatur mit Pfeiltasten oder ein Gamepad. Höchstens ein Spieler nutzt die Tastatur.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Eingabe-Profile: `wasd`, `pfeile`, `gamepad` (mit Pad-Nummer). Belegung je Profil komfortabel für einen Spieler (WASD: Bewegung A/D, Aktionen auf Leertaste/E/Q/Shift o. ä.; Pfeile: Bewegung ←/→, Aktionen um die Pfeile bzw. Ziffernblock) – genaue Belegung als Tabelle in `docs/rules/` bzw. Glossar.
- Auswahl je Spieler im Menü (Optionen › Steuerung), gespeichert in den Einstellungen.
- Ein zweiter Spieler kann die Tastatur nicht wählen, solange ein anderer sie hat; Standard: Spieler 1 Tastatur (WASD), Spieler 2–4 Gamepad.
- Die UI-/Entwickler-Tasten (F1–F12, B-888) bleiben unabhängig vom Profil.
- Die alte Zwei-Spieler-Tastatur-Belegung entfällt.

## Nicht-Ziele

Frei belegbare Einzeltasten (Rebinding) – später eigenes Ticket.

## Regeln und Einschränkungen

Spiel-Code fragt Aktionen ab, nie konkrete Tasten. Nur Xbox-Symbole für jedes Pad.

- **Bereich:** `native/k3c-game/input/**`, `native/k3c-game/menu/**`, `native/k3c-game/view/**`
- **Abhängig von:** B-888
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Jedes Profil liefert alle Spiel-Aktionen (Test gegen die Aktionsliste).
- **AC-02** Zwei Spieler können nicht beide ein Tastatur-Profil haben (Test).
- **AC-03** Umschalten im Menü wirkt sofort und bleibt nach Neustart erhalten.
- **AC-04** Sichtprüfung und Spieltest durch 🧑.

## Offene Fragen

- Genaue Aktionstasten für WASD und Pfeile? Default: Claude schlägt eine Tabelle vor, 🧑 bestätigt bei der Freigabe.
- Abhängigkeit: nach B-888 (CH1), da beide `native/k3c-game/input` ändern.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
