import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import { MainPage } from './pages/MainPage'
import { SettingsPage } from './pages/SettingsPage'
import { HistoryPage } from './pages/HistoryPage'
import { Layout } from './components/ui/Layout'
import './App.css'

function App() {
  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<MainPage />} />
          <Route path="/settings" element={<SettingsPage />} />
          <Route path="/history" element={<HistoryPage />} />
        </Routes>
      </Layout>
    </Router>
  )
}

export default App