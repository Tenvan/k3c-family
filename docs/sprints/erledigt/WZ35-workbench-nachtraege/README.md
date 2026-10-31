# WZ35 · DEV · Workbench-Nachträge: Ticket-Nummern, Lauf-IDs, schmale Fenster, Auswertung

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-615, B-616, B-668, B-669, B-678
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31, Ralf im Chat (Revision 1)

## Ausgangslage

Kleine Fehler und Schärfungen der Workbench aus Reviews: doppelte Ticket-Nummern bei parallelen Aufrufen (B-615), nicht eindeutige Lauf-IDs nach Neustart (B-616), umbrechender Kopf in schmalen Fenstern (B-668), Absturz der Auswertung bei Zielen ohne Grenzen (B-669), Schuld beim Übernehmen aus dem Vorrat (B-678).

## Ziel

Die Workbench bleibt bei parallelen Aufrufen, Neustarts, schmalen Fenstern und ungewöhnlichen Berichten stabil. Am Ende sichtbar: Tests grün, Web-Client im halben Fenster ohne waagrechten Überlauf.

## Beteiligte und Zielgruppen

Wer die Workbench und das Studio benutzt; Agent DEV; 🧑 sieht die schmalen Fenster (B-668).

## Anforderungen

`B-615 › Anforderungen`, `B-616 › Anforderungen`, `B-668 › Anforderungen`, `B-669 › Anforderungen`, `B-678 › Anforderungen`.

## Nicht-Ziele

Neue Seiten oder Funktionen der Workbench.

## Regeln und Einschränkungen

Nur DEV-Dateien (`tools/`, `tests/`); Oberfläche über `@webkit`, Atomic Design (`docs/arbeitsweise.md`); Go-Tests ohne Flackern.

## Beispiele

Zwei Prozesse legen gleichzeitig ein Ticket an → zwei verschiedene Nummern.

## Ausnahme- und Fehlerfälle

Bericht mit Ziel ohne Unter- und Obergrenze → Auswertung zeigt das Ziel ohne Bewertung, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Parallel angelegte Tickets haben nie dieselbe Nummer (`B-615/AC-01`).
- **AC-02** Lauf-IDs und ihre Ordner unter `reports/` sind über Neustarts eindeutig (`B-616/AC-01`).
- **AC-03** Der Kopf der Workbench läuft in schmalen Fenstern ohne waagrechten Überlauf (`B-668/AC-01`, `AC-02`).
- **AC-04** Die Auswertung stürzt bei Zielen ohne Grenzen nicht ab (`B-669/AC-01`).
- **AC-05** Übernehmen aus dem Vorrat ist atomar, die Abspielleiste spielt nie zwei Töne, die Download-Hosts sind geklärt (`B-678/AC-01` bis `AC-03`).

## Offene Fragen

keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ35.1 | `WZ35.1-nummern-lauf-ids.md` | Umsetzung | autonom | fertig |
| WZ35.2 | `WZ35.2-schmale-fenster-auswertung.md` | Umsetzung | autonom | fertig |
| WZ35.3 | `WZ35.3-uebernehmen-abspielleiste.md` | Umsetzung | autonom | fertig |
| WZ35.4 | `WZ35.4-review.md` | Review | autonom | fertig |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ35.1 (DEV) Nummern und Lauf-IDs eindeutig (AC-01, AC-02).
- WZ35.2 (DEV) Schmale Fenster und Auswertung (AC-03, AC-04).
- WZ35.3 (DEV) Übernehmen und Abspielleiste (AC-05).
- WZ35.4 Review, alle Kriterien, schließt ab.

## Abnahme

2026-10-31: AC-01 bis AC-05 belegt (WZ35.1 bis WZ35.3), `task check` und `task check:go` grün, `check:race` lokal ohne gcc übersprungen. Messung bei 600 px beobachtet, Ralf hat abgenommen. Keine neuen Tickets.
