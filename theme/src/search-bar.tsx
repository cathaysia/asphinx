import { useEffect, useMemo, useState } from "react";
import { Button } from "./components/ui/button";
import {
	CommandDialog,
	CommandEmpty,
	CommandGroup,
	CommandInput,
	CommandItem,
	CommandList,
} from "./components/ui/command";
import Fuse, { FuseResult } from "fuse.js";

import useSWR from "swr";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "./components/ui/input";
import { ScrollArea } from "./components/ui/scroll-area";

type CacheType = [string, [string, string]][];

export default function SearchBar() {
	const [open, setOpen] = useState(false);
	const [search, setSearch] = useState("");
	const [result, setResult] =
		useState<
			FuseResult<{
				file: string;
				content: [string, string];
			}>[]
		>();
	const { data } = useSWR<CacheType>(
		"/cache.json",
		(input: RequestInfo | URL, init?: RequestInit) => {
			return fetch(input, init).then((res) => res.json());
		},
	);
	const fuse = useMemo(() => {
		if (!data) {
			return;
		}
		const v = data as CacheType;
		const posts = v.map((item) => {
			return {
				file: item[0],
				content: item[1],
			};
		});
		return new Fuse(posts, {
			keys: ["file", "content"],
		});
	}, [data]);

	useEffect(() => {
		if (!data || !fuse) {
			return;
		}
		if (search.length === 0) {
			setResult(undefined);
			return;
		}

		const res = fuse.search(search);
		setResult(res);
	}, [data, search]);

	return (
		<Dialog>
			<DialogTrigger>
				<Button variant={"outline"} onClick={() => setOpen(true)}>
					Search
				</Button>
			</DialogTrigger>
			<DialogContent className="flex flex-col gap-2 w-full">
				<Input
					placeholder="type for search"
					value={search}
					onChange={(e) => setSearch(e.target.value)}
				/>
				<ScrollArea className="h-[320px]">
					<div className="flex flex-col gap-2">
						{result
							? result.map((item) => {
									return (
										<div
											key={item.item.file}
											className="p-2 flex flex-col w-full items-start border shadow rounded"
										>
											<a className="text-lg" href={`/${item.item.file}`}>
												{item.item.content[1] || item.item.file}
											</a>
											<span className="whitespace-nowrap overflow-x-hidden text-gray-600 text-sm">
												{item.item.content[0]}
											</span>
										</div>
									);
								})
							: data?.map((item) => {
									return (
										<div key={item[0]} className="border shadow p-2 rounded">
											<a href={`/${item[0]}`}>{item[0]}</a>
										</div>
									);
								})}
					</div>
				</ScrollArea>
			</DialogContent>
		</Dialog>
	);
}
