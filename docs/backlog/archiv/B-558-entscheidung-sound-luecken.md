# B-558 · Entscheidung S5: Die Sound-Lücken bekommen vorhandene Kenney-Töne, UI-Klicks und Reiten bleiben offen

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** KR4
- **Projekt:** SND
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-492 führt acht Ereignisse ohne Sound; die Wahl (SO6.3, Workshop) gehört 🧑. Claude hat autonom aus den schon vorhandenen Kenney-Kandidaten (CC0) gewählt und eingebaut (SO6.4), ohne neue Fremdquelle.

## Ziel

🧑 bestätigt oder ändert die Zuordnung und die Auslöser.

## Beteiligte und Zielgruppen

🧑 hört im Asset-Studio und am TV (SO6.6).

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Nur CC0 oder CC-BY; Client rechnet nichts; zwei Spieler.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen je Zeile bestätigt oder geändert.

## Offene Fragen

Siehe Notizen.

## Notizen

**Entscheidung 🧑 (KR4.1, 2026-10-23):**

- **Zuordnung der fünf eingebauten Töne (Münze geben, Bau-Fortschritt, Wiederbeleben, Boss-Auftritt, Hub-Ausbau) samt Auslösern:** A bestätigt. Hören am TV in SO6.6; Änderungen dann im Asset-Studio (Platz 1, danach `task gen:soundatlas`).
- **Reiten:** gewünscht → B-574.
- **UI-Klicks:** gewünscht → B-575.
- **Skill je Klasse:** gewünscht → B-576.
