# LA2.5 · Review und Abnahme-Ticket LA2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la2
- **Abhängig von:** LA2.1, LA2.2, LA2.3, LA2.4
- **Tickets:** B-1179, B-1181, B-1183
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Sprint LA2 ist geprüft; ein Abnahme-Ticket sammelt die Gen- und Bildläufe.

## Kontext

Review der Sessions LA2.1 bis LA2.4 gegen AC-01 bis AC-06 der Sprint-README. Gen-Läufe (`gen:paper`, Golden) und `check:race` nur im Abnahme-Ticket (B-742).

## Erlaubte Dateien

- `docs/sprints/aktiv/LA2-*/**`

## Nicht-Ziele

Keine neuen Funktionen.

## Schritte

1. Diff von LA2.1 bis LA2.4 lesen, Fachagenten k3c-hud und k3c-grafik laufen lassen.
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen (PC-Prüfung von 🧑).

## Fertig, wenn

- [ ] AC-06: Review ohne offene schwere Befunde, Abnahme-Ticket angelegt.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-06 geprüft: Review LA2.1 bis LA2.4 durch k3c-hud (LA2.1) und k3c-grafik (LA2.2 bis LA2.4): keine schweren Befunde, 6 mittlere und 5 geringe (siehe B-1185 AC-03). `task check` rot nur durch die drei bekannten `planning.test.ts`-Tests (3 von 2047), `task check:go` grün. Abnahme-Ticket B-1185 angelegt (auf Ralfs ausdrückliche Antwort, 2026-11-21).

Abweichung: Gen-Läufe (`gen:paper` Golden) lief nur indirekt in LA2.3 (`task test paper` grün), `check:race` nicht (B-1185).

### Für den Hauptrechner
- Befunde a bis h in B-1185 AC-03 entscheiden; e2e_flow und alloc_je_bild rot (B-1185 AC-04).
