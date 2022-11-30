<template>
	<div class="rounded-md overflow-hidden flex bg-plastic-800 border-y border-plastic-900 border-b-plastic-600">
		<slot name="before"></slot>
		<input type="text" spellcheck="false" :placeholder="placeholder || 'Example Text'" v-model="value"
			class="w-full bg-transparent border-0 text-plastic-300 placeholder-plastic-500 focus:ring-0" />
		<slot name="after"></slot>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
	modelValue: string,
	placeholder?: string,
}>();

const emit = defineEmits(["update:modelValue"])

const value = computed({
	get: () => props.modelValue,
	set: (value) => emit("update:modelValue", value),
});
</script>
