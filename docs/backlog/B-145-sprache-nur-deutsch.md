# B-145 · Das Spiel bleibt dauerhaft deutschsprachig, oder die Lokalisierung ist geplant

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** niedrig
- **Status:** eingeplant
- **Sprint:** F1
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Alle Texte des Spiels sind deutsch und im Code hart kodiert (`src/scenes/HudScene.ts`, `src/scenes/LobbyScene.ts`, `src/landing/pages.ts`). Ob das dauerhaft so bleibt, ist nicht festgehalten; B-023 (itch.io) würde Englisch berühren.

## Ziel

Eine Entscheidung steht in den Regeln: nur Deutsch oder Deutsch plus weitere Sprache mit Zeitpunkt. Nutzen: Neue Texte (Onboarding B-148, Credits B-165) müssen nicht ohne Wissen um spätere Lokalisierung gebaut werden.

## Beteiligte und Zielgruppen

Familie als Spieler; 🧑 entscheidet (Beschluss Q05).

## Anforderungen

- Die Entscheidung nennt „nur Deutsch“ oder „zusätzlich Sprache X ab Meilenstein Y“.
- Bei „nur Deutsch“ steht, ob Texte hart kodiert bleiben dürfen; bei weiterer Sprache, ob eine Textquelle (z. B. eine Datei in `data/`) Pflicht für neue Texte wird.

## Nicht-Ziele

Übersetzung selbst, itch.io-Veröffentlichung (B-023).

## Regeln und Einschränkungen

Domäne REG. Werte und Texte gehören nach `CLAUDE.md` nach `data/`, soweit es Balancing ist; UI-Text ist davon nicht erfasst.

## Beispiele

Entscheidung „nur Deutsch“ → neue Hinweistexte stehen im Code, ein Satz in der Regel sagt das.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Entscheidung ohne Laufzeitverhalten.

## Akzeptanzkriterien

- **AC-01** Abschnitt „Sprache“ in `docs/rules/bedienung.md` hält die Entscheidung mit Datum fest und beantwortet die Frage zu hart kodierten Texten (Sichtprüfung); `task check` grün.

## Offene Fragen

Nur Deutsch oder Englisch dazu, wann: `docs/fragenkatalog.md` Q05 (verwandt Q21), entscheidet 🧑.

## Notizen

Aus Plan Lücke 20. Verwandt: B-023.
