import {defineStore} from 'pinia'
import {Config} from '../types'
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

export const useConfigStore = defineStore('config', ()=>{
    const config = ref<Config | null>();

    async function loadConfig(){
        config.value = await invoke('load_config');
    }

    return {
        config,
        loadConfig
    }
})