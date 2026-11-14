# B-906 · Die Kristallhöhle leuchtet nach der Natur-Eigenschaft brightness

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BI2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Kristallhöhle hat seit B-891 die Natur-Eigenschaft `brightness` (Leuchtstärke 1–3, nur Darstellung).

## Ziel

Das native Spiel lässt Kristalle und Umgebungslicht nach dem Wert der Stufe schwächer oder stärker leuchten.

## Beteiligte und Zielgruppen

Spieler der Kristallhöhle; Claude setzt um; Ralf 🧑 prüft im Abnahme-Ticket.

## Anforderungen

- Biome mit `brightness` zeichnen hinter jeder sichtbaren Kristall-Ader einen Schein (Kreis) in der Farbe der Ader (blau, violett, grün wie die Materialien `kristall`, `purpur`, `smaragd`).
- Radius 0,8 + 0,6 · Wert Units, Deckkraft 0,12 · Wert, leicht pulsierend mit der Uhr (±10 %).
- Der Schein liegt hinter der Ader (früher in der Zeichenliste).

## Nicht-Ziele

Änderung der Nacht-Helligkeit oder Lichtquellen in `world/light.go`.

## Regeln und Einschränkungen

Wie B-902.

- **Bereich:** `native/k3c-game/props/decor_glow.go`, `native/k3c-game/props/decor_test.go`
- **Abhängig von:** B-901
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Kristallhöhle mit Wert 1 → kleiner schwacher Schein; mit 3 → großer heller Schein.

## Ausnahme- und Fehlerfälle

Biom ohne `brightness` → kein Schein. Unbekannte Farbe → blau.

## Akzeptanzkriterien

- **AC-01** In der Kristallhöhle plant props je sichtbarer Kristall-Ader einen Schein in ihrer Farbe vor der Ader; Biome ohne `brightness` keinen (Unit-Test).
- **AC-02** Radius und Deckkraft des Scheins steigen mit dem Wert (1 < 2 < 3) (Unit-Test).

## Offene Fragen

keine (Leuchten als Schein in props statt über die Nacht-Helligkeit in `world/light.go`).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
