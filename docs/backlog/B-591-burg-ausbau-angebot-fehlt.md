# B-591 · Vor der Burg erscheint kein Ausbau-Angebot, wenn der Spieler davor steht

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Abnahme GEB11.6 (Ralf, 2026-10-24, PC): Steht der Spieler vor der Burg, erscheint kein Ausbau-Angebot (Preisschild/Ausbauen).

## Ziel

Vor der Burg erscheint das Ausbau-Angebot mit Kosten, sobald der Ausbau möglich ist.

## Beteiligte und Zielgruppen

Spielende; Agent CLI (ggf. SRV); 🧑 nimmt ab.

## Anforderungen

- Ursache klären: Client zeichnet das Angebot nicht (`siteView.ts`, `priceTagRules.ts`, Burg ist `hubSite`) oder der Server meldet es nicht (`hubUpgrade`).
- Das Angebot mit Gold und Material steht vor der Burg, solange ein Ausbau frei ist.

## Nicht-Ziele

Neue Ausbau-Stufen (B-592).

## Regeln und Einschränkungen

Client zeigt nur, was der Server meldet; deterministisch; 2 Spieler.

## Beispiele

Spieler steht vor der Burg → „Ausbauen“ mit Kosten.

## Ausnahme- und Fehlerfälle

Ausbau läuft schon → Wartegrund statt Angebot.

## Akzeptanzkriterien

- **AC-01** Vor der Burg erscheint das Ausbau-Angebot (Test der Anzeige-Regel und Beobachtung am Gerät).

## Offene Fragen

Ob Fehler im Client oder Server: in der Umsetzung zuerst nachstellen (`room_snapshot`).

## Notizen

Aus GEB11.6. Spec: Entwurf.
