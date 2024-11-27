import "highlight.js/styles/github.min.css";
import hljs from "highlight.js";

if (!hljs.initHighlighting.called) {
	hljs.initHighlighting.called = true;
	[].slice
		.call(document.querySelectorAll("pre.highlight > code[data-lang]"))
		.forEach(function (el) {
			hljs.highlightBlock(el);
		});
}

import renderMathInElement from "katex/contrib/auto-render";

renderMathInElement(document.body);
