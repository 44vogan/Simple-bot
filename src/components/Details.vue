<script setup>
	import { ref } from "vue";
	import { invoke } from "@tauri-apps/api/tauri";
	import { store } from "./store.js";
	import { onMounted } from "vue";
	import { resourceDir } from "@tauri-apps/api/path";
	import {
		BaseDirectory,
		writeTextFile,
		readTextFile,
	} from "@tauri-apps/api/fs";
	//varibles
	//varibles
	const startButton = ref("开始 Bot");
	const savedRules = ref([
		{
			matchPoint: [0, 0],
			matchColor: [24, 36, 61],
			actionType: "左键点击",
			actionPoint: [0, 0],
			actionKey: "q",
			note: "备注一下",
		},
	]);
	const newRule = ref({
		matchPoint: [0, 0],
		matchColor: [24, 36, 61],
		actionType: "左键点击",
		actionPoint: [0, 0],
		actionKey: "q",
		note: "",
	});
	const showNewRule = ref(false);

	//functions
	//functions
	function delete_this_rule(i) {
		//删除一条rule
		store.add_log(
			`删除一条匹配:x,y${savedRules.value[i].matchPoint},rgb${savedRules.value[i].matchColor}`
		);
		savedRules.value.splice(i, 1);
		//保存rules到本地文件
		let savedRulesString = JSON.stringify(savedRules.value);
		save_bot_settings(savedRulesString);
	}
	function add_rule() {
		//显示增加规则div
		showNewRule.value = !showNewRule.value;
		// store.add_log(`显示增加规则`);
	}
	function comfirm_new_rule() {
		//先验证数据是否可靠，特别是按键,不通过则log
		let aRule = newRule.value;
		savedRules.value.push(aRule);
		store.add_log(
			`增加规则x,y${newRule.value.matchPoint},rgb${newRule.value.matchColor}`
		);
		//把newRule归位，不然有bug，最后一条savedRule会更着变
		newRule.value = {
			matchPoint: [0, 0],
			matchColor: [24, 36, 61],
			actionType: "左键点击",
			actionPoint: [0, 0],
			actionKey: "q",
			note: "",
		};
		//保存rules到本地文件
		let savedRulesString = JSON.stringify(savedRules.value);
		save_bot_settings(savedRulesString);
	}
	function a_rule_to_str(aRule) {
		let returnStr =
			aRule.matchPoint.join(",,") +
			",," +
			aRule.matchColor.join(",,") +
			",," +
			aRule.actionType +
			",," +
			aRule.actionPoint.join(",,") +
			",," +
			aRule.actionKey.toLowerCase();
		return returnStr;
	}
	function start_or_stop() {
		//开始bot，把 savedRules 转换成 &str 发送到 rust，注意不要有空字符
		//或者停止bot
		if (startButton.value == "开始 Bot") {
			startButton.value = "停止 Bot";
			store.add_log(`!!!start bot...`);
			store.add_log(`Bot进行中......`);
			let rustRulesArray = savedRules.value;
			let arrayTemp = [];
			rustRulesArray.forEach((item, index) => {
				// console.log(`${index}::${a_rule_to_str(item)}`);
				// store.add_log(`${index}::${a_rule_to_str(item)}`);
				arrayTemp.push(a_rule_to_str(item));
			});
			let rustRulesStr = arrayTemp.join("\r\n"); //把规则转换成了rust能用的参数
			// store.add_log(`${rustRulesStr}`);
			let intervalTextToCommandsToActions = setInterval(async () => {
				if (startButton.value == "停止 Bot") {
					let botResult = await invoke("text_to_commands_to_actions", {
						text: rustRulesStr,
					});
					// store.add_log(`${botResult}`);
					if (botResult != "This worked!") {
						store.add_log(`Bot出错!!!!!!,请检查匹配规则!!!!!!`);
						startButton.value = "开始 Bot";
					}
				}
				// store.add_log(`指针坐标x:${cursorPoint[0]},y:${cursorPoint[1]}`);
				else {
					clearInterval(intervalTextToCommandsToActions);
					store.add_log("停止 Bot!!");
				}
			}, 100); //这里设置100ms检测一遍，可以根据自己的电脑配置来更改
		} else {
			startButton.value = "开始 Bot";
			// store.add_log(`stop bot!!!`);
		}
	}
	async function save_bot_settings(bs) {
		await writeTextFile("botSetting.conf", bs, {
			dir: BaseDirectory.Resource,
		});
	}
	async function get_local_settings() {
		const resourceDirPath = await resourceDir();
		console.log("resourceDirPath :\n", resourceDirPath);
		let botSetting = await readTextFile("botSetting.conf", {
			//有配置文件就载入配置，没有配置文件就新建文件
			dir: BaseDirectory.Resource,
		}).catch(async (e) => {
			// console.log("error", e);
			// store.add_log(`error:\n${e}`);
			let settingContent = JSON.stringify(newRule.value);
			store.add_log(`默认设置初始化`);
			save_bot_settings(settingContent); //保存到本地文件
			botSetting = settingContent;
			savedRules.value = JSON.parse(botSetting); //更新到vue中savedRules
		});
		savedRules.value = JSON.parse(botSetting); //更新到vue中savedRules
		// store.add_log(`botSetting:\n${botSetting}`);
	}
	//Lifecycle
	//Lifecycle
	onMounted(() => {
		store.add_log(`加载设置...`);
		get_local_settings();
		store.add_log(`加载设置完成...`);
	});
</script>

<template>
	<div id="Details">
		<div class="top-buttons">
			<button class="start-stop-btn" @click="start_or_stop()">
				{{ startButton }}
			</button>
		</div>
		<!-- saved rules -->
		<div class="rules-div">
			<div class="a-rule" v-for="(rule, index) in savedRules">
				<div>
					<div class="rule-match">
						<span>匹配:: </span>
						<span>坐标::x:{{ rule.matchPoint[0] }},</span>
						<span>y:{{ rule.matchPoint[1] }}</span>
						<span>,颜色:: </span>
						<span>r:{{ rule.matchColor[0] }},</span>
						<span>g:{{ rule.matchColor[1] }},</span>
						<span>b:{{ rule.matchColor[2] }}</span>
					</div>
					<div class="rule-action">
						<div>
							<span>动作:: </span>
							<span>{{ rule.actionType }}</span>
							<span class="click-point" v-if="rule.actionType.includes('点击')">
								坐标::x:{{ rule.actionPoint[0] }},y:{{ rule.actionPoint[1] }}
							</span>
							<span class="tap-char" v-if="rule.actionType.includes('按')">
								键:{{ rule.actionKey }}
							</span>
						</div>
					</div>
				</div>
				<div class="rule-note-delete">
					<p>{{ rule.note }}</p>
					<button @click="delete_this_rule(index)" title="删除">x</button>
				</div>
			</div>
		</div>
		<!-- add rule -->
		<div class="add-new-rule">
			<button v-if="!showNewRule" @click="add_rule()" title="添加规则">
				+
			</button>
			<div v-if="showNewRule" class="new-rule">
				<div class="new-rule-match">
					<span>匹配: </span>
					x:<input
						type="number"
						min="0"
						v-model="newRule.matchPoint[0]"
						title="匹配坐标的x值"
					/>
					y:<input
						type="number"
						min="0"
						v-model="newRule.matchPoint[1]"
						title="匹配坐标的y值"
					/>
					,r:<input
						type="number"
						min="0"
						max="255"
						v-model="newRule.matchColor[0]"
						title="用来匹配坐标rgb颜色的r值(0-255)"
					/>
					g:<input
						type="number"
						min="0"
						max="255"
						v-model="newRule.matchColor[1]"
						title="用来匹配坐标rgb颜色的g值(0-255)"
					/>
					b:<input
						type="number"
						min="0"
						max="255"
						v-model="newRule.matchColor[2]"
						title="用来匹配坐标rgb颜色的b值(0-255)"
					/>
				</div>
				<div class="new-rule-action">
					<span>动作: </span>
					<select v-model="newRule.actionType">
						<option>触发按键</option>
						<option>左键点击</option>
						<option>右键点击</option>
					</select>
					<span class="click-point" v-if="newRule.actionType.includes('点击')">
						x:<input
							type="number"
							min="0"
							v-model="newRule.actionPoint[0]"
							title="点击的x坐标"
						/>
						y:<input
							type="number"
							min="0"
							v-model="newRule.actionPoint[1]"
							title="点击的y坐标"
						/>
					</span>
					<span class="tap-char" v-if="newRule.actionType.includes('按键')">
						键:<input
							type="text"
							size="1"
							maxlength="1"
							v-model="newRule.actionKey"
							title="输入单个按键(0-9,a-z,留空为q)"
						/>
					</span>
				</div>
				<div class="new-rule-comfirm">
					<button @click="comfirm_new_rule()" title="确认添加规则">确定</button>
					<span>
						备注:
						<input
							id="input-note"
							type="text"
							maxlength="12"
							v-model="newRule.note"
						/>
					</span>
					<button @click="showNewRule = !showNewRule" title="取消添加规则">
						取消
					</button>
				</div>
			</div>
		</div>
	</div>
</template>

<style scoped>
	#Details {
		padding: 5px;
		width: 600px;
		text-align: left;
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
	}
	.top-buttons {
		display: flex;
		justify-content: space-between;
		margin-bottom: 20px;
	}
	.start-stop-btn {
		background: #b71d1d;
	}
	.rules-div {
		margin-bottom: 5px;
	}
	.a-rule {
		display: flex;
		justify-content: space-between;
		margin-bottom: 10px;
		border-radius: 8px;
		padding: 5px;
		border: 1px solid transparent;
		font-family: inherit;
		color: #ffffff;
		background-color: #0f0f0f98;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}
	.a-rule div {
		padding: 2px;
	}
	.a-rule button {
		font-size: xx-small;
	}
	.rule-match {
		display: flex;
		flex-direction: row;
		justify-content: flex-start;
		align-items: baseline;
		font-size: 16px;
	}
	.rule-action {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: baseline;
		font-size: 16px;
	}
	.rule-note-delete {
		display: flex;
		flex-direction: row;
		justify-content: flex-start;
		align-items: center;
	}
	.rule-note-delete p {
		margin-right: 5px;
		border-radius: 8px;
		padding: 5px;
		border: 1px solid transparent;
		font-family: inherit;
		color: #ffffff;
		/* background-color: #20202098; */
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}
	.new-rule {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		margin-bottom: 10px;
		border-radius: 8px;
		padding: 5px;
		border: 1px solid transparent;
		font-family: inherit;
		color: #ffffff;
		background-color: #20202098;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}
	.new-rule div {
		margin: 2px 5px;
	}
	.new-rule-comfirm {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
	}
	#input-note {
		width: 150px;
	}
</style>
