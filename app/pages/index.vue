<script setup lang="ts">
definePageMeta({
	layout: "dashboard",
});
</script>

<template>
	<div>
		<UPageHero
			title="Data Structures and Algorithms Visualizer Platform"
			description="The DSA Visualizer Platform aims to bridge this gap by providing an interactive environment where users can perform operations, observe execution in real time, analyze complexity, and connect theoretical concepts with practical implementations."
			:links="[
				{
					label: 'Get started',
					to: '/array',
					trailingIcon: 'i-lucide-arrow-right',
					size: 'xl',
				},
			]"
		/>

		<UPageSection
			id="features"
			title="Everything you need to build modern Nuxt apps"
			description="Start with a solid foundation. This template includes all the essentials for building production-ready applications with Nuxt UI's powerful component system."
			:features="[
				{
					icon: 'mdi:learn-outline',
					title: 'Learn',
					description:
						'Learn Data Structures concepts visually​ and interact with them in real time.',
				},
				{
					icon: 'hugeicons:algorithm',
					title: 'Understand',
					description:
						'Observe algorithm execution step-by-step and Understand time and space complexity.',
				},
				{
					icon: 'carbon:flow',
					title: 'Execution flow',
					description:
						'Understand algorithm logic and execution flow.',
				},
			]"
		/>
	</div>
</template>
