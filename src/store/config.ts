import {defineStore} from 'pinia'
import {Config, Weather} from '../types'
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

export const useConfigStore = defineStore('config', ()=>{
    const config = ref<Config | null>();

    async function loadConfig(){
        config.value = await invoke('load_config');        
    }

    function setBasicConfig() {
        if (config.value) {
            document.body.style.fontFamily = config.value.font_family;
            //TODO:判断封面有没有，没有去空白页面
        }
    }
    async function updateWeather(city: string) {
        if (config.value) {
            const weather: Weather = await invoke('get_weather', {city})
            config.value.city = city;
            config.value.weather = weather
            await invoke('update_config', {config: config.value})
        }
    }

    async function updateCoverFilter() {
        if (config.value) {
            config.value.cover_filter = !config.value.cover_filter
            await invoke('update_config', {config: config.value})
        }
    }

    async function updateFontFamily(font: string) {
        if (config.value) {
            document.body.style.fontFamily = font;
            config.value.font_family = font;
            await invoke('update_config', {config: config.value})
        }
      }

    return {
        config,
        loadConfig,
        setBasicConfig,
        updateWeather,
        updateCoverFilter,
        updateFontFamily
    }
})