# B-759 · Docker-Build im Release scheitert an .work (EXDEV beim gen:paper)

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ41
- **Projekt:** WZG
- **Erstellt:** 2026-11-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Im Release-Lauf v0.22.0 scheitert der Docker-Schritt: `bun tools/paper/bin/gen.ts` im Dockerfile endet mit `EXDEV: cross-device link not permitted, rename '/src/.work/grafik/bosse/goblin-anfuehrer' -> '.../.goblin-anfuehrer.alt-1'`. Der Release-Job rendert vorher mit `task build` nach `.work/` und `public/grafik/`; das Build-Kontext-Verzeichnis enthält `.work/`, `COPY . .` bringt die Ordner in eine untere Image-Schicht, und Overlay-Dateisysteme lassen ein Umbenennen solcher Ordner nicht zu. Das GitHub-Release (Paket, Binaries) entstand, das Image nicht.

## Ziel

Das Docker-Image entsteht im Release (amd64 und arm64).

## Beteiligte und Zielgruppen

Entwickler (🧑), Raspberry Pi.

## Anforderungen

- `.dockerignore` schließt `.work` aus; `gen:paper` im Dockerfile arbeitet in einem frischen `.work/`.

## Nicht-Ziele

Kein Umbau des Rendererprozesses (kein Kopieren statt Umbenennen).

## Regeln und Einschränkungen

Release-Checkliste Pi-Image. Umgesetzt mit #529, Beleg in WZ41.1; ohne Release-Tag gilt ein Workflow-Lauf ohne Veröffentlichung oder ein lokaler `docker build` als Nachweis.

- **Bereich:** `Dockerfile`, `.dockerignore`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Tag v0.22.1 → Release-Lauf grün, `ghcr.io/tenvan/k3c-family:v0.22.1` vorhanden.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** Release-Lauf für das nächste Tag ist grün, das Image liegt in ghcr (amd64 + arm64).

## Offene Fragen

keine

## Notizen

v0.22.0 hat Release und Binaries, aber kein Image; deshalb Patch v0.22.1.
