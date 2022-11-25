<template>
	<div @mouseenter="onMouseEnter" @mouseleave="onMouseLeave" @click="onClick" class="px-2">
		<div class="flex" :class="[highlighted && !disabled ? 'bg-blue-600 rounded-sm' : '']">
			<div class="flex grow justify-between pl-7 space-x-14 py-1.5 whitespace-nowrap">
				<div :class="disabled ? 'text-zinc-600' : highlighted ? 'text-blue-100' : 'text-zinc-400'">
					{{ entry.name }}
				</div>
				<div :class="highlighted && !disabled ? 'text-blue-400' : 'text-zinc-600'">
					{{ entry.shortcut }}
				</div>
			</div>
			<div class="w-5 mx-1 flex">
				<ChevronRightIcon v-if="entry.submenus" class="w-5 translate-x-px"
					:class="disabled ? 'text-zinc-600' : highlighted ? 'text-blue-300' : 'text-zinc-400'" />
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { ChevronRightIcon } from "@heroicons/vue/20/solid"
import { MenuEntry } from "@/components/basic/MenuBar.vue"

const props = defineProps<{
	entry: MenuEntry,
	highlighted: boolean,
}>();

const emit = defineEmits<{
	(e: 'executed'): void
	(e: 'hovered', element: HTMLElement): void
	(e: 'unhovered', element: HTMLElement): void
}>();

const disabled = computed(() => props.entry.disabled || (!props.entry.action && !props.entry.submenus?.length));

function onMouseEnter(event: MouseEvent) {
	emit("hovered", event.currentTarget as HTMLElement);
}

function onMouseLeave(event: MouseEvent) {
	emit("unhovered", event.currentTarget as HTMLElement);
}

function onClick() {
	if (props.entry.action && !disabled.value) {
		emit("executed");
		props.entry.action();
	}
}
</script>
