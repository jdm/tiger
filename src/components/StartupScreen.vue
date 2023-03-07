<template>
	<div class="flex h-full justify-center pt-40">
		<div class="flex flex-col gap-8">
			<div class="flex items-center justify-center py-6 bg-amber-400 rounded-md">
				<img src="src/logo.svg" class="h-20" />
			</div>
			<div class="flex items-stretch gap-8 text-plastic-200">
				<Pane class="w-72 p-8">
					<div class="font-semibold text-lg mb-6">Get started</div>
					<div class="flex-1 flex flex-col gap-2">
						<Button custom-color="blue" :icon="DocumentPlusIcon" label="New" @click="newDocument" />
						<Button positive :icon=" FolderOpenIcon" label="Open" @click="openDocuments" />
					</div>
				</Pane>
				<Pane class="w-72 p-8">
					<div class="font-semibold text-lg mb-6">Recently opened</div>
					<div v-if="!!state.recentDocumentPaths.length" class="flex-1 flex flex-col gap-1">
						<div v-for="document in state.recentDocumentPaths.slice(0, 3)"
							class="bg-plastic-800 p-3 px-4 text-plastic-200 hover:bg-blue-600 hover:text-blue-100 text-sm font-medium rounded-md cursor-pointer"
							@click="openDocument(document.path)">
							<div class="flex gap-2 ">
								<DocumentTextIcon class="shrink-0 w-5" />
								<div class="overflow-clip text-ellipsis">
									{{ document.name }}
								</div>
							</div>
						</div>
					</div>
					<div v-else class="flex-1 flex flex-col">
						<div class="flex gap-4 items-center">
							<LightBulbIcon class="shrink-0 h-10 w-10 p-2 rounded-full text-amber-900 bg-amber-300" />
							<div class="text-plastic-300 ">
								No recent spritesheets found. <a @click="newDocument"
									class="text-blue-500 cursor-pointer hover:underline underline-offset-2">Create
									one</a> to get started.
							</div>
						</div>
					</div>
				</Pane>
				<Pane class="w-72 p-8">
					<div class="font-semibold text-lg mb-6">Learning</div>
					<div class="flex flex-col gap-2">
						<a href="https://agersant.github.io/tiger/exporting.html" target="_blank" tabindex="-1"
							class="text-orange-500 cursor-pointer hover:underline">Game engine integration</a>
						<a href="https://agersant.github.io/tiger/shortcuts.html" target="_blank" tabindex="-1"
							class="text-orange-500 cursor-pointer hover:underline">Keyboard shortcuts</a>
					</div>
				</Pane>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { DocumentTextIcon, DocumentPlusIcon, FolderOpenIcon } from "@heroicons/vue/20/solid"
import { LightBulbIcon } from "@heroicons/vue/24/outline"
import {newDocument, openDocument, openDocuments } from "@/backend/api"
import Button from "@/components/basic/Button.vue"
import Pane from "@/components/basic/Pane.vue"
import { useStateStore } from "@/stores/state";

const state = useStateStore();
</script>
