// store.js
import { reactive } from 'vue'

export const store = reactive({
    // count: 0,
    logs: ["自动化无聊的部分,享受有趣的部分"],
    // increment() {
    //     this.count++
    // },
    botRules: [],
    add_log(log) {//新log
        this.logs.unshift(log);
        if (this.logs.length > 30) {//控制logs长度
            this.logs.pop();
        }
    }
})
