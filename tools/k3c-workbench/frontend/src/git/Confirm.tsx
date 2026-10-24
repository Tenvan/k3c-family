import type { ReactNode } from 'react';
import { Button, Flex } from '@webkit/atoms';
import { AlertDialog } from '@webkit/organisms';

export interface ConfirmRequest {
  title: string;
  text: ReactNode;
  action: string;
  /** Rot für Zerstörendes (Löschen, Schließen). */
  danger?: boolean;
  run: () => void;
}

/** Bestätigung vor jeder Aktion, die sich nicht einfach zurücknehmen lässt (B-580/AC-04). */
export function Confirm({ request, onClose }: { request: ConfirmRequest | null; onClose: () => void }) {
  return (
    <AlertDialog.Root open={request !== null} onOpenChange={(open) => !open && onClose()}>
      <AlertDialog.Content maxWidth="480px">
        <AlertDialog.Title>{request?.title}</AlertDialog.Title>
        <AlertDialog.Description size="2">{request?.text}</AlertDialog.Description>
        <Flex gap="3" mt="4" justify="end">
          <AlertDialog.Cancel>
            <Button variant="soft" color="gray">Abbrechen</Button>
          </AlertDialog.Cancel>
          <AlertDialog.Action>
            <Button color={request?.danger ? 'red' : undefined} onClick={() => request?.run()}>{request?.action}</Button>
          </AlertDialog.Action>
        </Flex>
      </AlertDialog.Content>
    </AlertDialog.Root>
  );
}
