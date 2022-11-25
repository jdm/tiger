<template>
	<InputText v-model="value">
		<template #after>
			<span @click="openFilePicker" class="inline-flex items-center px-3 rounded-md
				cursor-pointer  bg-plastic-600
				border border-l-0 border-t-plastic-500 border-b-plastic-900 border-r-plastic-800">
				<FolderIcon v-if="isDirectory" class="w-5 h-5 text-plastic-300" />
				<EllipsisHorizontalIcon v-if="!isDirectory" class="w-5 h-5 text-plastic-300" />
			</span>
		</template>
	</InputText>
</template>

<script setup lang="ts">
import { open, save } from "@tauri-apps/api/dialog";
import { computed, WritableComputedRef } from "vue";
import { EllipsisHorizontalIcon, FolderIcon } from "@heroicons/vue/20/solid"
import InputText from "@/components/basic/InputText.vue"

const props = defineProps<{
	isDirectory?: boolean,
	pickExisting?: boolean,
	modelValue: string,
}>();

const emit = defineEmits(["update:modelValue"])

const value: WritableComputedRef<string> = computed({
	get: () => props.modelValue,
	set: (value) => emit("update:modelValue", value),
});

async function openFilePicker() {
	let file;
	if (props.pickExisting || props.isDirectory) {
		file = await open({ directory: props.isDirectory });
	} else {
		file = await save();
	}
	if (typeof file === "string") {
		value.value = file;
	}
}

</script>