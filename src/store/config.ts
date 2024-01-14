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
            const root = document.documentElement
            root.style.fontFamily = config.value.font_family;
            switch (config.value.ui_scale) {
                case 0:
                    root.style.fontSize = '5px';
                    break;
                case 1:
                    root.style.fontSize = '10px';
                    break;
                case 2:
                    root.style.fontSize = '15px';
                    break;
            }
            switch (config.value.ui_style) {
                case true:
                    root.style.setProperty('--primiary-color', 'rgb(40,44,52)')
                    root.style.setProperty('--secondry-color', 'rgb(33,37,43)')
                    root.style.setProperty('--letter-color', 'rgb(180,180,180)')
                    root.style.setProperty('--click-color', 'rgb(54,58,70)')
                    root.style.setProperty('--context-color', 'rgba(0, 0, 0, 0.5)')
                    break;
                case false:
                    root.style.setProperty('--primiary-color', 'rgb(247,247,247)')
                    root.style.setProperty('--secondry-color', 'rgb(255,255,255)')
                    root.style.setProperty('--letter-color', 'rgb(75,75,75)')
                    root.style.setProperty('--click-color', 'rgb(235,235,235)')
                    root.style.setProperty('--context-color', 'rgba(0, 0, 0, 0.1)')
                    break;
            }
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
            document.documentElement.style.fontFamily = font;
            config.value.font_family = font;
            await invoke('update_config', {config: config.value})
        }
      }

      async function updateUiSize() {
        if (config.value) {
            switch (config.value.ui_scale) {
                case 0:
                    config.value.ui_scale = 1;
                    break;
                case 1:
                    config.value.ui_scale = 2;
                    break;
                case 2:
                    config.value.ui_scale = 0;
                    break;
            }
            await invoke('resize_main_window', {uiScale: config.value.ui_scale})
            setBasicConfig()
            await invoke('update_config', {config: config.value})
        }
      }
      
      async function updateUiStyle() {
        if (config.value) {
            config.value.ui_style =!config.value.ui_style
            setBasicConfig()
            await invoke('update_config', {config: config.value})
        }
      }

    return {
        config,
        loadConfig,
        setBasicConfig,
        updateWeather,
        updateCoverFilter,
        updateFontFamily,
        updateUiSize,
        updateUiStyle
    }
})