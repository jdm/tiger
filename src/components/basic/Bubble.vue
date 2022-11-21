<template>
	<div>
		<div class="relative max-w-md text-sm font-medium p-2 rounded-sm" :class="bubbleClasses">
			<slot />
			<div class="absolute -right-1 top-1/2 -translate-y-1/2 w-4 h-4 rotate-45" :class="arrowClasses" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
	error?: boolean,
}>();

const bubbleClasses = computed(() => [...palette.value,]);
const arrowClasses = computed(() => [...palette.value,]);

const palette = computed(() => {
	if (props.error) {
		return ["bg-red-600", "text-red-100"];
	}
	return ["bg-gray-300", "text-gray-900"];
});
</script>