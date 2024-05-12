// store.js
import { reactive } from 'vue'

export const store = reactive({
    // count: 0,
    logs: ["Automate the boring stuff", "version = 0.6.0"],
    // increment() {
    //     this.count++
    // },
    lang: "cn",
    botRules: [],
    add_log(log: string) {//新log
        this.logs.unshift(log);
        if (this.logs.length > 30) {//控制logs长度
            this.logs.pop();
        }

        //不重复log
        // if (this.logs[0] !== log) {
        //     this.logs.unshift(log);
        //     if (this.logs.length > 30) {//控制logs长度
        //         this.logs.pop();
        //     }
        // }
    }
})
