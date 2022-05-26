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
import MenuBar from '@/components/basic/MenuBar.vue';
import WindowTitleBar from '@/components/basic/WindowTitleBar.vue';
import { useAppStore } from '@/stores/app'

const props = defineProps<{ debugMode: boolean, }>();
const emit = defineEmits(["update:debugMode"]);

const app = useAppStore();

function onToggleDevTools() {
	emit("update:debugMode", !props.debugMode);
}

const menuEntries = [
	{
		name: "File", content: [
			{ name: "New Spritesheet…", shortcut: "Ctrl+N" },
			{ name: "Open Spritesheet…", shortcut: "Ctrl+O" },
			{},
			{ name: "Save", shortcut: "Ctrl+S" },
			{ name: "Save As…", shortcut: "Ctrl+Shift+S" },
			{ name: "Save All", shortcut: "Ctrl+Alt+S" },
			{ name: "Export", shortcut: "Ctrl+E" },
			{ name: "Export As…", shortcut: "Ctrl+Shift+E" },
			{},
			{ name: "Close", shortcut: "Ctrl+W" },
			{ name: "Close All", shortcut: "Ctrl+Shift+W" },
		]
	},
	{
		name: "Edit", content: [
			{ name: "Undo", shortcut: "Ctrl+Z" },
			{ name: "Redo", shortcut: "Ctrl+Shift+Z" },
		]
	},
	{
		name: "View", content: [
			{ name: "Center Workbench", shortcut: "Ctrl+Space" },
			{ name: "Zoom In (Workbench)", shortcut: "Ctrl++" },
			{ name: "Zoom Out (Workbench)", shortcut: "Ctrl+-" },
			{ name: "Reset Zoom (Workbench)", shortcut: "Ctrl+0" },
			{},
			{ name: "Zoom In (Timeline)", shortcut: "Ctrl+Alt++" },
			{ name: "Zoom Out (Timeline)", shortcut: "Ctrl+Alt+-" },
			{ name: "Reset Zoom (Timeline)", shortcut: "Ctrl+Alt+0" },
		]
	}
];

</script>