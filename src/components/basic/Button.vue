<template>
	<button type="button"
		class="w-20 min-w-fit px-4 py-2 rounded-md bg-green-600 text-sm upper font-medium text-white hover:bg-green-700">{{
				label
		}}</button>
</template>

<script setup lang="ts">
defineProps<{
	label: string,
}>();
</script>