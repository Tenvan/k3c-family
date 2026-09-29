import { installPageChrome } from '../core/shell';
import { installPadScroll, lineup, renderReference } from './spriteReference';

installPageChrome();
installPadScroll();
renderReference(document.getElementById('cards')!, lineup(), true);
