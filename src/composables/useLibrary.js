import { ref } from 'vue';


export function useLibrary() {
  const musicPath = ref(localStorage.getItem('musicPath') || '');

  async function setMusicFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });
      
      if (selected) {
        musicPath.value = selected;
        localStorage.setItem('musicPath', selected);
        return selected;
      }
    } catch (err) {
      console.error('Error selecting folder:', err);
    }
    return null;
  }

  return {
    musicPath,
    setMusicFolder,
  };
}
