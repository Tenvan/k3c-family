# B-019 · Test-Abdeckung der Engine ist sichtbar

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Niemand sieht, welche Teile der Spiel-Logik ungetestet sind.

## Ziel

Test-Abdeckung der Engine ist sichtbar. Nutzen: Lücken in der Spiel-Logik früh sehen.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die Sessions autonom abarbeiten; Review-Session.

## Anforderungen

- Abdeckung für `engine/` messen.
- Die CI zeigt sie pro Paket.

## Nicht-Ziele

Abdeckung des TypeScript-Clients.

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.

## Beispiele

Ein PR ändert `engine/sim` → das CI-Log zeigt die Abdeckung von `engine/sim`.

## Ausnahme- und Fehlerfälle

Paket ohne Tests → erscheint mit 0 %, nicht gar nicht.

## Akzeptanzkriterien

- **AC-01** Die CI zeigt die Abdeckung pro Paket unter `engine/`.

## Offene Fragen

Soll eine Mindestabdeckung die CI scheitern lassen? (🧑)

## Notizen

–
