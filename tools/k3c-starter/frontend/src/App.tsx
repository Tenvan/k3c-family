import { StatusBadge, Tip } from '@webkit/atoms';
import { AppFrame } from '@webkit/templates';
import { backend } from './api';
import { ServerPage } from './server/ServerPage';

/** Starter-Fenster: Rahmen aus webkit, eine Seite ohne Reiter; die Seite hält das Fenster fest, nur ihr Inhalt scrollt. */
export function App() {
  const mock = backend.mock && (
    <Tip content="Keine Wails-Laufzeit: erfundene Daten">
      <StatusBadge tone="warn">Mock</StatusBadge>
    </Tip>
  );
  return (
    <AppFrame title="K3C Server" appearance="dark" right={mock} fixed>
      <ServerPage />
    </AppFrame>
  );
}
