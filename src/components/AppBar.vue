<template>
	<WindowTitleBar>
		<template #left>
			<div class="h-full flex items-center flex-row">
				<div class="pl-5 pr-3">🐯</div>
				<MenuBar :entries="menuEntries" class="h-full" />
			</div>
		</template>
		<template #right>
			<div>
				<div v-if="!app.isReleaseBuild" class="h-full inline-flex items-center">
					<button @click="onToggleDevTools" class="p-1 px-2 rounded-md text-white"
						:class="debugMode ? 'bg-green-500' : 'bg-red-500'">🐛</button>
				</div>
			</div>
		</template>
	</WindowTitleBar>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useAppStore } from "@/stores/app"
import { closeAllDocuments, closeCurrentDocument, saveAll } from "@/api/app"
import { beginExportAs, doExport, centerWorkbench, redo, resetTimelineZoom, resetWorkbenchZoom, save, undo, zoomInTimeline, zoomInWorkbench, zoomOutTimeline, zoomOutWorkbench } from "@/api/document"
import { newDocument, openDocuments, saveAs } from "@/api/local"
import MenuBar from "@/components/basic/MenuBar.vue"
import WindowTitleBar from "@/components/basic/WindowTitleBar.vue"

const props = defineProps<{ debugMode: boolean, }>();
const emit = defineEmits(["update:debugMode"]);

const app = useAppStore();

function onToggleDevTools() {
	emit("update:debugMode", !props.debugMode);
}

const fileMenuEntries = [
	{ name: "New Spritesheet…", shortcut: "Ctrl+N", action: newDocument },
	{ name: "Open Spritesheet…", shortcut: "Ctrl+O", action: openDocuments },
	{},
	{ name: "Save", shortcut: "Ctrl+S", action: save },
	{ name: "Save As…", shortcut: "Ctrl+Shift+S", action: saveAs },
	{ name: "Save All", shortcut: "Ctrl+Alt+S", action: saveAll },
	{ name: "Export", shortcut: "Ctrl+E", action: doExport },
	{ name: "Export As…", shortcut: "Ctrl+Shift+E", action: beginExportAs },
	{},
	{ name: "Close", shortcut: "Ctrl+W", action: closeCurrentDocument },
	{ name: "Close All", shortcut: "Ctrl+Shift+W", action: closeAllDocuments },
];

const editMenuEntries = computed(() => {
	return [
		{
			name: `Undo ${app.currentDocument?.undoEffect || ''}`,
			shortcut: "Ctrl+Z", action: undo,
			disabled: app.currentDocument?.undoEffect == null
		},
		{
			name:
				`Redo ${app.currentDocument?.redoEffect || ''}`,
			shortcut: "Ctrl+Shift+Z", action: redo,
			disabled: app.currentDocument?.redoEffect == null
		},
	];
});

const viewMenuEntries = [
	{ name: "Center Workbench", shortcut: "Ctrl+Space", action: centerWorkbench },
	{ name: "Zoom In (Workbench)", shortcut: "Ctrl++", action: zoomInWorkbench },
	{ name: "Zoom Out (Workbench)", shortcut: "Ctrl+-", action: zoomOutWorkbench },
	{ name: "Reset Zoom (Workbench)", shortcut: "Ctrl+0", action: resetWorkbenchZoom },
	{},
	{ name: "Zoom In (Timeline)", shortcut: "Ctrl+Alt++", action: zoomInTimeline },
	{ name: "Zoom Out (Timeline)", shortcut: "Ctrl+Alt+-", action: zoomOutTimeline },
	{ name: "Reset Zoom (Timeline)", shortcut: "Ctrl+Alt+0", action: resetTimelineZoom },
];

const menuEntries = computed(() => {
	return [
		{ name: "File", content: fileMenuEntries },
		{ name: "Edit", content: editMenuEntries.value },
		{ name: "View", content: viewMenuEntries }
	];
});

</script>