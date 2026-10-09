# SV1 · SRV · Raum mit allen Stufen, Voll-Ausbau-Spielstand, leere Test-Räume

- **Status:** geplant
- **Projekt:** WRT
- **Domäne:** SRV
- **Reife:** bereit
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

Neue Biome, Inselwechsel und Inseldaten (K2, B-103), Darstellung der Ausbaustufen im Renderer (B-208), Balancing.

## Regeln und Einschränkungen

SRV: `engine/room/`, `engine/net/`, `engine/store/`; Client-Anteil (Eisenstollen, Kristallhöhle zeichnen) als eigene Session nach Grenzfall Protokoll.

- Beschluss 🧑 2026-10-06 (Chat): Spielstand und Dev-API (SRV) sowie der Knopf im Level-Betrachter (PLAT, `leveltest.html`, `src/tools/`) kommen beide in SV1. Der Knopf ist eine eigene Session (SV1.4) als ausdrückliche **Domänen-Ausnahme** dieses Sprints. „Alle Level“ heißt alle Inselstufen eines Raums, nicht alle Biome.
- Beschluss 🧑 2026-10-06 (Chat): B-290 kommt in SV1, also vor K2/B-103, ohne Inseldaten (Stufen aus allen Biomen). Der Client-Anteil (Eisenstollen, Kristallhöhle zeichnen) ist eine eigene Session (SV1.2, Grenzfall Protokoll/Client).
- B-204: kein Beschluss. Vermutet, prüft SV1.1: Die Testseite (B-086) braucht den leeren Test-Raum nicht länger als bis zum nächsten Sweep.
- Fünf Sessions statt höchstens vier, weil beide Beschlüsse eigene Sessions für die Fremd-Domänen verlangen.

## Beispiele

Raum anlegen → fünf Stufen im Snapshot; Level-Betrachter „voll ausbauen“ → Spiel startet mit Hub-Stufe 5.

## Ausnahme- und Fehlerfälle

Biom ohne Daten → Stufe fehlt mit Log-Warnung 🤒, Raum läuft weiter.

## Akzeptanzkriterien

- **AC-01** Der Level-Betrachter erzeugt einen Spielstand mit allen Gebäuden voll ausgebaut (B-315/AC-01, B-315/AC-02, B-315/AC-03).
- **AC-02** Ein neuer Raum legt die Insel mit allen Stufen an, für die es ein Biom gibt (B-199/AC-01, B-199/AC-02, B-199/AC-03).
- **AC-03** Der Raum erzeugt alle fünf Stufen und der Client kennt Eisenstollen und Kristallhöhle (B-290/AC-01, B-290/AC-02).
- **AC-04** Leere Test-Räume schließen sofort statt nach der Leer-Frist (B-204/AC-01).
- **AC-05** Der Spielstand „voll ausgebaut“ ist nur im Dev-Mode erzeugbar; ohne Dev-Mode antwortet die API mit 403.
- **AC-06** `task check` und `task check:go` grün.

## Offene Fragen

- B-204 (nicht blockierend): Braucht die Testseite (B-086) den leeren Test-Raum länger als bis zum nächsten Sweep? Vermutet nein; SV1.1 prüft es (Schritt 6) und hält das Ergebnis fest. Bestätigt die Prüfung die Annahme nicht, bleibt AC-04 offen und 🧑 entscheidet.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SV1.1 | `SV1.1-alle-stufen-test-raeume.md` | Umsetzung | autonom | offen |
| SV1.2 | `SV1.2-client-biome.md` | Umsetzung | autonom | offen |
| SV1.3 | `SV1.3-spielstand-voll-ausgebaut.md` | Umsetzung | autonom | offen |
| SV1.4 | `SV1.4-knopf-level-betrachter.md` | Umsetzung | autonom | offen |
| SV1.5 | `SV1.5-review.md` | Review | autonom | offen |

## Abnahme

–
