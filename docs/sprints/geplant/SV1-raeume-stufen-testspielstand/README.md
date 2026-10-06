# SV1 · SRV · Raum mit allen Stufen, Voll-Ausbau-Spielstand, leere Test-Räume

- **Status:** geplant
- **Domäne:** SRV
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-315, B-199, B-290, B-204
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ein neuer Raum legt nicht alle Stufen an, für die es ein Biom gibt (B-199, B-290); Abnahmen brauchen einen voll ausgebauten Spielstand (B-315); leere Test-Räume bleiben bis zur Leer-Frist offen (B-204).

## Ziel

Jeder Raum spielt die ganze Insel, und ein Klick im Level-Betrachter startet einen Spielstand mit allen Gebäuden voll ausgebaut. Am Ende sichtbar: Neuer Raum mit allen fünf Stufen, Level-Betrachter startet einen voll ausgebauten Spielstand.

## Beteiligte und Zielgruppen

🧑 prüft Gebäude am Gerät; Agenten nutzen den Spielstand für Abnahmen und Messläufe.

## Anforderungen

B-315 › Anforderungen; B-199 › Anforderungen; B-290 › Anforderungen; B-204 › Anforderungen.

## Nicht-Ziele

Neue Biome, Inselwechsel (K2).

## Regeln und Einschränkungen

SRV: `engine/room/`, `engine/net/`, `engine/store/`; Client-Anteil (Eisenstollen, Kristallhöhle zeichnen) als eigene Session nach Grenzfall Protokoll.

## Beispiele

Raum anlegen → fünf Stufen im Snapshot; Level-Betrachter „voll ausbauen“ → Spiel startet mit Hub-Stufe 5.

## Ausnahme- und Fehlerfälle

Biom ohne Daten → Stufe fehlt mit Log-Warnung 🤒, Raum läuft weiter.

## Akzeptanzkriterien

- **AC-01** Der Level-Betrachter erzeugt einen Spielstand mit allen Gebäuden voll ausgebaut (B-315/AC-01, B-315/AC-02, B-315/AC-03).
- **AC-02** Ein neuer Raum legt die Insel mit allen Stufen an, für die es ein Biom gibt (B-199/AC-01, B-199/AC-02, B-199/AC-03).
- **AC-03** Der Raum erzeugt alle fünf Stufen und der Client kennt Eisenstollen und Kristallhöhle (B-290/AC-01, B-290/AC-02).
- **AC-04** Leere Test-Räume schließen sofort statt nach der Leer-Frist (B-204/AC-01).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SV1.1 Raum mit allen Stufen, Client kennt Eisenstollen und Kristallhöhle (AC-02, AC-03).
- SV1.2 Spielstand „voll ausgebaut“ aus dem Level-Betrachter (AC-01).
- SV1.3 Leere Test-Räume schließen sofort (AC-04).
- SV1.4 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
