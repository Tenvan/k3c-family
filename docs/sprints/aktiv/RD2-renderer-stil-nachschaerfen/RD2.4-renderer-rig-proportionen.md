# RD2.4 · Renderer: Rig-Proportionen je Figur und Tier

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** renderer
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** rd2/4-renderer-rig-proportionen
- **Abhängig von:** RD2.3
- **Tickets:** B-715
- **Kriterien:** AC-04

## Ziel

Die Rigs `vierbeiner` und `figur` tragen Proportionen, Fußformen und Gangarten je Tier bzw. Figur, ohne dass Assets umgestellt werden.

## Kontext

Ticket B-715. Rigs: `tools/paper/rig/vierbeiner.ts`, `vierbeinerBild.ts`, `gangarten.ts`, `beinpfad.ts`, `ik.ts`; `tools/paper/rig/figur.ts`, `figurBild.ts`; Validierung `rig/pruefen.ts`, Schema `paper-spec.schema.json`, Typen `types.ts`, Doku `docs/standards/paper-spec.md`. Heute teilen Wölfe, Höllenhund und Elefant die Proportionen des Pferds (D-MG2-08); der Rückenpivot liegt fest bei (0, −66), Neigung um (0, −80). Neu für `vierbeiner`: Beinlänge, Rumpf und Fußform (`huf`, `pfote`, `elefantenfuss`); Gangarten je Tier wählbar, Elefant ohne `galopp` gültig, Pferd ohne `galopp` ungültig. Neu für `figur`: Proportionen (Rumpf, Arme, Beine, Kopf) je Figur. Defaults müssen die bestehenden Assets pixelgleich lassen (Golden unverändert, sonst Fehler). Modell opus, weil Geometrie und Vertragsänderung.

## Erlaubte Dateien

- `tools/paper/rig/**`, `tools/paper/schema/**`, `tools/paper/render/**`
- `docs/standards/paper-spec.md`
- `tools/paper/fixtures/**` und `tools/paper/golden/**` (nur neue Fixtures für Pfote, Huf, Elefantenfuß, lange Arme)

## Nicht-Ziele

Assets umstellen (B-716, B-720 in MG9 und MG10); Änderungen der bestehenden Golden-Hashes.

## Schritte

1. Tests zuerst: Spec-Felder und Fußformen, Gangart-Pflicht je Tier, Figur mit langen Armen und kurzem Rumpf ohne Gelenkfehler (Golden-Fixture).
2. Felder in Schema, Typ, Doku.
3. `vierbeiner` und `figur` auswerten; Defaults = heutiges Verhalten.
4. `bun test` in `tools/paper`, Golden der bestehenden Gruppen unverändert.

## Fertig, wenn

- [ ] AC-04: Spec-Vertrag und Schema kennen die Felder; Tests für Pfote, Huf, Elefantenfuß (B-715/AC-01).
- [ ] AC-04: Elefant ohne `galopp` gültig, Pferd ohne nicht (B-715/AC-02).
- [ ] AC-04: Figur mit langen Armen und kurzem Rumpf rendert ohne Gelenkfehler, Golden-Fixture (B-715/AC-03).

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
