# BS3 · REG, DEV · Werte auf der Münzskala: Meilenstein-Lauf coop2 standard läuft durch

- **Status:** aktiv
- **Projekt:** BAS
- **Domäne:** REG, DEV
- **Reife:** bereit
- **Tickets:** B-635, B-638
- **Start-Commit:** 48e285ff
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑, Revision 1, mit den Defaults der offenen Fragen

## Ausgangslage

Nach BS1 (Seed `k3c-dev`, schnelle Läufe) und BS2 (Regeln der Münzskala) stehen die Werte in `data/` noch auf der alten Skala (Gebäude bis 300, Hub bis 200 Gold); das Balancing läuft so nicht durch.

## Ziel

**Meilenstein 1:** Alle regulären Preise, Einnahmen und Ziele stehen auf der Münzskala 2–30, die Schiffskosten sind in `data/`, und ein Balancing-Lauf `coop2`, Welt standard, Seed `k3c-dev` läuft mit den neuen Werten durch.

**Am Ende sichtbar:** `sim_test`-Bericht `coop2` standard mit Endboss Insel 1 besiegt und der Zeile „Schiff (Bedarf): … bezahlbar ab Tag …“.

## Beteiligte und Zielgruppen

Claude (Werte und Läufe), 🧑 nimmt den Meilenstein ab.

## Anforderungen

B-635 › Anforderungen (Werte), B-638 › Anforderungen Stufe 1 (Schiffskosten, Bedarf gegen Bestand im Bericht).

## Nicht-Ziele

Baumschule und Hafen als Mechanik (BS4, BS5); eigenes Balancing für Events, geplante Chunks, Bosse (später eigenes Ticket). Die Farm bleibt bis BS4 Farm, nur ihre Preise ändern sich. Den Meilenstein-Lauf als Baseline `M1` zu sichern, macht BS7.4; BS3.4 notiert dazu die Bericht-ID in B-635.

## Regeln und Einschränkungen

Abhängig von BS1, BS2 und **BS11** (Insel-Lauf bis zum Endboss, B-642 Variante A). Werte nur in `data/`; Golden über `task go:golden` mit Begründung. Läufe nur über `sim_test`, Seed-Modus `dev`. Keine Rücksicht auf alte Spielstände.

**Konflikt:** Die aktiven BAL-Sprints RG1 und BR2 messen auf der alten Skala; ihre Ergebnisse sind nach BS3 hinfällig (siehe Projekt BAS › Notizen).

## Beispiele

Holzmauer 2, Hub-Stufe 5 = 30; Lauf `coop2` standard → Endboss Tag ~10, Schiff bezahlbar T8–T11.

## Ausnahme- und Fehlerfälle

Lauf scheitert (Burg fällt, Gold reicht nie) → BS3.4 justiert iterativ, höchstens bis zum Session-Budget; bleibt es rot, Befund als Ticket und 🧑 fragen.

## Akzeptanzkriterien

- **AC-01** Reguläre Goldpreise in `data/` liegen in 2…30, Mauer Stufe 1 = 2; ein Daten-Test prüft das mit expliziter Ausnahme-Liste (B-635/AC-02).
- **AC-02** Einnahmen, Beute, Markieren und Ziele sind auf die Skala umgerechnet, Zielkorridore in `balance-targets.json` passen dazu.
- **AC-03** Schiffskosten je Abschnitt stehen in `data/`, Daten-Test prüft Skala und Schema (B-638/AC-01).
- **AC-04** Der Bericht nennt je Schiff-Abschnitt „bezahlbar ab Tag“ mit Ampel (B-638/AC-02).
- **AC-05** Meilenstein: Lauf `coop2`, Welt standard, Seed `k3c-dev` läuft durch (Endboss Insel 1 besiegt) (B-635/AC-03).

## Offene Fragen

keine (Defaults aus BS2).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BS3.1 | `BS3.1-preise.md` | Umsetzung | autonom | fertig |
| BS3.2 | `BS3.2-einnahmen-schiffskosten.md` | Umsetzung | autonom | fertig |
| BS3.3 | `BS3.3-bericht-schiff-bedarf.md` | Umsetzung | autonom | fertig |
| BS3.4 | `BS3.4-meilenstein-lauf.md` | Umsetzung | autonom | blockiert |
| BS3.5 | `BS3.5-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
