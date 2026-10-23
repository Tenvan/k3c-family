# B-556 · Entscheidung S3: Die Credits der Sounds bleiben in public/audio/README.md

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KR4
- **Projekt:** SND
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

SO2.2 nannte `public/audio/CREDITS.md`, k3c-workbench schreibt die Sound-Credits aber nach `public/audio/README.md` › Importe (`tools/assetkit/catalog.go`). B-493 › Offene Fragen.

## Ziel

🧑 bestätigt die Credit-Datei der Sounds.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Eine Credit-Datei je Wurzel (B-165).

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A oder B gewählt.

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Wo stehen die Credits der Sounds?

- **A (Default, gilt, umgesetzt in SO6.1):** `public/audio/README.md`; die Credits-Seite liest dort die Tabellen mit Kopf `| Ordner |`.
- **B:** eigene `public/audio/CREDITS.md`; ändert `tools/assetkit/catalog.go` (Workbench und Studio), `src/tools/credits.ts` und `soundKatalog.test.ts`.
- **Wenn 🧑 B wählt:** Ticket DEV (assetkit) plus CLI, eine Session.
