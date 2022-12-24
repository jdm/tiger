<template>
	<div class="px-2 py-1 shadow-sm shadow-black/25 rounded-sm text-xs bg-zinc-200">
		{{ text }}
	</div>
</template>

<script setup lang="ts">
defineProps<{
	text: string
}>();
</script>
