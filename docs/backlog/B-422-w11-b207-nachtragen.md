# B-422 · Die Spec von W11 nennt B-207 (gesperrte Plätze mit Grund) noch nicht

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WRT
- **Erstellt:** 2026-10-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Am 2026-10-14 hat 🧑 entschieden: B-207 wandert von W8 nach W11, weil es die Bauplätze aus B-208 (W11.2) braucht. Das Ticket B-207 trägt `Sprint: W11`. W11 lief da schon auf `sprint/w11`, deshalb fehlt B-207 in der README von W11 (Tickets, Anforderungen, Kriterium, Session).

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die README von W11 nennt B-207 unter Tickets und hat ein Kriterium für B-207/AC-01 bis AC-03; eine CLI-Session (W11.4 oder eine eigene) erfüllt es. Weil sich die Spec ändert, ist eine neue Freigabe durch 🧑 nötig.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

2026-10-16: Eingearbeitet in W11 Revision 3 (Entwurf): B-207 steht unter Tickets, AC-05 (B-207/AC-01, AC-02) und AC-06 (B-207/AC-03, Abnahme), Session W11.4 erfüllt sie. Umfang von B-207 auf „frei“ und „ab Hub-Stufe n“ begrenzt, weil „Linie fehlt“ ein Server-Feld braucht (B-447). Dieses Ticket wird `erledigt`, sobald 🧑 W11 Revision 3 freigibt.
