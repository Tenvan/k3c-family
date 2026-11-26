# B-1237 · Spieltest: Spielkontext, Bild und Vergleichsticket eines Befunds sind groß und gut lesbar

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
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

Auf der Seite Spieltest standen Spielkontext und Hinweis auf das Vergleichsticket eines Befunds klein und grau, das Bild nur 240 px breit.

## Ziel

Die wichtigen Angaben eines Befunds lassen sich ohne Zusammenkneifen lesen.

## Beteiligte und Zielgruppen

Ralf (🧑).

## Anforderungen

- Spielkontext in normaler Textgröße und Farbe.
- Bei Duplikaten eine auffällige Zeile mit dem Vergleichsticket.
- Bild bis 560 px breit.

## Nicht-Ziele

Titel des Vergleichstickets laden; Ticket anlegen.

## Regeln und Einschränkungen

Atomic Design.

- **Bereich:** `tools/k3c-workbench/frontend/src/playtest/FindingsPanel.tsx`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Befund mit duplicate_of B-1204: Zeile „Vergleichsticket B-1204 …“ in Fettschrift.

## Ausnahme- und Fehlerfälle

Befund mit Ticket: keine Vergleichszeile.

## Akzeptanzkriterien

- **AC-01** Typprüfung und Vitest `src/playtest` grün.
- **AC-02** Abnahme durch Ralf: Befunde sind gut lesbar.

## Offene Fragen

keine

## Notizen

–
