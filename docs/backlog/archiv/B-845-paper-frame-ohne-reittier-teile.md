# B-845 · paper frame --ohne kennt die Teile des Reittiers nicht

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** PT3
- **Projekt:** PAP
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13

## Ausgangslage

Review PT2.3: `paper frame` liefert bei Reit-Animationen Boxen `reittier:<teil>` (B-709), aber `--ohne reittier:hals` lehnt mit `teil-unbekannt` ab, weil `pruefeOhne` (`tools/paper/cli/frame.ts`) nur die Teile des Reiters kennt. Teile des Reittiers lassen sich in Vorschau und Katalog (B-734) nicht ausblenden. Daneben klein: Die Bein-Box in `tools/paper/rig/reitBoxen.ts` lässt den Wulst der Vorderbeine (+1,6 Einheiten an Stützpunkt 2, `beinpfad.ts`) weg und ist dort bis etwa 2 px zu klein.

## Ziel

`--ohne` nimmt bei Reit-Animationen auch `reittier:<teil>`, und die Bein-Boxen umfassen den Wulst.

## Beteiligte und Zielgruppen

Agenten, die über `assets_preview` Teile ausblenden; 🧑 im Bereich Assets.

## Anforderungen

- `frame --ohne reittier:<teil>` rendert bei Reit-Animationen (`reittier` an der Animation) das Teil des Reittiers ohne Lagen; seine Box fehlt, alle anderen Boxen bleiben. Präfixlose ids gelten weiter für den Reiter.
- Ober- oder Unterbein (`reittier:oberbein-<b>`, `reittier:unterbein-<b>`): der Beinpfad dieses Beins wird nicht gezeichnet (er ist ein Papierstück), beide Bein-Boxen des Beins fehlen.
- Bein-Box der Vorderbeine umfasst den Wulst (+1,6 Einheiten an Stützpunkt 2, `rig/beinpfad.ts`); der Wert steht einmal in `beinpfad.ts`.
- Vertrag `docs/standards/paper-spec.md` › 8 nennt `--ohne reittier:<teil>`.

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

- Unbekanntes `reittier:`-Teil → Exit 1 `teil-unbekannt` mit Liste der Reittier-Teile (mit Präfix).
- `reittier:<teil>` bei einer Animation ohne Reittier → Exit 1 `teil-unbekannt` (Liste der Teile der Spec).
- Fehlt das Reittier selbst → wie bisher `reittier-unbekannt` mit `feld` `animationen.<name>.reittier`.

## Akzeptanzkriterien

- **AC-01** `bin/paper.test.ts`: `--ohne reittier:hals` bei `galopp` → Exit 0, Box fehlt, andere Boxen bleiben.
- **AC-02** Bein-Box eines Vorderbeins umfasst den Wulst (Test gegen die Punkte des Beinpfads).

## Offene Fragen

keine

## Notizen

Quelle: Review PT2.3 (k3c-reviewer). Umnummeriert 2026-11-13 von B-839 (Kollision mit WZ44 in Welle 5b).
