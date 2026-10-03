import { installPageChrome } from '../core/shell';
import { CREDITS, renderCredits } from './credits';
import { installPadScroll } from './spriteReference';

installPageChrome();
installPadScroll();
const target = document.getElementById('credits-grafik');
if (target) target.innerHTML = renderCredits(CREDITS);
