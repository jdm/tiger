<template>
	<div class="self-center flex flex-row bg-plastic-900 outline outline-1 outline-plastic-700 rounded-sm">
		<input class="w-full  text-xs bg-transparent text-plastic-300 rounded-sm border-0 focus:ring-0" type="text"
			spellcheck="false" value="48" />
		<div v-if="unit" class="text-xs self-center px-2 text-plastic-500">{{ unit }}</div>
	</div>
</template>

<script setup lang="ts">
defineProps<{
	unit?: string,
}>();
</script>