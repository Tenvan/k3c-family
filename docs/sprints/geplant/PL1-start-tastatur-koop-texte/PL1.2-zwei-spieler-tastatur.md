# PL1.2 · Zwei Spieler an einer Tastatur

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** pl1/2-zwei-spieler-tastatur
- **Abhängig von:** –
- **Tickets:** B-316
- **Kriterien:** AC-02

## Ziel

An einer Tastatur spielen zwei Spieler mit getrennten Layouts; Spieler 2 tritt mit seiner eigenen Bestätigen-Taste bei, und ein Test belegt, dass die Layouts keine gemeinsame Taste haben und jede Aktion in beiden belegt ist.

## Kontext

- **Belegung (Beschluss 🧑 2026-10-06):** Spieler 1 links: A/D, Shift, Leertaste, E, Q, R, T, Z, K. Spieler 2 rechts: Pfeiltasten, Strg rechts, Enter, Ziffernblock. Spieler 2 tritt mit seiner Bestätigen-Taste (Enter) bei. Welche Ziffernblock-Taste welche Aktion trägt, legt diese Session fest und nennt es im Ergebnis.
- **Heute:** `src/input/playerInput.ts` › `KeyboardInput` ist ein Spieler: `addKeys({ left: K.A, right: K.D, altLeft: K.LEFT, altRight: K.RIGHT, sprint: K.SHIFT, ...actions })`, die Aktionen kommen aus `KEY_ACTIONS` in `src/input/slotBindings.ts` (`SLOT_KEYS.keyboard`: attack E, skill1 Q, skill2 R, skill3 T, skill4 Z, skillMenu K; dazu `confirm`, `pause`, `fullscreen`). Die Pfeiltasten bewegen heute Spieler 1 mit; das fällt weg.
- **Reservierte Tasten:** Pos1 = zurück (`src/core/shell.ts`), F = Vollbild, Esc = Pause, Ö/Ä = Debug-Overlay. Keine davon in Layout 2; `pause` und `fullscreen` bleiben bei Spieler 1.
- **Einbindung im Spiel:** `src/scenes/GameScene.ts` erzeugt `new KeyboardInput(this.input.keyboard!)` (Zeile ~130), ruft `update()` (~148) und gibt die Eingaben als `[this.keyboard, ...this.pads, ...touch]` (~223) an `this.slots.join(…)` (`src/scenes/localSlots.ts`); `pause` liest nur `this.keyboard`. Für Spieler 2 braucht `GameScene.ts` eine zweite Instanz in dieser Liste. `src/scenes/` ist CLI: Die Änderung bleibt auf diese drei Stellen begrenzt (Grenzfall Domäne, siehe Sprint-README › Offene Fragen); alles Weitere in `src/scenes/` → Ticket.
- **Hilfe:** Glyphen und Aktionshinweise lesen die Belegung aus `slotBindings.ts` (`src/scenes/glyphs.ts`, `src/scenes/actionHints.ts`). Zeigen sie Layout 2 nicht ohne Änderung in `src/scenes/`, wird das ein Ticket (Domäne CLI), nicht Teil dieser Session.
- **Fallstricke:** Ghosting: übliche Kombinationen (Laufen + Sprint + Aktion je Spieler) gleichzeitig drücken. `K.SHIFT` meint beide Shift-Tasten; Strg rechts und Enter (Haupt- und Ziffernblock) über `event.code` bzw. `location` unterscheiden, damit Layout 1 sie nicht mitliest.
- Regeln: Spiel-Code fragt Aktionen ab, nie Tasten; 2 Spieler gleichzeitig; B nicht belegen; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `src/input/playerInput.ts`, `src/input/slotBindings.ts`, `src/input/slotBindings.test.ts`, neue Dateien in `src/input/` (z. B. `keyboardLayouts.ts` mit Test)
- `src/scenes/GameScene.ts` (nur Erzeugen, `update()` und Eingabeliste für die zweite Tastatur-Instanz)
- `docs/sprints/geplant/PL1-start-tastatur-koop-texte/`, `docs/sprints/aktiv/PL1-start-tastatur-koop-texte/`, `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Tastenbelegung frei einstellbar; Controller-Änderungen; Umbau von Hilfe, Glyphen oder HUD in `src/scenes/`; Abnahme am PC (PL1.5).

## Schritte

1. Branch anlegen und pushen, `Status: in Arbeit`. `docs/arbeitsweise.md` lesen.
2. Test zuerst (`src/input/`): Layout 1 und Layout 2 haben keine gemeinsame Taste; Laufen, Sprint, `confirm` und alle Slot-Aktionen sind in beiden belegt; die reservierten Tasten (Pos1, F, Esc, Ö, Ä) fehlen in Layout 2.
3. Layouts als Daten in `src/input/` ablegen; `KeyboardInput` bekommt das Layout im Konstruktor, die Pfeiltasten verlassen Layout 1.
4. `GameScene.ts`: zweite Instanz mit Layout 2 erzeugen, in `update()` und in die Eingabeliste aufnehmen; der Beitritt läuft über `confirm` wie bei Pads.
5. Im Browser-Pane mit `KeyboardEvent`s (Leertaste, dann Enter) prüfen: zwei Monarchen im Split-Screen, A/D bewegt nur Spieler 1, Pfeile nur Spieler 2.
6. `task check` ausführen, Ergebnis schreiben (Ziffernblock-Zuordnung, Beobachtung aus Schritt 5), `Status: fertig`, Tabelle der Sprint-README anpassen.

## Fertig, wenn

- [ ] AC-02: Ein Test in `src/input/` belegt: keine gemeinsame Taste in den Layouts, jede Aktion in beiden belegt (B-316/AC-01).
- [ ] AC-02: Im Browser-Pane treten zwei Tastatur-Spieler bei (Leertaste, Enter) und bewegen sich unabhängig (Beobachtung im Ergebnis; die Abnahme B-316/AC-02 macht 🧑 in PL1.5).
- [ ] `task check` grün; keine Datei über 400 Zeilen, keine Funktion über 60.

## Prüfen

```bash
task test -- input
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
