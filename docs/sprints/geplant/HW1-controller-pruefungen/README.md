# HW1 · PLAT, SRV, INF, CLI · Zurückgestellte Controller-Prüfungen nachholen

- **Status:** geplant
- **Projekt:** ABN
- **Domäne:** PLAT, SRV, INF, CLI
- **Reife:** Entwurf
- **Tickets:** B-314, B-042, B-090, B-092, B-335
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Controller-Prüfungen sind zurückgestellt, bis 🧑 wieder mit Controller testen kann (B-314).

## Ziel

Alle offenen Controller-Abnahmen sind in einem Durchgang nachgeholt. Am Ende sichtbar: Alle zurückgestellten Controller-Abnahmen in einem Durchgang erledigt.

## Beteiligte und Zielgruppen

🧑 prüft am Gerät; Agent bereitet die Liste vor.

## Anforderungen

B-314 › Anforderungen.

## Nicht-Ziele

Neue Funktionen.

## Regeln und Einschränkungen

Einschiebbar; Hardware entkoppelt, nur Mensch-Sessions.

## Beispiele

Liste aus „Offen am Gerät“ → je Punkt bestanden oder Ticket.

## Ausnahme- und Fehlerfälle

Punkt scheitert → Ticket, Sprint der Session bleibt aktiv.

## Akzeptanzkriterien

- **AC-01** Alle Controller-Prüfungen sind gesammelt nachgeholt (B-314/AC-01, B-314/AC-02).
- **AC-02** DBG3/AC-04 am Handy: `/dm` neben dem laufenden Spiel bedient (aus DBG3.4, PJ3).
- **AC-03** LP1/AC-04: Landingpage und Entwicklerseite am PC mit Tastatur bedient (aus LP1.4, PJ3).
- **AC-04** LT1/AC-06: Messlauf am Pi über eine Nacht mit Tabelle und Bewertung in B-042 (aus LT1.3, PJ3).
- **AC-05** MON2/AC-05 am Handy: Monitoring-Seite unter Last angesehen (aus MON2.4, PJ3).
- **AC-06** RL1/AC-03 am Gerät: Pi-Image-Pull und Version am Pi und auf der Xbox (aus RL1.2, PJ3).
- **AC-07** SO1/AC-04 am TV: Entsperren, Format mit Fallback und Dämpfung im Split-Screen (aus SO1.5, PJ3).
- **AC-08** SO3/AC-03 und SO3/AC-04 am TV: Bedienung der Hörprobenseite und Crossfade ohne Knacken (aus SO3.3, PJ3).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| HW1.3 | `HW1.3-dm-handy.md` | Workshop | Mensch | offen |
| HW1.4 | `HW1.4-landingpage-pc.md` | Workshop | Mensch | offen |
| HW1.5 | `HW1.5-messlauf-pi.md` | Workshop | Mensch | offen |
| HW1.6 | `HW1.6-monitor-handy.md` | Workshop | Mensch | offen |
| HW1.7 | `HW1.7-release-geraet.md` | Workshop | Mensch | offen |
| HW1.8 | `HW1.8-audio-tv.md` | Workshop | Mensch | offen |
| HW1.9 | `HW1.9-hoerprobe-tv.md` | Workshop | Mensch | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- HW1.1 Prüfliste aus dem Fahrplan zusammenstellen (AC-01).
- HW1.2 Durchgang am Gerät (Mensch), schließt ab (AC-01).

## Abnahme

–
