<template>
	<Pane>
		<div class="flex flex-row bg-plastic-900">
			<PaneTab :selected="true">Hitboxes</PaneTab>
		</div>
		<div class="flex-1 flex flex-col min-h-0 p-4 space-y-4">
			<div class="w-full flex flex-row space-x-2 items-center">
				<Toggle :toggled="!!app.currentDocument?.lockHitboxes" @toggled="onToggleLockHitboxes"
					:icon="LockClosedIcon" />
				<div class="flex-1 flex flex-row justify-end">
					<Button :positive="true" :icon="TagIcon" custom-color="pink" label="Add" @click="onAddClicked" />
				</div>
			</div>
			<PaneInset class="flex-1 min-h-0">
				<div class="p-4 overflow-y-auto h-full styled-scrollbars" @contextmenu.stop.prevent="onOpenContextMenu">
					<div class="flex flex-col">
						<Hitbox v-for="hitbox in app.currentKeyframe?.hitboxes" :hitbox="hitbox" :key="hitbox.key" />
					</div>
					<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
				</div>
			</PaneInset>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from "vue";
import { LockClosedIcon, TagIcon } from "@heroicons/vue/20/solid";
import { createHitbox, lockHitboxes, paste, unlockHitboxes } from "@/api/document";
import { ClipboardManifest } from "@/api/dto";
import { useAppStore } from "@/stores/app";
import Button from "@/components/basic/Button.vue"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import Hitbox from "@/components/keyframe/Hitbox.vue";
import Toggle from "@/components/basic/Toggle.vue"

const app = useAppStore();
const contextMenu: Ref<typeof ContextMenu | null> = ref(null);

const contextMenuEntries = computed(() => [
	{ name: "Paste", shortcut: "Ctrl+V", action: paste, disabled: app.clipboardManifest != ClipboardManifest.Hitboxes },
]);

function onToggleLockHitboxes(toggled: boolean) {
	if (toggled) {
		lockHitboxes();
	} else {
		unlockHitboxes();
	}
}

function onAddClicked() {
	createHitbox(null);
}

function onOpenContextMenu(event: MouseEvent) {
	if (contextMenu.value) {
		contextMenu.value.show(event);
	}
}
</script>