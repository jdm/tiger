<template>
	<div class="self-center flex flex-row bg-plastic-900 outline outline-1 outline-plastic-700 rounded-sm">
		<input ref="inputElement" v-model.lazy="value" @keydown.escape="onInputCancelled"
			class="w-full text-xs bg-transparent text-plastic-300 rounded-sm border-0 focus:ring-0" type="text"
			spellcheck="false" />
		<div v-if="unit" class="text-xs self-center px-2 text-plastic-500">{{ unit }}</div>
	</div>
</template>

<script setup lang="ts">
import { computed, ref, Ref } from "vue";

const props = defineProps<{
	values: number[],
	unit?: string,
}>();

const emit = defineEmits<{
	(e: "update", newValue: number): void
}>();

const inputElement: Ref<HTMLInputElement | null> = ref(null);

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

function onInputCancelled() {
	if (!inputElement.value) {
		return;
	}
	inputElement.value.value = value.value;
	inputElement.value.blur();
}
</script>
