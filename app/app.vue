<script setup>
import applogo from "/applogo.svg";

useHead({
	meta: [
		{ name: "viewport", content: "width=device-width, initial-scale=1" },
	],
	link: [{ rel: "icon", href: "/favicon.ico" }],
	htmlAttrs: {
		lang: "en",
	},
});

const title = "Data Structures and Algorithms Visualizer Platform";
const description =
	"The DSA Visualizer Platform aims to bridge this gap by providing an interactive environment where users can perform operations, observe execution in real time, analyze complexity, and connect theoretical concepts with practical implementations.";

useSeoMeta({
	title,
	description,
	ogTitle: title,
	ogDescription: description,
	ogImage: applogo,
	twitterCard: "summary_large_image",
});
</script>

<template>
	<UApp>
		<UHeader>
			<template #left>
				<NuxtLink to="/">
					<AppLogo class="w-auto h-10 shrink-0" />
				</NuxtLink>
			</template>
		</UHeader>

		<NuxtLayout />

		<USeparator icon="i-simple-icons-nuxtdotjs" />

		<UFooter>
			<template #left>
				<p class="text-sm text-muted">
					Built by BhJaipal • © {{ new Date().getFullYear() }}
					<UButton
						to="https://codeberg.org/BhJaipal"
						target="_blank"
						icon="i-simple-icons-codeberg"
						aria-label="GitHub"
						color="neutral"
						variant="ghost"
					/><UButton
						to="https://github.com/BhJaipal"
						target="_blank"
						icon="i-simple-icons-github"
						aria-label="GitHub"
						color="neutral"
						variant="ghost"
					/>
				</p>
			</template>

			<template #right>
				<UButton
					to="https://github.com/BhJaipal/dsa-visualizor"
					target="_blank"
					label="Source"
					icon="i-simple-icons-github"
					aria-label="GitHub"
					color="neutral"
					variant="ghost"
				/>
			</template>
		</UFooter>
	</UApp>
</template>
