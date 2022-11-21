<template>
	<div>
		<InputLabel>{{ label }}</InputLabel>
		<div class="relative">
			<slot name="content" />
			<div class="absolute top-1/2 -translate-y-1/2 -translate-x-full pr-4">
				<slot name="error" />
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import InputLabel from "@/components/basic/InputLabel.vue"
defineProps<{
	label: string,
}>();
</script>