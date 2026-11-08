# B-744 · k3c-sim: Replay-Dateinamen aus Seeds absichern und Segment-Kopie der Workbench per Test halten

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** RST
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Review RS2.6: `cmd/k3c-sim/balance.go` (`writeReplays`) baut Dateinamen mit `filepath.Join(dir, fmt.Sprintf("%s-p%d-…", res.Seed, …))`; ein Seed aus `--seed-list` mit `/` oder `..` schreibt außerhalb von `--replay-dir` (vermutlich schon in `bin/k3c-balance` so, lokal, eigene Eingabe). Außerdem ist `segment` in `tools/k3c-workbench/internal/mcpsrv/tools_engine.go` eine Kopie von `enginetools.Segment`; `k3c-sim run` liest mit `DisallowUnknownFields`, ein neues Feld auf einer Seite bricht mit Exit 2, ein Gleichlauf-Test fehlt (ungeprüft). Nebenbei: `docs/protocol.md` nennt in der Nachrichten-Tabelle `v` = 9, Code und Kopf 11 (RS2.4).

## Ziel

Eingaben von `k3c-sim` bleiben im vorgesehenen Ordner, und die Workbench-Kopie der Eingabe-Typen kann nicht still auseinanderlaufen.

## Beteiligte und Zielgruppen

Agenten (Balancing, `sim_run`).

## Anforderungen

- Seed im Dateinamen säubern (nur `a-z0-9-_`) oder Pfad nach dem Join gegen `--replay-dir` prüfen.
- Test, der die Felder von `mcpsrv.segment` gegen `enginetools.Segment` vergleicht (Quelltext-Abgleich wie `names_test.go`, kein Import).
- `docs/protocol.md` Versionsangabe auf 11.

## Nicht-Ziele

Protokolländerung.

## Regeln und Einschränkungen

depguard der Workbench (kein Import von `engine/enginetools`), B-726.

## Beispiele

`--seed-list ../x` → Datei liegt in `--replay-dir` oder Abbruch mit Exit 2.

## Ausnahme- und Fehlerfälle

nicht relevant

## Akzeptanzkriterien

- **AC-01** Test in `cmd/k3c-sim`: Seed mit `/` und `..` schreibt nicht außerhalb von `--replay-dir`.
- **AC-02** Test in der Workbench bricht, wenn `enginetools.Segment` ein Feld bekommt, das `segment` nicht hat.
- **AC-03** `docs/protocol.md` nennt überall Version 11.

## Offene Fragen

keine

## Notizen

Aus Review RS2.6, Befunde mittel 3 und 6; Protokoll-Doku aus RS2.4.
