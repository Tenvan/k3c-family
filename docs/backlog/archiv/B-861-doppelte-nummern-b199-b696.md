# B-861 · Die doppelt vergebenen Ticket-Nummern B-199 und B-696 sind aufgelöst

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ57
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

Je zwei Archiv-Dateien tragen B-199 und B-696 (`archiv/B-199-raum-fuenf-stufen.md`, `archiv/B-199-review-diff-befehl.md`; `B-696-ticket-relationen-und-index-verdichtung.md`, `archiv/B-696-gebaeude-in-allen-aufloesungen-rendern.md`). WZ47 toleriert das im Index, die Sonderfälle bleiben aber.

## Ziel

Jede Ticket-Nummer kommt genau einmal vor; die jüngere Datei jeder Dublette trägt eine neue Nummer mit Verweis auf die alte.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

- Neue Nummern über `plan_create` (Inhalt übernommen), alte Datei per `Ersetzt`/Notiz verknüpft und entfernt.
- Verweise in Sprints und Tickets nachziehen.

## Nicht-Ziele

Andere Archiv-Pflege.

## Regeln und Einschränkungen

Klasse A (Schuld, DEV). Löschen einer Ticket-Datei nur per `git rm` mit Freigabe von 🧑 am Stand, solange kein Werkzeug es kann.

- **Bereich:** `docs/backlog/**`, `docs/sprints/**`
- **Abhängig von:** WZ47
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`archiv/B-199-review-diff-befehl.md` → neue Nummer, Notiz „umnummeriert von B-199“.

## Ausnahme- und Fehlerfälle

Verweis auf die alte Nummer ohne Kontext → bleibt, Notiz klärt.

## Akzeptanzkriterien

- **AC-01** Planungstest: keine doppelte Nummer unter `docs/backlog`.

## Offene Fragen

keine

## Notizen

Befund WZ47.4 (2026-11-14).
