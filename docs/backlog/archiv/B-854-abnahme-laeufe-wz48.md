# B-854 · Abnahme-Läufe WZ48

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** WZ48
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Sprint WZ48 (Tests und CI) ist erledigt. Der CI-Nachweis für B-828 und die Windows-Runner (neue Short-Skips, Grenzen für `-test.parallel`) geht erst nach dem Merge auf `develop`.

## Ziel

Die Abnahme-Läufe von WZ48 sind grün, die CI auf `develop` ist grün.

## Beteiligte und Zielgruppen

Claude autonom; 🧑 liest das Ergebnis.

## Anforderungen

Läufe laut `docs/arbeitsweise.md` › Sim, Balance und Gen, nur über `check_run`; CI-Lauf auf `develop` ansehen.

## Nicht-Ziele

Nachbessern im Ticket.

## Regeln und Einschränkungen

Sim und Balancing gesperrt.

- **Bereich:** – (nur Prüfläufe)
- **Abhängig von:** WZ48
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Prüfläufe.

## Ausnahme- und Fehlerfälle

Rot → Problem-Ticket im Projekt WZG.

## Akzeptanzkriterien

- **AC-01** Sim und Balance verschoben (Sperre 2026-11-11).
- **AC-02** Gen: `check_run task:test` grün.
- **AC-03** Race: `check_run task:check:race` (lokal ohne gcc als übersprungen nennen).
- **AC-04** CI auf `develop`: Jobs `go` und `k3c-workbench` grün (B-828/AC-01).

## Offene Fragen

keine

## Notizen

Vorschlag aus WZ48.4 (2026-11-14).

2026-11-14 (Claude, auf `sprint/welle6`): AC-01 verschoben (Sperre). AC-02 `check_run task:test` grün (4 min 4 s). AC-03 `check_run task:check:race` lokal ohne gcc übersprungen. Offen: AC-04 (CI auf `develop`), erst nach dem Merge.
