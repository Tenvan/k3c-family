# CI1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** CI-Nachweis, Release-Image ohne Dev-Mode, Test-Abdeckung
- **Projekt:** REL

## Ziel

Release-Images sind sicher, Abdeckungslücken sind sichtbar. Am Ende sichtbar: Release-Image lehnt Dev-Aktionen ab (Smoke-Test der CI prüft es), Abdeckung pro `engine/`-Paket im CI-Bericht.

## Ergebnis

2026-10-18, Review CI1.3 (getrennter Reviewer): AC-01 und AC-02 geprüft (Diff, Lauf 39128694546 auf `sprint/ci1` grün; `dev=false`/`dev=true`, Abdeckung je Paket in der Job-Zusammenfassung; `task check` grün). Behoben: Widerspruch zur Spec-Freigabe, Kontrollkästchen CI1.1/CI1.2. Neue Tickets: keine. B-053 vorab erledigt. Version: v– vorgeschlagen (Patch: Release-Image-Standard `K3C_DEV=0`, kein Spielinhalt), gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Das Release-Image startet den Server ohne Dev-Mode: `docker compose up -d` loggt `dev=fals … |
| AC-02 | geprüft | Test-Abdeckung der Engine ist sichtbar: Task und CI-Bericht zeigen die Abdeckung pro Paket … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-273 | erledigt | AC-01 geprüft | Das Release-Image startet den Server ohne Dev-Mode |
| B-019 | erledigt | AC-02 geprüft | Test-Abdeckung der Engine ist sichtbar |
