<template>
	<div class="rounded-md overflow-hidden flex bg-plastic-800 border-y border-plastic-900 border-b-plastic-600">
		<input type="text" :placeholder="placeholder || 'Example Text'" v-model="value"
			class="w-full px-4 bg-transparent border-0 text-plastic-300 placeholder-plastic-500" />
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
	get() {
		return props.modelValue;
	},
	set(value) {
		emit("update:modelValue", value);
	}
});

</script>