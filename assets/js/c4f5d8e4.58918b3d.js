(self.webpackChunkdoc_ops=self.webpackChunkdoc_ops||[]).push([[195],{1089:function(e,t,a){"use strict";a.r(t),a.d(t,{default:function(){return p}});var n=a(7294),i=a(8001),c=a(6832),r=a(2122),l=a(5977),s=a(6010),o=a(1402),m=a(6266);function u(e){var t=e.title,a=e.link,i=e.description,c=(0,n.useState)(!1),r=c[0],o=c[1],m=(0,l.k6)();return n.createElement("div",{className:"col col--4 margin-vert--md"},n.createElement("div",{className:"action padding--lg",onClick:function(e){e.preventDefault(),m.push(a)},onMouseOver:function(){return o(!0)},onMouseOut:function(){return o(!1)}},n.createElement("div",{className:"action__header"},n.createElement("span",{className:"action__title"},t),n.createElement("a",{href:a,className:"action__button"},n.createElement("span",{className:"action__icon material-icons"},"navigate_next"))),n.createElement("div",{className:(0,s.Z)("headline-stick",{"size-m":r,"size-s":!r})}),n.createElement("div",{className:"action__description"},i)))}var d=function(e){var t=e.actionList,a=e.title,i=e.tagline,c=(0,m.Z)().isDarkTheme;return n.createElement("header",{className:"header padding-vert--xl"},n.createElement("div",{className:"title margin-horiz--sm"},n.createElement("img",{className:"title__image",src:c?(0,o.Z)("/img/globe_dark.svg"):(0,o.Z)("/img/globe_light.svg"),alt:"IOTA Wiki"}),n.createElement("div",null,n.createElement("h1",{className:"title__text"},a),n.createElement("span",{className:"title__subtext grey"},i))),n.createElement("div",{className:"margin-top--xl"},n.createElement("div",{className:"section-header text--center margin-bottom--sm"},"Get started, right away"),n.createElement("div",{className:"actionlist row"},t.map((function(e,t){return n.createElement(u,(0,r.Z)({key:t},e))})))))},g=[{title:"Learn",link:"welcome",description:"Learn the Basics about the IOTA Bee node and how it works behind the scenes."},{title:"Build",link:"setup_a_node",description:"Follow our tutorial to run your own IOTA Bee node."},{title:"Participate",link:"contribute/contribute",description:"Do you want to be a part of the IOTA mission? Join the IOTA community."}];function p(){var e=(0,c.Z)().siteConfig;return n.createElement(i.Z,{title:""+e.title,description:""+e.tagline},n.createElement(d,{actionList:g,title:"Bee",tagline:"Official IOTA Bee Software"}))}}}]);