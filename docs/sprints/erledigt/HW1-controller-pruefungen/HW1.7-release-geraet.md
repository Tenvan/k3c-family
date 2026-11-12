# HW1.7 · Release-Checkliste am Gerät: Pi-Image und Version (aus RL1.2)

- **Status:** verworfen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** INF
- **Umgebung:** live
- **Branch:** hw1/7-release-geraet
- **Abhängig von:** –
- **Tickets:** B-170
- **Kriterien:** AC-06

## Ziel

Die Geräte-Punkte der Release-Checkliste sind geprüft: Pi-Image per Pull am Pi und Version stimmt am Pi und auf der Xbox.

## Kontext

Übernommen aus RL1.2 (PJ3, 2026-10-08). Dort stehen der Agenten-Teil des Probelaufs (2026-10-04, Tabelle je Punkt) und der PC-Nachweis „Version stimmt“ (2026-10-07). Offen sind nur die Punkte, die ein Gerät brauchen: Pi-Image (Pull am Pi, `docker compose pull && docker compose up -d`) und Version am Pi und auf der Xbox (Landingpage-Versionszeile, `/api/health`). Liste: `docs/arbeitsweise.md` › Release. Kein Tag.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status), HW1-README (Tabelle), `docs/backlog/` (neue Tickets für rote Punkte)

## Nicht-Ziele

Tag setzen, Befunde beheben (nur Tickets), die Liste ändern.

## Schritte

1. Am Pi das aktuelle Image ziehen und starten.
2. Versionszeile der Landingpage auf der Xbox und `/api/health` am Pi vergleichen.
3. Je Punkt grün oder rot (Ticket) ins Ergebnis, `Status: fertig`.

## Fertig, wenn

- [ ] AC-06: Pi-Image und Version am Gerät mit Ergebnis je Punkt; rote Punkte als Tickets.

## Prüfen

Manuell durch 🧑 am Pi und an der Xbox.

## Ergebnis

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009).
