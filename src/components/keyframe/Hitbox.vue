<template>
	<div ref="el">
		<Selectable @click.stop="onHitboxClicked" @contextmenu.stop.prevent="onOpenContextMenu"
			:selected="hitbox.selected" :text="hitbox.name" :left-icon="TagIcon" :actions="renaming ? [] :
			[
				{ icon: PencilSquareIcon, callback: onRenameClicked },
				{ icon: XMarkIcon, callback: onDeleteClicked }
			]">
			<template #content v-if="renaming">
				<InputRename :original-name="hitbox.name" @complete-rename="onRenameInputComplete"
					@cancel-rename="cancelRename" />
			</template>
		</Selectable>
		<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from "vue"
import { PencilSquareIcon, TagIcon, XMarkIcon } from "@heroicons/vue/20/solid"
import { beginRenameHitbox, cancelRename, copy, cut, deleteHitbox, deleteSelectedHitboxes, endRenameHitbox, selectHitbox } from "@/backend/api"
import { Hitbox as HitboxDTO } from "@/backend/dto"
import { useStateStore } from "@/stores/state"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import Selectable from "@/components/basic/Selectable.vue"
import InputRename from "@/components/basic/InputRename.vue"

const props = defineProps<{
	hitbox: HitboxDTO
}>();

defineExpose({
	getHitbox: () => props.hitbox
});

const el: Ref<HTMLElement | null> = ref(null);
const contextMenu: Ref<typeof ContextMenu | null> = ref(null);

const contextMenuEntries = [
	{ name: "Cut", shortcut: "Ctrl+X", action: cut },
	{ name: "Copy", shortcut: "Ctrl+C", action: copy },
	{},
	{ name: "Delete", shortcut: "Del", action: deleteSelectedHitboxes },
];

const state = useStateStore();
const renaming = computed(() => state.currentDocument?.hitboxBeingRenamed == props.hitbox.name);

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
	beginRenameHitbox(props.hitbox.name);
}

function onRenameInputComplete(newName: string) {
	endRenameHitbox(newName);
}

function onDeleteClicked() {
	deleteHitbox(props.hitbox.name);
}
</script>
