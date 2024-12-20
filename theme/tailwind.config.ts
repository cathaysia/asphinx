import type { Config } from "tailwindcss";
import defaultTheme from "tailwindcss/defaultTheme";

const config: Config = {
	darkMode: ["class"],
	content: ["layouts/*.html", "src/*.js"],
	theme: {
		extend: {
			fontFamily: {
				sans: [
					"Inter Variable",
					"Noto Sans SC Variable",
					...defaultTheme.fontFamily.sans,
				],
				mono: ["Fira Code Variable", ...defaultTheme.fontFamily.mono],
			},
			typography: {
				DEFAULT: {
					css: {
						"code::before": {
							content: "",
						},
						"code::after": {
							content: "",
						},
					},
				},
			},
		},
	},
	plugins: [require("@tailwindcss/typography")],
};

export default config;
