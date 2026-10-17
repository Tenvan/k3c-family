# WZ7 · DEV · Welt-Betrachter im Asset-Werkzeug mit Chunk- und Live-Vorschau

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-460
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-460 › Ausgangslage: `leveltest.html` zeigt ein generiertes Level als Chunk-Streifen (B-092) und erzeugt einen voll ausgebauten Spielstand (B-315); keine maximierte Vorschau, keine Live-Ansicht mit Spielgrafik. Voraussetzung: Asset-Werkzeug aus WZ6.

## Ziel

Der Welt-Betrachter läuft im Asset-Werkzeug: Chunk-Vorschau im Standard maximiert, darunter eine Live-Vorschau mit Spielgrafik, Hub-Stufe 1–5 umschaltbar und Gebäude einzeln ein- und ausschaltbar. Am Ende von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-460/AC-01` Funktionsumfang von `leveltest.html` im Asset-Werkzeug.
- **AC-02** `B-460/AC-02` Chunk-Vorschau maximiert beim Öffnen.
- **AC-03** `B-460/AC-03` Live-Vorschau mit Spielgrafik, Hub-Stufen und Gebäuden schaltbar, deterministisch.
- **AC-04** `B-460/AC-04` Umgang mit `leveltest.html` nach Entscheidung 🧑.
- **AC-05** 🧑 nimmt den Welt-Betrachter ab (Review-Session).

## Offene Fragen

B-460 › Offene Fragen 1–3 (Technik der Live-Vorschau, Gebäude nur Darstellung, Tag/Nacht und Hintergrund). Entscheidet 🧑 vor der Freigabe.

## Sessions

- WZ7.1 (DEV) Leveltest-Ansicht ins Asset-Werkzeug migrieren, Chunk-Vorschau maximiert (AC-01, AC-02).
- WZ7.2 (SRV) Snapshot für Seed, Biom und Hub-Stufe bereitstellen (Voll-Ausbau aus `engine/room/fullsave.go`) (AC-03).
- WZ7.3 (DEV) Live-Vorschau mit Spielgrafik, Hub-Stufe umschalten, Gebäude ein/aus (AC-03).
- WZ7.4 (PLAT) `leveltest.html` nach Entscheidung behalten, entfernen oder verweisen, Kacheln nachziehen (AC-04).
- WZ7.5 (DEV) Review und Abnahme durch 🧑 (AC-05, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
