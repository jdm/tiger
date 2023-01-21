<template>
	<WindowTitleBar>
		<template #left>
			<div class="h-full flex items-center">
				<div class="pl-5 pr-3">🐯</div>
				<MenuBar :entries="menuEntries" class="h-full" />
			</div>
		</template>
		<template #right>
			<div>
				<div v-if="!state.isReleaseBuild" class="h-full inline-flex items-center">
					<button @click="onToggleDevTools" tabindex="-1" class="p-1 px-2 rounded-md text-white"
						:class="dev.debugModeEnabled ? 'bg-green-500' : 'bg-red-500'">🐛</button>
				</div>
			</div>
		</template>
	</WindowTitleBar>
</template>

<script setup lang="ts">
import { computed, reactive } from "vue"
import { useStateStore } from "@/stores/state"
import { useDevStore } from "@/stores/dev"
import { closeAllDocuments, closeCurrentDocument, revealInExplorer, saveAll, beginExportAs, doExport, centerWorkbench, redo, resetTimelineZoom, resetWorkbenchZoom, save, undo, zoomInTimeline, zoomInWorkbench, zoomOutTimeline, zoomOutWorkbench, copy, paste, cut, newDocument, openDocument, openDocuments, saveAs } from "@/backend/api"
import MenuBar, { MenuBarEntry, MenuEntry, Separator } from "@/components/basic/MenuBar.vue"
import WindowTitleBar from "@/components/basic/WindowTitleBar.vue"

const dev = useDevStore();
const state = useStateStore();

function onToggleDevTools() {
	dev.toggleDebugModeEnabled();
}

const fileMenuEntries = computed((): (MenuEntry|Separator)[] => reactive([
	{ name: "New Spritesheet…", shortcut: "Ctrl+N", action: newDocument },
	{ name: "Open Spritesheet…", shortcut: "Ctrl+O", action: openDocuments },
	{ name: "Open Recent", submenus: state.recentDocumentPaths.map(d => {
		return {
			key: d.path,
			name: d.name,
			action: () => openDocument(d.path),
		}}
	)},
	{},
	{ name: "Save", shortcut: "Ctrl+S", action: save, disabled: !state.currentDocument },
	{ name: "Save As…", shortcut: "Ctrl+Shift+S", action: () => saveAs(state.currentDocumentPath), disabled: !state.currentDocument },
	{ name: "Save All", shortcut: "Ctrl+Alt+S", action: saveAll, disabled: !state.currentDocument },
	{ name: "Export", shortcut: "Ctrl+E", action: doExport, disabled: !state.currentDocument },
	{ name: "Export As…", shortcut: "Ctrl+Shift+E", action: beginExportAs, disabled: !state.currentDocument },
	{},
	{ name: "Reveal in Explorer", action: () => {
		if (state.currentDocumentPath) {
			revealInExplorer(state.currentDocumentPath);
		}
	}, disabled: !state.currentDocument },
	{},
	{ name: "Close", shortcut: "Ctrl+W", action: closeCurrentDocument, disabled: !state.currentDocument },
	{ name: "Close All", shortcut: "Ctrl+Shift+W", action: closeAllDocuments, disabled: !state.documents.length },
]));

const editMenuEntries = computed((): (MenuEntry|Separator)[] => reactive([
	{
		name: `Undo ${state.currentDocument?.undoEffect || ''}`,
		shortcut: "Ctrl+Z", action: undo,
		disabled: state.currentDocument?.undoEffect == null
	},
	{
		name:
			`Redo ${state.currentDocument?.redoEffect || ''}`,
		shortcut: "Ctrl+Shift+Z", action: redo,
		disabled: state.currentDocument?.redoEffect == null
	},
	{},
	{ name: "Cut", shortcut: "Ctrl+X", action: cut, disabled: !state.canCut },
	{ name: "Copy", shortcut: "Ctrl+C", action: copy, disabled: !state.canCopy },
	{ name: "Paste", shortcut: "Ctrl+V", action: paste, disabled: !state.canPaste },
]));

const viewMenuEntries = computed((): (MenuEntry|Separator)[] => reactive([
	{ name: "Center Workbench", shortcut: "Ctrl+Space", action: centerWorkbench, disabled: !state.currentDocument },
	{ name: "Zoom In (Workbench)", shortcut: "Ctrl++", action: zoomInWorkbench, disabled: !state.currentDocument },
	{ name: "Zoom Out (Workbench)", shortcut: "Ctrl+-", action: zoomOutWorkbench, disabled: !state.currentDocument },
	{ name: "Reset Zoom (Workbench)", shortcut: "Ctrl+0", action: resetWorkbenchZoom, disabled: !state.currentDocument },
	{},
	{ name: "Zoom In (Timeline)", shortcut: "Ctrl+Alt++", action: zoomInTimeline, disabled: !state.currentDocument },
	{ name: "Zoom Out (Timeline)", shortcut: "Ctrl+Alt+-", action: zoomOutTimeline, disabled: !state.currentDocument },
	{ name: "Reset Zoom (Timeline)", shortcut: "Ctrl+Alt+0", action: resetTimelineZoom, disabled: !state.currentDocument },
]));

const helpMenuEntries = computed((): (MenuEntry|Separator)[] => reactive([
	{ name: "Documentation…", url: "https://agersant.github.io/tiger" },
	{ name: "Report a Bug…", url: "https://github.com/agersant/tiger/issues/new" },
	{ name: "Share Ideas and Feedback…", url: "https://github.com/agersant/tiger/discussions/new/choose" },
]));

const menuEntries = computed((): MenuBarEntry[] => {
	return reactive([
		{ name: "File", content: fileMenuEntries },
		{ name: "Edit", content: editMenuEntries },
		{ name: "View", content: viewMenuEntries },
		{ name: "Help", content: helpMenuEntries }
	]);
});

</script>