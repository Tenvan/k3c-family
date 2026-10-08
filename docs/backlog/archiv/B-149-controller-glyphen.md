# B-149 · Hinweise zeigen Controller-Glyphen statt Tasten-Text

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** S6
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, mit Sprint S6

## Ausgangslage

Hinweise nennen Tasten als Text (z. B. „A“, „Space“) in `src/scenes/HudScene.ts`; Symbole für Xbox-Tasten, Tastatur und Touch fehlen. Das Aktionen-Overlay (B-125) zeigt Tastensymbole nach dem zuletzt benutzten Gerät und braucht dafür dieselben Symbole.

## Ziel

Jeder Hinweis zeigt die Taste als Glyph des zuletzt benutzten Geräts (Xbox-Controller, Tastatur, Touch). Nutzen: Kinder erkennen die Taste am Bild statt am Namen.

## Beteiligte und Zielgruppen

Spieler am TV, PC und Handy; 🧑 nimmt am Gerät ab.

## Anforderungen

- Reine Funktion Aktion + Gerät → Glyph-Schlüssel (mit Text als Rückfall); das Gerät ergibt sich aus der zuletzt benutzten Eingabe (`src/input/`).
- Glyph-Quelle ohne neue Lizenzpflicht: selbst gezeichnet (Formen/Text) oder Pack mit CC0/CC-BY und Credit-Eintrag; Auswahl 🧑.
- Glyphen für A, X, Y, LB, RB, LT, RT, D-Pad, Menu (B und View nicht); Tastatur-Tasten als Kappen, Touch als Symbol.
- Funktioniert in jedem Split-Viertel, Mindestgröße nach B-136.

## Nicht-Ziele

Das Aktionen-Overlay selbst (B-125), neue Tastenbelegung (B-026, B-124), Grafik-Suche (B-162).

## Regeln und Einschränkungen

`CLAUDE.md` (B-Taste frei, View + Menu reserviert, Client rechnet nichts); Credits in der CREDITS-Datei des Packs. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`.

## Beispiele

Zuletzt benutzt Controller, Aktion „bezahlen“ → Glyph „A“ über dem Bauplatz; zuletzt Tastatur → Kappe der belegten Taste.

## Ausnahme- und Fehlerfälle

Unbekannte Aktion oder unbekanntes Gerät → Text-Rückfall statt leerem Symbol.

## Akzeptanzkriterien

- **AC-01** Test: Aktion + Gerät → Glyph-Schlüssel für alle Aktionen aus `src/input/playerInput.ts`; Unbekanntes liefert den Text-Rückfall.
- **AC-02** Hinweise zeigen am Gerät Glyphen für Controller, Tastatur und Touch (Beobachtung, je ein Hinweis).
- **AC-03** Jede verwendete Glyph-Quelle hat einen Credit-Eintrag oder ist selbst gezeichnet (Datei prüfbar).
- **AC-04** 🧑 hat die Glyphen am TV abgenommen und im Viertel-Split lesbar gefunden.

## Offene Fragen

Zeichenstil der Glyphen (selbst gezeichnet oder Pack): 🧑, `docs/fragenkatalog.md Q13`.

## Notizen

Aus Plan Phase 1 (S3-Zusatz „Controller-Glyphen statt Text“, hier in S6 umgesetzt, weil der Sprint-Plan die CLI-Sprints so schneidet).

2026-10-07: Ohne Abnahme am Gerät abgeschlossen (Entscheidung 🧑); das Abnahme-Kriterium geht in die Gesamtprüfung B-337/AC-05 über.
