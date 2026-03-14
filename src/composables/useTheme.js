import { ref, watch } from "vue"

const temas = ["light", "dark", "cupcake", "winter", "night", "emerald", "forest", "dracula"]
const temaActual = ref(localStorage.getItem("tema") || "cupcake")

// Initialize theme
document.documentElement.setAttribute("data-theme", temaActual.value)

watch(temaActual, (nuevo) => {
  document.documentElement.setAttribute("data-theme", nuevo)
  localStorage.setItem("tema", nuevo)
})

export function useTheme() {
  return {
    temas,
    temaActual
  }
}