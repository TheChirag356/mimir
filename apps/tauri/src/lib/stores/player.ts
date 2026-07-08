import { writable, derived, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { auth } from "./auth";

export interface PlayerItem {
  id: string;
  title: string;
  author: string;
  duration_seconds: number;
  audio_file_id: string;
  library_id: string;
}

interface PlayerState {
  item: PlayerItem | null;
  sessionId: string | null;
  currentTime: number;
  duration: number;
  isPlaying: boolean;
  speed: number;
  volume: number;
}

function createPlayerStore() {
  const { subscribe, set, update } = writable<PlayerState>({
    item: null,
    sessionId: null,
    currentTime: 0,
    duration: 0,
    isPlaying: false,
    speed: 1,
    volume: 1,
  });

  let audio: HTMLAudioElement | null = null;
  let syncInterval: ReturnType<typeof setInterval> | null = null;

  function startSyncInterval(sessionId: string) {
    if (syncInterval) clearInterval(syncInterval);
    syncInterval = setInterval(async () => {
      const state = get({ subscribe });
      if (!state.isPlaying || !state.item) return;
      const $auth = get(auth);
      try {
        await invoke("sync_session", {
          serverUrl: $auth.serverUrl,
          token: $auth.token,
          sessionId,
          currentTimeSeconds: state.currentTime,
          timeListenedSeconds: 15,
        });
      } catch (e) {
        console.error("sync failed:", e);
      }
    }, 15000);
  }

  return {
    subscribe,

    play: async (item: PlayerItem) => {
      const $auth = get(auth);

      const session = await invoke<{ id: string; current_time_seconds: number }>(
        "start_session", {
          serverUrl: $auth.serverUrl,
          token: $auth.token,
          itemId: item.id,
          durationSeconds: item.duration_seconds,
        }
      );

      if (audio) {
        audio.pause();
        audio.src = "";
      }

      const streamUrl = `${$auth.serverUrl}/api/items/${item.id}/audio/${item.audio_file_id}/stream?token=${$auth.token}`;
      audio = new Audio(streamUrl);
      audio.currentTime = session.current_time_seconds;
      audio.playbackRate = get({ subscribe }).speed;

      audio.ontimeupdate = () => {
        update(s => ({ ...s, currentTime: audio!.currentTime }));
      };

      audio.onended = () => {
        update(s => ({ ...s, isPlaying: false }));
        if (syncInterval) clearInterval(syncInterval);
      };

      audio.onerror = (e) => {
        console.error("audio error:", e);
        update(s => ({ ...s, isPlaying: false }));
      };

      await audio.play();

      update(s => ({
        ...s,
        item,
        sessionId: session.id,
        currentTime: session.current_time_seconds,
        duration: item.duration_seconds,
        isPlaying: true,
      }));

      startSyncInterval(session.id);
    },

    pause: () => {
      audio?.pause();
      update(s => ({ ...s, isPlaying: false }));
    },

    resume: () => {
      audio?.play();
      update(s => ({ ...s, isPlaying: true }));
    },

    seek: (seconds: number) => {
      if (audio) audio.currentTime = seconds;
      update(s => ({ ...s, currentTime: seconds }));
    },

    skipBack: (seconds = 30) => {
      const state = get({ subscribe });
      const newTime = Math.max(0, state.currentTime - seconds);
      if (audio) audio.currentTime = newTime;
      update(s => ({ ...s, currentTime: newTime }));
    },

    skipForward: (seconds = 30) => {
      const state = get({ subscribe });
      const newTime = Math.min(state.duration, state.currentTime + seconds);
      if (audio) audio.currentTime = newTime;
      update(s => ({ ...s, currentTime: newTime }));
    },

    setSpeed: (speed: number) => {
      if (audio) audio.playbackRate = speed;
      update(s => ({ ...s, speed }));
    },

    setVolume: (volume: number) => {
      if (audio) audio.volume = volume;
      update(s => ({ ...s, volume }));
    },
  };
}

export const player = createPlayerStore();
export const isPlaying = derived(player, $p => $p.isPlaying);
export const currentItem = derived(player, $p => $p.item);
