# B-1267 · Abnahme-Läufe WE6: Race, Gen und Sichtprüfungen Endboss, Bau, Skins, Welten-Editor

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WN4
- **Projekt:** WZN
- **Erstellt:** 2026-11-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint WE6 (Endboss, Bau und Skins im Rust-Spiel, Welten-MCP und Welten-Editor mit neuen Feldern) ist per Review WE6.5 geprüft; Sim, Gen und Race liefen dort nicht (B-742).

## Ziel

Die gesperrten und manuellen Prüfungen des Sprints WE6 sind abgenommen.

## Beteiligte und Zielgruppen

Claude (autonome Läufe), 🧑 Ralf (Sichtprüfungen).

## Anforderungen

- `check_run task:check:race` grün.
- `gen:paper` samt Grafik-Golden grün, falls WE6 Grafik berührt (Skins).
- 🧑 Sichtprüfung: Endboss und Bau je Biom im Spiel (B-993/AC-03), Welten-Editor mit Endboss-Auswahl, Events, Plätzen, Skin-Tabelle (B-987/AC-04).

## Nicht-Ziele

Neue Funktionen; Befunde der Review stehen in B-1266.

## Regeln und Einschränkungen

Sim und Balancing bleiben gesperrt (Beschluss 2026-11-11); hier nur Race und Gen.

## Beispiele

-

## Ausnahme- und Fehlerfälle

Rotes Kriterium: neues Ticket, Messwerte in die Notizen.

## Akzeptanzkriterien

- **AC-01** `task:check:race` grün.
- **AC-02** `gen:paper` und Golden grün oder nicht betroffen.
- **AC-03** 🧑 Sichtprüfung Endboss und Bau je Biom (B-993/AC-03).
- **AC-04** 🧑 Sichtprüfung Welten-Editor (B-987/AC-04).

## Offene Fragen

Keine.

## Notizen

Angelegt von WE6.5.
