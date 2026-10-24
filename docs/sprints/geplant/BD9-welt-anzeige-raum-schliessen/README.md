# BD9 · SRV · Welt-Anzeige größer und je Spielstand, offene Räume schließen

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** SRV
- **Reife:** Entwurf
- **Tickets:** B-604, B-581
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus der Abnahme BD5.4 (Ralf, 2026-10-24, PC): Die Welt im Pausenmenü ist zu klein, in der Lobby fehlt die Welt bei Spielständen, und offene Räume lassen sich nicht schließen (B-604, B-581).

## Ziel

Die Welt ist im Pausenmenü groß lesbar und steht in der Lobby bei jedem Spielstand; ein offener Raum lässt sich über „Entfernen“ schließen. Am Ende sichtbar: Lobby und Pausenmenü am PC.

## Beteiligte und Zielgruppen

Spielende; Agent in SRV und CLI; 🧑 entscheidet die offene Frage zu B-581 und nimmt am PC ab.

## Anforderungen

B-604 › Anforderungen, B-581 › Anforderungen.

## Nicht-Ziele

Neue Welt-Auswahl (BD8), automatisches Löschen nach dem Schließen.

## Regeln und Einschränkungen

Nur zeichnen im Client (ADR 001); Raum schließen braucht ggf. eine Protokoll-Änderung, dann eigene SRV-Session vor der CLI-Session (`docs/protocol.md`); Texte zweisprachig über `t()`; B nicht belegen.

## Beispiele

Lobby mit drei Spielständen → jede Zeile nennt ihre Welt. Offener Raum, X → „Raum schließen?“ → Ja → Raum zu, Stand bleibt und ist löschbar.

## Ausnahme- und Fehlerfälle

Mitspieler noch verbunden → Rückfrage nennt es. Spielstand ohne Welt → Platzhalter „–“.

## Akzeptanzkriterien

- **AC-01** Das Pausenmenü zeigt die Welt in größerer Schrift (B-604/AC-01).
- **AC-02** Jeder Spielstand der Lobby nennt seine Welt (B-604/AC-02).
- **AC-03** Entfernen auf einem offenen Raum bietet „Schließen“; danach lässt sich der Stand löschen (B-581/AC-01, B-581/AC-02).
- **AC-04** Ralf hat AC-01 bis AC-03 am PC abgenommen.

## Offene Fragen

B-581: Wer darf einen Raum mit mehreren Geräten schließen? Default: jedes Gerät mit Rückfrage; 🧑 entscheidet. Spec-Freigabe steht aus.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD9.1 | – | Umsetzung | autonom | offen |
| BD9.2 | – | Umsetzung | autonom | offen |
| BD9.3 | – | Review | autonom | offen |
| BD9.4 | – | Workshop | Mensch | offen |

- BD9.1 (CLI) Welt im Pausenmenü größer, Welt je Spielstand in der Lobby (AC-01, AC-02).
- BD9.2 (SRV, CLI) Offenen Raum schließen: Server-Weg, Menü-Eintrag mit Rückfrage (AC-03).
- BD9.3 Review: alle Kriterien prüfen.
- BD9.4 (Mensch) Abnahme am PC (AC-04).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
