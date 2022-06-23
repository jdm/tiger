<template>
	<div>
		<Selectable @click="(event) => onAnimationClicked(event)" @dblclick="onAnimationDoubleClicked"
			@contextmenu.prevent="onOpenContextMenu" :selected="animation.selected" :text="animation.name"
			left-icon="DocumentIcon" :actions="renaming ? [] :
			[
				{ icon: 'PencilAltIcon', callback: onRenameClicked },
				{ icon: 'XIcon', callback: onDeleteClicked }
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
import { Animation as AnimationDTO } from "@/api/dto"
import { deleteAnimation, deleteSelectedAnimations, editAnimation, renameAnimation, selectAnimation } from "@/api/document"
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
	{ name: "Delete", action: deleteSelectedAnimations },
];

function onOpenContextMenu(event: MouseEvent) {
	if (contextMenu.value) {
		if (!props.animation.selected) {
			selectAnimation(props.animation.name, event.shiftKey, event.ctrlKey);
		}
		contextMenu.value.open(event);
	}
}

function onAnimationClicked(event: MouseEvent) {
	selectAnimation(props.animation.name, event.shiftKey, event.ctrlKey);
}

function onAnimationDoubleClicked() {
	editAnimation(props.animation.name);
}

function onRenameClicked() {
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
