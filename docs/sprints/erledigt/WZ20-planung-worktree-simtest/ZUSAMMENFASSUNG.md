# WZ20 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** k3c-dev: Planungstest für Entwürfe, Worktree-Einträge vollständig und lesend, sim_test im Worktree
- **Projekt:** WZG

## Ziel

Planung und Testläufe aus Worktree-Sessions sind verlässlich: Entwurfs-Sprints bestehen den Planungstest, Worktree-Einträge erscheinen vollständig und nur lesend, `sim_test` rechnet mit dem eigenen Checkout. Am Ende sichtbar: `check_run dev:test` grün, Planungsseite mit vollständigem neuem Worktree-Sprint, `sim_test status` nennt den Checkout.

## Ergebnis

2026-10-21: AC-01 bis AC-04 nachgewiesen (WZ20.1–WZ20.4), `check:diff` und `check:go` grün; keine Befunde, keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Der Go-Planungstest akzeptiert Sprints mit `Reife: Entwurf` ohne Session-Dateien; `check_r … |
| AC-02 | – | Ein Sprint, der nur in einem Worktree existiert, erscheint auf der Planungsseite mit Sessi … |
| AC-03 | – | Einträge nur im Worktree sind als solche erkennbar und nur lesend; Menüs und Rang-Knöpfe, … |
| AC-04 | geprüft | `sim_test start` nimmt `checkout` und rechnet mit Code und Daten dieses Checkouts; der Sta … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-440 | erledigt | AC-01 – | k3c-dev-Planungstest lehnt Entwurfs-Sprints ohne Sessions ab |
| B-484 | erledigt | AC-02 – | Worktree-Vergleich: Sessions und Projekt eines neuen Sprints fehlen in den Pills |
| B-483 | erledigt | AC-03 – | Planungsseite: Einträge nur im Worktree sind nur lesbar (Detail, Menüs, Rang) |
| B-423 | erledigt | AC-04 geprüft | sim_test rechnet immer mit dem Code der Repo-Wurzel, nicht mit dem Worktree der … |
