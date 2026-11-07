# MG9.2 · Monarch: Reit-Idle, Gangarten je Reittier, Zustände, Spielerfarben

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** monarch
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/mg9
- **Abhängig von:** MG9.1
- **Tickets:** B-717
- **Kriterien:** AC-02

## Ziel

Der Monarch ist in jeder Lage animiert: Reit-Idle, Gangarten je Reittier, `give`, `hit`, `death`, `respawn`; die vier Spielerfarben sind klar unterscheidbar, die Satteldecke folgt dem Rock.

## Kontext

Ticket B-717 (schließt B-711). Spec `public/grafik/figuren/monarch.paper.json` (Rig `figur`, Reiten über `reittier`-Animationen, Varianten spieler1–4). Heute nur Pferd als Reittier (D-US2-03), `give/hit/death/respawn` fallen auf `idle` (D-US1-03), Spieler 4 nutzt `leder`. Nach MG9.1 (gleiche Reittier-Liste, einheitlicher Maßstab). Truppen-Zustände liegen in MG9.3.

## Erlaubte Dateien

- `public/grafik/figuren/**`
- `public/grafik/stil.paper.json` (nur neue Spielerfarben; nicht nötig, `kristall` existiert)
- `tools/paper/golden/**`, `tools/paper/rig/gangarten.ts`, `proportionen.ts`, `tools/paper/schema/**` (neue Gangart `stand` für das Reit-Idle)
- `docs/standards/paper-spec.md`
- `docs/sprints/aktiv/MG9-papier-figuren-nachbessern/**`

## Nicht-Ziele

Einbau ins Spiel (B-724, US3); Truppen (MG9.3).

## Schritte

1. Test zuerst: Animationen `reiten-idle` und `schritt|trab|galopp-<reittier>` für jedes Reittier der Gruppe, Keyframes `give`, `hit`, `death`, `respawn`, vier unterscheidbare Rock- und Satteldeckenfarben ohne Braun.
2. Animationen und Varianten in der Spec ergänzen.
3. `task gen:paper` für `figuren`, Vorschau je Reittier und Spieler ansehen, Golden schreiben.

## Fertig, wenn

- [ ] AC-02: `figuren/monarch` hat `reiten-idle` und je Reittier `schritt/trab/galopp-<id>` (B-717/AC-01).
- [ ] AC-02: `give`, `hit`, `death`, `respawn` vorhanden; Truppen siehe MG9.3 (B-717/AC-02).
- [ ] AC-02: Varianten spieler1–4 mit unterscheidbarer Rock- und Satteldeckenfarbe, keine auf Braun (B-717/AC-03).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

2026-11-07, Claude.

- AC-02 / B-717/AC-01 umgesetzt, geprüft (`tools/paper/golden/monarch.test.ts`, `rig/stand.test.ts`): `reiten-idle` (Pferd, neue Gangart `stand`: alle Hufe am Boden, Atmen, Nicken; in Schema, Typ, `paper-spec.md`; immer erlaubt, nie Pflicht) und `<gangart>-<reittier>` für die zehn übrigen Reittiere (Elefant ohne `galopp`: 28 Animationen); das Pferd bleibt ohne Zusatz, wie `mountedAnim` im Client es erwartet. Abweichung: `reiten-idle` gibt es nur für das Pferd (`reiten-idle-<reittier>` hätte die Ausgabe um weitere ~10 Animationen × 5 Varianten vergrößert; Client fällt aufs Pferd zurück).
- AC-02 / B-717/AC-02 für den Monarchen umgesetzt, Sichtprüfung per `assets_preview` (Claude, Tod und Aufstehen als einfache Rumpf-Drehung, optisch schlicht): `give`, `hit`, `death`, `respawn` als Keyframes; die Truppen folgen in MG9.3 (gleiche Teilnamen).
- AC-02 / B-717/AC-03 umgesetzt, geprüft (Test) und per Vorschau gesehen: Satteldecke folgt der Rockfarbe (Variante tauscht auch das Reittier, MG9.1); Spieler 4 jetzt `kristall` (Türkis) statt `leder`, Default-Entscheidung, Sichtprüfung durch Ralf offen.
- Nebenbei B-737 (Ralf, im Chat): Die PNGs der neuen Animationen hätten das Repo um ~60 MB vergrößert, deshalb liegen gerenderte Bilder seit Commit 8ab1bf22 nicht mehr im Repo (`.work/grafik`, `task gen:paper`).
