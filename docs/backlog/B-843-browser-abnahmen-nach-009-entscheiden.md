# B-843 · Offene Browser-Abnahmen KB4.4, KB5.6, W15.3 und PB1.2 sind nach Entscheidung 009 entschieden

- **Domäne:** INF
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Vier aktive Sprints warten nur noch auf eine Mensch-Session, die den Browser-Client prüft: KB4.4 (Debug-Panel, Boss und Event), KB5.6 (Spiel zu zweit und Studio; das Studio ist seit Entscheidung 005 entfallen), W15.3 (Fackeln, „auf der Xbox oder PC“) und PB1.2 (itch.io-Paket mit Server-EXE und Browser-Client, B-023). Entscheidung 009 löst den Browser-Client ab und hat offene Browser-Planung gesammelt verworfen (Punkt 7); diese vier sind dabei stehen geblieben (WZ44.4, 2026-11-13).

## Ziel

🧑 hat je Session entschieden: verwerfen (Sprint danach erledigt) oder am PC im Browser-Client noch abnehmen, solange er lauffähig ist.

## Beteiligte und Zielgruppen

🧑 entscheidet; Claude setzt den Status über `plan_set`.

## Anforderungen

- Je Session eine Entscheidung mit Datum.
- Verworfene Sessions auf `verworfen`, Sprint danach `erledigt` mit Vermerk in der Abnahme.

## Nicht-Ziele

Code; Ablösung selbst (B-823).

## Regeln und Einschränkungen

Nur Planungsdateien über `plan_*`.

- **Bereich:** `docs/sprints/**`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** S
- **Sperre:** –

## Beispiele

PB1.2 verworfen → PB1 erledigt; Veröffentlichung neu als natives Paket in NAT.

## Ausnahme- und Fehlerfälle

Abnahme gewünscht, Browser-Client schon entfernt (B-823) → verwerfen und Befund als Ticket für das native Spiel.

## Akzeptanzkriterien

- **AC-01** KB4.4, KB5.6, W15.3 und PB1.2 sind fertig oder verworfen; ihre Sprints liegen danach in `erledigt/`.

## Offene Fragen

Vorschlag Claude: alle vier verwerfen, PB1 als natives Paket neu planen.

## Notizen

Befund aus WZ44.4 (B-785). Umnummeriert 2026-11-13 von B-827 (Kollision mit `sprint/ng19`).
