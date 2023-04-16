<script setup>
	import { ref } from "vue";
	import { invoke } from "@tauri-apps/api/tauri";
	import { store } from "./store.js";
	//varibles
	const greetMsg = ref("");
	const name = ref("");
	//functiongs
	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		greetMsg.value = await invoke("greet", { name: name.value });
	}
</script>

<template>
	<div id="Logs">
		<li v-for="(log, index) in store.logs">
			<span v-if="index == 0">=></span>{{ log }}
		</li>
	</div>
</template>

<style scoped>
	#Logs {
		padding: 15px;
		width: 300px;
		text-align: left;
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
		overflow: hidden;
		font-size: smaller;

		/* height: 92vh;
		border-radius: 8px;
		border: 1px solid transparent;
		background-color: #0f0f0f98;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2); */
	}
</style>
