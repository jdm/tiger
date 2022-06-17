<template>
	<div ref="el" class="relative">
		<div @click="onOpen" class="rounded-md border-2 border-plastic-900">
			<div class="flex flex-row items-center space-x-2 py-1.5 pl-4 pr-2 rounded-md
				border-y  border-t-zinc-300 border-b-zinc-700 bg-zinc-400">
				<div class="flex-1 grid pr-8 mt-px ">
					<div v-for="option in options"
						class="col-start-1 row-start-1 whitespace-nowrap font-semibold text-plastic-800"
						:class="option == selectedOption ? 'visible' : 'invisible'">
						{{ option.name }}
					</div>
				</div>
				<ChevronDownIcon class="w-6 -mb-0.5 text-plastic-600" />
			</div>
		</div>
		<Transition>
			<div v-if="open" class="absolute mt-2 z-50 w-full shadow-lg shadow-black/30 origin-top">
				<div
					class="flex flex-col py-2 rounded-md mx-0.5 bg-zinc-900 border-2 border-zinc-700 outline outline-zinc-900">
					<div v-for="option in options" @click="onOptionSelected(option)"
						class="px-4 py-1 font-semibold text-zinc-400 hover:bg-blue-600 hover:text-blue-100">
						{{ option.name }}
					</div>
				</div>
			</div>
		</Transition>
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref, watch } from "vue"
import { ChevronDownIcon } from "@heroicons/vue/solid"

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

const selectedOption = computed(() => {
	return props.options.find((o) => o.value == props.selected);
});

function onOpen() {
	open.value = !open.value;
}

function onOptionSelected(option: SelectOption) {
	emit("selected", option);
	open.value = false;
}

function onClickedAnywhere(event: MouseEvent) {
	if (!el.value?.contains(event.target as HTMLElement)) {
		open.value = false;
	}
}

watch(open, (isOpen, wasOpen) => {
	if (isOpen && !wasOpen) {
		window.addEventListener("mousedown", onClickedAnywhere);
	}
	if (!isOpen && wasOpen) {
		window.removeEventListener("mousedown", onClickedAnywhere);
	}
});
</script>

<style scoped>
.v-leave-active {
	transition: 0.1s ease-in;
}

.v-leave-to,
.v-enter-from {
	opacity: 0;
}
</style>