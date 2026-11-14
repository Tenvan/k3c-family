# B-763 · Bei WebGL-Kontextverlust Renderer neu aufbauen statt abschalten

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** verworfen
- **Sprint:** RX1
- **Projekt:** LST
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Auf der Xbox-Edge (192.168.2.209) meldet der Client nach 3–4 s 'WebGL Context lost. Renderer disabled' (dreimal in Folge, 17:56–18:01). Phaser schaltet den Renderer ab, das Spiel läuft mit etwa 12 FPS und 480–570 ms Latenz weiter und zeigt eine weiße Seite; Start in `src/main.ts` mit `Phaser.AUTO`.

## Ziel

Ein Kontextverlust führt zu einer spielbaren Anzeige statt zur weißen Seite.

## Beteiligte und Zielgruppen

🧑 spielt auf Xbox und Pi.

## Anforderungen

- Bei `webglcontextlost` den Kontext wiederherstellen (`webglcontextrestored`) und Texturen neu hochladen, oder sauber mit `?renderer=canvas` neu laden.
- Höchstens ein automatischer Neustart je Sitzung, danach Canvas und sichtbarer Hinweis im Spiel.
- Spielstand und Verbindung zum Server bleiben erhalten (der Client hält keinen Spiel-Zustand).
- Jedes Ereignis ins Client-Log (B-762).

## Nicht-Ziele

Keine Verkleinerung der Texturen (B-764), keine Leistungsoptimierung des Canvas-Renderers.

## Regeln und Einschränkungen

B nicht belegen; Shell-Regeln für Seitenwechsel beachten (kein `location` außer über `openPage()`).

## Beispiele

Kontext geht verloren → Spiel baut neu auf oder lädt mit Canvas neu, Hinweis erscheint, Spiel bleibt bedienbar.

## Ausnahme- und Fehlerfälle

Verlust wiederholt sich sofort → keine Schleife: Canvas bleibt, Hinweis bleibt.

## Akzeptanzkriterien

- **AC-01** Test: Simulierter Kontextverlust löst genau einen Neuaufbau oder Canvas-Neustart aus.
- **AC-02** Test: Zweiter Verlust in derselben Sitzung startet keinen weiteren Versuch.
- **AC-03** Abnahme (🧑): Auf der Xbox bleibt das Spiel bedienbar, keine weiße Seite.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Umgesetzt in RX1.2 (`src/core/contextGuard.ts`): Phaser 4.2 baut den Kontext bei `webglcontextrestored` selbst wieder auf (`dispatchContextRestored`); die Seite wartet 4 s darauf, sonst genau ein Neustart mit `?renderer=canvas` und Beitritt zum offenen Raum, höchstens einmal je 10 Minuten (Sitzungsmarke mit Zeitstempel). Ein zweiter Verlust nach einer Wiederherstellung startet sofort den Neustart; nach dem Neustart gibt es nur den Hinweis „Bitte die Seite neu laden“. Offen: AC-03 (Xbox) im Xbox-Lauf. Bekannt: Nach einem Server-Neustart ist der Raum neu; ein laufender Stresstest misst dann ab der nächsten Stufe ohne die zuvor gesetzten Figuren.

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009).
