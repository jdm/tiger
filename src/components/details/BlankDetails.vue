<template>
	<div class="m-4 h-full flex flex-col items-center justify-center stripes rounded-lg">
		<div class="flex flex-row items-center py-2 p-4 space-x-2 bg-plastic-700 rounded-lg text-plastic-400">
			<ExclamationIcon class="w-6 h-6" />
			<div class="font-semibold">N/A</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ExclamationIcon } from "@heroicons/vue/solid"
</script>

<style scoped>
.stripes {
	background: repeating-linear-gradient(-45deg,
			theme("colors.plastic.800"),
			theme("colors.plastic.800") 10px,
			theme("colors.plastic.700") 10px,
			theme("colors.plastic.700") 20px);
}
</style>
