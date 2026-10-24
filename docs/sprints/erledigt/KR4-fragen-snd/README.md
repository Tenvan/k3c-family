# KR4 · REG · Kritische Fragen zu SND

- **Status:** erledigt
- **Projekt:** KRT
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-554, B-555, B-556, B-557, B-558
- **Start-Commit:** 3f288313
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Das Projekt SND (Sound) ist autonom so weit umgesetzt, wie es ohne Ralf geht: SO4.3, SO4.4 und SO6.1, SO6.2, SO6.4, SO6.5 sind fertig. Dabei hat Claude die Fragen S1–S5 mit Default vorentschieden (B-554 bis B-558). Offen bei Ralf bleiben die Hörproben und Workshops SO2.5, SO4.1 (Stückwahl bestätigen, B-554), SO4.5, SO6.3 (Lücken-Sounds, B-558) und SO6.6; sie bleiben in ihren Sprints.

## Ziel

Ralf hat alle kritischen Fragen zu SND an einer Stelle entschieden. Am Ende sichtbar: kein offenes Frage-Ticket im Projekt SND, Folgearbeit mit klarer Entscheidung.

## Beteiligte und Zielgruppen

Entscheidet: 🧑 Ralf. Vorbereitet: Claude. Betroffen: Spielende am TV (Musik, Effekte).

## Anforderungen

Je Ticket `B-5NN › Notizen` (Frage, Default, Alternative, Folgen):

| Ticket | S | Thema | Default gilt |
|---|---|---|---|
| B-554 | S1 | Musik je Zustand (SO4.1), übrige Kandidaten | erste Präferenz, Kandidaten bleiben |
| B-555 | S2 | Musik nach dem Fall der Burg | still |
| B-556 | S3 | Credit-Datei der Sounds | `public/audio/README.md` |
| B-557 | S4 | Sound-Atlas | bauen und nutzen |
| B-558 | S5 | Sounds für die Lücken, UI-Klicks, Reiten | Kenney-Töne, UI/Reiten offen |

S1, S2 und S5 am besten zusammen mit SO4.5 und SO6.6 am TV hören; S4 nach der Messung am TV.

## Nicht-Ziele

Umsetzung der Folgearbeit (eigene Tickets/Sprints im Projekt SND); die Hörproben selbst.

## Regeln und Einschränkungen

Nur Ralf bestätigt; bis dahin gelten die Defaults. Nur CC0 oder CC-BY (Q15).

## Beispiele

B-556 bestätigt → erledigt. B-558 Reiten gewünscht → Ticket CLI „Hufschlag-Schleife je Spieler“ im Projekt SND.

## Ausnahme- und Fehlerfälle

Eine Frage hängt an einer Hörprobe (S1/S2 an SO4.5, S4/S5 an SO6.6) → Ticket bleibt eingeplant, KR4.1 bleibt offen.

## Akzeptanzkriterien

- **AC-01** Jedes der Tickets B-554 bis B-558 trägt unter Notizen Ralfs Entscheidung.
- **AC-02** Bestätigte Tickets sind erledigt; zu jeder geänderten Entscheidung existiert ein Umsetzungs-Ticket oder Sprint im Projekt SND.

## Offene Fragen

keine (die Fragen sind der Inhalt der Tickets; Defaults gelten).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KR4.1 | `KR4.1-fragen-snd.md` | Workshop | Mensch | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-24: KR4.1 fertig, alle fünf Fragen (B-554 bis B-558) entschieden und erledigt; Folgearbeit B-573 bis B-576.
