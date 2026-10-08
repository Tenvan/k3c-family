# B-193 · Figuren-Lücken unter public/sprites/ haben Kandidaten und eine Auswahl

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** GR7
- **Projekt:** –
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GR2 (B-162) sucht nur Umgebungs-Assets für `public/grafik/`. Figuren liegen unter `public/sprites/` (`data/sprites.json`, `public/sprites/CREDITS.md`); Lücken dort (z. B. Bosse aus K2, Monarch auf dem Reittier B-173) deckt kein Sprint ab. Herkunft: Spec-Review GR2 am 2026-10-03.

## Ziel

Jede Figuren-Lücke aus `docs/assets/zuordnung.md` hat Kandidaten, eine Auswahl durch 🧑 und liegt danach mit Credits unter `public/sprites/`.

## Beteiligte und Zielgruppen

Der Agent recherchiert; 🧑 wählt je Lücke (Q14).

## Anforderungen

- Verfahren wie B-162 (Q14: 3 Kandidaten je Lücke, Referenzseite unter `docs/funde/`), Ziel aber `public/sprites/` mit Eintrag in `data/sprites.json` und `public/sprites/CREDITS.md`.

## Nicht-Ziele

Umgebungs-Assets (B-162), Einbau in den Renderer (B-010), selbst gezeichnete Figuren.

## Regeln und Einschränkungen

Lizenzregel Q70 (CC0, CC-BY, OGA-BY, CC-BY-SA; CC-BY-NC nur markiert), Credits sofort; Grundstil nach Q13. Auswahl und Recherche über den ResourcenManager (B-298), sobald er steht.

## Beispiele

Lücke „Boss“ (K2) → drei Kandidaten → 🧑 wählt → Ordner unter `public/sprites/` mit Credit und Eintrag in `data/sprites.json`.

## Ausnahme- und Fehlerfälle

Kein Treffer → Lücke bleibt mit Vermerk, Platzhalter bleibt.

## Akzeptanzkriterien

- **AC-01** Zu jeder Figuren-Lücke aus `docs/assets/zuordnung.md` liegt unter `docs/funde/` eine Kandidatenliste mit drei Einträgen vor (weniger nur mit Vermerk).
- **AC-02** 🧑 hat je Lücke gewählt oder „kein Treffer“ bestätigt.
- **AC-03** Gewählte Figuren liegen unter `public/sprites/` mit Credit-Eintrag; `task test` ist grün.

## Offene Fragen

- Welche Figuren-Lücken es gibt, steht erst nach GR1 fest. Sprint offen, entscheidet 🧑 beim Planen.

## Notizen

–
