# B-758 · Release-Workflow: Typecheck und Tests vor dem Bauen wieder einschalten

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ41
- **Projekt:** WZG
- **Erstellt:** 2026-11-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Für v0.22.0 (Test auf dem Raspberry Pi) hat 🧑 am 2026-11-08 entschieden, die Tests vor dem Release vorläufig herauszunehmen: `release.yml` führt `task typecheck` und `task test` nicht mehr aus, die Zeitgrenze steigt auf 30 Minuten. Die Release-Checkliste (check:all, CI grün) wurde für v0.22.0 nicht erfüllt.

## Ziel

Der Release-Workflow prüft wieder Typecheck und Tests, bevor Paket und Image entstehen; die CI auf develop ist grün.

## Beteiligte und Zielgruppen

Entwickler (🧑), Release.

## Anforderungen

- `task typecheck` und `task test` stehen wieder im Release-Workflow vor `task build`.
- Die CI auf develop ist grün (B-756, B-757 behoben; Restbefunde als Tickets).
- Nachholen: `task check:all` für den Stand von v0.22.0, Befunde als Tickets.

## Nicht-Ziele

Kein neues Release.

## Regeln und Einschränkungen

Release-Checkliste in `docs/arbeitsweise.md` › Release. Default (Claude, 2026-11-12): AC-03 prüft den aktuellen Stand von `develop` statt des Tags v0.22.0.

- **Bereich:** `.github/workflows/release.yml`
- **Abhängig von:** B-756, B-757
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

Tag v0.23.0 → Release-Lauf führt Typecheck und Tests aus, bevor er das Image veröffentlicht.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** `release.yml` enthält `task typecheck` und `task test` vor `task build`.
- **AC-02** CI auf develop grün.
- **AC-03** `task check:all` für v0.22.0 nachgeholt, Befunde als Tickets.

## Offene Fragen

keine

## Notizen

Vorläufige Ausnahme für den Pi-Test, Beschluss 🧑 2026-11-08 (Chat).
