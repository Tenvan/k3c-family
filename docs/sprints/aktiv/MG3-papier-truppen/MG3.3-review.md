# MG3.3 · Review des Sprints MG3

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** truppen
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg3/3-review
- **Abhängig von:** MG3.1, MG3.2
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Der Sprint MG3 ist geprüft: alle sechs Truppen gültig, gerendert, mit Golden; Abnahme der README ausgefüllt.

## Kontext

Sprint-README und Ergebnisse MG3.1/MG3.2; Stil-Regeln aus `assets_guide {rig: figur}`; Proportion und Lesbarkeit der Silhouetten im Preview (Truppen müssen sich auf einen Blick unterscheiden).

## Erlaubte Dateien

- `public/grafik/truppen/**` (nur Korrekturen aus dem Review, über `assets_set`)
- `docs/sprints/aktiv/MG3-papier-truppen/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine neuen Assets, keine Sichtprüfung durch 🧑 (PS1).

## Schritte

1. `assets_list {gruppe: truppen}`, `assets_review` gegen den Start-Commit.
2. Je Asset jede Animation mit `assets_preview` ansehen (frischer Blick), Befunde beheben oder als D-Zeile.
3. `task test -- paper`, `task check`.
4. Abnahme (höchstens fünf Zeilen), Commit `chore(pap): Review MG3`.

## Fertig, wenn

- [ ] AC-01 bis AC-03 mit Nachweis im Ergebnis.
- [ ] `task check` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `assets_list {gruppe: truppen}` → sechs gültig. AC-02: `task gen:paper` grün, `golden.json` und Manifeste stimmen mit allen 18 PNGs (Review-Agent: Hashes und `renderKey` einzeln verglichen), `bunx vitest run tools/paper` 187/187. AC-03: `assets_review` gegen Start-Commit 44f2d30c → sechs `neu`, kein roter Eintrag; Preview je Asset und Animation (MG3.1/MG3.2 und nach den Korrekturen).
- Review mit frischem Blick (Opus): Vertrag und Technik ohne Befund, kein Frame am Rand, Loops sauber. Drei wichtige Befunde behoben: Bogen-Angriff war ein Standbild → Zughand zieht mit Pfeil bis zur Wange und löst; Elite-Bogenschütze kaum unterscheidbar → rote Hutfeder, goldener Bogen, goldene Federn; Heugabel vor dem Gesicht → nach vorn gedreht (Ruhe 38°). Bogenschützen dafür Rahmen 160 (Randprüfung aller Angriffs-Frames leer).
- Kleinbefunde zurückgestellt als D-MG3-08 (Helm-Nasal, Schild-Sichtbarkeit, Waffenschwung in `lauf`, Stabknauf, Strohhut-Farbe), Pfeil-Sichtbarkeit als D-MG3-07.
- Prüfung: `task check` grün (154 Dateien, 3173 Tests) vor den Korrekturen; danach `tools/paper` 187/187 und Planungs-Test grün.
