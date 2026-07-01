import { Icon } from "@iconify/react";
import Link from "next/link";

function feature({
	icon,
	title,
	description,
}: {
	icon: string;
	title: string;
	description: string;
}) {
	return (
		<li className="relative rounded-sm flex items-start gap-2.5">
			<div className="inline-flex items-center justify-center p-0.5">
				<Icon
					icon={icon}
					className="size-5 shrink-0 text-primary"
					aria-hidden="true"
				></Icon>
			</div>
			<div className="text-start">
				<div className="text-base text-pretty font-semibold text-highlighted">
					{title}
				</div>
				<div className="text-[15px] text-pretty text-muted mt-1 text-slate-400">
					{description}
				</div>
			</div>
		</li>
	);
}

export default function Home() {
	const data = [
		{
			icon: "mdi:learn-outline",
			title: "Learn",
			description:
				"Learn Data Structures concepts visually​ and interact with them in real time.",
		},
		{
			icon: "hugeicons:algorithm",
			title: "Understand",
			description:
				"Observe algorithm execution step-by-step and Understand time and space complexity.",
		},
	];
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
			<ul className="grid sm:grid-cols-2 lg:grid-cols-2 gap-8 w-1/2 pt-5 pb-12">
				{data.map((el) => feature(el))}
			</ul>
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
