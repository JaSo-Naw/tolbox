import {getOrInitSetting, setSetting} from "@/db/index.js";

export const getOrInitAutoStart = async () => {
   return await getOrInitSetting('autoStart', false)
}

export const setAutoStart = async (value) => {
    await setSetting('autoStart', value)
}