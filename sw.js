if(!self.define){let e,s={};const d=(d,i)=>(d=new URL(d+".js",i).href,s[d]||new Promise((s=>{if("document"in self){const e=document.createElement("script");e.src=d,e.onload=s,document.head.appendChild(e)}else e=d,importScripts(d),s()})).then((()=>{let e=s[d];if(!e)throw new Error(`Module ${d} didn’t register its module`);return e})));self.define=(i,t)=>{const r=e||("document"in self?document.currentScript.src:"")||location.href;if(s[r])return;let c={};const n=e=>d(e,r),o={module:{uri:r},exports:c,require:n};s[r]=Promise.all(i.map((e=>o[e]||n(e)))).then((e=>(t(...e),c)))}}define(["./workbox-d4314735"],(function(e){"use strict";self.addEventListener("message",(e=>{e.data&&"SKIP_WAITING"===e.data.type&&self.skipWaiting()})),e.precacheAndRoute([{url:"index.html",revision:"06dac2c18c993cb50bbc9e839a56fd35"},{url:"rusted_note-d252c63f271ed05a_bg.wasm",revision:"6cf014d268555bbaf06ed2df3584ada9"},{url:"rusted_note-d252c63f271ed05a.js",revision:"04baf237db725e2c239b83777c04879d"},{url:"snippets/leptos_reactive-f7acfdcc44aba8c4/inline0.js",revision:"812dd7e5247090819ed2f0afd0eb6b69"},{url:"style-a276dd8c07fdc2d2.css",revision:"d5e5e5590c43b264f96eef60572f2075"}],{ignoreURLParametersMatching:[/^utm_/,/^fbclid$/]})}));
//# sourceMappingURL=sw.js.map
