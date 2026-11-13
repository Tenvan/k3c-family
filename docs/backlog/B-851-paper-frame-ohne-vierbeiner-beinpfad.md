# B-851 · paper frame --ohne oberbein-<b> blendet bei Vierbeiner-Animationen den Beinpfad aus

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** PT4
- **Projekt:** PAP
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13

## Ausgangslage

Review PT3.2: Bei `vierbeinerBild` und den Reittier-Ebenen fragt `einschub` nur `nur(id)` ab, das dort immer true ist; `paper frame --ohne oberbein-<b>` lässt den Beinpfad eines Vierbeiners deshalb sichtbar.

## Ziel

`--ohne oberbein-<b>` bzw. `unterbein-<b>` blendet bei Vierbeiner-Animationen den Beinpfad dieses Beins aus, wie bei `reittier:`-Teilen (B-845).

## Beteiligte und Zielgruppen

Agenten, die über `assets_preview` Teile ausblenden; 🧑 im Bereich Assets.

## Anforderungen

- Beinpfad entfällt, wenn Ober- oder Unterbein in `--ohne` steht.
- Andere Teile unverändert.

## Nicht-Ziele

Workbench-Teil.

## Regeln und Einschränkungen

Klasse A (Problem, PAP). Vertrag `docs/standards/paper-spec.md` › 8 nachziehen.

- **Bereich:** `tools/paper/cli/frame*.ts`, `tools/paper/rig/**`, `tools/paper/bin/paper.test.ts`, `docs/standards/paper-spec.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`paper frame wolf --animation lauf --ohne oberbein-vl` → Bein vorne links fehlt samt Beinpfad.

## Ausnahme- und Fehlerfälle

Unbekanntes Teil → Exit 1 `teil-unbekannt` wie bisher.

## Akzeptanzkriterien

- **AC-01** Test in `tools/paper`: `--ohne oberbein-<b>` bei einem Vierbeiner entfernt den Beinpfad dieses Beins, andere Beine bleiben.

## Offene Fragen

keine

## Notizen

Befund Review PT3.2 (2026-11-13).
