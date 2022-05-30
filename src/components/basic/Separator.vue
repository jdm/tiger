<template>
	<div class="">
		<div
			:class="vertical ? 'w-0 h-full border-x border-l-plastic-600 border-r-plastic-800' : 'h-0 w-full border-y border-t-plastic-600 border-b-plastic-800'" />
	</div>
</template>

<script setup lang="ts">
defineProps<{ vertical?: boolean }>();
</script>