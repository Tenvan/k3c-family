# B-1238 · Knöpfe „In Claude öffnen“ entfallen, Prompt-Knöpfe kopieren nur

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WN1
- **Projekt:** WZN
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Auftrag im Chat
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Workbench und Studio hatten neben dem Kopieren einen Knopf „In Claude öffnen“ (Deep Link). Ralf will Claude damit nicht mehr aufrufen.

## Ziel

Prompt-Knöpfe kopieren nur in die Zwischenablage; im Spieltest zwei Textknöpfe „Prompt: Auswerten“ und „Prompt: Tickets anlegen“.

## Beteiligte und Zielgruppen

Ralf (🧑).

## Anforderungen

- OpenInClaude samt Oberfläche, API, Go-Binding und Tests in Workbench und Studio entfernen.
- Spieltest: zwei Textknöpfe statt Symbolen.

## Nicht-Ziele

Paket tools/workkit/claude löschen (ungenutzt, eigenes Ticket bei Bedarf).

## Regeln und Einschränkungen

Atomic Design.

## Beispiele

Befunde-Kopf zeigt „Prompt: Auswerten“.

## Ausnahme- und Fehlerfälle

Zwischenablage nicht verfügbar: Fehlertext wie bisher.

## Akzeptanzkriterien

- **AC-01** Typprüfung, Vitest und go test beider Apps grün.
- **AC-02** Abnahme durch Ralf: kein Claude-Knopf mehr sichtbar.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
