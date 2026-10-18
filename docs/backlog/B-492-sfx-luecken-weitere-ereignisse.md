# B-492 · Weitere Ereignisse bekommen eigene Sound-Slots und Sounds

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** SND
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

SO2 deckt elf Ereignis-Slots ab (`sfx:hit`, `kill`, `coin`, `arrow`, `strike`, `build`, `death`, `revive`, `skill`, `warn`, `portal`). Im Katalog `docs/assets/sounds.md` stehen weitere Ereignisse aus B-167 als Lücke: Münze geben, Bau-Fortschritt, `revived` (Wiederbeleben durch einen Mitspieler, Q62), Hub-Ausbau, Skill je Klasse, Boss-Auftritt und UI-Klicks. Ihre Auslöser und die Wahl durch 🧑 waren in SO2.1 nicht Teil der Auswahl.

## Ziel

Jedes im Katalog als Lücke geführte Ereignis hat einen Sound-Slot in `data/assets.json`, einen gewählten Sound mit Quelle und Lizenz und ist eingebaut, oder 🧑 hat die Lücke bewusst bestätigt.

## Beteiligte und Zielgruppen

🧑 wählt im Asset-Studio (Sound-Seite, Reiter Slots); der Agent legt Slots und Kandidaten an und baut ein; Spieler am TV und am PC.

## Anforderungen

- Je Ereignis der Lücken-Liste ein Slot mit Kandidaten (CC0 oder CC-BY, Credits in `public/audio/README.md`), oder die Lücke bleibt mit Begründung.
- Auslöser klären: Hub-Ausbau ist nur als Wechsel von `hubLevel` im Snapshot sichtbar, UI-Klicks haben kein Feedback-Event (Q08); Boss-Auftritt hat `bossSpawned`, `bossPhase`, `bossDefeated` (`docs/protocol.md`).
- Katalog `docs/assets/sounds.md` und das Mapping im Client (SO2.3) nachziehen; keine Spiel-Logik im Audio-Code.

## Nicht-Ziele

Audio-Kern (SO1), Musik (B-168), die elf Slots aus SO2.

## Regeln und Einschränkungen

`src/scenes` rechnet nichts; nur CC0 oder CC-BY (Q15); B-Taste nicht belegen; 2 Spieler gleichzeitig; Stil Retro/Chiptune, kindgerecht, Nacht leise (Q15). Voraussetzung: SO2.

## Beispiele

Boss erscheint → Boss-Auftritt-Sound bei beiden Spielern; Mitspieler belebt wieder → `revived`-Sound statt des Respawn-Tons.

## Ausnahme- und Fehlerfälle

Sound-Datei fehlt oder lädt nicht → stumm, höchstens ein Logeintrag je Schlüssel (SO2.3/AC-05).

## Akzeptanzkriterien

- **AC-01** Jede Zeile des Katalogs `docs/assets/sounds.md` mit Status Lücke hat einen Slot und gewählten Sound oder eine von 🧑 bestätigte Begründung (Test auf den Katalog).
- **AC-02** Jede neue Sound-Datei hat einen Credit-Eintrag in `public/audio/README.md` (Test wie B-167/AC-02).
- **AC-03** Beobachtung am TV: Boss-Auftritt, Wiederbeleben durch Mitspieler und Hub-Ausbau lösen je ihren Sound aus.

## Offene Fragen

- Woher kommen die Auslöser für Hub-Ausbau und UI-Klicks (kein Feedback-Event in Q08)? Entscheidet 🧑.
- Gehören „Skill je Klasse“ und „Münze geben“ wirklich zum Umfang, oder reicht je ein gemeinsamer Ton (`sfx:skill`, `sfx:coin`)? Entscheidet 🧑.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
