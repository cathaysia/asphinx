import { Moon, Sun } from "lucide-react";
import React, { useEffect, useState } from "react";
import reactDom from "react-dom/client";
import SearchBar from "@/search-bar";

import renderMathInElement from "katex/contrib/auto-render";

renderMathInElement(document.body);

function ThemeButton() {
	document.documentElement.classList.toggle(
		"dark",
		localStorage.theme === "dark",
	);
	const [isDark, setIsdark] = useState(localStorage.them === "dark");

	return (
		<div
			onClick={() => {
				localStorage.theme = localStorage.theme === "dark" ? "light" : "dark";

				document.documentElement.classList.toggle(
					"dark",
					localStorage.theme === "dark",
				);
				setIsdark(localStorage.theme === "dark");
			}}
		>
			{isDark ? (
				<Moon className="h-[1.2rem] w-[1.2rem]" />
			) : (
				<Sun className="h-[1.2rem] w-[1.2rem]" />
			)}
		</div>
	);
}

const themeBtn = document.getElementById("theme_btn");
const btn = reactDom.createRoot(themeBtn!);

btn.render(<ThemeButton />);

const searchBar = document.getElementById("search_bar");
const bar = reactDom.createRoot(searchBar!);

bar.render(<SearchBar />);
