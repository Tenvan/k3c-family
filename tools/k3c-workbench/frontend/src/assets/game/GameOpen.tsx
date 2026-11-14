import { useState } from 'react';
import { ActionButton, Flex, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { backend } from '../../api';

/** Fester Seed des Testspielstands „voll ausgebaut“ (D-04): voll-ansicht. */
export const GAME_SEED = 'ansicht';

/** „Im Spiel ansehen“ (005 › 5, B-847): legt den Testspielstand an und startet das native Spiel mit ihm; der Stand
 *  liegt außerhalb des Repos (gilt als Ansehen, nicht als schreibendes Control). */
export function GameOpen() {
  const [state, setState] = useState<{ busy: boolean; error: string; info: string }>({ busy: false, error: '', info: '' });
  const open = async () => {
    setState({ busy: true, error: '', info: '' });
    try {
      setState({ busy: false, error: '', info: await backend.studioGameOpen(GAME_SEED) });
    } catch (e) {
      setState({ busy: false, error: errorText(e), info: '' });
    }
  };
  return (
    <Flex gap="2" align="center">
      <ActionButton size="1" variant="soft" disabled={state.busy} onClick={open}>Im Spiel ansehen</ActionButton>
      {state.info && <Text size="1" color="gray">{state.info}</Text>}
      {state.error && <Text size="1" color="red">{state.error}</Text>}
    </Flex>
  );
}
