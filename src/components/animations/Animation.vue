<template>
	<div>
		<Selectable @click.stop="onAnimationClicked" @dblclick.stop="onAnimationDoubleClicked"
			@contextmenu.stop.prevent="onOpenContextMenu" :selected="animation.selected" :text="animation.name"
			:left-icon="FilmIcon" :actions="renaming ? [] :
			[
				{ icon: PencilSquareIcon, callback: beginRename },
				{ icon: XMarkIcon, callback: onDeleteClicked }
			]">
			<template #content v-if="renaming">
				<InputRename :original-name="animation.name" @complete-rename="onRenameInputComplete"
					@cancel-rename="cancelRename" />
			</template>
		</Selectable>
		<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref, nextTick } from "vue"
import { FilmIcon, PencilSquareIcon, XMarkIcon } from "@heroicons/vue/20/solid"
import { beginRenameAnimation, cancelRename, copy, cut, deleteAnimation, deleteSelectedAnimations, editAnimation, endRenameAnimation, selectAnimation } from "@/backend/api"
import { Animation as AnimationDTO } from "@/backend/dto"
import { useStateStore } from "@/stores/state"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import Selectable from "@/components/basic/Selectable.vue"
import InputRename from "@/components/basic/InputRename.vue"

const props = defineProps<{
	animation: AnimationDTO
}>();

defineExpose({
	getAnimation: () => props.animation
});

const contextMenu: Ref<typeof ContextMenu | null> = ref(null);

const contextMenuEntries = [
	{ name: "Cut", shortcut: "Ctrl+X", action: cut },
	{ name: "Copy", shortcut: "Ctrl+C", action: copy },
	{},
	{ name: "Rename", action: beginRename },
	{ name: "Delete", shortcut: "Del", action: deleteSelectedAnimations },
];

const state = useStateStore();

const renaming = computed(() => state.currentDocument?.animationBeingRenamed == props.animation.name);

function onOpenContextMenu(event: MouseEvent) {
	if (contextMenu.value) {
		if (!props.animation.selected) {
			selectAnimation(props.animation.name, event.shiftKey, event.ctrlKey);
		}
		contextMenu.value.show(event);
	}
}

function onAnimationClicked(event: MouseEvent) {
	selectAnimation(props.animation.name, event.shiftKey, event.ctrlKey);
}

function onAnimationDoubleClicked() {
	editAnimation(props.animation.name);
}

function beginRename() {
	beginRenameAnimation(props.animation.name);
}

function onRenameInputComplete(newName: string) {
	endRenameAnimation(newName);
}

function onDeleteClicked() {
	deleteAnimation(props.animation.name);
}
</script>
