# B-885 · Nativer Rig-Export der Bosse ist nach MG10 veraltet

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NA1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

MG10 (B-720) hat die Boss-Specs unter `public/grafik/bosse/` neu gebaut (`version` 3 bei `goblin-anfuehrer`, `lava-golem`, `ratten-koenigin`, `splitter-titan`, `troll-koenig`; neue Teile wie `keule`, `hackmesser`, `zepter`, `klinge`, `kristalle`, eigene Pivots). Der eingecheckte native Rig-Export `native/k3c-rig/assets/bosse/*/rig.json` steht noch auf `version` 1 (geprüft 2026-11-15), ebenso `native/k3c-rig/assets/soll/bosse/**`. `kristallherz-waechter` (Spec `version` 1) hat keinen Export. `task native:rig:export:check` (`bun native/k3c-rig/export/export.ts --pruefen`, vergleicht byte-genau) läuft nicht in der CI.

## Ziel

Der native Rig-Export der Bosse passt wieder zu den Specs.

## Beteiligte und Zielgruppen

Agent PAP (Agent `k3c-grafik` prüft); Rig-Prototyp `native/k3c-rig`.

## Anforderungen

- `task native:rig:export` einmal laufen lassen; der Export schreibt Teil-PNGs, Rig-Dateien und Sollwerte der Bosse neu.
- Der Diff bleibt auf die Bosse begrenzt; ändern sich andere Gruppen, im Ergebnis nennen und nur übernehmen, wenn `rig:export:check` es verlangt.
- `task native:rig:test` (Rig-Rechnung gegen die Sollwerte) ist danach grün.

## Nicht-Ziele

Änderungen an den Specs selbst (B-720); `kristallherz-waechter` neu bauen; `rig:export:check` in die CI aufnehmen.

## Regeln und Einschränkungen

Gen-Lauf am Feature (B-742): Dieses Ticket ist das Feature. Der Export liest Specs und Stil nur.

- **Bereich:** `native/k3c-rig/assets/**`
- **Abhängig von:** B-720
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`task native:rig:export:check` vor dem Lauf → Abweichung bei den Bossen; nach `task native:rig:export` → grün, `rig.json` der fünf Bosse nennt `version` 3.

## Ausnahme- und Fehlerfälle

Der Export scheitert an einer Boss-Spec (Teil ohne Pivot) → nicht an der Spec drehen, Ticket anlegen, Session blockiert. `rig:test` rot wegen neuer Teile → Ticket für `native/k3c-rig/rig`, hier nur die Assets.

## Akzeptanzkriterien

- **AC-01** `task native:rig:export:check` grün mit den Boss-Specs `version` 3.
- **AC-02** `task native:rig:test` grün; `git diff --stat` zeigt nur Dateien unter `native/k3c-rig/assets/`.

## Offene Fragen

- Bekommt `kristallherz-waechter` einen Export? → Default: nur wenn `rig:export` ihn von selbst schreibt; sonst unverändert (vorläufig, 🧑 bestätigt).

## Notizen

Gefunden im Review MG10.4 (2026-11-14). 2026-11-15 von Sprint MG10 (erledigt) nach NA1 verschoben.
