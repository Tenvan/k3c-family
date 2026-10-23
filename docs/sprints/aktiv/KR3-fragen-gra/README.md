# KR3 · REG · Kritische Fragen zu GRA

- **Status:** aktiv
- **Projekt:** KRT
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-546, B-547, B-548, B-549, B-550, B-551, B-552, B-553
- **Start-Commit:** 25466242
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Das Projekt GRA (Grafik) ist autonom so weit umgesetzt, wie es ohne Ralf geht: GR7.1, GR7.2, GR7.5, GR8 und GR9.1–GR9.4 sind fertig. GR10 hat die dabei getroffenen Entscheidungen G1–G8 gesammelt; Claude hat sie mit Default vorentschieden (B-546 bis B-553). Offen bei Ralf sind außerdem GR7.3 (Figurenwahl, B-546), GR7.4 (Skalierung am TV, B-547) und GR9.5 (Monarch-Animationen am TV, B-552); sie bleiben in ihren Sprints.

## Ziel

Ralf hat alle kritischen Fragen zu GRA an einer Stelle entschieden. Am Ende sichtbar: kein offenes Frage-Ticket im Projekt GRA, Folgearbeit mit klarer Entscheidung.

## Beteiligte und Zielgruppen

Entscheidet: 🧑 Ralf. Vorbereitet: Claude. Betroffen: Spielende am TV (Gegner, Monarch, Grafikstil).

## Anforderungen

Je Ticket `B-5NN › Notizen` (Frage, Default, Alternative, Folgen):

| Ticket | G | Thema | Default gilt |
|---|---|---|---|
| B-546 | G1 | Gegner-Figuren, GR7.3-Kandidaten | GR8-Figuren, kein Import |
| B-547 | G2 | Krumme Skalierung (B-251) | so lassen bis GR7.4 |
| B-548 | G3 | Stil-Bibel (B-331/AC-01) | Entwurf gilt |
| B-549 | G4 | Umschalten auf `grafik-db32/` | nicht umschalten |
| B-550 | G5 | DB32-Doppelung | doppelt lassen |
| B-551 | G6 | `kandidaten/` kein Pack | umgesetzt |
| B-552 | G7 | Monarch-Animationen abgeleitet | umgesetzt, Abnahme GR9.5 |
| B-553 | G8 | Specs GR8, GR9, GR10 | gelten |

G1, G2 und G7 am besten mit GR7.3, GR7.4 und GR9.5 am TV und im Asset-Studio ansehen; G4 auf `pipeline.html`.

## Nicht-Ziele

Umsetzung der Folgearbeit (eigene Tickets/Sprints im Projekt GRA); die Geräte-Abnahmen selbst.

## Regeln und Einschränkungen

Nur Ralf bestätigt; bis dahin gelten die Defaults. GR10.1 ist dieselbe Durchsicht aus Sicht des Projekts GRA; beide zusammen abschließen.

## Beispiele

B-551 bestätigt → erledigt. B-549 = B → Sprint „Umschalten auf DB32“ im Projekt GRA, B-549 erledigt mit Verweis.

## Ausnahme- und Fehlerfälle

Eine Frage hängt an einer Abnahme (G2 an GR7.4, G7 an GR9.5) → Ticket bleibt eingeplant, KR3.1 bleibt offen.

## Akzeptanzkriterien

- **AC-01** Jedes der Tickets B-546 bis B-553 trägt unter Notizen Ralfs Entscheidung.
- **AC-02** Bestätigte Tickets sind erledigt; zu jeder geänderten Entscheidung existiert ein Umsetzungs-Ticket oder Sprint im Projekt GRA.

## Offene Fragen

keine (die Fragen sind der Inhalt der Tickets; Defaults gelten).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KR3.1 | `KR3.1-fragen-gra.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
