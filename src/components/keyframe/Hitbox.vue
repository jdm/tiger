<template>
	<div ref="el">
		<Selectable @click="(event) => onHitboxClicked(event)" @contextmenu.prevent="onOpenContextMenu"
			:selected="hitbox.selected" :text="hitbox.name" left-icon="TagIcon" :actions="renaming ? [] :
			[
				{ icon: 'PencilSquareIcon', callback: onRenameClicked },
				{ icon: 'XMarkIcon', callback: onDeleteClicked }
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
import { Hitbox as HitboxDTO } from "@/api/dto"
import { copy, cut, deleteHitbox, deleteSelectedHitboxes, renameHitbox, selectHitbox } from "@/api/document"
import { Ref, ref } from "@vue/reactivity"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import Selectable from "@/components/basic/Selectable.vue"
import InputRename from "@/components/basic/InputRename.vue"

const props = defineProps<{
	hitbox: HitboxDTO
}>();

const renaming = ref(false);
const newName = ref("");
const el: Ref<HTMLElement | null> = ref(null);
const contextMenu: Ref<typeof ContextMenu | null> = ref(null);

const contextMenuEntries = [
	{ name: "Cut", shortcut: "Ctrl+X", action: cut },
	{ name: "Copy", shortcut: "Ctrl+C", action: copy },
	{},
	{ name: "Delete", shortcut: "Del", action: deleteSelectedHitboxes },
];

function onOpenContextMenu(event: MouseEvent) {
	if (contextMenu.value) {
		if (!props.hitbox.selected) {
			selectHitbox(props.hitbox.name, event.shiftKey, event.ctrlKey);
		}
		contextMenu.value.show(event);
	}
}

function onHitboxClicked(event: MouseEvent) {
	selectHitbox(props.hitbox.name, event.shiftKey, event.ctrlKey);
}

function onRenameClicked() {
	newName.value = props.hitbox.name;
	renaming.value = true;
}

function onRenameInputComplete() {
	renameHitbox(props.hitbox.name, newName.value);
	renaming.value = false;
}

function onRenameInputCancelled() {
	renaming.value = false;
}

function onDeleteClicked() {
	deleteHitbox(props.hitbox.name);
}
</script>
