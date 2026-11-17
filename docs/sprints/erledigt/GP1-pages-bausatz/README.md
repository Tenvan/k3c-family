# GP1 · PLAT · Präsentationsseite als Bausatz mit Lizenz und eigenen Welten

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** PLAT
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14

## Ausgangslage

Ralf hat den impeccable-Entwurf „Bausatz“ gewählt (B-908).

## Ziel

`site/` ist der Bausatz mit Lizenz-Abschnitt und Bogen „Eigene Welten bauen“.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Siehe B-908.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `site/**`, `tests/pagesSite.test.ts`
- **Abhängig von:** B-849
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Seitentest grün (`B-908/AC-01`).
- **AC-02** Lizenz-Texte und -Links stimmen (`B-908/AC-02`).
- **AC-03** Desktop und Handy sauber (`B-908/AC-03`).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`. Blockierende Fragen verhindern die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GP1.1 | `GP1.1-bausatz-uebernehmen.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
