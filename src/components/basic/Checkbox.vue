<template>
	<input type="checkbox" v-model="value" class="bg-plastic-800 border-plastic-500 focus:ring-offset-0 focus:ring-0">
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
	modelValue: boolean,
}>();

const emit = defineEmits<{
	(e: "update:modelValue", checked: boolean): void,
}>();

const value = computed({
	get: () => props.modelValue,
	set: (checked) => emit("update:modelValue", checked),
});
</script>
