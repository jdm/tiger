<template>
	<div class="pointer-events-none">
		<ScreenCover :visible="settings != null" />
		<Transition name="pane-slide" @after-leave="onHidden" @after-enter="onVisible">
			<div v-if="settings" class="absolute inset-0 pointer-events-auto ">
				<div class="w-full h-full flex justify-end">
					<div class="h-full w-[40rem] p-10 flex flex-col gap-16 bg-plastic-700">
						<div class="flex flex-col gap-4">
							<h1 class="text-plastic-200 text-xl">Output Files</h1>
							<InputField label="Texture File">
								<template #content>
									<InputPath v-model="textureFile" class="mt-1"
										placeholder="C:\ExampleGame\Assets\Sprites\Hero.png" />
								</template>
								<template #error>
									<Transition name="error-slide">
										<InputError v-if="introComplete && textureFile && validation?.textureFileError"
											:shortErrorText="shortErrorText(validation.textureFileError)" />
									</Transition>
								</template>
							</InputField>
							<InputField label="Metadata File">
								<template #content>
									<InputPath v-model="metadataFile" class="mt-1"
										placeholder="C:\ExampleGame\Assets\Sprites\Hero.json" />
								</template>
								<template #error>
									<Transition name="error-slide">
										<InputError
											v-if="introComplete && metadataFile && validation?.metadataFileError"
											:shortErrorText="shortErrorText(validation.metadataFileError)" />
									</Transition>
								</template>
							</InputField>
						</div>

						<div class="flex flex-col gap-4">
							<h1 class="text-plastic-200 text-xl">Metadata Format</h1>
							<InputField label="Metadata Template File">
								<template #content>
									<InputPath v-model="templateFile" class="mt-1"
										placeholder="C:\ExampleGame\Tooling\SpritesheetFormat.liquid" />
								</template>
								<template #error>
									<Transition name="error-slide">
										<InputError
											v-if="introComplete && templateFile && validation?.templateFileError"
											:shortErrorText="shortErrorText(validation.templateFileError)"
											:longErrorText="longErrorText(validation.templateFileError) || undefined" />
									</Transition>
								</template>
							</InputField>
							<InputField label="Metadata Root Directory">
								<template #content>
									<InputPath v-model="metadataRoot" :isDirectory="true" class="mt-1"
										placeholder="C:\ExampleGame" />
								</template>
								<template #error>
									<Transition name="error-slide">
										<InputError
											v-if="introComplete && metadataRoot && validation?.metadataPathsRootError"
											:shortErrorText="shortErrorText(validation.metadataPathsRootError)" />
									</Transition>
								</template>
							</InputField>
						</div>

						<div class="flex gap-4 justify-end">
							<Button label="Export" :positive="true" @click="endExportAs"
								:disabled="!validation?.validSettings" />
							<Button label="Cancel" @click="cancelExportAs" />
						</div>

						<div class="flex-1 flex flex-col justify-end">
							<div
								class="flex items-center rounded-md text-md p-4 text-amber-800 bg-amber-400 shadow-lg shadow-amber-600/25">
								<BookOpenIcon class="mr-4 w-8 h-8" />
								<div>
									Confused about these options? Check out the <a
										href="https://agersant.github.io/tiger/exporting.html" target="_blank"
										class="underline underline-offset-2">documentation</a>.
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</Transition>
	</div>
</template>


<script setup lang="ts">
import { computed, ref } from "vue"
import { BookOpenIcon } from "@heroicons/vue/24/outline"
import { ExportSettingsError } from "@/api/dto"
import { cancelExportAs, endExportAs, setExportMetadataFile, setExportMetadataPathsRoot, setExportTemplateFile, setExportTextureFile } from "@/api/document"
import { useAppStore } from "@/stores/app"
import Button from "@/components/basic/Button.vue"
import InputError from "@/components/basic/InputError.vue"
import InputField from "@/components/basic/InputField.vue"
import InputPath from "@/components/basic/InputPath.vue"
import ScreenCover from "@/components/basic/ScreenCover.vue"

const app = useAppStore();
const settings = computed(() => app.currentDocument?.exportSettingsBeingEdited);
const validation = computed(() => app.currentDocument?.exportSettingsValidation);

const textureFile = computed({
	get: () => settings.value?.textureFile || "",
	set: setExportTextureFile,
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

function shortErrorText(error: ExportSettingsError): string {
	switch (error) {
		case "ExpectedAbsolutePath": return "This path should be absolute, not relative.";
		case "ExpectedDirectory": return "This path should be a directory, not a file.";
		case "ExpectedFile": return "This path should be a file, not a directory.";
		case "FileNotFound": return "This file does not exist.";
	}
	if (error.templateParseError) {
		return "This template file has invalid syntax.";
	}
	return "Unknown Error";
}


function longErrorText(error: ExportSettingsError): string | null {
	switch (error) {
		case "ExpectedAbsolutePath": return null;
		case "ExpectedDirectory": return null;
		case "ExpectedFile": return null;
		case "FileNotFound": return null;
	}
	if (error.templateParseError) {
		return error.templateParseError;
	}
	return null;
}
</script>

<style>
.pane-slide-enter-active {
	transition-property: transform;
	transition: 0.3s ease-out;
}

.pane-slide-leave-active {
	transition-property: transform;
	transition: 0.2s ease-in;
}

.pane-slide-enter-from,
.pane-slide-leave-to {
	transform: translateX(100%);
}

.error-slide-enter-active {
	transition-property: opacity, transform;
	transition: 0.3s ease-out;
}

.error-slide-leave-active {
	transition-property: opacity, transform;
	transition: 0.2s ease-in;
}

.error-slide-enter-from,
.error-slide-leave-to {
	opacity: 0;
	transform: translateX(-10px);
}
</style>