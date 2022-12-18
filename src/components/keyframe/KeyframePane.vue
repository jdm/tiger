<template>
	<Pane>
		<div class="flex bg-plastic-900">
			<PaneTab :selected="true">Hitboxes</PaneTab>
		</div>
		<div class="flex-1 flex flex-col min-h-0 p-4 gap-4">
			<div class="w-full flex gap-2 items-center">
				<Toggle :toggled="!!app.currentDocument?.lockHitboxes" @toggled="onToggleLockHitboxes"
					:icon="LockClosedIcon" />
				<div class="flex-1 flex justify-end">
					<Button :positive="true" :icon="TagIcon" custom-color="pink" label="Add" @click="onAddClicked" />
				</div>
			</div>
			<PaneInset class="flex-1 min-h-0">
				<StatefulScroll ref="scrollableElement" v-model:scroll-top="scrollPosition"
					class="p-4 h-full styled-scrollbars" @click="clearSelection"
					@contextmenu.stop.prevent="onOpenContextMenu">
					<div class="flex flex-col">
						<Hitbox ref="hitboxElements" v-for="hitbox in app.currentKeyframe?.hitboxes" :hitbox="hitbox"
							:key="hitbox.key" />
					</div>
					<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
				</StatefulScroll>
			</PaneInset>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { computed, nextTick, Ref, ref, watch } from "vue";
import { LockClosedIcon, TagIcon } from "@heroicons/vue/20/solid";
import { clearSelection, createHitbox, lockHitboxes, paste, setHitboxesListOffset, unlockHitboxes } from "@/api/document";
import { ClipboardManifest } from "@/api/dto";
import { useAppStore } from "@/stores/app";
import Button from "@/components/basic/Button.vue"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import Hitbox from "@/components/keyframe/Hitbox.vue";
import StatefulScroll from "@/components/basic/StatefulScroll.vue"
import Toggle from "@/components/basic/Toggle.vue"

const app = useAppStore();
const contextMenu: Ref<typeof ContextMenu | null> = ref(null);
const scrollableElement: Ref<typeof StatefulScroll | null> = ref(null);
const hitboxElements: Ref<(typeof Hitbox)[]> = ref([]);

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

const scrollPosition =  computed({
	get: () => app.currentDocument?.hitboxesListOffset || 0,
	set: (offset) => setHitboxesListOffset(offset),
});

watch(() => app.currentDocument?.lastInteractedHitbox, (name) => {
	if (!name) {
		return;
	}
	nextTick(() => {
		const target = hitboxElements.value.find((el) => el.getHitbox().name == name);
		if (!target || !scrollableElement.value) {
			return;
		}
		scrollableElement.value.scrollToElement(target.$el);
	});
});

function onOpenContextMenu(event: MouseEvent) {
	if (contextMenu.value) {
		contextMenu.value.show(event);
	}
}
</script>