# B-430 · Frage: Wie lauten Ziel, Erlaubte Dateien und Form der actions-Einträge für W11.3 (Ausbilden, Tauschen)?

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** W11
- **Projekt:** WRT
- **Erstellt:** 2026-10-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Session-Datei W11.3 ist seit Anlage (ee52a371) eine unveränderte Kopie der Vorlage: Ziel, Kontext, Erlaubte Dateien, Nicht-Ziele, Schritte und „Fertig, wenn“ fehlen. Auch B-410 › Anforderungen, Beispiele und Akzeptanzkriterien sind nur Vorlagentext. Ein autonomer Lauf kann die Session so nicht ohne Raten umsetzen.

Offen für 🧑:
1. Form der Einträge in `actions` für Ausbilden und Tauschen (Name der Aktion, Felder wie Preis/Ziel, wann sichtbar: in Reichweite des Berufs-Platzes bzw. Händlers?).
2. Welche Eingabe löst sie aus (bestehendes Zahlziel/`buyPaid` oder neue Eingabe)?
3. Erlaubte Dateien (vermutet: `engine/net/protocol.go`, `engine/net/*_test.go`, ggf. `engine/sim/*_view.go`, `docs/protocol.md`, `testdata/protocol/`, `src/model/types.ts`) und ob eine neue Protokoll-Version nötig ist.
4. Prüfbares Kriterium für AC-03 (Test, Beispiel-Datei).

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Beantwortet 2026-10-15: W11.3 und B-410 (Revision 2) ausgeplant, von 🧑 freigegeben. Antworten: 1. `train`/`trade` laut B-410 › Anforderungen; 2. bestehendes A halten (`findPayTarget`), keine neue Eingabe; 3. Erlaubte Dateien in W11.3, keine neue Version; 4. B-410/AC-01–AC-03.
