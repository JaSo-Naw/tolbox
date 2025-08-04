

import {getCurrentWindow} from "@tauri-apps/api/window";
import {ElMessage} from "element-plus";
const appWindow = getCurrentWindow()
import { invoke } from "@tauri-apps/api/core";
import {getOrInitAutoStart, setAutoStart} from "@/db/modules/autostart.js";
import {clearSecretKey} from "@/db/modules/user.js";

export const mainNav = [
    { id: 'home', text: '首页', icon: 'home' },
    { id: 'chat', text: '聊天', icon: 'chat' },
    { id: 'tool', text: '工具', icon: 'tool' },
    { id: 'extend', text: '扩展', icon: 'extend'},
]

export const subNav = {
    chat: [
        { id: 'friends', text: '好友消息', icon: 'user', badge: 5 },
        { id: 'groups', text: '群聊', icon: 'users', badge: 12 },
        { id: 'devices', text: '设备', icon: 'device' }
    ],
    tool: [
        { id: 'translate', text: '翻译', icon: 'translate'},
        { id: 'tido', text: 'todo', icon: 'tido' },
        { id: 'ai', text: 'AI', icon: 'ai' },
        { id: 'codeFeeder', text: '文件夹转文件' ,icon: 'codeFeeder' },
    ],
    extend: [
        { id: 'getByBilibili', text: '获取b站视频', icon: 'getByBilibili'}
    ]
}

export const settingItem = [
    {
        id: 'system',
        text: '系统',
        submenu: [
            {
                id: 'shortcuts',
                text: '快捷键',
                action: () => {
                    //todo
                    ElMessage.error("快捷键未实现")
                }
            },
            {
                id: 'update',
                text: '检查更新',
                action: () => {
                    //todo
                    ElMessage.error("检查更新未实现")
                }
            },
            {
                id: 'autoStart',
                text: '自启动',
                action: async () => {
                    try {
                        const res1 = await getOrInitAutoStart()

                        if (res1 === 'true') {
                            //设置开机自启false
                            await setAutoStart(false)
                            await invoke('change_autostart',{open: false})
                        }else if (res1 === 'false') {
                            //设置开机自启true
                            await setAutoStart(true)
                            await invoke('change_autostart',{open: true})
                        }

                        const res2 = await getOrInitAutoStart()
                        ElMessage.success("当前开机自启状态:" + (res2 === 'true' ? "开启" : "关闭"))

                    } catch (error) {
                        console.log(error)
                        ElMessage.error("获取或修改开机自启状态失败")
                    }


                }
            },
        ]
    },
    {
        id: 'account',
        text: '账号',
        submenu: [
            {
                id: 'logout',
                text: '退出登录',
                action: async () => {
                    await clearSecretKey()
                    await invoke('create_login_window')
                    await appWindow.close()
                }
            },
        ]
    },
    {
        id: 'window',
        text: '程序',
        submenu: [
            {
                id: 'minimize',
                text: '最小化',
                action: async () => {
                    await appWindow.minimize()
                }
            },
            {
                id: 'alwaysOnTop',
                text: '窗口置顶',
                action: async () => {
                    const isAlwaysOnTop = await appWindow.isAlwaysOnTop()
                    await appWindow.setAlwaysOnTop(!isAlwaysOnTop)
                    ElMessage.success("当前窗口置顶状态：" + (isAlwaysOnTop ? "关闭" : "开启"))
                }
            },
            {
                id: 'close',
                text: '关闭',
                action: async () => {
                    await appWindow.close()
                }
            }
        ]
    },
    { type: 'divider' },
    {
        id: 'about',
        text: '关于',
        action: () => {
            //todo
            ElMessage.error("关于未实现")
        }
    }
]


