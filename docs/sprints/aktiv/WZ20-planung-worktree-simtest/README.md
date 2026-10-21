# WZ20 · DEV · k3c-dev: Planungstest für Entwürfe, Worktree-Einträge vollständig und lesend, sim_test im Worktree

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-440, B-484, B-483, B-423
- **Start-Commit:** 376bf508
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-21, Chat (Ralf), Revision 1

## Ausgangslage

Der Go-Planungstest (`tools/k3c-dev/internal/planning/planning_test.go`, `TestRepo`) lehnt Sprints ohne Session-Tabelle ab, auch gültige Entwürfe (B-440). Auf der Planungsseite erscheinen Einträge, die nur in einem Worktree existieren, wie Wurzel-Einträge mit Menüs, die gegen die Wurzel scheitern (B-483); ein neuer Sprint aus einem Worktree erscheint ohne Sessions und Projekt (`diffSessions` in `wtdiff.go`, B-484). `sim_test` rechnet immer mit Code und Daten der Repo-Wurzel, nicht mit dem Worktree der Session (B-423).

## Ziel

Planung und Testläufe aus Worktree-Sessions sind verlässlich: Entwurfs-Sprints bestehen den Planungstest, Worktree-Einträge erscheinen vollständig und nur lesend, `sim_test` rechnet mit dem eigenen Checkout. Am Ende sichtbar: `check_run dev:test` grün, Planungsseite mit vollständigem neuem Worktree-Sprint, `sim_test status` nennt den Checkout.

## Beteiligte und Zielgruppen

Agenten in Worktree-Sessions und 🧑 auf der Planungsseite; Umsetzung durch Agent in DEV; 🧑 gibt die Spec frei.

## Anforderungen

B-440, B-484, B-483, B-423 › Anforderungen; für B-440 und B-423 gelten Ziel und Akzeptanzkriterium dieser README.

## Nicht-Ziele

Schreiben in Worktree-Einträge von der Planungsseite aus; neue Planungs-Tools (`plan_project` usw.); Testläufe selbst (Projekt TST).

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-dev/`). `sim_test` übernimmt das Argument `checkout` wie `check_run` (B-388); Tests nicht abschwächen. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Sprint `Reife: Entwurf` mit Stichpunkten statt Session-Dateien → `dev:test` grün. Neuer Sprint nur im Worktree → Planungsseite zeigt ihn mit Sessions im richtigen Projekt, Pill „neu“, ohne Menüs. `sim_test start {checkout: "<worktree>"}` → Lauf rechnet mit `data/` und `engine/` des Worktrees.

## Ausnahme- und Fehlerfälle

Unbekannter oder mehrdeutiger `checkout` bei `sim_test` → Ablehnung mit Liste der Worktrees. Sprint ohne Titel → Planungstest scheitert weiter.

## Akzeptanzkriterien

- **AC-01** Der Go-Planungstest akzeptiert Sprints mit `Reife: Entwurf` ohne Session-Dateien; `check_run dev:test` ist grün (B-440).
- **AC-02** Ein Sprint, der nur in einem Worktree existiert, erscheint auf der Planungsseite mit Sessions, Projekt und Pill „neu“ (B-484).
- **AC-03** Einträge nur im Worktree sind als solche erkennbar und nur lesend; Menüs und Rang-Knöpfe, die gegen die Wurzel scheitern würden, erscheinen nicht (B-483).
- **AC-04** `sim_test start` nimmt `checkout` und rechnet mit Code und Daten dieses Checkouts; der Status nennt den Checkout (B-423).

## Offene Fragen

keine. Entschieden von 🧑 (2026-10-21): `sim_test` mit Worktree baut die Binaries im Worktree (fester EXE-Pfad je Worktree, Proben nur auf Loopback) und rechnet mit dessen `data/`.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ20.1 | `WZ20.1-planungstest-entwuerfe-wt-sprints.md` | Umsetzung | autonom | fertig |
| WZ20.2 | `WZ20.2-wt-eintraege-nur-lesend.md` | Umsetzung | autonom | offen |
| WZ20.3 | `WZ20.3-sim-test-checkout.md` | Umsetzung | autonom | offen |
| WZ20.4 | `WZ20.4-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
