# B-558 · Entscheidung S5: Die Sound-Lücken bekommen vorhandene Kenney-Töne, UI-Klicks und Reiten bleiben offen

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
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

**Frage:** Passen Sound und Auslöser je Lücke?

| Ereignis | Slot | Default (Kenney, CC0) | Auslöser |
|---|---|---|---|
| Münze geben | `sfx:coinGive` | RPG Audio handleCoins2 | `coinGive` |
| Bau-Fortschritt | `sfx:buildProgress` | Impact Sounds impactMining_000 | `buildProgress` |
| Wiederbeleben durch Mitspieler | `sfx:revived` | Digital Audio highUp | `revived` |
| Boss-Auftritt | `sfx:boss` | Digital Audio lowThreeTone | `bossSpawned`, global |
| Hub-Ausbau | `sfx:hub` | Digital Audio threeTone2 | Anstieg von `hubLevel` im Snapshot, global |
| Skill je Klasse | – | gemeinsamer `sfx:skill` | – (Lücke bewusst) |
| UI-Klicks | – | keiner | kein Ereignis (Q08), Lücke |
| Reiten | `sfx:ride` | Slot da, kein Einbau | Schleife je Spieler braucht Kern-Erweiterung |

- **A (Default, gilt, umgesetzt):** wie Tabelle.
- **B:** andere Kandidaten im Asset-Studio an Platz 1 ziehen (wirkt nach `task gen:soundatlas`), oder neue Fremd-Sounds suchen (Kandidatenliste, Lizenzprüfung).
- **Reiten / UI-Klicks / Skill je Klasse:** je eigenes Ticket CLI, wenn 🧑 sie will (Reiten: Schleife je Spieler im Audio-Kern aus `players` im Snapshot; UI-Klicks: Ton in den Menü-Szenen; Skill je Klasse: Slot je Klasse).
