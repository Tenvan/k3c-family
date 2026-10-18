# CI1 · INF · CI-Nachweis, Release-Image ohne Dev-Mode, Test-Abdeckung

- **Status:** aktiv
- **Projekt:** REL
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** B-273, B-019
- **Start-Commit:** 8652833b
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-18, Ralf im Chat (Revision 1)

## Ausgangslage

Das Release-Image startet mit Dev-Mode (B-273: `Dockerfile` und `compose.yaml` setzen `K3C_DEV=0` nicht), die Test-Abdeckung ist nicht sichtbar (B-019). Der CI-Nachweis für SP01 (B-053) ist erledigt: die CI läuft auf `develop` grün in allen Jobs (Stand 2026-10-18).

## Ziel

Release-Images sind sicher, Abdeckungslücken sind sichtbar. Am Ende sichtbar: Release-Image lehnt Dev-Aktionen ab (Smoke-Test der CI prüft es), Abdeckung pro `engine/`-Paket im CI-Bericht.

## Beteiligte und Zielgruppen

🧑 veröffentlicht Releases; Agent ändert CI und Docker.

## Anforderungen

B-273 › Anforderungen; B-019 › Anforderungen. Sprint-eigen:
- **Dev-Mode:** `Dockerfile` und `compose.yaml` setzen `K3C_DEV=0` (Standard im Image), `K3C_DEV=1` schaltet ihn am Pi ein; `task start` am Entwicklerrechner bleibt Dev (Entscheidung 🧑 2026-10-18).
- **Smoke-Test:** Der Docker-Job der CI (`.github/workflows/ci.yml`) prüft, dass der Container aus dem Image `dev=false` loggt.
- **Abdeckung:** `go test -cover` je Paket unter `engine/`, als Task und im CI-Bericht; nur Anzeige, kein Mindestwert, kein Gate (Entscheidung 🧑 2026-10-18); Pakete ohne Tests erscheinen mit 0 %.

## Nicht-Ziele

Neue Prüfungen über die Abdeckung hinaus.

## Regeln und Einschränkungen

INF; `Dockerfile` und `compose.yaml` berühren SRV (Grenzfall, Hinweis in der Session CI1.1); Docker-Smoke-Test der CI muss grün bleiben; Befehle nur über `task`.

## Beispiele

Release-Image starten, Dev-Aktion senden → 🚫 abgelehnt.

## Ausnahme- und Fehlerfälle

CI rot wegen Flake → Ticket (NT1), nicht wiederholen bis grün.

## Akzeptanzkriterien

- **AC-01** Das Release-Image startet den Server ohne Dev-Mode: `docker compose up -d` loggt `dev=false`, `K3C_DEV=1` schaltet ihn ein, `task start` bleibt Dev; der Docker-Job der CI prüft `dev=false` (B-273/AC-01, B-273/AC-02, B-273/AC-03).
- **AC-02** Test-Abdeckung der Engine ist sichtbar: Task und CI-Bericht zeigen die Abdeckung pro Paket unter `engine/`, kein Gate (B-019/AC-01).

## Offene Fragen

Entschieden von 🧑 am 2026-10-18 (Chat): Dev-Mode im Image aus (B-273), Abdeckung nur anzeigen (B-019), B-053 erledigt. Offen: keine; die Spec-Freigabe steht aus.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| CI1.1 | `CI1.1-release-image-dev-mode.md` | Umsetzung | autonom | fertig |
| CI1.2 | `CI1.2-abdeckung-engine.md` | Umsetzung | autonom | fertig |
| CI1.3 | `CI1.3-review.md` | Review | autonom | offen |

- CI1.1 Release-Image ohne Dev-Mode: `Dockerfile`, `compose.yaml`, Smoke-Test in der CI (AC-01).
- CI1.2 Abdeckung pro `engine/`-Paket als Task und im CI-Bericht (AC-02).
- CI1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
