# B-767 · Nachbesserungen aus dem ersten Xbox-Test: frischer Client, Pad auf allen Seiten, Lobby-Löschen, Stress ohne Dev-Mode

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** RX1
- **Projekt:** LST
- **Erstellt:** 2026-11-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Erster Test von v0.22.3 auf Xbox und Pi (2026-11-09): (1) Der Client bleibt immer wieder in einer alten Fassung (Cache); die Landingpage bleibt dauerhaft offen. (2) Spielstände lassen sich in der Lobby nicht mit der Taste X löschen. (3) Die Stress-Kacheln mit Server liefen als normaler Start ohne Spawns. (4) Pad-Tasten funktionieren nicht auf allen Seiten.

## Ziel

Der Client lädt nach einem neuen Server-Image sicher die neue Fassung; Pad-Bedienung funktioniert auf jeder Seite; das Löschen in der Lobby klappt; ein Stresslauf ohne Dev-Mode meldet das sofort.

## Beteiligte und Zielgruppen

🧑 testet an der Xbox; Claude liest das Log vom Pi (Session „PI Diagnose-Logs“).

## Anforderungen

- **Frischer Client:** Meldet `api/health` eine andere Version als der Client, lädt die Landingpage einmal je Version mit `?v=<Version>` neu (Hash-Link der offenen Seite bleibt), nur wenn keine Seite offen ist; Prüfung beim Start und alle 60 s. Seiten im iframe bekommen `v=<Version>` (umgeht den Zwischenspeicher). Server liefert HTML mit `no-cache`, Assets mit `immutable` (schon so).
- **Lobby-Löschen:** X und zusätzlich Y (und Entf) lösen die Rückfrage aus; die Hilfezeile nennt beides; jede neu gedrückte Pad-Taste (Index) steht in der Lobby im Client-Log (höchstens 30 je Besuch), damit auf dem Pi sichtbar wird, ob X ankommt.
- **Pad auf allen Seiten:** `installPageChrome()` aktiviert eine gemeinsame Bedienung (`src/core/padNav.ts`): D-Pad und linker Stick bewegen den Fokus zum nächsten Element in der Richtung, A klickt, ohne Ziel scrollt die Seite; B bleibt frei, View+Menu reserviert. Seiten mit eigener Pad-Bedienung oder Phaser (`game`, `dev`, `monitor`, `testing`, `soundtest`, `grafiken`, `gamepad-test`) schalten sie mit `padNav: false` ab.
- **Stress ohne Dev-Mode:** Antwortet der Server auf `spawn` mit `forbidden`, endet der Lauf sofort mit dem Hinweis „Server ohne Dev-Mode (K3C_DEV=1 …)“ (Das Release-Image hat `K3C_DEV=0`; am Pi `K3C_DEV=1 docker compose up -d`).

## Nicht-Ziele

Kein Service-Worker, keine neue Cache-Strategie am Server; keine Neubelegung von B.

## Regeln und Einschränkungen

Seiten wechseln nur über `openPage()`; die Landingpage darf `location.replace` (Shell-Ausnahme). Nie mitten im Spiel neu laden. Pad-Tasten B, View, Menu unverändert.

## Beispiele

Xbox: Landingpage zeigt v0.22.2, Pi liefert v0.22.3 → lädt einmal neu, zeigt v0.22.3. Stress-Kachel gegen einen Pi ohne Dev-Mode → Log „🚦 Server ohne Dev-Mode: spawn abgelehnt (K3C_DEV=1 setzen …)“ und Endzeile.

## Ausnahme- und Fehlerfälle

Bleibt die Version nach dem Neuladen verschieden (hartnäckiger Cache), lädt die Seite nicht erneut, die Versionszeile bleibt als Abweichung markiert. Ohne Sitzungsspeicher kein automatisches Neuladen.

## Akzeptanzkriterien

- **AC-01** Test: Neuladen nur bei erreichbarem Server mit anderer Version und ohne Versuch für diese Version; Adresse behält Abfrage und Hash; iframe-Seiten bekommen `v`.
- **AC-02** Test: Y und X lösen in der Lobby das Löschen aus (Lobby-Eingabe), Hilfezeile nennt X / Y / Entf.
- **AC-03** Test: `PadNav` (Wiederholung, A einmal, B/X/Y/View/Menu ohne Wirkung) und Zielwahl im Raster.
- **AC-04** Test: `forbidden` beendet den Stresslauf sofort mit Hinweis auf K3C_DEV.
- **AC-05** Abnahme (🧑): Xbox lädt nach einem neuen Image die neue Fassung; D-Pad/A bedienen Lizenzen, Pipeline und Stress-Menü; X oder Y löscht in der Lobby (oder das Log zeigt, welche Taste ankommt).

## Offene Fragen

keine

## Notizen

Warum X auf der Xbox nicht ankam, ist offen; auf dem PC (standard mapping) löst Index 2 die Rückfrage aus (im Browser mit Mock-Pad geprüft). Das Lobby-Log (`🎮 Lobby: Pad … Taste n`) klärt es im nächsten Test.

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009).
