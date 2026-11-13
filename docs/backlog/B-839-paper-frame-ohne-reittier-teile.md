# B-839 · paper frame --ohne kennt die Teile des Reittiers nicht

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Review PT2.3: `paper frame` liefert bei Reit-Animationen Boxen `reittier:<teil>` (B-709), aber `--ohne reittier:hals` lehnt mit `teil-unbekannt` ab, weil `pruefeOhne` (`tools/paper/cli/frame.ts`) nur die Teile des Reiters kennt. Teile des Reittiers lassen sich in Vorschau und Katalog (B-734) nicht ausblenden. Daneben klein: Die Bein-Box in `tools/paper/rig/reitBoxen.ts` lässt den Wulst der Vorderbeine (+1,6 Einheiten an Stützpunkt 2, `beinpfad.ts`) weg und ist dort bis etwa 2 px zu klein.

## Ziel

`--ohne` nimmt bei Reit-Animationen auch `reittier:<teil>`, und die Bein-Boxen umfassen den Wulst.

## Beteiligte und Zielgruppen

Agenten, die über `assets_preview` Teile ausblenden; 🧑 im Bereich Assets.

## Anforderungen

- `frame --ohne reittier:<teil>` rendert das Teil des Reittiers ohne Lagen (Beinteile: ohne Beinpfad).
- Bein-Box mit Wulst.

## Nicht-Ziele

Workbench-Teil (WZ43).

## Regeln und Einschränkungen

Domäne PAP, Klasse A (Problem). Vertrag `docs/standards/paper-spec.md` › 8 nachziehen.

- **Bereich:** `tools/paper/cli/frame*.ts`, `tools/paper/rig/**`, `tools/paper/bin/paper.test.ts`, `docs/standards/paper-spec.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`paper frame monarch --animation galopp --frame 2 --ohne reittier:hals --out f.png` → Exit 0, `teile` ohne `reittier:hals`.

## Ausnahme- und Fehlerfälle

Unbekanntes `reittier:`-Teil → Exit 1 `teil-unbekannt` mit Liste der Reittier-Teile.

## Akzeptanzkriterien

- **AC-01** `bin/paper.test.ts`: `--ohne reittier:hals` bei `galopp` → Exit 0, Box fehlt, andere Boxen bleiben.
- **AC-02** Bein-Box eines Vorderbeins umfasst den Wulst (Test gegen die Punkte des Beinpfads).

## Offene Fragen

keine

## Notizen

Quelle: Review PT2.3 (k3c-reviewer).
