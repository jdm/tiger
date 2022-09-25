<template>
	<InputText v-model="value">
		<template #before>
			<div class="inline-flex items-center pl-3 -mr-1">
				<SearchIcon class="w-5 text-plastic-400" />
			</div>
		</template>
		<template #after v-if="!isEmpty">
			<div class="inline-flex items-center">
				<XIcon @click="value = ''"
					class="w-7 mx-2 p-1.5 cursor-pointer text-plastic-300 rounded-lg hover:text-plastic-200 hover:bg-plastic-900 hover:visible" />
			</div>
		</template>
	</InputText>
</template>

<script setup lang="ts">
import { computed, WritableComputedRef } from "vue";
import { SearchIcon, XIcon } from "@heroicons/vue/solid"
import InputText from "@/components/basic/InputText.vue"

const props = defineProps<{
	modelValue: string,
}>();

const emit = defineEmits(["update:modelValue"])

const value: WritableComputedRef<string> = computed({
	get: () => props.modelValue,
	set: (value) => emit("update:modelValue", value),
});

const isEmpty = computed(() => value.value.length == 0);

</script>