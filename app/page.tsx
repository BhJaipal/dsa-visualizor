import { Icon } from "@iconify/react";
import Link from "next/link";

export default function Home() {
	return (
		<div className="flex flex-col flex-1 items-center justify-center font-sans text-center">
			<h1 className="text-6xl font-bold">
				Data Structures and Algorithms
				<br /> Visualizer Platform
			</h1>
			<p className="text-lg py-8 text-slate-400">
				The DSA Visualizer Platform aims to bridge this gap by providing
				an interactive environment where users can perform operations,
				<br />
				an interactive environment where users can perform operations,
				observe execution in real time, analyze complexity, and connect
				<br />
				theoretical concepts with practical implementations.
			</p>
			<div>
				<Link href={"/array"}>
					<button className="cursor-pointer px-3 py-1 rounded-lg bg-green-400 dark:text-slate-800 flex flex-row items-center gap-2 text-lg">
						Array <Icon icon={"lucide:arrow-right"} fontSize={20} />
					</button>
				</Link>
			</div>
		</div>
	);
}
