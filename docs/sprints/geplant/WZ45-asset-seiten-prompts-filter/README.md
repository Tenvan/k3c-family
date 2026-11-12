# WZ45 · DEV · Asset-Seiten: Filter und Prompt-Buttons für Styleguide, Sound und Quellen

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-727, B-728, B-729
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Katalog und Review im Bereich Assets haben Filter und Prompt-Buttons; Styleguide, Sound-Seite und Quellen-Seite noch nicht (B-727, B-728, B-729, Ideen aus PAP). Eingereiht am 2026-11-12 bei der Bereinigung des Backlogs (Entscheidung 009), nicht Teil der von 🧑 freigegebenen Welle 5.

## Ziel

Styleguide, Sound- und Quellen-Seite lassen sich filtern und schicken je Eintrag einen kurzen Prompt an Claude.

Am Ende sichtbar: Klick auf ein Material, einen Sound-Slot oder eine Quelle legt einen passenden Prompt in die Zwischenablage bzw. den Chat.

## Beteiligte und Zielgruppen

🧑 Ralf (nutzt die Seiten, gibt die Spec frei); Claude (Umsetzung).

## Anforderungen

`B-727 › Ziel`, `B-728 › Ziel`, `B-729 › Ziel`; Prompt-Füllung über `fillPrompt` (nach B-703).

## Nicht-Ziele

Neue Asset-Arten, Änderungen an Assets ohne MCP (Entscheidung 005).

## Regeln und Einschränkungen

Domäne DEV; Oberfläche nur über `@webkit`; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Typ Idee: Freigabe durch 🧑.

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/**`
- **Abhängig von:** WZ43
- **Braucht 🧑:** Freigabe
- **Sperre:** –

## Beispiele

Styleguide › Material `koenigsrot` › „Alle Nutzer zeigen“ → Prompt mit Material-ID.

## Ausnahme- und Fehlerfälle

Leere Filtertreffer → Hinweis statt leerer Liste.

## Akzeptanzkriterien

- **AC-01** Styleguide mit Prompt-Buttons für Materialien, Töne und Papier (B-727).
- **AC-02** Sound-Seite mit Filter (Text, Kategorie, Status) und Prompt-Buttons je Slot (B-728).
- **AC-03** Quellen-Seite mit Filter (Lizenz, Pack, Art) und Prompt-Buttons (B-729).

## Offene Fragen

Welche Prompts je Seite genau, und sollen die Kriterien der drei Tickets als `AC-01` … ausformuliert werden? Entscheidet 🧑 bei der Freigabe.

## Sessions

Entwurf (Stichpunkte, Session-Dateien vor dem Aktivieren):

- WZ45.1 Styleguide-Prompts (AC-01)
- WZ45.2 Sound- und Quellen-Seite: Filter und Prompts (AC-02, AC-03)
- WZ45.3 Review (alle)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
