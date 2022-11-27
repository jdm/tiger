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
				<InputRename v-model="newName" @complete-rename="onRenameInputComplete"
					@cancel-rename="onRenameInputCancelled" />
			</template>
		</Selectable>
		<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
	</div>
</template>

<script setup lang="ts">
import { FilmIcon, PencilSquareIcon, XMarkIcon } from "@heroicons/vue/20/solid"
import { Animation as AnimationDTO } from "@/api/dto"
import { copy, cut, deleteAnimation, deleteSelectedAnimations, editAnimation, renameAnimation, selectAnimation } from "@/api/document"
import { Ref, ref } from "@vue/reactivity"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import Selectable from "@/components/basic/Selectable.vue"
import InputRename from "@/components/basic/InputRename.vue"

const props = defineProps<{
	animation: AnimationDTO
}>();

const renaming = ref(false);
const newName = ref("");
const contextMenu: Ref<typeof ContextMenu | null> = ref(null);

const contextMenuEntries = [
	{ name: "Cut", shortcut: "Ctrl+X", action: cut },
	{ name: "Copy", shortcut: "Ctrl+C", action: copy },
	{},
	{ name: "Rename", action: beginRename },
	{ name: "Delete", shortcut: "Del", action: deleteSelectedAnimations },
];

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
	newName.value = props.animation.name;
	renaming.value = true;
}

function onRenameInputComplete() {
	renameAnimation(props.animation.name, newName.value);
	renaming.value = false;
}

function onRenameInputCancelled() {
	renaming.value = false;
}

function onDeleteClicked() {
	deleteAnimation(props.animation.name);
}
</script>
