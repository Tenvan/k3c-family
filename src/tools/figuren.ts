import { installPageChrome } from '../core/shell';
import { allFigures, installPadScroll, renderReference } from './spriteReference';

installPageChrome();
installPadScroll();
renderReference(document.getElementById('cards')!, allFigures(), false);
