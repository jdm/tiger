<template>
	<div class="flex flex-col">
		<div class="flex bg-zinc-900">
			<slot name="header"></slot>
		</div>
		<div class="flex-1 overflow-y-scroll bg-gray-800 text-white styled-scrollbars">
			<slot name="content"></slot>
		</div>
	</div>
</template>
