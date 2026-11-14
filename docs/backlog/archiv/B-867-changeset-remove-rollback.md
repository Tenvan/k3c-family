# B-867 · Ein gescheitertes Entfernen in changeSet.apply wird zurückgenommen

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ55
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WZ54.2: `changeSet.apply` nimmt seit B-862 gescheiterte Schreib- und Verschiebe-Schritte zurück, ein gescheitertes `remove` aber nicht (Altverhalten).

## Ziel

Auch ein gescheitertes Entfernen lässt den Planungsstand unverändert.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

- `remove` sichert die Datei vorher, Rollback stellt sie wieder her.
- Test mit erzwungenem Fehler.

## Nicht-Ziele

Andere Schritte.

## Regeln und Einschränkungen

Klasse A (Schuld, DEV).

- **Bereich:** `tools/k3c-workbench/internal/planning/**`
- **Abhängig von:** WZ54
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Entfernen scheitert nach zwei Schreibschritten → alle drei zurückgenommen.

## Ausnahme- und Fehlerfälle

Rücknahme scheitert → Meldung wie in B-862.

## Akzeptanzkriterien

- **AC-01** Test: gescheitertes `remove` hinterlässt byte-gleichen Stand.

## Offene Fragen

keine

## Notizen

Befund WZ54.2 (2026-11-14).
