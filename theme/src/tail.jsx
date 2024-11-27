import hljs from "highlight.js";

import { Moon, Sun } from "lucide-react";
import React from "react";
import reactDom from "react-dom/client";

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

function ThemeButton() {
	document.documentElement.classList.toggle(
		"dark",
		localStorage.theme === "dark",
	);

	return (
		<div
			onClick={() => {
				localStorage.theme = localStorage.theme === "dark" ? "light" : "dark";

				document.documentElement.classList.toggle(
					"dark",
					localStorage.theme === "dark",
				);
			}}
		>
			{localStorage.theme === "dark" ? (
				<Moon className="h-[1.2rem] w-[1.2rem]" />
			) : (
				<Sun className="h-[1.2rem] w-[1.2rem]" />
			)}
		</div>
	);
}

const rootElement = document.getElementById("theme_btn");
const root = reactDom.createRoot(rootElement);

root.render(<ThemeButton />);