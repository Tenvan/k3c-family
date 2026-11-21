import { Button, Flex } from '@webkit/atoms';
import type { TreeOpen } from './useTreeOpen';

/** Leiste über einem Baum: alles aufklappen, alles einklappen (Katalog, Werte, Geschichte). */
export function TreeBar({ tree }: { tree: Pick<TreeOpen, 'expandAll' | 'collapseAll'> }) {
  return (
    <Flex gap="1" className="tb-bar" role="toolbar" aria-label="Baum auf- und zuklappen">
      <Button type="button" size="1" variant="soft" color="gray" aria-label="Alle aufklappen" onClick={tree.expandAll}>⊞ Alle auf</Button>
      <Button type="button" size="1" variant="soft" color="gray" aria-label="Alle einklappen" onClick={tree.collapseAll}>⊟ Alle zu</Button>
    </Flex>
  );
}
