# B-094 · Im Repo liegen keine Alt-Binaries und keine npm-Skripte mehr

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RP1
- **Projekt:** REL
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit SP03 (Commit `917d818`) liegen `k3c-server` und `k3c-server.exe` (zusammen rund 23 MB) eingecheckt im Repo-Wurzelordner.
Sie sind veraltet (enthalten noch die alte Meldung mit `npm run build`); `task start` baut den Server nach `bin/`.
`grep -rn "npm run" .` meldet deshalb zwei Binärdateien, der Rest von I1/AC-04.
Außerdem hat `tools/k3c-dev/frontend/package.json` noch `scripts` (`dev`, `build`), die nach I1.3 niemand mehr aufruft
(`wails.json` und `task dev:frontend` rufen `npx` direkt).

## Ziel

Genau ein Weg für Befehle (`task`), ohne Reste: kein veraltetes Build-Artefakt im Repo, keine ungenutzten npm-Skripte.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten.

## Anforderungen

- `k3c-server` und `k3c-server.exe` sind nicht mehr versioniert; `.gitignore` verhindert, dass sie an der Wurzel wieder eingecheckt werden.
- `tools/k3c-dev/frontend/package.json` enthält keine `scripts`, solange nichts sie aufruft.

## Nicht-Ziele

Git-Historie umschreiben (die Dateien bleiben in alten Commits).

## Regeln und Einschränkungen

Domäne INF (Repo-Aufbau, `.gitignore`); in `tools/k3c-dev/` nur `frontend/package.json` (Grenzfall wie in I1).

## Beispiele

`grep -rn "npm run" .` mit den Ausnahmen aus I1/AC-04 → kein Treffer, auch keine Binärdatei.

## Ausnahme- und Fehlerfälle

Wer die Server-EXE lokal braucht, baut sie mit `task start` nach `bin/` (bleibt ignoriert).

## Akzeptanzkriterien

- **AC-01** `git ls-files k3c-server k3c-server.exe` ist leer; `grep -rn "npm run" .` (Ausnahmen wie I1/AC-04) liefert keinen Treffer.
- **AC-02** `tools/k3c-dev/frontend/package.json` hat keine `scripts`; `task check:dev` ist grün.

## Offene Fragen

keine

## Notizen

Gefunden im Review I1.4 (2026-10-02).