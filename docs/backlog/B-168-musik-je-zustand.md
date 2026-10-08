# B-168 · Die Musik wechselt je Spielzustand mit Crossfade

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** SO4
- **Projekt:** SND
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint SO4

## Ausgangslage

Das Spiel hat keine Musik (B-011/AC-02: „Musik wechselt je Tageszeit“). SO1 liefert den Audio-Kern, SO2 die Soundeffekte, SO3 die Hörprobenseite (B-169), mit der 🧑 Stücke auswählt. Die Zustände des Spiels (Tageszeit, Kampf, Tiefe, Boss) liegen im Snapshot beziehungsweise in Events des Servers.

## Ziel

Die Musik passt zum Zustand (Tag, Abend, Nacht, Kampf, Höhle/Tiefe je Stufe, Boss, Niederlage/Sieg, Lobby) und wechselt weich per Crossfade; Warnungen senken die Musik kurz. Nutzen: Stimmung, besonders nachts.

## Beteiligte und Zielgruppen

🧑 wählt die Stücke aus den Hörproben (Q16 in `docs/fragenkatalog.md`); der Agent baut den Zustands-Automaten ein; Spieler am TV und am Handy.

## Anforderungen

- 8 Zustände (Q16): Tag, Abend (= Dämmerung, Q65), Nacht, Kampf, Höhle/Tiefe je Stufe, Boss, Niederlage/Sieg, Lobby; je Zustand 1–2 Stücke; Tabelle Zustand → Stück → Quelle → Lizenz in `docs/assets/sounds.md` (aus B-167) oder einer Schwesterdatei.
- Übergänge per Crossfade (Dauer als Wert in `data/` oder in der Konfiguration des Audio-Kerns); Ducking: Warn-Sounds senken die Musik kurz ab.
- Der Zustand kommt aus dem Snapshot beziehungsweise Events; der Client rechnet keinen Spielzustand.
- Quellen CC0 oder CC-BY, Credits sofort (CREDITS-Datei, Credits-Seite B-165); Musik-Dateien klein halten (Format laut B-166).
- Die Musik-Lautstärke ist über den Bus „Musik“ des Audio-Kerns einstellbar (B-011/AC-03).

## Nicht-Ziele

Eigene Kompositionen (B-011), Soundeffekte (B-167), Hörprobenseite (B-169), dynamische Musik über Layer.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts; B nicht belegen; 2 Spieler gleichzeitig (eine Musik je Gerät, nicht je Viertel); Voraussetzungen: SO1, B-166, Auswahl aus B-169.

## Beispiele

Die Nacht beginnt → die Musik blendet in 2 Sekunden von „Abend“ nach „Nacht“ über; ein Boss erscheint → Wechsel zu „Boss“.

## Ausnahme- und Fehlerfälle

Stück fehlt → die Musik bleibt beim vorherigen Zustand oder still, kein Absturz. Zustand wechselt schneller als der Crossfade dauert → der laufende Übergang wird ohne Knacken auf das neue Ziel umgelenkt.

## Akzeptanzkriterien

- **AC-01** Test: Der Zustands-Automat bildet jeden der 8 Zustände (Tag, Abend = Dämmerung, Nacht, Kampf, Tiefe, Boss, Niederlage/Sieg, Lobby) auf 1–2 Stücke ab (Q16).
- **AC-02** Beobachtung am TV: Beim Wechsel Tag → Dämmerung → Nacht → Morgengrauen (Q65) wechselt die Musik hörbar ohne Pause und ohne Knacken (Crossfade).
- **AC-03** Ein Warn-Sound senkt die Musik hörbar ab und stellt sie danach wieder her (Beobachtung oder Test auf dem Pegel).
- **AC-04** Jede Musik-Datei hat einen Credit-Eintrag, CC-BY-Stücke erscheinen auf der Credits-Seite (Test, B-165).
- **AC-05** Die Lautstärke des Busses „Musik“ ist getrennt von der der Effekte einstellbar.
- **AC-06** Beobachtung am TV: Die Lautheit der Musik ist geprüft und passt zu den Effekten (Q16).

## Offene Fragen

- Welches Stück läuft im Morgengrauen (Q65)? Q16 nennt dafür keinen eigenen Zustand; klärt B-213 bzw. 🧑 mit den Hörproben.

## Notizen

Quelle: `docs/plan-weiterentwicklung.md` Schiene A (Musik); deckt B-011/AC-02 inhaltlich ab.
