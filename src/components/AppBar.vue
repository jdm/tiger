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
import { computed, reactive } from "vue"
import { useAppStore } from "@/stores/app"
import { closeAllDocuments, closeCurrentDocument, openDocuments as doOpenDocuments, revealInExplorer, saveAll } from "@/api/app"
import { beginExportAs, doExport, centerWorkbench, redo, resetTimelineZoom, resetWorkbenchZoom, save, undo, zoomInTimeline, zoomInWorkbench, zoomOutTimeline, zoomOutWorkbench, copy, paste, cut } from "@/api/document"
import { newDocument, openDocuments, saveAs } from "@/api/local"
import MenuBar, { MenuBarEntry, MenuEntry, Separator } from "@/components/basic/MenuBar.vue"
import WindowTitleBar from "@/components/basic/WindowTitleBar.vue"

const props = defineProps<{ debugMode: boolean, }>();
const emit = defineEmits(["update:debugMode"]);

const app = useAppStore();

function onToggleDevTools() {
	emit("update:debugMode", !props.debugMode);
}

const fileMenuEntries = computed((): (MenuEntry|Separator)[] => reactive([
	{ name: "New Spritesheet…", shortcut: "Ctrl+N", action: newDocument },
	{ name: "Open Spritesheet…", shortcut: "Ctrl+O", action: openDocuments },
	{ name: "Open Recent", submenus: app.recentDocumentPaths.map(d => {
		return {
			key: d.path,
			name: d.name,
			action: () => doOpenDocuments([d.path]),
		}}
	)},
	{},
	{ name: "Save", shortcut: "Ctrl+S", action: save, disabled: !app.currentDocument },
	{ name: "Save As…", shortcut: "Ctrl+Shift+S", action: saveAs, disabled: !app.currentDocument },
	{ name: "Save All", shortcut: "Ctrl+Alt+S", action: saveAll, disabled: !app.currentDocument },
	{ name: "Export", shortcut: "Ctrl+E", action: doExport, disabled: !app.currentDocument },
	{ name: "Export As…", shortcut: "Ctrl+Shift+E", action: beginExportAs, disabled: !app.currentDocument },
	{},
	{ name: "Reveal in Explorer", action: () => {
		if (app.currentDocumentPath) {
			revealInExplorer(app.currentDocumentPath);
		}
	}, disabled: !app.currentDocument },
	{},
	{ name: "Close", shortcut: "Ctrl+W", action: closeCurrentDocument, disabled: !app.currentDocument },
	{ name: "Close All", shortcut: "Ctrl+Shift+W", action: closeAllDocuments, disabled: !app.documents.length },
]));

const editMenuEntries = computed((): (MenuEntry|Separator)[] => reactive([
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
	{},
	{ name: "Cut", shortcut: "Ctrl+X", action: cut, disabled: !app.canCut },
	{ name: "Copy", shortcut: "Ctrl+C", action: copy, disabled: !app.canCopy },
	{ name: "Paste", shortcut: "Ctrl+V", action: paste, disabled: !app.canPaste },
]));

const viewMenuEntries = computed((): (MenuEntry|Separator)[] => reactive([
	{ name: "Center Workbench", shortcut: "Ctrl+Space", action: centerWorkbench, disabled: !app.currentDocument },
	{ name: "Zoom In (Workbench)", shortcut: "Ctrl++", action: zoomInWorkbench, disabled: !app.currentDocument },
	{ name: "Zoom Out (Workbench)", shortcut: "Ctrl+-", action: zoomOutWorkbench, disabled: !app.currentDocument },
	{ name: "Reset Zoom (Workbench)", shortcut: "Ctrl+0", action: resetWorkbenchZoom, disabled: !app.currentDocument },
	{},
	{ name: "Zoom In (Timeline)", shortcut: "Ctrl+Alt++", action: zoomInTimeline, disabled: !app.currentDocument },
	{ name: "Zoom Out (Timeline)", shortcut: "Ctrl+Alt+-", action: zoomOutTimeline, disabled: !app.currentDocument },
	{ name: "Reset Zoom (Timeline)", shortcut: "Ctrl+Alt+0", action: resetTimelineZoom, disabled: !app.currentDocument },
]));

const menuEntries = computed((): MenuBarEntry[] => {
	return reactive([
		{ name: "File", content: fileMenuEntries },
		{ name: "Edit", content: editMenuEntries },
		{ name: "View", content: viewMenuEntries }
	]);
});

</script>