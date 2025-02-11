<template>
	<InputText v-model="value" :placeholder="placeholder">
		<template #after>
			<button @click="openFilePicker" @mousedown="onMouseDown" @mouseup="onMouseUp" @mouseenter="onMouseEnter"
				@mouseleave="onMouseLeave" :class="buttonClass" class="relative inline-flex items-center px-3 rounded-md
				cursor-pointer  bg-plastic-600 border border-l-0 border-t-plastic-500 border-b-plastic-900 border-r-plastic-800 overflow-clip
				outline-offset-2 active:outline-0">
				<div class="absolute h-full w-full left-0 top-0 scale-75 blur-md bg-plastic-500/50 mix-blend-screen"
					v-if="hovered" />
				<FolderIcon v-if="isDirectory" class="w-5" :class="iconClass" />
				<EllipsisHorizontalIcon v-if="!isDirectory" class="w-5" :class="iconClass" />
			</button>
		</template>
	</InputText>
</template>

<script setup lang="ts">
import { DialogFilter, open, save } from "@tauri-apps/api/dialog";
import { computed, ref, WritableComputedRef } from "vue";
import { EllipsisHorizontalIcon, FolderIcon } from "@heroicons/vue/20/solid"
import InputText from "@/components/basic/InputText.vue"

const props = defineProps<{
	isDirectory?: boolean,
	pickExisting?: boolean,
	placeholder?: string,
	filters?: DialogFilter[],
	modelValue: string,
}>();

const emit = defineEmits(["update:modelValue"])

const value: WritableComputedRef<string> = computed({
	get: () => props.modelValue,
	set: (value) => emit("update:modelValue", value),
});

const hovered = ref(false);
const active = ref(false);

const iconClass = computed(()=>[
	...hovered.value ? ["text-plastic-200"] : ["text-plastic-300"],
]);

const buttonClass = computed(() => [
	!active.value ? ["focus:outline-2", "focus:outline-blue-500", "focus:outline-dotted"] : ["focus:outline-0"],
]);

async function openFilePicker() {
	let file;
	const defaultPath = value.value || undefined;
	if (props.pickExisting || props.isDirectory) {
		file = await open({ directory: props.isDirectory, defaultPath: defaultPath, filters: props.filters });
	} else {
		file = await save({ defaultPath: defaultPath, filters: props.filters });
	}
	if (typeof file === "string") {
		value.value = file;
	}
}

function onMouseEnter() {
	hovered.value = true;
}

function onMouseLeave() {
	hovered.value = false;
	active.value = false;
}

function onMouseDown() {
	active.value = true;
}

function onMouseUp() {
	active.value = false;
}
</script>
