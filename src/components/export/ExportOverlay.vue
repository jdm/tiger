<template>
	<div class="pointer-events-none">
		<ScreenCover :visible="settings != null" />
		<Transition name="pane-slide" @after-leave="onHidden" @after-enter="onVisible">
			<FocusTrap v-if="settings" class="absolute inset-0 pointer-events-auto" @escape="cancelExportAs">
				<div class="w-full h-full flex justify-end">
					<div class="h-full w-[40rem] p-10 flex flex-col gap-16 bg-plastic-700">
						<div class="flex flex-col gap-4">
							<h1 class="text-plastic-200 text-xl">Output Files</h1>
							<InputField label="Atlas Image File">
								<template #content>
									<InputPath v-model="atlasImageFile" class="mt-1"
										placeholder="C:\ExampleGame\Assets\Sprites\Hero.png"
										:filters="[{ name: 'Image', extensions: ['png'] }]" />
								</template>
								<template #error>
									<InputError v-if="validation && introComplete"
										:visible="!!atlasImageFile && !!validation?.atlasImageFileError"
										:shortErrorText="shortErrorText(validation.atlasImageFileError)" />
								</template>
							</InputField>
							<InputField label="Metadata File">
								<template #content>
									<InputPath v-model="metadataFile" class="mt-1"
										placeholder="C:\ExampleGame\Assets\Sprites\Hero.json"
										:filters="[{ name: 'Any', extensions: [] }]" />
								</template>
								<template #error>
									<InputError v-if="validation && introComplete"
										:visible="!!metadataFile && !!validation?.metadataFileError"
										:shortErrorText="shortErrorText(validation.metadataFileError)" />
								</template>
							</InputField>
						</div>

						<div class="flex flex-col gap-4">
							<h1 class="text-plastic-200 text-xl">Metadata Format</h1>
							<InputField label="Metadata Template File">
								<template #content>
									<InputPath v-model="templateFile" pick-existing class="mt-1"
										placeholder="C:\ExampleGame\Tooling\SpritesheetFormat.template" />
								</template>
								<template #error>
									<InputError v-if="validation && introComplete"
										:visible="!!templateFile && !!validation?.templateFileError"
										:shortErrorText="shortErrorText(validation.templateFileError)"
										:longErrorText="longErrorText(validation.templateFileError) || undefined" />
								</template>
							</InputField>
							<InputField label="Metadata Root Directory">
								<template #content>
									<InputPath v-model="metadataRoot" :isDirectory="true" class="mt-1"
										placeholder="C:\ExampleGame" />
								</template>
								<template #error>
									<InputError v-if="validation && introComplete"
										:visible="!!metadataRoot && !!validation?.metadataPathsRootError"
										:shortErrorText="shortErrorText(validation.metadataPathsRootError)" />
								</template>
							</InputField>
						</div>

						<div class="flex gap-4 justify-end">
							<Button label="Export" :positive="true" tabbable @click="endExportAs"
								:disabled="!validation?.validSettings" />
							<Button label="Cancel" tabbable @click="cancelExportAs" />
						</div>

						<div class="flex-1 flex flex-col justify-end">
							<div
								class="flex items-center rounded-md text-md p-4 text-amber-800 bg-amber-400 shadow-lg shadow-amber-600/25">
								<BookOpenIcon class="mr-4 w-8 h-8" />
								<div>
									Confused about these options? Check out the <a
										href="https://agersant.github.io/tiger/exporting.html" target="_blank"
										class="underline underline-offset-2" tabindex="-1">documentation</a>.
								</div>
							</div>
						</div>
					</div>
				</div>
			</FocusTrap>
		</Transition>
	</div>
</template>


<script setup lang="ts">
import { computed, ref } from "vue"
import { BookOpenIcon } from "@heroicons/vue/24/outline"
import { cancelExportAs, endExportAs, setExportMetadataFile, setExportMetadataPathsRoot, setExportTemplateFile, setExportAtlasImageFile } from "@/backend/api"
import { ExportSettingsError } from "@/backend/dto"
import { useStateStore } from "@/stores/state"
import Button from "@/components/basic/Button.vue"
import FocusTrap from "@/components/basic/FocusTrap.vue"
import InputError from "@/components/basic/InputError.vue"
import InputField from "@/components/basic/InputField.vue"
import InputPath from "@/components/basic/InputPath.vue"
import ScreenCover from "@/components/basic/ScreenCover.vue"

const state = useStateStore();
const settings = computed(() => state.currentDocument?.exportSettingsBeingEdited);
const validation = computed(() => state.currentDocument?.exportSettingsValidation);

const atlasImageFile = computed({
	get: () => settings.value?.atlasImageFile || "",
	set: setExportAtlasImageFile,
});

const metadataFile = computed({
	get: () => settings.value?.metadataFile || "",
	set: setExportMetadataFile,
});

const templateFile = computed({
	get: () => settings.value?.templateFile || "",
	set: setExportTemplateFile,
});

const metadataRoot = computed({
	get: () => settings.value?.metadataPathsRoot || "",
	set: setExportMetadataPathsRoot,
});

const introComplete = ref(false);

function onHidden() {
	introComplete.value = false;
}

function onVisible() {
	introComplete.value = true;
}

function shortErrorText(error: ExportSettingsError | null): string {
	if (!error) {
		return "";
	}
	switch (error) {
		case "ExpectedAbsolutePath": return "This path should be absolute, not relative.";
		case "ExpectedDirectory": return "This path should be a directory, not a file.";
		case "ExpectedFile": return "This path should be a file, not a directory.";
		case "FileNotFound": return "This file does not exist.";
	}
	if (error.templateError) {
		return "This template file has invalid syntax.";
	}
	return "Unknown Error";
}


function longErrorText(error: ExportSettingsError | null): string | null {
	if (!error) {
		return null;
	}
	switch (error) {
		case "ExpectedAbsolutePath": return null;
		case "ExpectedDirectory": return null;
		case "ExpectedFile": return null;
		case "FileNotFound": return null;
	}
	if (error.templateError) {
		return error.templateError;
	}
	return null;
}
</script>

<style>
.pane-slide-enter-active {
	transition-property: transform;
	transition: 0.2s ease-out;
}

.pane-slide-leave-active {
	transition-property: transform;
	transition: 0.1s ease-in;
}

.pane-slide-enter-from,
.pane-slide-leave-to {
	transform: translateX(100%);
}
</style>
