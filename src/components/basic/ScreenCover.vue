<template>
	<Transition name="fade">
		<div v-if="visible == true" class="absolute inset-0 pointer-events-auto bg-black/70" />
	</Transition>
</template>

<script setup lang="ts">
defineProps<{
	visible?: boolean
}>();
</script>

<style scoped>
.fade-enter-active {
	transition: opacity 0.2s ease-out;
}

.fade-leave-active {
	transition: opacity 0.1s ease-in;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
}
</style>