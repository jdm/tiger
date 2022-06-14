<template>
	<InputText v-model="value">
		<template #after>
			<span class="inline-flex items-center px-3 rounded-md
				bg-plastic-600
				border border-l-0 border-t-plastic-500 border-b-plastic-900 border-r-plastic-800">
				<FolderIcon v-if="isDirectory" class="w-5 h-5 text-plastic-300" />
				<DotsHorizontalIcon v-if="!isDirectory" class="w-5 h-5 text-plastic-300" />
			</span>
		</template>
	</InputText>
</template>


<script setup lang="ts">
import { computed } from "vue";
import { DotsHorizontalIcon, FolderIcon } from "@heroicons/vue/solid"
import InputText from "@/components/basic/InputText.vue"

const props = defineProps<{
	isDirectory?: boolean,
	modelValue: string,
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