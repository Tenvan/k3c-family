import { Box, Button, Flex, Heading, StatusBadge, type Tone } from '@webkit/atoms';
import { backend, type ServerState } from '../api';
import { AddressList } from './AddressList';
import { LogBox } from './LogBox';
import { useServerState } from './useServerState';

const toneOf = (s: ServerState): Tone => (s.running ? 'ok' : s.starting ? 'warn' : 'error');

/** Seite des Starters: Status, Adressen im Heimnetz, Starten/Beenden, Meldungen des Servers. */
export function ServerPage() {
  const { state, refresh } = useServerState();
  // Start und Beenden laufen im Hintergrund (Beenden wartet bis zu 10 s); der Zustand wird gleich danach neu gelesen.
  const act = (run: () => Promise<void>) => {
    void run();
    void refresh();
  };
  const busy = state.running || state.starting;
  return (
    <Flex direction="column" gap="4">
      <Box className="srv-status">
        <StatusBadge tone={toneOf(state)}>{state.message}</StatusBadge>
      </Box>
      {state.urls.length > 0 && (
        <Flex direction="column" gap="2">
          <Heading as="h2" size="2" color="gray">
            Adressen im Heimnetz (Xbox und PC im Browser öffnen)
          </Heading>
          <AddressList urls={state.urls} />
        </Flex>
      )}
      <Flex gap="2">
        <Button disabled={busy} onClick={() => act(backend.start)}>
          Starten
        </Button>
        <Button color="red" variant="soft" disabled={!busy} onClick={() => act(backend.stop)}>
          Beenden
        </Button>
      </Flex>
      <Flex direction="column" gap="2">
        <Heading as="h2" size="2" color="gray">
          Meldungen des Servers
        </Heading>
        <LogBox lines={state.log} />
      </Flex>
    </Flex>
  );
}
