<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

function getItems(state: "collapsed" | "expanded") {
	return [
		{
			label: "Array",
			icon: "carbon:array",
		},
		{
			label: "Linked List",
			icon: "carbon:flow",
		},
		{
			label: "Settings",
			icon: "i-lucide-settings",
			defaultOpen: true,
			children:
				state === "expanded"
					? [
							{
								label: "General",
								icon: "i-lucide-house",
							},
							{
								label: "Team",
								icon: "i-lucide-users",
							},
							{
								label: "Billing",
								icon: "i-lucide-credit-card",
							},
					  ]
					: [],
		},
	] satisfies NavigationMenuItem[];
}
</script>

<template>
	<UMain>
		<USidebar
			collapsible="icon"
			rail
			:ui="{
				container: 'h-full',
				inner: 'bg-elevated/25 divide-transparent',
				body: 'py-0',
			}"
		>
			<template #default="{ state }">
				<UNavigationMenu
					class="mt-20"
					:key="state"
					:items="getItems(state)"
					orientation="vertical"
					:ui="{ link: 'p-1.5 overflow-hidden' }"
				/>
			</template>
		</USidebar>

		<NuxtPage />
	</UMain>
</template>