# SK3.2 · Eigener Effekt je Skill im Client

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sk3/2-skill-effekte
- **Abhängig von:** –
- **Tickets:** B-578
- **Kriterien:** AC-02

## Ziel

Jeder Skill hat einen eigenen Effekt, ein unsichtbarer Monarch wird abgedunkelt gezeichnet.

## Kontext

B-578; `src/scenes/effects`.

## Erlaubte Dateien

- `src/scenes/effects/`, `src/scenes/render/worldRenderer.ts`, `src/model/types.ts`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. `skillLooks.ts`. 2. Effekt-Rendering. 3. Tests.

## Fertig, wenn

- [x] AC-02.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 umgesetzt: 26 Looks in `SKILL_LOOKS` (Farbe aus DB32, Radius, Dauer, Partikel, Bewegung), Rückfall auf den Ring der Linie; Stealth mit Deckkraft 0,35. Geprüft mit Vitest und tsc; Abnahme am TV bleibt bei 🧑 (angenommen, Validierung offen).
