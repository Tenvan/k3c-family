import { useState } from 'react';
import { backend } from './api';
import { errorText } from '@webkit/lib/errors';
import { ActionButton, StatusBadge, Tip } from '@webkit/atoms';

/** Verweis „Asset-Studio öffnen“ (B-459): startet bin/k3c-studio.exe; die Assets-Seite lebt seit WZ6 dort.
 *  Fehlt die EXE, steht der Hinweis aus Go daneben, bis der nächste Versuch gelingt. */
export function StudioLink() {
  const [error, setError] = useState('');
  const open = async () => {
    try {
      await backend.studioOpen();
      setError('');
    } catch (e) {
      setError(errorText(e));
    }
  };
  return (
    <>
      <ActionButton size="1" variant="soft" onClick={open}>Asset-Studio öffnen</ActionButton>
      {error && (
        <Tip content={error}>
          <StatusBadge tone="warn">{error}</StatusBadge>
        </Tip>
      )}
    </>
  );
}
