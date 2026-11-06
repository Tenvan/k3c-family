import { useState } from 'react';
import { ActionButton, Flex, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { backend } from '../../api';

/** Fester Seed des Testspielstands „voll ausgebaut“ (D-04): voll-ansicht. */
export const GAME_SEED = 'ansicht';

/** „Im Spiel ansehen“ (005 › 5, WB1.4): legt den Testspielstand in der eingebundenen Engine an und öffnet das Spiel im
 *  Browser; der Stand liegt außerhalb des Repos (gilt als Ansehen, nicht als schreibendes Control). */
export function GameOpen() {
  const [state, setState] = useState<{ busy: boolean; error: string }>({ busy: false, error: '' });
  const open = async () => {
    setState({ busy: true, error: '' });
    try {
      backend.openUrl(await backend.studioGameOpen(GAME_SEED));
      setState({ busy: false, error: '' });
    } catch (e) {
      setState({ busy: false, error: errorText(e) });
    }
  };
  return (
    <Flex gap="2" align="center">
      <ActionButton size="1" variant="soft" disabled={state.busy} onClick={open}>Im Spiel ansehen</ActionButton>
      {state.error && <Text size="1" color="red">{state.error}</Text>}
    </Flex>
  );
}
