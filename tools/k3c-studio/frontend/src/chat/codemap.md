# tools/k3c-studio/frontend/src/chat/

## Responsibility
Chat rechts im Asset-Studio (B-488, WZ19.2): assistant-ui mit der Claude Code CLI als Modell, auf allen Seiten, ein- und ausblendbar.

## Design
- `ChatPanel.tsx`: `ChatPanel` (assistant-ui `useLocalRuntime`, Nachrichten, Eingabefeld, Senden/Abbrechen, Hinweiszeile) bleibt auch ausgeblendet eingehängt; `ChatToggle` für die Kopfzeile.
- `chatAdapter.ts`: `createChatAdapter(stream, onNotice)` macht aus dem Antwort-Strom eine fortlaufende Antwort; `done` setzt den Ausgang (complete, cancelled, error), der Chat des Servers bleibt über Nachrichten erhalten.
- `chatBridge.ts`: Zustand außerhalb der Oberfläche: `useChatOpen`/`setChatOpen` (gemerkt unter `k3c-studio.chat.state`), `fillChat(text)` legt einen Text ins Eingabefeld und blendet den Chat ein (für den Prompt-Knopf, WZ19.3).
- Tests: `chatAdapter.test.ts`, `ChatPanel.test.tsx` (happy-dom gegen den Mock).

## Flow
Eingabe → assistant-ui → Adapter → `backend.chatStream` → `POST /api/studio/chat` (Go `backend/chat_http.go`) → Stücke zurück in die Antwort.

## Integration
- Backend: `tools/k3c-studio/backend/chat_*.go`; Einbindung in `../App.tsx`; Stile `../styles/app.css` (`.chat*`).
