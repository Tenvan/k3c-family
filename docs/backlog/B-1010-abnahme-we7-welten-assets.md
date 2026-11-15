# B-1010 · Abnahme WE7: Welten-Assets sind gesichtet, Gen-Läufe und Texturbudget geprüft

- **Domäne:** PAP
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WE7
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint WE7 hat 14 Papierschnitt-Assets für Sengoku, Nil und Britannien geliefert (PRs #637, #641, #643). Die Review-Session WE7.4 fand keine schweren Befunde; offen sind die Sichtprüfung durch 🧑 und die Einordnung des Texturbudgets.

## Ziel

WE7 ist abgenommen: Assets gesichtet, Gen-Läufe grün, Texturbudget eingeordnet.

## Beteiligte und Zielgruppen

🧑 (Sichtprüfung im Bereich Assets der Workbench), Claude (Gen-Läufe, Messung).

## Anforderungen

- Sichtprüfung der 14 Assets: Torii, Samurai-Lager, Pagode, Ninja, Samurai, Palme, Obelisk, Skorpion, Mumie, Barke, Steinkreis, Rundhaus, römischer Späher, Moorgeist; Barke und Rundhaus in allen Stufen; Ohr-Teil des Spähers (darf nicht wie ein Goblin-Ohr wirken).
- Gen-Läufe am Endstand grün.
- Texturbudget neu gemessen und gegen Entscheidung 008 eingeordnet.

## Nicht-Ziele

Zuordnung der Assets im Spiel (B-996, WE8), Simulation, Balancing.

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/objekte/**`, `public/grafik/gegner/**`, `public/grafik/gebaeude-paper/**`
- **Abhängig von:** WE7.4
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –
- Assets ändern nur über `assets_*`.

## Beispiele

🧑 öffnet den Bereich Assets, sichtet die 14 Einträge und markiert jeden als abgenommen oder mit Hinweis.

## Ausnahme- und Fehlerfälle

Ein Asset fällt durch → Nachbesserung über `assets_*` als eigene Session, dieses Ticket bleibt offen.

## Akzeptanzkriterien

- **AC-01** 🧑 hat die 14 Assets im Bereich Assets gesichtet (zugleich B-988/AC-04, B-989/AC-04, B-990/AC-04).
- **AC-02** `check_run task:test` samt `gen:paper` und Grafik-Golden grün (Endstand in WE7.4 geprüft: 411 Tests grün, Golden unverändert).
- **AC-03** Texturbudget gemessen und eingeordnet (WE7.4: 880 MiB RGBA unter `public/grafik`, davon 23,6 MiB neu; Entscheidung 008 nennt 580 MiB).
- **AC-04** Rig-Typ von Ninja und Skorpion (`figur` bzw. `requisite` statt `vierbeiner`) für WE8 bestätigt.

## Offene Fragen

Gilt das Budget von 580 MiB aus Entscheidung 008 für alle Assets auf der Platte oder nur für die je Level geladenen? Entscheidet 🧑.

## Notizen

Befunde aus WE7.4 (k3c-grafik).
