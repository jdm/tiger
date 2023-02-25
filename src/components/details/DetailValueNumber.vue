<template>
	<DetailValueString :values="values" @update="onUpdate" :unit="unit" />
</template>

<script setup lang="ts">
import { computed } from "vue";
import DetailValueString from "@/components/details/DetailValueString.vue"

const props = defineProps<{
	values: number[],
	unit?: string,
}>();

const emit = defineEmits<{
	(e: "update", newValue: number): void
}>();

const values = computed(() => props.values.map(v => v.toString()));

function onUpdate(newValue: string) {
	const numberValue = Number(newValue);
	if (numberValue != null) {
		emit("update", numberValue);
	}
}
</script>
