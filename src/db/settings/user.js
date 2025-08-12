import {getOrInitSetting, getSetting, setSetting} from "@/db/settings/index.js";


export const getSecretKey = async () => {
    return await getOrInitSetting('secretKey', '')
}


export const getSecretKeyExpiry = async () => {
    return await getOrInitSetting('secretKeyExpiry', '')
}
export const setSecretKeyWithDefaultExpiry = async (sk) => {
    await setSetting('secretKey', sk)
    await setSetting('secretKeyExpiry', Date.now() + 3 * 24 * 60 * 60 * 1000)
}

export const clearSecretKey = async () => {
    await setSetting('secretKey', '')
    await setSetting('secretKeyExpiry', '')
}


export const checkSecretKeyExpiry = async () => {
    const sk = await getSecretKey()
    const ske = await getSecretKeyExpiry()

    if ( sk !== '' && ske !== '' && Date.now() > parseInt(ske)) {
        await clearSecretKey()
        return true //过期
    }
    return false
}