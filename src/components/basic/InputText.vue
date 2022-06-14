<template>
	<div class="rounded-md overflow-hidden flex bg-plastic-800 border-y border-plastic-900 border-b-plastic-600">
		<input type="text" :placeholder="placeholder || 'Example Text'"
			class="w-full px-4 bg-transparent border-0 text-plastic-300 placeholder-plastic-500" />
		<slot name="after"></slot>
	</div>
</template>

<script setup lang="ts">

defineProps<{
	placeholder?: string,
}>();

</script>