import { defineStore } from 'pinia';
import { Player, Settings } from '../types';
import { invoke } from '@tauri-apps/api/core';

export const useStore = defineStore('app', {
  state: () => ({
    settings: {} as Settings,
    loading: false,
    players: [] as Player[],
    party_players: [] as Player[],
    update_url: ""
  }),

  actions: {
    async get_settings(){
        const fetched_settings = await invoke<Settings>("get_settings");
        console.log(fetched_settings);
        this.$patch({
            settings: fetched_settings,
        })
      }
  }

});