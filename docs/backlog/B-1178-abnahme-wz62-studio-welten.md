# B-1178 · Abnahme WZ62: Studio feste Welten, Welt-Filter, Größenvergleich, Welten-Editor (PC-Abnahme, Sim, Balance, Gen, Race)

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WZ62
- **Projekt:** WZG
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint WZ62 (Studio: feste Welten, Welt-Filter mit Vererbung, Größenvergleich mit Lineal, Welten-Editor mit Link, Löschen und Werte-Tabelle; B-1168 bis B-1173) ist in main (PR #791). Review WZ62.5 ist durch, Fixes sind in main (PRs #792 und #793). Offen sind die Läufe, die in Sessions gesperrt sind (B-742), und die Abnahme am PC.

## Ziel

Abnahme-Ticket des Sprints WZ62 (genau eins): (1) Studio am PC abnehmen: feste Welten (standard, sengoku, nil, britannien) gelistet, Welt-Filter mit Vererbung, Größenvergleich mit Lineal, Welten-Editor (Link zum Katalog, Welt löschen mit Namensbestätigung, Werte-Tabelle); (2) `task check:race`; (3) `task gen:paper` samt Golden; (4) Sim und Balance nur über `sim_test`, falls die Werte-Tabelle (WZ62.7) Balancing-Werte berührt. Rot: neues Ticket.

## Beteiligte und Zielgruppen

🧑 Ralf (Abnahme am PC, Freigabe); Entwickler des Studios (Workbench).

## Anforderungen

- **AC-01** Studio-Abnahme am PC durch 🧑: die sechs Funktionen oben funktionieren wie in den Session-Ergebnissen WZ62.1 bis WZ62.4, WZ62.6, WZ62.7 beschrieben.
- **AC-02** `task check:race` und `task gen:paper` samt Golden laufen grün (oder Befund als Ticket).
- **AC-03** Offene Review-Befunde aus WZ62.5 entschieden: stille Lese- und Parsefehler in `assetsui/welt.go` (`leseWelten`, `leseJSON`), `world_delete` ohne eigenes `checkout`-Feld (Worktree-Auflösung ungeprüft), Atomic-Design-Prüfung von `WeltFilter`, `ValueField`, `SizeScene` (vom Reviewer nicht gelesen).
- **AC-04** Planungsdaten wieder grün: `planning.test.ts` (RU30 Vorlage, Projekt je Sprint, Fahrplan nennt jeden Sprint-Ordner).

## Nicht-Ziele

Neue Funktionen im Studio; Balancing der Spielwerte.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

Siehe Anforderungen AC-01 bis AC-04; Nachweis im Ergebnis dieses Tickets.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
