<template>
	<div ref="el" class="relative">
		<div @click="onOpen" class="h-11 rounded-md border-2 border-plastic-900">
			<div class="h-full flex items-center space-x-2 pl-4 pr-2 rounded-md
				border-y  border-t-zinc-300 border-b-zinc-700 bg-zinc-400">
				<div class="flex-1 grid pr-8">
					<div v-for="option in options"
						class="col-start-1 row-start-1 whitespace-nowrap font-semibold text-plastic-800"
						:class="option == selectedOption ? 'visible' : 'invisible'">
						{{ option.name }}
					</div>
				</div>
				<ChevronDownIcon class="w-5 -mb-0.5 text-plastic-600" />
			</div>
		</div>

		<FloatingWidget :open="open" :position="listPosition" @dismissed="close">
			<MenuBackground class="mt-2 shadow-black/30" :style="`width: ${listWidth}px`">
				<div class="flex flex-col">
					<div v-for="option in options" @click="onOptionSelected(option)"
						class="px-4 py-1 rounded-sm font-semibold text-zinc-400 hover:bg-blue-600 hover:text-blue-100">
						{{ option.name }}
					</div>
				</div>
			</MenuBackground>
		</FloatingWidget>
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref, watch } from "vue";
import { ChevronDownIcon } from "@heroicons/vue/20/solid";
import FloatingWidget from "@/components/basic/FloatingWidget.vue";
import MenuBackground from "@/components/basic/MenuBackground.vue";

export type SelectOption = {
	name: string,
	value: any,
};

const props = defineProps<{
	options: SelectOption[],
	selected: any,
}>();

const emit = defineEmits<{
	(e: "selected", option: SelectOption): void
}>();

const el: Ref<HTMLElement | null> = ref(null);
const open = ref(false);
const listPosition = ref([0, 0] as [number, number]);

const selectedOption = computed(() => {
	return props.options.find((o) => o.value == props.selected);
});

const listWidth = computed(() => {
	if (!el.value) {
		return 0;
	}
	return el.value.clientWidth;
});

function onOpen() {
	if (el.value) {
		const boundingBox = el.value.getBoundingClientRect();
		listPosition.value = [boundingBox.left, boundingBox.bottom];
	}
	open.value = !open.value;
}

function onOptionSelected(option: SelectOption) {
	emit("selected", option);
	open.value = false;
}

function close() {
	open.value = false;
}
</script>
