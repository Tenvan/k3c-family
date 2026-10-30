# BS6 · CLI, DEV · Client: Baumschule, Hafen und Schiff, neue Münzskala

- **Status:** aktiv
- **Projekt:** BAS
- **Domäne:** CLI, DEV
- **Reife:** bereit
- **Tickets:** B-634, B-636, B-637
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (2026-10-29, Defaults aus B-634/B-636/B-637)

## Ausgangslage

Server kennt Baumschule (BS4), Hafen und Schiff (BS5) und die Münzskala (BS3); der Client zeigt noch Farm, kein Hafen, und Kosten-Anzeigen sind auf große Beträge ausgelegt. Balancing läuft noch im Seed-Modus `dev`.

## Ziel

Am TV sieht man Baumschule (mit „erst roden“), Hafen und Schiff je Abschnitt und die kleinen Münzpreise; nach der Abnahme misst das Balancing wieder über alle Seeds.

**Am Ende sichtbar:** Browser-Pane mit Baumschule und Hafen, Bericht „Seed-Modus: all“.

## Beteiligte und Zielgruppen

Spieler am TV, Claude, 🧑 nimmt am Gerät ab.

## Anforderungen

B-636 › Anforderungen (Client), B-637 › Anforderungen (Client), B-634 (Zurückschalten auf `all`).

## Nicht-Ziele

Neue Grafiken (Projekt GEB), Sound.

## Regeln und Einschränkungen

Abhängig von BS4, BS5 und BS9 (Meilenstein 3). Hardware-Abnahme entkoppelt (Sprint bleibt aktiv bis zur Geräte-Session). ⚠️ BS6.4 ändert Workbench-Daten/Code → Neustart deutlich melden.

## Beispiele

Baumschul-Platz mit Bäumen → Bauschild grau „erst roden“; Hafen → Schiff-Abschnitt mit Preis „10 🪙 + 60 Holz“.

## Ausnahme- und Fehlerfälle

Asset fehlt → vorhandene Farm-/Platzhalter-Grafik, Ticket an GEB.

## Akzeptanzkriterien

- **AC-01** Client zeigt Baumschule und Hinweis „erst roden“; „Farm“ nicht mehr in `src/` (B-636/AC-02, AC-05).
- **AC-02** Client zeichnet Hafen und Schiff je Abschnitt, Hinweis bei fehlender Bedingung (B-637/AC-05).
- **AC-03** Kosten-Anzeigen und HUD passen zur Skala 2–30.
- **AC-04** **Meilenstein 4:** `seedMode: all`, Abschluss-Lauf `coop2` standard über alle Seeds mit `baseline=M3` ohne gekippte Gruppe; als `M4` gesichert (B-634, B-640).
- **AC-05** Abnahme am Gerät durch 🧑 (angenommen bis zur Hardware-Session).

## Offene Fragen

keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BS6.1 | `BS6.1-baumschule-erst-roden.md` | Umsetzung | autonom | fertig |
| BS6.2 | `BS6.2-hafen-schiff-zeichnen.md` | Umsetzung | autonom | fertig |
| BS6.3 | `BS6.3-kosten-anzeige-skala.md` | Umsetzung | autonom | fertig |
| BS6.4 | `BS6.4-seed-modus-all-m4.md` | Umsetzung | autonom | fertig |
| BS6.5 | `BS6.5-review-bs6.md` | Review | autonom | fertig |
| BS6.6 | `BS6.6-abnahme-geraet.md` | Workshop | Mensch | fertig |

Reife Entwurf, Stichpunkte:
- BS6.1 · CLI · Baumschule im Client (AC-01)
- BS6.2 · CLI · Hafen und Schiff zeichnen (AC-02)
- BS6.3 · CLI · Kosten-Anzeige neue Skala (AC-03)
- BS6.4 · DEV · Seed-Modus `all`, Abschluss-Lauf, Baseline `M4` (AC-04)
- BS6.5 · Review (alle)
- BS6.6 · Abnahme am Gerät, Mensch (AC-05)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
