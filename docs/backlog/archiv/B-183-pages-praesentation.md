# B-183 · GitHub Pages zeigt eine Präsentationsseite des Spiels

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** –
- **Erstellt:** 2026-10-03
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GitHub Pages lieferte bisher den Build (`dist/`) aus: Startseite war die Shell mit deaktivierten Spiel-Kacheln (B-032).
Wer den Link bekam, sah keine Vorstellung des Spiels. Umgesetzt als Nebenaufgabe ohne Sprint (🧑 Chat, 2026-10-03).

## Ziel

Die Pages-Startseite stellt das Spiel vor (Prinzip, Features, Stufen, Skills, Steuerung, Technik, Stand), ohne einen Server zu brauchen.

## Beteiligte und Zielgruppen

Interessierte, Familie und Freunde mit dem Link; 🧑 pflegt den Inhalt.

## Anforderungen

- Rein statisch: kein Script, keine Anfrage an den Heimnetz-Server oder Fremddienste (keine Web-Fonts, kein CDN).
- Bilder nur aus dem eigenen Build (`public/` → `app/`), Pixel-Art scharf (`image-rendering: pixelated`).
- Läuft am Handy (375 px ohne waagrechtes Scrollen) und am PC; `prefers-reduced-motion` hält die Animationen an.
- Die bisherigen Testseiten (B-032) bleiben unter `app/` erreichbar.

## Nicht-Ziele

Spielbar auf Pages (braucht den Server, B-032). Echte Screenshots oder Videos aus dem Spiel (B-184). Englische Fassung (B-172).

## Regeln und Einschränkungen

`site/` liegt außerhalb des Projekt-Roots, wird also nicht von Vite als Seite gebaut und fällt nicht unter die Regel
„Seiten & Navigation“ (kein `installPageChrome()`, kein Eintrag in `src/landing/pages.ts`). Werte und Regeln stammen aus
`docs/game-design.md`; ändert sich dort Wesentliches, wird die Seite nachgezogen.

## Beispiele

Aufruf der Pages-Adresse → Präsentationsseite mit laufendem Wald-Parallax; Link „Testseiten“ → `app/` mit der Shell aus B-032.

## Ausnahme- und Fehlerfälle

Fehlt ein Bild im Build → `tests/pagesSite.test.ts` schlägt fehl, bevor die Seite ausgeliefert wird.

## Akzeptanzkriterien

- **AC-01** `tests/pagesSite.test.ts` ist grün: kein Script, keine Server- oder Fremdanfrage, jedes Bild unter `app/` existiert in `public/`.
- **AC-02** `task pages` baut `_site/` mit `index.html` (aus `site/`) und dem Build unter `_site/app/`.
- **AC-03** Die CI (`ci.yml`, Job `check`) baut `task pages`; `deploy-pages.yml` lädt `_site/` hoch.
- **AC-04** Lokal (`npx vite preview --outDir _site`) lädt die Seite nur Dateien von der eigenen Adresse, alle mit 200.

## Offene Fragen

keine

## Notizen

- Lokal ansehen: `task pages`, dann Launch-Konfiguration `k3c-pages` (Port 4174).
- Anpassungen: neues PLAT-Ticket mit Verweis auf B-183. Den Abschnitt „Wo wir stehen“ nach jedem Meilenstein (Roadmap) aktualisieren.
- Figuren-Fußpunkte (`--foot`) kommen aus `originY` in `data/sprites.json`.
