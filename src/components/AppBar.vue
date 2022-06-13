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
				<div v-if="!app.isReleaseBuild">
					<button @click="onToggleDevTools" class="p-2 px-4 text-white"
						:class="debugMode ? 'bg-green-500' : 'bg-red-500'">🐛</button>
				</div>
			</div>
		</template>
	</WindowTitleBar>
</template>

<script setup lang="ts">
import { closeAllDocuments, closeCurrentDocument } from "@/api/app"
import { beginExportAs, centerWorkbench, redo, resetTimelineZoom, resetWorkbenchZoom, save, undo, zoomInTimeline, zoomInWorkbench, zoomOutTimeline, zoomOutWorkbench } from "@/api/document"
import { newDocument, openDocuments } from "@/api/local"
import MenuBar from "@/components/basic/MenuBar.vue"
import WindowTitleBar from "@/components/basic/WindowTitleBar.vue"
import { useAppStore } from "@/stores/app"

const props = defineProps<{ debugMode: boolean, }>();
const emit = defineEmits(["update:debugMode"]);

const app = useAppStore();

function onToggleDevTools() {
	emit("update:debugMode", !props.debugMode);
}

const menuEntries = [
	{
		name: "File", content: [
			{ name: "New Spritesheet…", shortcut: "Ctrl+N", action: newDocument },
			{ name: "Open Spritesheet…", shortcut: "Ctrl+O", action: openDocuments },
			{},
			{ name: "Save", shortcut: "Ctrl+S", action: save },
			{ name: "Save As…", shortcut: "Ctrl+Shift+S" },
			{ name: "Save All", shortcut: "Ctrl+Alt+S" },
			{ name: "Export", shortcut: "Ctrl+E" },
			{ name: "Export As…", shortcut: "Ctrl+Shift+E", action: beginExportAs },
			{},
			{ name: "Close", shortcut: "Ctrl+W", action: closeCurrentDocument },
			{ name: "Close All", shortcut: "Ctrl+Shift+W", action: closeAllDocuments },
		]
	},
	{
		name: "Edit", content: [
			{ name: "Undo", shortcut: "Ctrl+Z", action: undo },
			{ name: "Redo", shortcut: "Ctrl+Shift+Z", action: redo },
		]
	},
	{
		name: "View", content: [
			{ name: "Center Workbench", shortcut: "Ctrl+Space", action: centerWorkbench },
			{ name: "Zoom In (Workbench)", shortcut: "Ctrl++", action: zoomInWorkbench },
			{ name: "Zoom Out (Workbench)", shortcut: "Ctrl+-", action: zoomOutWorkbench },
			{ name: "Reset Zoom (Workbench)", shortcut: "Ctrl+0", action: resetWorkbenchZoom },
			{},
			{ name: "Zoom In (Timeline)", shortcut: "Ctrl+Alt++", action: zoomInTimeline },
			{ name: "Zoom Out (Timeline)", shortcut: "Ctrl+Alt+-", action: zoomOutTimeline },
			{ name: "Reset Zoom (Timeline)", shortcut: "Ctrl+Alt+0", action: resetTimelineZoom },
		]
	}
];

</script>