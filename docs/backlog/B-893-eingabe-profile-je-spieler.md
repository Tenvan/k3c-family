# B-893 · Ein Tastenlayout: jeder Spieler wählt WASD, Pfeiltasten oder Gamepad, die Tastatur hat höchstens ein Spieler

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
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

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- AC-01: Jedes Profil liefert alle Spiel-Aktionen (Test gegen die Aktionsliste).
- AC-02: Zwei Spieler können nicht beide ein Tastatur-Profil haben (Test).
- AC-03: Umschalten im Menü wirkt sofort und bleibt nach Neustart erhalten.
- AC-04: Sichtprüfung/Spieltest durch 🧑.

## Offene Fragen

- Genaue Aktionstasten für WASD und Pfeile? Default: Claude schlägt eine Tabelle vor, 🧑 bestätigt bei der Freigabe.
- Abhängigkeit: nach B-888 (CH1), da beide `native/k3c-game/input` ändern.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
