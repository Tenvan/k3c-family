# tools/workkit/winicon/

## Responsibility

Setzt das Fenster-Icon einer Wails-App zur Laufzeit (Windows); `wails build` bettet `icon/icon.ico` nur als EXE-Icon ein.

## Design

- `winicon_windows.go`: `Set`, `Handle` (Win32); `winicon_other.go`: leere Fassung für andere Plattformen.

## Flow

1. App ruft `Set` nach dem Start des Fensters mit dem eingebetteten Icon.

## Integration

- Konsumenten: `tools/k3c-workbench`, `tools/k3c-studio` (`icon/`).
