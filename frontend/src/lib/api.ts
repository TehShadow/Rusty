import axios from 'axios'

const api = axios.create({
  baseURL: 'http://localhost:4000', // 🔧 Change to your Rust backend URL if needed
})

api.interceptors.request.use((config) => {
  if (typeof window !== 'undefined') {
    const token = localStorage.getItem('token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
  }
  return config
})

export default api
