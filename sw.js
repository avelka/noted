if(!self.define){let e,s={};const i=(i,t)=>(i=new URL(i+".js",t).href,s[i]||new Promise((s=>{if("document"in self){const e=document.createElement("script");e.src=i,e.onload=s,document.head.appendChild(e)}else e=i,importScripts(i),s()})).then((()=>{let e=s[i];if(!e)throw new Error(`Module ${i} didn’t register its module`);return e})));self.define=(t,d)=>{const r=e||("document"in self?document.currentScript.src:"")||location.href;if(s[r])return;let n={};const o=e=>i(e,r),c={module:{uri:r},exports:n,require:o};s[r]=Promise.all(t.map((e=>c[e]||o(e)))).then((e=>(d(...e),n)))}}define(["./workbox-d4314735"],(function(e){"use strict";self.addEventListener("message",(e=>{e.data&&"SKIP_WAITING"===e.data.type&&self.skipWaiting()})),e.precacheAndRoute([{url:"index.html",revision:"4da7331b9f19548a12e4fc6b8b0ef6f0"},{url:"rusted_note-d252c63f271ed05a_bg.wasm",revision:"6cf014d268555bbaf06ed2df3584ada9"},{url:"rusted_note-d252c63f271ed05a.js",revision:"04baf237db725e2c239b83777c04879d"},{url:"snippets/leptos_reactive-f7acfdcc44aba8c4/inline0.js",revision:"812dd7e5247090819ed2f0afd0eb6b69"},{url:"style-a276dd8c07fdc2d2.css",revision:"d5e5e5590c43b264f96eef60572f2075"}],{ignoreURLParametersMatching:[/^utm_/,/^fbclid$/]})}));
//# sourceMappingURL=sw.js.map
