<script setup>
	import { ref } from "vue";
	import { invoke } from "@tauri-apps/api/tauri";
	import { store } from "./store.js";
	//varibles
	const x = ref(0);
	const y = ref(0);
	const theSwitch = ref("off");
	//functiongs
	// async function greet() {
	// 	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	// 	greetMsg.value = await invoke("greet", { name: name.value });
	// }
	function turnOffSwitch() {
		theSwitch.value = "off";
	}
	async function get_cursor() {
		if (theSwitch.value != "get_cursor") {
			theSwitch.value = "get_cursor";
			store.add_log("!!开始获取指针坐标 / 秒...");
			let intervalGetCursor = setInterval(async () => {
				let cursorPoint = await invoke("get_cursor_position");
				store.add_log(`指针坐标x:${cursorPoint[0]},y:${cursorPoint[1]}`);
				if (theSwitch.value != "get_cursor") {
					clearInterval(intervalGetCursor);
					store.add_log("停止获取指针坐标!!");
				}
			}, 1000);
		} else {
			turnOffSwitch();
		}
	}
	function get_pixel_color() {
		if (theSwitch.value != "get_pixel_color") {
			theSwitch.value = "get_pixel_color";
			store.add_log(`!!开始获取坐标x:${x.value},y:${y.value}的颜色 / 秒...`);
			let intervalGetPointColor = setInterval(async () => {
				let pointColor = await invoke("get_pixel_color", {
					x: x.value,
					y: y.value,
				});
				store.add_log(
					`rgb颜色::${pointColor[0]},${pointColor[1]},${pointColor[2]}`
				);
				if (theSwitch.value != "get_pixel_color") {
					clearInterval(intervalGetPointColor);
					store.add_log("停止获取指坐标颜色!!");
				}
			}, 1000);
		} else {
			turnOffSwitch();
		}
	}
</script>

<template>
	<div id="Profiles">
		<button @click="get_cursor()" title="开始获取指针坐标/秒">
			<span v-if="theSwitch != 'get_cursor'">指针坐标</span>
			<span v-if="theSwitch == 'get_cursor'">停止</span>
		</button>
		<div class="getCursorColor">
			<div>
				<div>
					<span>x:</span>
					<input v-model="x" type="number" />
				</div>

				<div>
					<span>y:</span>
					<input v-model="y" type="number" />
				</div>
			</div>
			<button @click="get_pixel_color()" title="开始获取rgb颜色/秒">
				<span v-if="theSwitch != 'get_pixel_color'"
					>坐标({{ x }},{{ y }})的 rgb 颜色</span
				>
				<span v-if="theSwitch == 'get_pixel_color'">停止</span>
			</button>
		</div>
	</div>
</template>

<style scoped>
	#Profiles {
		padding: 5px;
		width: 300px;
		text-align: left;
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
	}
	button {
		width: 100%;
	}
	.getCursorColor {
		margin: 20px 0;
		border-radius: 8px;
		border: 1px solid transparent;
		font-size: 1em;
		font-weight: 500;
		font-family: inherit;
		color: #ffffff;
		background-color: #0f0f0f98;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}
	.getCursorColor div {
		padding: 0 5px;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
	}
	input {
		margin: 10px;
	}
</style>
