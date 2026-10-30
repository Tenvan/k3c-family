# KB5.3 · Client zeichnet DoTs, Buffs und Kontrolleffekte (B-675)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** kb5/3-status-darstellung
- **Abhängig von:** KB5.2
- **Tickets:** B-675
- **Kriterien:** B-675 AC-03

## Ziel

Der Client zeichnet Betäubung, Verlangsamung, Spott, Sturm-Fläche, Schild, Auren und Regeneration aus dem Snapshot.

## Kontext

Spec: `docs/backlog/B-675-status-effekte-sichtbar.md`. Snapshot-Felder aus KB5.2. Zeichnen: `src/scenes/render/worldRenderer.ts`, Effekte `src/scenes/effects/` (Muster `skillLooks.ts` für Werte, damit das Studio sie importiert), Leistung `src/scenes/perfProbe.ts`.

## Erlaubte Dateien

- `src/scenes/**`

## Nicht-Ziele

- Snapshot-Änderungen (KB5.2), Studio (KB5.4), Ton.

## Schritte

1. Werte je Status-Effekt in `src/scenes/effects/statusLooks.ts`.
2. Darstellung mit Pool, Split-Screen tauglich.
3. Vitest je Effektart (zeichnen, entfernen).
4. `task check:diff`.

## Fertig, wenn

- [ ] B-675 AC-03: Vitest jede Effektart gezeichnet und entfernt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
