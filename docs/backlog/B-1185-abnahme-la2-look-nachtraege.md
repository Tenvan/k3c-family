# B-1185 · Abnahme LA2: Cheat-Panel, Wolken, Wellenstriche, Licht-Assets, Leitmotive (PC-Abnahme, Gen, Race)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** LA2
- **Projekt:** RSU
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint LA2 (B-1179, B-1181, B-1183) ist umgesetzt (LA2.1 bis LA2.4) und im Review (LA2.5). Offen sind die in Sessions gesperrten Läufe (B-742) und die Abnahme am PC.

## Ziel

Abnahme-Ticket des Sprints LA2 (genau eins): (1) am PC abnehmen: Cheat-Fenster (F4) Gruppe „Umgebungseffekte“ schaltet jeden Effekt live; Wolken ziehen im k2c-Profil; Glanzstriche des Spiegel-Flusses dunkler, nachts ruhiger; Licht-Assets (Fackel, Laterne, Feuerschale) je Welt mit animierter Flamme an den Ankern; Leitmotiv je Welt als feste Hintergrund-Ebene; (2) `task check:race`; (3) `task gen:paper` samt Golden; (4) Textur-Budget der vier geladenen Leitmotive messen. Rot: neues Ticket.

## Beteiligte und Zielgruppen

🧑 Ralf (Abnahme am PC, Freigabe).

## Anforderungen

- **AC-01** PC-Abnahme durch 🧑 der fünf Punkte oben.
- **AC-02** `task check:race` und `task gen:paper` samt Golden grün (oder Befund als Ticket).
- **AC-03** Review-Befunde LA2.5 entschieden: (a) Panel-Umschalter startet immer „an“, Ist-Zustand (z. B. `K3C_FX_OFF`) fehlt im State; (b) fx-Tests ändern globalen Zustand parallel (flaky-Risiko); (c) `MOTIFS` im Code statt `Paper::motif_asset` aus `data/paper.json`; (d) alle vier Leitmotive je Stufe mit Himmel geladen (+5,6 MiB), nur eines gezeichnet; (e) Welt-ID je Bild (`world_of(f)`) statt je Zelle: im Split-Screen mit zwei Welten falsches Motiv/Licht-Asset; (f) Hof der Fackel (`halo.rs`) sitzt auf 1,45 u, die Flamme des Sprites höher; (g) `cloud_boxes` allokiert je Bild einen `Vec`; (h) `fx/festival.rs` fragt Animation `stand`, die Assets haben `idle`.
- **AC-04** `tests/e2e_flow.rs` (`skillpunkt` bei Takt 411 statt 535) und `alloc_je_bild` wieder grün (vorher schon rot, ungeprüft seit wann).

## Nicht-Ziele

Neue Funktionen; Balancing.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
