import { cn } from "@/lib/utils";
import { highlighter } from "./hlight";

export interface HighlightProps {
	/** Substring or an array of substrings to highlight in `children` */
	highlight: string | string[];

	/** Key of `theme.colors` or any valid CSS color, passed to `Mark` component `color` prop, `yellow` by default */
	color?: string;

	/** String parts of which must be highlighted */
	children: string;
}

export function Highlight({ highlight, children, color }: HighlightProps) {
	const highlightChunks = highlighter(children, highlight);

	return (
		<span>
			{highlightChunks.map(({ chunk, highlighted }, i) =>
				highlighted ? (
					<span
						key={i}
						className={cn("bg-yellow-500 dark:bg-yellow-500", color)}
						data-highlight={chunk}
					>
						{chunk}
					</span>
				) : (
					<span key={i}>{chunk}</span>
				),
			)}
		</span>
	);
}
