<template>
	<div class="self-center flex flex-row bg-plastic-900 outline outline-1 outline-plastic-700 rounded-sm">
		<input v-model="value" class="w-full text-xs bg-transparent text-plastic-300 rounded-sm border-0 focus:ring-0"
			type="text" spellcheck="false" />
		<div v-if="unit" class="text-xs self-center px-2 text-plastic-500">{{ unit }}</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
	values: number[],
	unit?: string,
}>();

const emit = defineEmits<{
	(e: "update", newValue: number): void
}>();

const multipleValues = computed(() => {
	return !props.values.every(v => v == props.values[0])
});

const value = computed({
	get: () => (multipleValues.value ? "<?>" : props.values[0] || 0).toString(),
	set: (value) => {
		const newValue = Number(value);
		if (newValue != null) {
			emit("update", newValue);
		}
	},
});
</script>
